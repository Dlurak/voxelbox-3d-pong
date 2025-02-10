pub trait ResultExtender {
    fn log<T>(&self, msg: T)
    where
        T: std::fmt::Display;
}

impl<V, E> ResultExtender for Result<V, E> {
    fn log<T>(&self, msg: T)
    where
        T: std::fmt::Display,
    {
        if self.is_err() {
            eprintln!("{}", msg);
        }
    }
}
