#[macro_export]
macro_rules! get_env_var {
    ($var_name:expr) => {
        match std::env::var($var_name) {
            Ok(value) => value,
            Err(err) => {
                match err {
                    std::env::VarError::NotPresent => {
                        eprintln!("{} not found, must be set. Exiting.", $var_name);
                    }
                    std::env::VarError::NotUnicode(_) => {
                        eprintln!("{} not valid unicode. Exiting.", $var_name);
                    }
                }
                std::process::exit(3)
            }
        }
    };
}
