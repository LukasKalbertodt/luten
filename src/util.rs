/// Matches the current expression. On `None`, `Ok(None)` is `return`ed; on
/// `Some`, the inner value is returned from the expression.
macro_rules! try_opt_ok {
    ($e:expr) => {
        match $e {
            None => return Ok(None),
            Some(v) => v,
        }
    }
}
