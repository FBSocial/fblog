use std::io::Write;

use env_logger::Builder;

// lazy_static::lazy_static! {
//         static ref LOCAL_HOST_NAME: String = "".to_string();
//         static ref LOCAL_IP: String = "".to_string();
//         // static ref LOCAL_HOST_NAME: String = crate::hostname();
//         // static ref LOCAL_IP: String = crate::get_proper_ip();
//         // static ref l
//     }
// use once_cell::sync::OnceCell;
// static LOCAL_HOST_NAME: OnceCell<String> = OnceCell::new();
// static LOCAL_IP: OnceCell<String> = OnceCell::new();

// const log_record_format_relative: &str = "[{}] {} {}:{} {}:{} {}";
// const log_record_format_absolute: &str = "[{}] {} {}:{} file://{}:{} {}";

// log_filters specification is conformed to rust log-specification which can be: debug, xx_module=xxx
pub fn get_default_env_logger_builder(log_filters: &str) -> Builder {
    // let local_host_name = LOCAL_HOST_NAME.get_or_init(||crate::hostname());
    // let local_ip = LOCAL_IP.get_or_init(||crate::get_proper_ip());
    //
    let local_host_name = crate::hostname();
    let local_ip = crate::get_proper_ip();

    let mut builder = env_logger::Builder::new();
    builder.format(move |buf, record| {

        // 1. if file path is relative, nothing changes.
        // 2. if it is absolute, prefixed with "file://"
        // // **************************************************
        // // low_todo: string/memory optimization
        // // * solution-a:
        // //   use 2 writeln!, 1st is for relative path, 2nd is for absolute.
        // //   * good:
        // //     more performative
        // //   * bad:
        // //     more redundant code
        // // * solution-b:
        // //   use 1 writeln!, but return 2 static "[{}] {} {}:{} {}:{} {}" according to property: its relative/absolute
        // //   (might use many escape, eg: {{ }})
        // //   stuck(2021-10-19T16:07:13.381Z): the 1st parameter of 'format!', 'write!' must be a literal (determined by macro)
        // //   return to solution-a:
        // // **************************************************
        // let log_record_format = if record.file().unwrap_or("").starts_with("/") {
        //     log_record_format_absolute
        // } else {log_record_format_relative};
        // writeln!(buf, log_record_format,
        //          record.level(),
        //          chrono::Local::now(),
        //
        //          // eg: portal::oauth2::http:205
        //          record.module_path().unwrap_or_default(),
        //          record.line().unwrap_or(0),
        //
        //          // files relative to project will show in relative format: eg: src/oauth2/http.rs:205
        //          // or show in absolute format: /absolute-path/rust/cargo/registry/src/github.com-1ecc6299db9ec823/actix-web-3.3.2/src/middleware/logger.rs:337
        //          record.file().unwrap_or(""),
        //          file_path,
        //          record.args())

        let file_path = record
            .file()
            .map(|p| if p.starts_with("/") { "file://".to_string() + p } else { p.to_string() })
            .unwrap_or("".to_string());
        writeln!(buf, "[{}] {} {}-{} {}:{} {}:{} {}",
                 record.level(),
                 chrono::Local::now(),
                 local_host_name,
                 local_ip,

                 // eg: portal::oauth2::http:205
                 record.module_path().unwrap_or_default(),
                 record.line().unwrap_or(0),
                 file_path,
                 record.line().unwrap_or(0),
                 record.args())
    }).parse_filters(log_filters);
    builder
}


pub fn init_default_env_logger(log_filters: &str) {
    get_default_env_logger_builder(log_filters).init()
}

// set log level to "debug", convenient for test env
pub fn init_default_env_logger_with_debug_level() {
    get_default_env_logger_builder("debug").init()
}


// set log level to "info", convenient for test env
pub fn init_default_env_logger_with_info_level() {
    get_default_env_logger_builder("info").init()
}

// set log level to "warn", convenient for test env
pub fn init_default_env_logger_with_warn_level() {
    get_default_env_logger_builder("warn").init()
}


// set log level to "error", convenient for test env
pub fn init_default_env_logger_with_error_level() {
    get_default_env_logger_builder("error").init()
}

#[test]
fn test_get_default_env_logger_builder() {
    use log::*;
    let mut builder = get_default_env_logger_builder("debug");
    builder.init();
    debug!("hello");
    info!("hello");
    warn!("hello");
    error!("hello");
}