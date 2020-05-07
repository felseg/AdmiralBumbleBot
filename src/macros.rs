#[macro_export]
macro_rules! get_env {
    ($name:expr, $type:ty) => {
        std::env::var($name)
            .expect(format!("{} not found", $name).as_str())
            .parse::<$type>()
            .expect(format!("Error parsing {}", $name).as_str());
    };

    ($name:expr) => {
        std::env::var($name).expect(format!("{} not found", $name).as_str())
    };
}

#[cfg(test)]
mod macro_tests {
    use crate::macros;
    use dotenv::dotenv;

    #[test]
    fn env_test() {
        dotenv().ok();

        let numeric = get_env!("ABB_MUTE_ROLE", u64);
        let string_slice = get_env!("ABB_MUTE_ROLE");
        assert_eq!(numeric, Variables::mute_role());
        assert_eq!(string_slice, Variables::mute_role().to_string().as_str());
    }
}
