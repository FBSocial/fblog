use env_logger::Builder;

use crate::config_for_env_logger::get_default_env_logger_builder;

pub fn try_init_logger(log_spec: &str) {
    get_default_env_logger_builder(log_spec)
        .is_test(true)
        .try_init()
        .map_err(|err| {
            println!("log init err: {}", err);
            err
        })
        .ok();
}

pub fn default_logger(log_spec: &str) -> Builder {
    let mut builder = get_default_env_logger_builder(log_spec);
    builder.is_test(true);
    builder
}

#[cfg(test)]
pub mod test {
    use log::*;

    use super::*;

    #[test]
    fn test_init_logger() {
        try_init_logger("debug");
        debug!("hello");
        info!("hello");
        warn!("hello");
        error!("hello");
    }
}
