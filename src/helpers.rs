#[macro_export]
/// A String literal
macro_rules! strlit {
    ($a:expr) => {
        $a.to_string()
    };
}
