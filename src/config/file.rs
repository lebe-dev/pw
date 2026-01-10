use log::info;
use std::env;

use config::{Config, File};
use serde_json;

use super::model::{AppConfig, IpLimitEntry, IpLimitsConfig};
use super::validation::{format_validation_errors, validate_ip_limits_config};

pub fn load_config_from_file(file_path: &str) -> anyhow::Result<AppConfig> {
    info!("load config from file '{file_path}'");

    let config_builder = Config::builder()
        .add_source(
            config::Environment::with_prefix("PW")
                .try_parsing(true)
                .separator("_"),
        )
        .add_source(File::with_name(file_path));

    let settings = config_builder.build()?;

    let config = settings.clone().try_deserialize::<AppConfig>()?;

    let listen = get_env_var("PW_LISTEN").unwrap_or(config.listen.to_string());
    let log_level = get_env_var("PW_LOG_LEVEL").unwrap_or(config.log_level);
    let log_target = get_env_var("PW_LOG_TARGET").unwrap_or(config.log_target);
    let message_max_length =
        get_env_var("PW_MESSAGE_MAX_LENGTH").unwrap_or(config.message_max_length.to_string());
    let file_upload_enabled =
        get_env_var("PW_FILE_UPLOAD_ENABLED").unwrap_or(config.file_upload_enabled.to_string());
    let file_max_size = get_env_var("PW_FILE_MAX_SIZE").unwrap_or(config.file_max_size.to_string());
    let encrypted_message_max_length = get_env_var("PW_ENCRYPTED_MESSAGE_MAX_LENGTH")
        .and_then(|v| v.parse::<u64>().ok())
        .or(config.encrypted_message_max_length);
    let redis_url = get_env_var("PW_REDIS_URL").unwrap_or(config.redis_url);

    let ip_limits = get_ip_limits_config(config.ip_limits)?;

    let config = AppConfig {
        listen: listen.parse()?,
        log_level,
        log_target,
        message_max_length: message_max_length.parse()?,
        encrypted_message_max_length,
        file_upload_enabled: file_upload_enabled.parse()?,
        file_max_size: file_max_size.parse()?,
        redis_url,
        ip_limits,
    };

    info!("config: {}", config);

    Ok(config)
}

fn get_env_var(name: &str) -> Option<String> {
    env::var(name).ok()
}

fn get_ip_limits_config(
    yaml_config: Option<IpLimitsConfig>,
) -> anyhow::Result<Option<IpLimitsConfig>> {
    let mut ip_limits = yaml_config;

    if let Some(enabled_str) = get_env_var("PW_IP_LIMITS_ENABLED") {
        let enabled = enabled_str.parse::<bool>()?;

        if let Some(ref mut limits) = ip_limits {
            limits.enabled = enabled;
        } else {
            ip_limits = Some(IpLimitsConfig {
                enabled,
                whitelist: Vec::new(),
            });
        }
    }

    if let Some(whitelist_json) = get_env_var("PW_IP_LIMITS_WHITELIST") {
        let whitelist_entries: Vec<IpLimitEntry> = serde_json::from_str(&whitelist_json)
            .map_err(|e| anyhow::anyhow!("Failed to parse PW_IP_LIMITS_WHITELIST JSON: {}", e))?;

        if let Some(ref mut limits) = ip_limits {
            limits.whitelist = whitelist_entries;
        } else {
            ip_limits = Some(IpLimitsConfig {
                enabled: false, // Default to false if only whitelist is provided
                whitelist: whitelist_entries,
            });
        }
    }

    if let Some(ref ip_config) = ip_limits
        && let Err(validation_errors) = validate_ip_limits_config(ip_config)
    {
        let error_message = format_validation_errors(&validation_errors);
        return Err(anyhow::anyhow!(
            "Environment variable IP limits configuration validation failed:\n{}",
            error_message
        ));
    }

    Ok(ip_limits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    #[test]
    #[serial]
    fn test_get_ip_limits_config_with_yaml_only() {
        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
        }

        let yaml_config = Some(IpLimitsConfig {
            enabled: true,
            whitelist: vec![IpLimitEntry {
                ip: "192.168.1.1".to_string(),
                message_max_length: None,
                file_max_size: None,
            }],
        });

        let result = get_ip_limits_config(yaml_config.clone()).unwrap();
        assert_eq!(result, yaml_config);
    }

    #[test]
    #[serial]
    fn test_get_ip_limits_config_with_env_enabled_override() {
        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
            env::set_var("PW_IP_LIMITS_ENABLED", "false");
        }

        let yaml_config = Some(IpLimitsConfig {
            enabled: true,
            whitelist: vec![IpLimitEntry {
                ip: "192.168.1.1".to_string(),
                message_max_length: None,
                file_max_size: None,
            }],
        });

        let result = get_ip_limits_config(yaml_config).unwrap();
        let config = result.unwrap();
        assert_eq!(config.enabled, false);

        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
        }
    }

    #[test]
    #[serial]
    fn test_get_ip_limits_config_with_env_whitelist_override() {
        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
            env::set_var(
                "PW_IP_LIMITS_WHITELIST",
                r#"[{"ip": "10.0.0.1"}, {"ip": "10.0.0.2"}]"#,
            );
        }

        let yaml_config = Some(IpLimitsConfig {
            enabled: true,
            whitelist: vec![IpLimitEntry {
                ip: "192.168.1.1".to_string(),
                message_max_length: None,
                file_max_size: None,
            }],
        });

        let result = get_ip_limits_config(yaml_config).unwrap().unwrap();
        assert_eq!(result.enabled, true);
        assert_eq!(result.whitelist.len(), 2);
        assert_eq!(result.whitelist[0].ip, "10.0.0.1");
        assert_eq!(result.whitelist[1].ip, "10.0.0.2");

        unsafe {
            env::remove_var("PW_IP_LIMITS_WHITELIST");
        }
    }

    #[test]
    #[serial]
    fn test_get_ip_limits_config_create_from_env_only() {
        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
            env::set_var("PW_IP_LIMITS_ENABLED", "true");
            env::set_var("PW_IP_LIMITS_WHITELIST", r#"[{"ip": "127.0.0.1"}]"#);
        }

        let result = get_ip_limits_config(None).unwrap().unwrap();
        assert_eq!(result.enabled, true);
        assert_eq!(result.whitelist.len(), 1);
        assert_eq!(result.whitelist[0].ip, "127.0.0.1");

        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
        }
    }

    #[test]
    #[serial]
    fn test_get_ip_limits_config_invalid_json() {
        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
            env::set_var("PW_IP_LIMITS_WHITELIST", "invalid json");
        }

        let result = get_ip_limits_config(None);
        assert!(result.is_err());

        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
        }
    }

    #[test]
    #[serial]
    fn test_get_ip_limits_config_invalid_bool() {
        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
            env::set_var("PW_IP_LIMITS_ENABLED", "not_a_bool");
        }

        let result = get_ip_limits_config(None);
        assert!(result.is_err());

        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
        }
    }

    #[test]
    #[serial]
    fn test_get_ip_limits_config_complex_whitelist() {
        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
            env::set_var(
                "PW_IP_LIMITS_WHITELIST",
                r#"[
                {
                    "ip": "192.168.1.100",
                    "message-max-length": 8192,
                    "file-max-size": 104857600
                },
                {
                    "ip": "10.0.0.0/8",
                    "message-max-length": 4096
                },
                {
                    "ip": "172.16.1.5",
                    "file-max-size": 209715200
                }
            ]"#,
            );
        }

        let result = get_ip_limits_config(None).unwrap().unwrap();
        assert_eq!(result.enabled, false); // Default when only whitelist provided
        assert_eq!(result.whitelist.len(), 3);

        // Check first entry (all fields)
        assert_eq!(result.whitelist[0].ip, "192.168.1.100");
        assert_eq!(result.whitelist[0].message_max_length, Some(8192));
        assert_eq!(result.whitelist[0].file_max_size, Some(104857600));

        // Check second entry (only message limit)
        assert_eq!(result.whitelist[1].ip, "10.0.0.0/8");
        assert_eq!(result.whitelist[1].message_max_length, Some(4096));
        assert_eq!(result.whitelist[1].file_max_size, None);

        // Check third entry (only file size)
        assert_eq!(result.whitelist[2].ip, "172.16.1.5");
        assert_eq!(result.whitelist[2].message_max_length, None);
        assert_eq!(result.whitelist[2].file_max_size, Some(209715200));

        unsafe {
            env::remove_var("PW_IP_LIMITS_WHITELIST");
        }
    }

    #[test]
    #[serial]
    fn test_get_ip_limits_config_empty_whitelist() {
        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
            env::set_var("PW_IP_LIMITS_ENABLED", "true");
            env::set_var("PW_IP_LIMITS_WHITELIST", "[]");
        }

        let result = get_ip_limits_config(None).unwrap().unwrap();
        assert_eq!(result.enabled, true);
        assert_eq!(result.whitelist.len(), 0);

        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
        }
    }

    #[test]
    #[serial]
    fn test_get_ip_limits_config_both_yaml_and_env() {
        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
            env::set_var("PW_IP_LIMITS_ENABLED", "false");
            env::set_var(
                "PW_IP_LIMITS_WHITELIST",
                r#"[
                    {
                        "ip": "203.0.113.1",
                        "message-max-length": 4096,
                        "file-max-size": 104857600
                    }
                ]"#,
            );
        }

        let yaml_config = Some(IpLimitsConfig {
            enabled: true,
            whitelist: vec![IpLimitEntry {
                ip: "192.168.1.1".to_string(),
                message_max_length: Some(2048),
                file_max_size: Some(52428800),
            }],
        });

        let result = get_ip_limits_config(yaml_config).unwrap().unwrap();

        // Environment variables should override YAML
        assert_eq!(result.enabled, false); // env overrides yaml (true -> false)
        assert_eq!(result.whitelist.len(), 1);
        assert_eq!(result.whitelist[0].ip, "203.0.113.1"); // env overrides yaml IP
        assert_eq!(result.whitelist[0].message_max_length, Some(4096)); // env overrides yaml (2048 -> 4096)
        assert_eq!(result.whitelist[0].file_max_size, Some(104857600)); // env overrides yaml (52428800 -> 104857600)

        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
        }
    }

    #[test]
    #[serial]
    fn test_get_ip_limits_config_only_enabled_env() {
        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
            env::set_var("PW_IP_LIMITS_ENABLED", "true");
        }

        let result = get_ip_limits_config(None).unwrap().unwrap();
        assert_eq!(result.enabled, true);
        assert_eq!(result.whitelist.len(), 0);

        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
        }
    }

    #[test]
    #[serial]
    fn test_get_ip_limits_config_only_whitelist_env() {
        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
            env::set_var("PW_IP_LIMITS_WHITELIST", r#"[{"ip": "127.0.0.1"}]"#);
        }

        let result = get_ip_limits_config(None).unwrap().unwrap();
        assert_eq!(result.enabled, false); // Default when only whitelist provided
        assert_eq!(result.whitelist.len(), 1);
        assert_eq!(result.whitelist[0].ip, "127.0.0.1");

        unsafe {
            env::remove_var("PW_IP_LIMITS_WHITELIST");
        }
    }

    #[test]
    #[serial]
    fn test_get_ip_limits_config_invalid_whitelist_json_structure() {
        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
            env::set_var("PW_IP_LIMITS_WHITELIST", r#"[{"not_ip": "192.168.1.1"}]"#);
        }

        let result = get_ip_limits_config(None);
        assert!(result.is_err());

        unsafe {
            env::remove_var("PW_IP_LIMITS_WHITELIST");
        }
    }

    #[test]
    #[serial]
    fn test_get_ip_limits_config_ipv6_addresses() {
        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
            env::set_var(
                "PW_IP_LIMITS_WHITELIST",
                r#"[
                {"ip": "2001:db8::1", "message-max-length": 16384},
                {"ip": "::1", "file-max-size": 104857600},
                {"ip": "2001:db8::/32", "message-max-length": 8192, "file-max-size": 209715200}
            ]"#,
            );
        }

        let result = get_ip_limits_config(None).unwrap().unwrap();
        assert_eq!(result.whitelist.len(), 3);

        assert_eq!(result.whitelist[0].ip, "2001:db8::1");
        assert_eq!(result.whitelist[0].message_max_length, Some(16384));
        assert_eq!(result.whitelist[0].file_max_size, None);

        assert_eq!(result.whitelist[1].ip, "::1");
        assert_eq!(result.whitelist[1].message_max_length, None);
        assert_eq!(result.whitelist[1].file_max_size, Some(104857600));

        assert_eq!(result.whitelist[2].ip, "2001:db8::/32");
        assert_eq!(result.whitelist[2].message_max_length, Some(8192));
        assert_eq!(result.whitelist[2].file_max_size, Some(209715200));

        unsafe {
            env::remove_var("PW_IP_LIMITS_WHITELIST");
        }
    }

    #[test]
    #[serial]
    fn test_get_ip_limits_config_extreme_values() {
        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
            env::set_var(
                "PW_IP_LIMITS_WHITELIST",
                &format!(
                    r#"[
                {{"ip": "192.168.1.1", "message-max-length": {}, "file-max-size": {}}}
            ]"#,
                    u16::MAX,
                    u64::MAX
                ),
            );
        }

        let result = get_ip_limits_config(None);
        // This should fail because u64::MAX exceeds our file size limit
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("File max size"));

        unsafe {
            env::remove_var("PW_IP_LIMITS_WHITELIST");
        }
    }

    #[test]
    #[serial]
    fn test_get_ip_limits_config_no_config() {
        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
        }
        let result = get_ip_limits_config(None).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    #[serial]
    fn test_get_ip_limits_config_validation_failure() {
        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
            env::set_var(
                "PW_IP_LIMITS_WHITELIST",
                r#"[
                {"ip": "invalid.ip.address", "message-max-length": 0}
            ]"#,
            );
        }

        let result = get_ip_limits_config(None);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("validation failed"));

        unsafe {
            env::remove_var("PW_IP_LIMITS_WHITELIST");
        }
    }

    #[test]
    #[serial]
    fn test_get_ip_limits_config_validation_success() {
        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
            env::set_var(
                "PW_IP_LIMITS_WHITELIST",
                r#"[
                {"ip": "192.168.1.100", "message-max-length": 2048, "file-max-size": 52428800}
            ]"#,
            );
        }

        let result = get_ip_limits_config(None);
        assert!(result.is_ok());
        let config = result.unwrap().unwrap();
        assert_eq!(config.whitelist.len(), 1);
        assert_eq!(config.whitelist[0].ip, "192.168.1.100");

        unsafe {
            env::remove_var("PW_IP_LIMITS_WHITELIST");
        }
    }

    #[test]
    #[serial]
    fn test_get_ip_limits_config_duplicate_ips() {
        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
            env::set_var(
                "PW_IP_LIMITS_WHITELIST",
                r#"[
                {"ip": "192.168.1.100"},
                {"ip": "192.168.1.100"}
            ]"#,
            );
        }

        let result = get_ip_limits_config(None);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Duplicate IP entry"));

        unsafe {
            env::remove_var("PW_IP_LIMITS_WHITELIST");
        }
    }

    #[test]
    #[serial]
    fn test_get_ip_limits_config_out_of_bounds_limits() {
        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
            env::set_var(
                "PW_IP_LIMITS_WHITELIST",
                &format!(
                    r#"[
                {{"ip": "192.168.1.100", "message-max-length": {}, "file-max-size": {}}}
            ]"#,
                    u16::MAX as u32 + 1,
                    u64::MAX
                ),
            ); // Invalid message length over u16::MAX
        }

        let result = get_ip_limits_config(None);
        // This should fail during JSON parsing since u16::MAX + 1 can't fit in u16
        assert!(result.is_err());

        unsafe {
            env::remove_var("PW_IP_LIMITS_WHITELIST");
        }
    }

    #[test]
    #[serial]
    fn test_env_var_helpers() {
        unsafe {
            env::remove_var("TEST_VAR");
            env::set_var("TEST_VAR", "test_value");
        }

        assert_eq!(get_env_var("TEST_VAR"), Some("test_value".to_string()));
        assert_eq!(get_env_var("NON_EXISTENT_VAR"), None);

        unsafe {
            env::remove_var("TEST_VAR");
        }
    }

    #[test]
    #[serial]
    fn test_different_message_and_file_limits() {
        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
            env::set_var(
                "PW_IP_LIMITS_WHITELIST",
                r#"[
                {
                    "ip": "192.168.1.100",
                    "message-max-length": 1024,
                    "file-max-size": 10485760
                },
                {
                    "ip": "10.0.0.1",
                    "message-max-length": 4096,
                    "file-max-size": 52428800
                },
                {
                    "ip": "172.16.0.1",
                    "message-max-length": 8192,
                    "file-max-size": 104857600
                }
            ]"#,
            );
        }

        let result = get_ip_limits_config(None).unwrap().unwrap();
        assert_eq!(result.whitelist.len(), 3);

        // Verify different message and file size combinations
        assert_eq!(result.whitelist[0].ip, "192.168.1.100");
        assert_eq!(result.whitelist[0].message_max_length, Some(1024));
        assert_eq!(result.whitelist[0].file_max_size, Some(10485760)); // 10 MB

        assert_eq!(result.whitelist[1].ip, "10.0.0.1");
        assert_eq!(result.whitelist[1].message_max_length, Some(4096));
        assert_eq!(result.whitelist[1].file_max_size, Some(52428800)); // 50 MB

        assert_eq!(result.whitelist[2].ip, "172.16.0.1");
        assert_eq!(result.whitelist[2].message_max_length, Some(8192));
        assert_eq!(result.whitelist[2].file_max_size, Some(104857600)); // 100 MB

        unsafe {
            env::remove_var("PW_IP_LIMITS_WHITELIST");
        }
    }

    #[test]
    #[serial]
    fn test_mixed_limit_configurations() {
        unsafe {
            env::remove_var("PW_IP_LIMITS_ENABLED");
            env::remove_var("PW_IP_LIMITS_WHITELIST");
            env::set_var(
                "PW_IP_LIMITS_WHITELIST",
                r#"[
                {
                    "ip": "192.168.1.50",
                    "message-max-length": 2048
                },
                {
                    "ip": "192.168.1.51",
                    "file-max-size": 26214400
                },
                {
                    "ip": "192.168.1.52",
                    "message-max-length": 16384,
                    "file-max-size": 209715200
                }
            ]"#,
            );
        }

        let result = get_ip_limits_config(None).unwrap().unwrap();
        assert_eq!(result.whitelist.len(), 3);

        // Entry with only message limit
        assert_eq!(result.whitelist[0].ip, "192.168.1.50");
        assert_eq!(result.whitelist[0].message_max_length, Some(2048));
        assert_eq!(result.whitelist[0].file_max_size, None);

        // Entry with only file limit
        assert_eq!(result.whitelist[1].ip, "192.168.1.51");
        assert_eq!(result.whitelist[1].message_max_length, None);
        assert_eq!(result.whitelist[1].file_max_size, Some(26214400)); // 25 MB

        // Entry with both limits
        assert_eq!(result.whitelist[2].ip, "192.168.1.52");
        assert_eq!(result.whitelist[2].message_max_length, Some(16384));
        assert_eq!(result.whitelist[2].file_max_size, Some(209715200)); // 200 MB

        unsafe {
            env::remove_var("PW_IP_LIMITS_WHITELIST");
        }
    }
}
