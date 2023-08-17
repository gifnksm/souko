macro_rules! itry {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(err) => return Some(Err(From::from(err))),
        }
    };
}

macro_rules! bail {
    ($e:expr) => {
        return Err(From::from($e))
    };
}
