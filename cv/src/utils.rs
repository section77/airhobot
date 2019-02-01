macro_rules! err {
    ($($arg:tt)*) => (Err(std::fmt::format(format_args!($($arg)*))));
}

/// TryFrom is currently only in nightly
///   https://github.com/rust-lang/rust/issues/33417
///   maybe in 1.33 or 1.34
pub trait TryFrom<T>: Sized {
    fn try_from(value: T) -> Result<Self, String>;
}

impl TryFrom<i32> for u8 {
    fn try_from(value: i32) -> Result<u8, String> {
        if value < 0 || value > 255 {
            err!("value out of range - value: {}, range: 0 - 255", value)
        } else {
            Ok(value as u8)
        }
    }
}

impl TryFrom<u64> for i32 {
    fn try_from(value: u64) -> Result<i32, String> {
        if value >= 0 as u64 && value <= i32::max_value() as u64 {
            Ok(value as i32)
        } else {
            err!("value out of range")
        }
    }
}

impl TryFrom<i128> for i32 {
    fn try_from(value: i128) -> Result<i32, String> {
        if value >= i32::min_value() as i128 && value <= i32::max_value() as i128 {
            Ok(value as i32)
        } else {
            err!("value out of range")
        }
    }
}
