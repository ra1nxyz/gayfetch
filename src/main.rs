mod modules;

fn main() {
    println!("{}@{}", modules::user::get_user(), modules::hostname::get_host());
    println!("{:#?}", modules::os::get_os());
}
