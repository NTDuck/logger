pub mod result {
    pub type Fallible<T> = Result<T, anyhow::Error>;
}

pub mod string {
    pub type String = std::string::String;
}

pub use axiom_derive::Erratum;

#[macro_export]
macro_rules! ok {
    ($val:expr) => {
        Ok(Ok($val))
    };
}

#[macro_export]
macro_rules! err {
    ($err:expr) => {
        Ok(Err(vec![$err]))
    };
}

#[macro_export]
macro_rules! errs {
    ($errs:expr) => {
        Ok(Err($errs))
    };
}
