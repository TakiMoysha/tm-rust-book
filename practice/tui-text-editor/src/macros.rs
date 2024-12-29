#[macro_export]
macro_rules! debug_panic {
    ($($t:tt)*) => {{
        if cfg!(debug_assertions) {
            panic!($($t)*);
        }
    }};
}
