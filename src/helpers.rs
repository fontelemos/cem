use env_logger::Env;

pub fn init_log() {
    let log_env = Env::default()
        .filter_or("MY_LOG_LEVEL", "debug")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(log_env);
}