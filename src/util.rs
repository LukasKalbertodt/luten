macro_rules! try_opt_ok {
    ($e:expr) => {
        match $e {
            None => return Ok(None),
            Some(v) => v,
        }
    }
}
