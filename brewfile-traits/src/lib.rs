use std::str::FromStr;

pub trait Deserialize<T> {
    fn deserialize(cmd: &str) -> T;
}

impl<T> Deserialize<T> for T
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    fn deserialize(cmd: &str) -> T {
        cmd.parse::<T>().expect("Failed to parse command into expected type")
    }
}

