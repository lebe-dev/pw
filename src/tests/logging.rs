use log::LevelFilter;

pub fn init_logging() {
    let _ = env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .is_test(true)
        .try_init();
}
