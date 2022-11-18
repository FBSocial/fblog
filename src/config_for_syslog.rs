use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};

use log::*;
use log::LevelFilter;
use syslog::{BasicLogger, Facility, Formatter3164};

static G_UDP_LOGGER_STARTED_IN_TEST: AtomicBool = AtomicBool::new(false);

/// fallback/default is: LevelFilter::Info
pub fn get_formal_log_level_from_str(s: &str) -> log::LevelFilter {
    match s.to_lowercase().as_str() {
        s if s.starts_with("trace") => LevelFilter::Trace,
        s if s.starts_with("debug") => LevelFilter::Debug,
        s if s.starts_with("info") => LevelFilter::Info,
        s if s.starts_with("warn") => LevelFilter::Warn,
        s if s.starts_with("error") => LevelFilter::Error,
        s if s.starts_with("off") => LevelFilter::Off,
        _ => LevelFilter::Info,
    }
}

/// used in test.
/// For main process_name, you can use:
/// env: CARGO_PKG_NAME
/// eg:
/// const VERSION: &'static str = env!("CARGO_PKG_VERSION");
/// const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
pub fn start_udp_logger_in_test(
    facility: Facility,
    process_name: &str,
    local_address: SocketAddr,
    remote_address: SocketAddr,
    level_filter: LevelFilter,
) {
    if G_UDP_LOGGER_STARTED_IN_TEST.load(Ordering::SeqCst) {
        debug!("udp logger already started.");
        return;
    }
    start_udp_logger(facility, process_name, local_address, remote_address, level_filter);
    G_UDP_LOGGER_STARTED_IN_TEST.store(true, Ordering::SeqCst);
    println!("started syslog in test");
}


/// For main process_name, you can use:
/// env: CARGO_PKG_NAME
/// eg:
/// const VERSION: &'static str = env!("CARGO_PKG_VERSION");
/// const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
pub fn start_udp_logger(
    facility: Facility,
    process_name: &str,
    local_address: SocketAddr,
    remote_address: SocketAddr,
    level_filter: LevelFilter,
) {
    let local_host_name = crate::hostname();
    let local_ip = crate::get_proper_ip().replace('/', "N");
    let hostname_in_log = local_host_name + "_" + &local_ip;
    let pid = std::process::id() as i32;

    let formatter = Formatter3164 {
        facility,
        hostname: Some(hostname_in_log.clone()),
        process: process_name.into(),
        pid: pid,
    };

    // let logger = syslog::unix(formatter).expect("could not connect to syslog");
    let logger = syslog::udp(formatter, local_address, remote_address)
        .expect("could not connect to syslog");
    println!("Starting syslog(udp) with facility: {:?}.  Checkout rsyslogd config(/etc/rsyslog.conf), and find these 2 similar lines:\n
local1.*			-/data/log/collected_by_rsyslog/open_platform.log
local2.*			-/data/log/collected_by_rsyslog/bot_platform.log

# these are the file locations in udp server.

local_address: {:?},
remote_address: {:?}
", facility, local_address, remote_address);
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(level_filter))
        .map_err(|err| {
            error!("could not init syslog logger, err: {:#?}", err);
            err
        })
        .expect("could not init syslog logger");

    println!(r#"Started syslog(udp) with
    facility: {:?},
    hostname_in_log: {},
    process_name: {},
    pid: {},
    local_address: {},
    remote_address: {},
    level_filter: {:?}"#,
             facility,
             hostname_in_log,
             process_name,
             pid,
             local_address,
             remote_address,
             level_filter
    );
    debug!("debug");
    info!("info");
}


#[cfg(test)]
mod test {
    use std::str::FromStr;

    use log::LevelFilter;

    // use get_formal_log_level_from_str;
    use super::*;

    #[test]
    fn test_syslog_udp_sending() {
        // start_default_logger("debug", true);
        start_udp_logger_in_test(
            Facility::LOG_USER,
            "logger",
            // SocketAddr::from_str("127.0.0.1:15541").unwrap(),
            // SocketAddr::from_str("172.17.0.1:15541").unwrap(),
            SocketAddr::from_str("0.0.0.0:0").unwrap(),
            SocketAddr::from_str("172.17.0.2:514").unwrap(), // docker: 172.17.0.2
            // SocketAddr::from_str("127.0.0.1:514").unwrap(),
            LevelFilter::Debug);

        trace!("trace");
        debug!("debug");
        info!("info");
        warn!("warn");
        error!("error");
        info!("multiline:\r\nline1\r\nline2\nline3\n");
        // info!("local_host_name: {}, local_ip: {}", local_host_name, local_ip);
    }

    #[test]
    fn test_get_log_level() {
        assert_eq!(get_formal_log_level_from_str("trace"), LevelFilter::Trace);
        assert_eq!(get_formal_log_level_from_str("debUG"), LevelFilter::Debug);
        assert_eq!(get_formal_log_level_from_str("info"), LevelFilter::Info);
        assert_eq!(get_formal_log_level_from_str("warn"), LevelFilter::Warn);
        assert_eq!(get_formal_log_level_from_str("error"), LevelFilter::Error);
    }
}