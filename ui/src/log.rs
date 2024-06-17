//! Simplifies the debug log

use core::fmt::{self, Display};

/// A data type that provides
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Log {
    /// File that the log reports from
    pub file: String,
    /// The line that the log is returned from
    pub line: u32,
    /// The message
    pub message: String,
    /// The severity of the message
    pub severity: Severity,
}

impl Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}:{} > {}",
            self.severity, self.file, self.line, self.message
        )
    }
}

/// The severity of the warning
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Severity {
    /// Application must panic
    Crash,
    /// Some unintended effect has performed, but crash is not required
    Error,
    /// Notification / Warnings
    Warning,
    /// Info
    Info,
}

impl Log {
    /// Formats and prints to the log
    pub fn log(&self) {
        println!("{self}");
    }
}

impl Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Crash => write!(f, "Crash"),
            Self::Error => write!(f, "Error"),
            Self::Warning => write!(f, "Warning"),
            Self::Info => write!(f, "Info"),
        }
    }
}

/// Creates a log message that causes the application to crash
#[macro_export]
macro_rules! crash {
    ($msg: expr) => {
        $crate::log::Log {
            file: file!().to_owned(),
            line: line!(),
            message: $msg.into(),
            severity: $crate::log::Severity::Crash,
        }
    };
}

/// Creates a log message that provides info
#[macro_export]
macro_rules! info {
    ($msg: expr) => {
        $crate::log::Log {
            file: file!().to_owned(),
            line: line!(),
            message: $msg.into(),
            severity: $crate::log::Severity::Info,
        }
    };
}

/// Creates a log message that provides a warning
#[macro_export]
macro_rules! warning {
    ($msg: expr) => {
        $crate::log::Log {
            file: file!().to_owned(),
            line: line!(),
            message: $msg.into(),
            severity: $crate::log::Severity::Warning,
        }
    };
}

/// Creates a log message that indicates an error, but not enough to crash
#[macro_export]
macro_rules! error {
    ($msg: expr) => {
        $crate::log::Log {
            file: file!().to_owned(),
            line: line!(),
            message: $msg.into(),
            severity: $crate::log::Severity::Error,
        }
    };
}
