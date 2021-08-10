#[macro_export]
macro_rules! get_env {
    ($name:expr, $type:ty) => {
        std::env::var($name)
            .expect(format!("{} not found", $name).as_str())
            .parse::<$type>()
            .expect(format!("Error parsing {}", $name).as_str())
    };

    ($name:expr) => {
        std::env::var($name).expect(format!("{} not found", $name).as_str())
    };
}
