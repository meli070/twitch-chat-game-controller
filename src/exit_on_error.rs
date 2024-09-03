use log::error;

pub trait ExitOnError<T> {
    /// exit the program on error state else return `T`
    fn exit_on_error(self, info: &str) -> T;
}

impl<T, E> ExitOnError<T> for Result<T, E> {
    fn exit_on_error(self, info: &str) -> T {
        if let Ok(result) = self {
            result
        } else {
            error!("{}", info);
            std::process::exit(1);
        }
    }
}

impl<T> ExitOnError<T> for Option<T> {
    fn exit_on_error(self, info: &str) -> T {
        if let Some(result) = self {
            result
        } else {
            error!("{}", info);
            std::process::exit(1);
        }
    }
}
