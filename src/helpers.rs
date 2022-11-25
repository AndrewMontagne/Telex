#[macro_export]
macro_rules! strlit {
    ($a:expr) => {
        $a.to_string()
    };
}
