#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {{
        $crate::Logger::global().debug(&format!($($arg)*), Some(file!()));
    }};
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {{
        $crate::Logger::global().info(&format!($($arg)*), Some(file!()));
    }};
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {{
        $crate::Logger::global().warn(&format!($($arg)*), Some(file!()));
    }};
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {{
        $crate::Logger::global().error(&format!($($arg)*), Some(file!()));
    }};
}