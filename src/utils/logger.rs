use env_logger::{Builder, Env};

pub fn init(verbose: bool) {
    let default_level = if verbose { "debug" } else { "info" };
    Builder::from_env(Env::default().default_filter_or(default_level))
        .format_timestamp(None)
        .init();
}
