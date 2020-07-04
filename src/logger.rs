use std::fs::{metadata, OpenOptions, File};

use simplelog::{ConfigBuilder, Config, LevelFilter, WriteLogger, TermLogger, CombinedLogger, TerminalMode};

use crate::props::Props;

const LOG_OFF:&str = "OFF";
const LOG_ERROR:&str = "ERROR";
const LOG_WARN:&str = "WARN";
const LOG_INFO:&str = "INFO";
const LOG_DEBUG:&str = "DEBUG";
const LOG_TRACE:&str = "TRACE";

pub fn init_log(props:Props){
    let log_level_prop:&str = &(props.clone().log_level.clone());
    let log_level = match log_level_prop{
        LOG_ERROR => LevelFilter::Error,
        LOG_OFF => LevelFilter::Off,
        LOG_DEBUG => LevelFilter::Debug,
        LOG_WARN => LevelFilter::Warn,
        LOG_INFO => LevelFilter::Info,
        LOG_TRACE => LevelFilter::Trace,
        _ => LevelFilter::Off
    };

    let mut log_config = ConfigBuilder::new();
    log_config.set_time_format(props.log_time_format.clone());

    match metadata(props.log_file_path.clone()){
        Ok(md) => {
            if md.is_file(){
                let log_file = OpenOptions::new().append(true).open(props.log_file_path.clone()).unwrap();
                init_logger(props.log_to_console, log_level, log_config.build(), log_file);
            }
        }
        Err(_e) => {
            let log_file = OpenOptions::new().create(true).append(true).open(props.log_file_path.clone()).unwrap();
            init_logger(props.log_to_console, log_level, log_config.build(), log_file);
        }
    };
}

fn init_logger(log_to_console:bool, log_level:LevelFilter, log_config:Config, log_file:File){
    if log_to_console{
        CombinedLogger::init(
            vec![
                TermLogger::new(log_level, log_config.clone(), TerminalMode::Mixed),
                WriteLogger::new(log_level, log_config.clone(), log_file),
            ]
        ).unwrap();    
    }else{
        WriteLogger::init(log_level, log_config.clone(), log_file).unwrap();
    }
}
