use std::collections::HashSet;
use std::fmt::Debug;
use std::net::SocketAddr;
use std::str::FromStr;

use gethostname;
use pnet::datalink;
pub use syslog::Facility;

pub use config_for_env_logger::get_default_env_logger_builder;


// pub use config_for_env_logger::*;
// pub use config_for_flexi_logger::*;
// pub use config_for_syslog::{start_udp_logger, start_udp_logger_in_test};

pub mod config_for_env_logger;
pub mod config_for_flexi_logger;
pub mod config_for_syslog;
pub mod test_helper;
pub mod toolbox;

pub fn hostname() -> String {
    gethostname::gethostname().to_string_lossy().to_string()
}


pub fn get_proper_ip() -> String {
    let mut hs = HashSet::new();
    for iface in datalink::interfaces() {
        for ip in iface.ips {
            // println!("inserting: {}", ip);
            hs.insert(ip);
        }
    }

    let mut proper_ip = "".to_string();
    for (i, ip) in hs.iter().enumerate() {
        if i == 0 {
            proper_ip = ip.to_string()
        }
        if ip.is_ipv4() && !ip.to_string().starts_with("127") {
            proper_ip = ip.to_string();
            // println!("proper_ip in log chosed: {}", proper_ip);
            return proper_ip;
        }
    }

    if !proper_ip.is_empty() {
        for ip in hs.iter() {
            if ip.is_ipv6() && !ip.to_string().starts_with("::1/128") {
                proper_ip = ip.to_string();
                // println!("proper_ip in log chosed: {}", proper_ip);
                return proper_ip;
            }
        }
    }

    // println!("proper_ip in log chosed: {}", proper_ip);
    proper_ip
}


pub fn str_to_socket_addr(s: &str) -> SocketAddr {
    SocketAddr::from_str(s)
        .map_err(|err| {
            println!("could not parse SocketAddr from str: {:?}, err: {}", s, err);
            err
        })
        .expect(&format!("Parse SocketAddr error from input str: {:?}", s))
}
//
// #[derive(Debug, Clone, )]
// pub enum LoggerType {
//     UseSysLog(SysLog),
//     // String is remote address
//     UseLocalLog(LocalLog), // .0 -> enabled console log .1 -> enabled file log
// }
//
// impl LoggerType {
//     pub fn new_syslog(remote_address: SocketAddr) -> Self {
//         LoggerType::UseSysLog(SysLog::new(remote_address))
//     }
//
//     /// panic if both are false.
//     pub fn new_local_log(enabled_console_log: bool, enabled_file_log: bool) -> Self {
//         LoggerType::UseLocalLog(LocalLog::new(enabled_console_log, enabled_file_log))
//     }
//
//     pub fn new_by_trying_each_log(enabled_syslog: bool,
//                                   syslog_remote_address: SocketAddr,
//                                   enabled_local_console_log: bool,
//                                   enabled_local_file_log: bool) -> Self {
//         if enabled_syslog {
//             Self::new_syslog(syslog_remote_address)
//         } else {
//             Self::new_local_log(enabled_local_console_log, enabled_local_file_log)
//         }
//     }
//
//     pub fn enabled_syslog(&self) -> bool {
//         match self {
//             Self::UseSysLog(_) => true,
//             _ => false,
//         }
//     }
//     pub fn enabled_local_log(&self) -> bool {
//         match self {
//             Self::UseLocalLog(local) => local.enabled_console_log || local.enabled_file_log,
//             _ => false,
//         }
//     }
//     pub fn enabled_local_console_log(&self) -> bool {
//         match self {
//             Self::UseLocalLog(local) => local.enabled_console_log,
//             _ => false,
//         }
//     }
//     pub fn enabled_local_file_log(&self) -> bool {
//         match self {
//             Self::UseLocalLog(local) => local.enabled_file_log,
//             _ => false,
//         }
//     }
//
//     pub fn enabled_log(&self) -> bool {
//         self.enabled_syslog() || self.enabled_local_log()
//     }
// }

#[derive(Debug, Clone)]
pub struct SysLog {
    facility: Facility,
    remote_address: SocketAddr,
}

impl SysLog {
    pub fn new(facility: Facility, remote_address: SocketAddr) -> Self {
        Self {
            facility,
            remote_address
        }
    }

    pub fn start_udp_logger(&self, log_spec: &str, process_name: &str) {
        start_udp_logger(self.facility, self.remote_address, log_spec, process_name)
    }
}

#[derive(Debug, Clone)]
pub struct LocalLog {
    enabled_console_log: bool,
    enabled_file_log: bool,
}

impl LocalLog {
    /// panic if both are false
    pub fn new(enabled_console_log: bool, enabled_file_log: bool) -> Self {
        if !enabled_console_log && !enabled_file_log {
            panic!("Must enable at least 1 log: console/file")
        }
        Self {
            enabled_console_log,
            enabled_file_log,
        }
    }

    pub fn start_local_logger(&self, log_spec: &str) {
        start_local_logger(log_spec, self.enabled_console_log, self.enabled_file_log)
    }
}

pub fn start_udp_logger(
    facility: Facility,
    remote_address: SocketAddr,
    log_spec: &str,
    process_name: &str) {
    println!("Try starting udp logger with process_name: {:?}, log_spec: {:?}", process_name, log_spec);
    // remote syslog udp server is: 514, so we using 15514 as local
    let local_address = SocketAddr::from_str("0.0.0.0:0")
        .map_err(|err| {
            println!("Failed to parse socket address, err: {}", err);
        })
        .expect("failed to parse sockt address");
    let log_level = config_for_syslog::get_formal_log_level_from_str(log_spec);
    println!("Final sys_log_level: {:?}, local_address: {:?}, remote_address: {:?}", log_level, local_address, remote_address);
    config_for_syslog::start_udp_logger(facility, process_name, local_address, remote_address, log_level);
    println!("Started udp logger @{}", chrono::Local::now());
}

pub fn start_local_logger(log_spec: &str,
                          enabled_console_log: bool,
                          enabled_file_log: bool,
) {
    println!("Using triditional console/file log");
    if enabled_console_log || enabled_file_log {
        config_for_flexi_logger::start_default_logger(log_spec, enabled_console_log, enabled_file_log);
    } else {
        panic!("Use local log, but no console-log or file-log is specified!")
    }
}


/// 1. try udp logger(default) (use Facility::LOG_USER if not supplied)
/// 2. try local logger(console-logger or file-logger)
pub fn start_logger_automatically(
    process_name: &str,
    log_spec: &str,
    enabled_udp_logger_arg: &str,
    udp_server_address_if_udp_enabled: &str,
    facility_if_udp_enabled: Option<Facility>,
    enabled_local_console_log_arg: &str,
    enabled_local_file_log_arg: &str) {
    println!(r##"Try starting logger automatically,
process_name: {:?},
log_spec: {:?},
enabled_udp_logger_arg: {:?},
udp_server_address_if_enabled: {:?},
facility_if_udp_enabled: {:?},
enabled_local_console_log_arg: {:?},
enabled_local_file_log_arg: {:?}
"##, process_name, log_spec, enabled_udp_logger_arg, udp_server_address_if_udp_enabled, facility_if_udp_enabled,  enabled_local_console_log_arg, enabled_local_file_log_arg);
    let enabled_udp_logger = toolbox::is_bool_true(enabled_udp_logger_arg);
    if enabled_udp_logger {
        println!("Using syslog(udp) by parsing cli");
        SysLog::new(facility_if_udp_enabled.unwrap_or(Facility::LOG_USER),str_to_socket_addr(udp_server_address_if_udp_enabled))
            .start_udp_logger(log_spec, process_name);
    } else {
        println!("Using traditional console/file log by parsing cli");
        let enable_local_console_log = toolbox::is_bool_true(enabled_local_console_log_arg);
        let enable_local_file_log = toolbox::is_bool_true(enabled_local_file_log_arg);
        LocalLog::new(enable_local_console_log, enable_local_file_log)
            .start_local_logger(log_spec);
    }
}

// /// try each log
// /// udp first
// /// panic if start log error or no log is enabled.
// /// Note:
// /// currently, syslog only supports only simple directions like "trace", "debug", "info", "warn", "error",
// /// and complicated "debug;h2=info" will be transformed to "debug" only.
// /// luckily, we set level=info in online-env and use complicated-spec in dev env.
// /// so it is ok.
// ///
// /// return is_using_syslog: bool
// pub fn start_logger(logger_type: LoggerType,
//                     log_spec: &str,
//                     process_name: &str) -> bool {
//     if !logger_type.enabled_log() {
//         panic!("No log is enabled, one of syslog/console_log/file_log must be enabled first!")
//     }
//     match logger_type {
//         LoggerType::UseSysLog(syslog) => {
//             start_udp_logger(log_spec, process_name);
//             true
//         }
//         LoggerType::UseLocalLog(local_log) => {
//             start_local_logger(log_spec, local_log.enabled_console_log, local_log.enabled_file_log);
//             false
//         }
//     }
// }

#[macro_export]
macro_rules! ctx_debug {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, log::Level::Debug, $($arg)+)
    );
    ($($arg:tt)+) => (
        log!(log::Level::Debug, $($arg)+)
    )
}

#[cfg(test)]
mod test {
    use log::*;

    use super::*;

    #[test]
    fn test_ctx_debug() {
        test_helper::try_init_logger("trace");
        ctx_debug!("{}", 10);
    }

    #[test]
    fn test_get_ip() {
        let proper_ip = get_proper_ip();
        println!("proper_ip: {}", proper_ip);
    }

    #[test]
    fn test_start_logger_automatically_udp() {
        start_logger_automatically("process_name",
                                   "debug",
                                   "true",
                                   "127.0.0.1:514",
                                   None,
                                   "false",
                                   "false");
    }


    #[test]
    fn test_start_logger_automatically_local_console_and_local_file() {
        let log_spec = "debug";

        LocalLog::new(true, true).start_local_logger(log_spec);

        info!("info");
    }

    #[test]
    fn test_start_logger_automatically_local_console_only() {
        let log_spec = "debug";

        LocalLog::new(true, false).start_local_logger(log_spec);

        info!("info");
    }

    #[test]
    fn test_start_logger_automatically_local_file_only() {
        let log_spec = "debug";

        LocalLog::new(false, true).start_local_logger(log_spec);

        info!("info");
    }
}


