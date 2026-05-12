#[macro_export]
macro_rules! mock_file {
    ($($line:literal),* $(,)?) => {
        concat!($($line, "\n"),*)
    };
}

