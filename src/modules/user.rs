pub fn get_user() -> String {
    return std::env::var("USER").unwrap();
}
