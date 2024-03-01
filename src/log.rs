use {
    colored::{ColoredString, Colorize},
    std::{
        env,
        error::Error,
        fmt::{self, Display, Formatter},
        process,
        str::FromStr,
        string::ToString,
    },
};

use crate::errors::CompilerError;

const SURROUNDING_LINES: usize = 3;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "DEBUG" => Ok(LogLevel::Debug),
            "INFO" => Ok(LogLevel::Info),
            "WARN" => Ok(LogLevel::Warn),
            "ERROR" => Ok(LogLevel::Error),
            _ => Err(format!("Invalid log level: {}", s)),
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let str = match self {
            LogLevel::Debug => "DEBUG".to_string(),
            LogLevel::Info => "INFO".to_string(),
            LogLevel::Warn => "WARN".to_string(),
            LogLevel::Error => "ERROR".to_string(),
        };
        write!(f, "{}", str)
    }
}

pub fn debug(message: &str) {
    log(message, LogLevel::Debug);
}

pub fn info(message: &str) {
    log(message, LogLevel::Info);
}

pub fn warn(message: &str) {
    log(message, LogLevel::Warn);
}

pub fn error(message: &str) {
    log(message, LogLevel::Error);
}

fn log(message: &str, log_level: LogLevel) {
    if log_level >= env_log_level() {
        println!(
            "{:>5} {}",
            color(&log_level.to_string(), log_level),
            message
        );
    }
}

fn color(s: &str, log_level: LogLevel) -> ColoredString {
    match log_level {
        LogLevel::Debug => s.blue(),
        LogLevel::Info => s.green(),
        LogLevel::Warn => s.yellow(),
        LogLevel::Error => s.red(),
    }
}

pub fn env_log_level() -> LogLevel {
    let env_log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "DEBUG".to_string());
    LogLevel::from_str(&env_log_level).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1)
    })
}

pub fn exit<T: Error>(err: T) -> ! {
    let error_message = err.to_string();
    error(&error_message);
    process::exit(1)
}

pub fn exit_with_info(err: CompilerError, source: &str) -> ! {
    let full_message = err.to_string();
    error(&full_message);
    let error_line = match err {
        CompilerError::Lexer(_, line) => line,
        CompilerError::Parser(_, line) => line,
        CompilerError::Interpreter(_, line) => line,
    };

    if error_line < 1 {
        process::exit(1)
    }

    let lines: Vec<&str> = source.split('\n').collect();
    let start = if error_line <= SURROUNDING_LINES {
        1
    } else {
        std::cmp::max(1, error_line - SURROUNDING_LINES)
    };
    let end = std::cmp::min(lines.len(), error_line + SURROUNDING_LINES);

    let short_message = match err {
        CompilerError::Lexer(e, _) => e,
        CompilerError::Parser(e, _) => e,
        CompilerError::Interpreter(e, _) => e,
    };

    for i in start..=end {
        let line = lines[i - 1];
        let padding = " ".repeat(5 - i.to_string().len());
        let num = format!("{}{}| ", padding, i).blue();
        println!("{}{}", num, line);
        if i == error_line {
            let padding = " ".repeat(num.len());
            println!("{}{}", padding, "^".repeat(line.len()).red());
            println!("{}{}", padding, short_message.red());
        }
    }

    process::exit(1)
}
