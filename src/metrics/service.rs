use crate::VERSION;
use crate::config::model::AppConfig;
use crate::metrics::model::{BuildInfo, ConfigMetrics, Metrics, RedisMetrics};
use crate::metrics::ports::MetricsService;
use redis::Commands;
use std::time::Instant;
use uuid::Uuid;

const REDIS_CHECK_TTL_SECONDS: u64 = 2;

#[derive(Debug, Clone)]
pub struct MetricsServer {
    config: AppConfig,
    body_limit: usize,
    start_time: Instant,
}

impl MetricsServer {
    pub fn new(config: AppConfig, body_limit: usize) -> Self {
        Self {
            config,
            body_limit,
            start_time: Instant::now(),
        }
    }

    pub async fn get_metrics(&self) -> Metrics {
        let uptime_seconds = self.start_time.elapsed().as_secs_f64();

        let (redis_up, redis_latency_seconds) =
            match check_redis_availability(&self.config.redis_url) {
                Some(latency) => (true, latency),
                None => (false, f64::NAN),
            };

        let ip_limits_enabled = self
            .config
            .ip_limits
            .as_ref()
            .map(|limits| limits.enabled)
            .unwrap_or(false);

        Metrics {
            up: true,
            build: BuildInfo {
                version: VERSION.to_string(),
            },
            uptime_seconds,
            redis: RedisMetrics {
                up: redis_up,
                latency_seconds: redis_latency_seconds,
            },
            config: ConfigMetrics {
                message_max_length: self.config.message_max_length,
                file_max_size_bytes: self.config.file_max_size,
                file_upload_enabled: self.config.file_upload_enabled,
                ip_limits_enabled,
                body_limit_bytes: self.body_limit,
            },
        }
    }
}

impl MetricsService for MetricsServer {
    fn get_metrics(&self) -> impl std::future::Future<Output = Metrics> + Send + '_ {
        MetricsServer::get_metrics(self)
    }
}

fn check_redis_availability(redis_url: &str) -> Option<f64> {
    let client = redis::Client::open(redis_url).ok()?;
    let mut connection = client.get_connection().ok()?;

    let key = format!("pw:metrics:{}", Uuid::new_v4());
    let value = "1";
    let start = Instant::now();

    let _: () = connection
        .set_ex(&key, value, REDIS_CHECK_TTL_SECONDS)
        .ok()?;
    let fetched: Option<String> = connection.get(&key).ok()?;

    if fetched.as_deref() == Some(value) {
        Some(start.elapsed().as_secs_f64())
    } else {
        None
    }
}
