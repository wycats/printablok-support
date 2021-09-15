#[macro_export]
macro_rules! ret {
    ($expr:expr) => {
        Ok($expr?)
    };
}

#[macro_export]
macro_rules! fail {
    ($expr:expr) => {
        Err(Box::new($expr))
    };
}
