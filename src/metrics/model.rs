use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct Metrics {
    pub up: bool,
    pub build: BuildInfo,
    pub uptime_seconds: f64,
    pub redis: RedisMetrics,
    pub config: ConfigMetrics,
}

#[derive(Debug, Clone)]
pub struct BuildInfo {
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct RedisMetrics {
    pub up: bool,
    pub latency_seconds: f64,
}

#[derive(Debug, Clone)]
pub struct ConfigMetrics {
    pub message_max_length: u16,
    pub file_max_size_bytes: u64,
    pub file_upload_enabled: bool,
    pub ip_limits_enabled: bool,
    pub body_limit_bytes: usize,
}

impl Metrics {
    pub fn to_prometheus_text(&self) -> String {
        let mut body = String::new();

        push_help_and_type(&mut body, "pw_up", "Application availability", "gauge");
        push_metric_line(&mut body, "pw_up", bool_to_u8(self.up));

        push_help_and_type(&mut body, "pw_build_info", "Build information", "gauge");
        let version_label = escape_label_value(&self.build.version);
        let _ = writeln!(body, "pw_build_info{{version=\"{}\"}} 1", version_label);

        push_help_and_type(
            &mut body,
            "pw_uptime_seconds",
            "Process uptime in seconds",
            "gauge",
        );
        push_metric_line(&mut body, "pw_uptime_seconds", self.uptime_seconds);

        push_help_and_type(
            &mut body,
            "pw_redis_up",
            "Redis availability check result",
            "gauge",
        );
        push_metric_line(&mut body, "pw_redis_up", bool_to_u8(self.redis.up));

        push_help_and_type(
            &mut body,
            "pw_redis_latency_seconds",
            "Redis availability check latency in seconds",
            "gauge",
        );
        push_metric_line(
            &mut body,
            "pw_redis_latency_seconds",
            self.redis.latency_seconds,
        );

        push_help_and_type(
            &mut body,
            "pw_config_message_max_length",
            "Configured message max length",
            "gauge",
        );
        push_metric_line(
            &mut body,
            "pw_config_message_max_length",
            self.config.message_max_length,
        );

        push_help_and_type(
            &mut body,
            "pw_config_file_max_size_bytes",
            "Configured file max size in bytes",
            "gauge",
        );
        push_metric_line(
            &mut body,
            "pw_config_file_max_size_bytes",
            self.config.file_max_size_bytes,
        );

        push_help_and_type(
            &mut body,
            "pw_config_file_upload_enabled",
            "Configured file upload enabled flag",
            "gauge",
        );
        push_metric_line(
            &mut body,
            "pw_config_file_upload_enabled",
            bool_to_u8(self.config.file_upload_enabled),
        );

        push_help_and_type(
            &mut body,
            "pw_ip_limits_enabled",
            "IP limits feature enabled",
            "gauge",
        );
        push_metric_line(
            &mut body,
            "pw_ip_limits_enabled",
            bool_to_u8(self.config.ip_limits_enabled),
        );

        push_help_and_type(
            &mut body,
            "pw_body_limit_bytes",
            "Configured HTTP body limit in bytes",
            "gauge",
        );
        push_metric_line(
            &mut body,
            "pw_body_limit_bytes",
            self.config.body_limit_bytes,
        );

        body
    }
}

fn push_help_and_type(buffer: &mut String, name: &str, help: &str, metric_type: &str) {
    let _ = writeln!(buffer, "# HELP {} {}", name, help);
    let _ = writeln!(buffer, "# TYPE {} {}", name, metric_type);
}

fn push_metric_line<T: std::fmt::Display>(buffer: &mut String, name: &str, value: T) {
    let _ = writeln!(buffer, "{} {}", name, value);
}

fn bool_to_u8(value: bool) -> u8 {
    if value { 1 } else { 0 }
}

fn escape_label_value(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            _ => escaped.push(ch),
        }
    }
    escaped
}
