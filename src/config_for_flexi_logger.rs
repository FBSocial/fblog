use flexi_logger::{Age, Cleanup, Criterion, DeferredNow, Duplicate, FileSpec, Logger, Naming};
use log::*;
use time::macros::offset;

// const TS_S: &str = "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:6][offset_hour sign:mandatory]";


/// A logline-formatter that produces log lines like
/// <br>
/// ```[2016-01-13 15:25:01.640870 +01:00] INFO [foo::bar] src/foo/bar.rs:26: Task successfully read from conf.json```
/// <br>
/// i.e. with timestamp, module path and file location.
///
/// # Errors
///
/// See `std::write`
pub fn detailed_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let local_host_name = crate::hostname();
    let local_ip = crate::get_proper_ip();
    let line_number = record.line().unwrap_or(0);


    let file_path = record
        .file()
        .map(|p| if p.starts_with("/") { "file://".to_string() + p } else { p.to_string() })
        .unwrap_or("<unnamed>".to_string());

    write!(
        w,
        "[{}] {} {}-{} {}:{} {}:{} {}",
        // now.now().to_offset(offset!(+8)).format(&TS_S).unwrap_or_default(),
        record.level(),
        now.now().to_offset(offset!(+8)),
        local_host_name,
        local_ip,
        record.module_path().unwrap_or("<unnamed>"),
        line_number,
        file_path,
        record.line().unwrap_or(0),
        &record.args()
    )
}

// pub fn colored_detailed_format(
//     w: &mut dyn std::io::Write,
//     now: &mut DeferredNow,
//     record: &Record,
// ) -> Result<(), std::io::Error> {
//     let file_path = record
//         .file()
//         .map(|p| if p.starts_with("/") { "file://".to_string() + p } else { p.to_string() })
//         .unwrap_or("<unnamed>".to_string());
//
//     let level = record.level();
//     write!(
//         w,
//         "[{}] {} [{}] {}:{}: {}",
//         style(level).paint(now.now().to_offset(offset!(+8)).to_string()),
//         style(level).paint(record.level().to_string()),
//         record.module_path().unwrap_or("<unnamed>"),
//         file_path,
//         record.line().unwrap_or(0),
//         style(level).paint(&record.args().to_string())
//     )
// }

/// note:
/// 1. Current log will always output to File1: ${package_name}_rCurrent.log, eg: buff_rCurrent.log
/// 2. Additionally, (an empty file) File2: ${package_name}_r<Datetime-the-program-started>.log is also created. eg: buff_r2011-11-14T11:51:24+08.log.
///    At every new day (00:00), all logs in File1 will move to File2, then File1 is truncated and served as current logging file.
///    Thus, File2 includes the last day's log.
///    eg: buff_r2011-11-14T11:51:24+08.log has logs from 2011-11-14T11:51:24+08.log to 2011-11-14T59:59:59+08.log
///
pub fn default_logger(log_spec: &str, log_to_stdout: bool, log_to_file: bool) -> Logger {
    let mut logger = Logger::try_with_str(log_spec)
        .map_err(|err| {
            println!("could not init logger with spec: {}, err: {:?}", log_spec, err);
            err
        })
        .expect("could not init logger")
        .format(detailed_format)
        .print_message();
    if log_to_stdout {
        logger = logger.duplicate_to_stdout(Duplicate::All)
    }
    if log_to_file {
        logger = logger.log_to_file(FileSpec::default())
            .rotate(
                Criterion::Age(Age::new_with_splitting_at_every_new_day_by_offset_hour(8)),
                Naming::Timestamps(time::UtcOffset::from_hms(8, 0, 0).unwrap()),          // - let the rotated files have a timestamp in their name
                Cleanup::KeepLogFiles(1024),    // - keep at most 1024 log files
            );
    }

    logger
}

pub fn start_default_logger(log_spec: &str, log_to_stdout: bool, log_to_file: bool) -> flexi_logger::LoggerHandle {
    default_logger(log_spec, log_to_stdout, log_to_file)
        .start()
        .map_err(|err| {
            println!("Could not start logger, err: {:?}", err);
        })
        .expect("start default logger error")
}


#[cfg(test)]
mod test {
    use std::time::Duration;

    #[test]
    fn test_flexi_logger() {
        use super::*;
        start_default_logger("debug", true, true);

        // debug!("logger: {:#?}", logger);
        for i in 0..1 {
            debug!("debug: {}", i);
            info!("info: {}", i);
            warn!("warn: {}", i);
            error!("error: {}", i);
            std::thread::sleep(Duration::from_secs(2));
        }
    }
}