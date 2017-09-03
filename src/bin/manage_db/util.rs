//! Several helper functions and types.

use std::fmt::Debug;
use std::io::{self, Write};

use luten::errors::*;

#[macro_export]
macro_rules! value_t_opt {
    ($matches:ident, $name:expr, $ty:ty) => {
        if $matches.is_present($name) {
            Some(value_t!($matches, $name, $ty).unwrap_or_else(|e| e.exit()))
        }  else {
            None
        }
    }
}

/// Reads one line from stdin and trims leading and trailing whitespace.
pub fn read_trimmed_line() -> Result<String> {
    let mut out = String::new();
    io::stdin().read_line(&mut out)?;
    Ok(out.trim().to_owned())
}

/// A helper struct which holds the global config. Actions influenced by the
/// global config should be done through this struct.
pub struct Global {
    pub compact_output: bool,
}

impl Global {
    pub fn debug_output<T: Debug>(&self, v: T) {
        if self.compact_output {
            println!("{:?}", v);
        } else {
            println!("{:#?}", v);
        }
    }
}

/// Reads a type from stdin with a proper message.
pub fn read<T: FromStdin>(field_name: &str) -> Result<T> {
    loop {
        print!("| {} ({}): ", field_name, <T as FromStdin>::desc());
        io::stdout().flush()?;

        let line = read_trimmed_line()?;
        match T::from_input(&line) {
            Ok(v) => return Ok(v),
            Err(e) => println!("Input error: {}", e),
        }
    }
}

/// Types that can be read from stdin.
pub trait FromStdin: Sized {
    fn from_input(s: &str) -> StdResult<Self, String>;
    fn desc() -> String;
}

macro_rules! impl_from_stdin {
    ($ty:ident) => {
        impl FromStdin for $ty {
            fn from_input(s: &str) -> StdResult<Self, String> {
                s.parse::<$ty>().map_err(|e| e.to_string())
            }
            fn desc() -> String {
                format!("`{}`", stringify!($ty))
            }

        }

        impl FromStdin for Option<$ty> {
            fn from_input(s: &str) -> StdResult<Self, String> {
                if s.is_empty() {
                    Ok(None)
                } else {
                    s.parse::<$ty>()
                        .map(|v| Some(v))
                        .map_err(|e| e.to_string())
                }
            }

            fn desc() -> String {
                format!("`Option<{}>`, empty input for `None`", stringify!($ty))
            }
        }
    }
}

impl_from_stdin!(u8);
impl_from_stdin!(u16);
impl_from_stdin!(u32);
impl_from_stdin!(u64);
impl_from_stdin!(i8);
impl_from_stdin!(i16);
impl_from_stdin!(i32);
impl_from_stdin!(i64);
impl_from_stdin!(String);
