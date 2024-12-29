#[macro_export]
macro_rules! debug_panic {
    ($msg:expr) => {
        if cfg!(debug_assertions) {
            panic!("{}", $msg);
        }
    };
}
