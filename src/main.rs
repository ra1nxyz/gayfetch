mod modules;

fn main() {
    println!("{}@{}", modules::user::get_user(), modules::hostname::get_host());
    println!("{:#?}", modules::os::get_os());
    println!("{:#?}", modules::processor::get_processor());
    println!("{:#?}", modules::memory::get_host_memory());
    println!("{:#?}", modules::terminal::get_terminal_info());
}
