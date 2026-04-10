use std::fmt;

macro_rules! _message_info {
    ($($arg:tt)*) => {
        $crate::cli::message::_info(::std::format_args!($($arg)*))
    };
}

macro_rules! _message_warn {
    ($($arg:tt)*) => {
       $crate::cli::message::_warn(::std::format_args!($($arg)*))
    };
}

pub(in crate::cli) use _message_info as info;
pub(in crate::cli) use _message_warn as warn;

pub(in crate::cli) fn _info(message: fmt::Arguments<'_>) {
    eprintln!("info: {message}");
}

pub(in crate::cli) fn _warn(message: fmt::Arguments<'_>) {
    eprintln!("warning: {message}");
}
