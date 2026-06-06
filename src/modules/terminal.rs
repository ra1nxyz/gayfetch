use std::fs;

#[derive(Debug)]
pub struct Terminal {
    terminal_name: Option<String>,
    session_type: Option<String>,
    user_shell: Option<String>,
    proc_shell: Option<String>,
}

pub fn get_terminal_info() -> Option<Terminal> {
   let procpid = fs::read_to_string("/proc/self/stat").ok();

   match procpid {
        Some(proc) => {
            
   }
}

fn get_proc_shell_by_pid(procpath: &proc) -> Option<String> {
    
}

fn get_user_shell_by_var(var: &String) -> Option<String> {

}

// later work! i wanna go get a drink :3

