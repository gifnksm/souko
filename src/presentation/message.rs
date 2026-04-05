use std::fmt;

macro_rules! _message_info {
    ($($arg:tt)*) => {
        $crate::presentation::message::_info(::std::format_args!($($arg)*))
    };
}

macro_rules! _message_warn {
    ($($arg:tt)*) => {
       $crate::presentation::message::_warn(::std::format_args!($($arg)*))
    };
}

pub(in crate::presentation) use _message_info as info;
pub(in crate::presentation) use _message_warn as warn;

pub(in crate::presentation) fn _info(message: fmt::Arguments<'_>) {
    eprintln!("info: {message}");
}

pub(in crate::presentation) fn _warn(message: fmt::Arguments<'_>) {
    eprintln!("warning: {message}");
}
