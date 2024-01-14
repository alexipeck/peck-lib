#[macro_export]
macro_rules! impl_error_wrapper {
    ($wrapper_type:ident, $inner_type:ty) => {
        #[derive(thiserror::Error, Debug)]
        pub struct $wrapper_type(#[from] pub $inner_type);

        impl std::fmt::Display for $wrapper_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_error_wrapper_clonable {
    ($wrapper_type:ident, $inner_type:ty) => {
        #[derive(thiserror::Error, Debug, Clone)]
        pub struct $wrapper_type(#[from] pub $inner_type);

        impl std::fmt::Display for $wrapper_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}
