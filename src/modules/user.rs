pub fn get_user() -> String {
    return std::env::var("USER").unwrap(); // can this ever not exist on distros??? maybe change
                                           // for ok and filter
}
