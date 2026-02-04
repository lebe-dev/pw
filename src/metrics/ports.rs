use crate::metrics::model::Metrics;
use std::future::Future;

pub trait MetricsService: Clone + Send + Sync + 'static {
    fn get_metrics(&self) -> impl Future<Output = Metrics> + Send + '_;
}
