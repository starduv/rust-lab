use std::fs::{self};

pub fn read_to_format<T>(path: &str, formatter: impl FnOnce(String) -> T) -> Result<T, std::io::Error>{
    match fs::read_to_string(path) {
        Ok(input) => {
            Ok(formatter(input))
        },
        Err(error) => Err(error),
    }
}