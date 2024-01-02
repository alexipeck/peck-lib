#[macro_export]
macro_rules! impl_error_wrapper {
    ($wrapper_type:ident, $inner_type:ty) => {
        #[derive(Error, Debug)]
        pub struct $wrapper_type(#[from] pub $inner_type);

        impl fmt::Display for $wrapper_type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}
