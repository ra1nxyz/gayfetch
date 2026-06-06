use std::fs;

#[derive(Debug)]
pub struct Terminal {
    terminal_name: Option<String>,
    session_type: Option<String>,
    user_shell: Option<String>,
    proc_shell: Option<String>,
}

pub fn get_terminal_info() -> Terminal {

    let proc_shell = get_proc_shell_by_pid();
    let mut proc_shell_return: Option<String> = None;

    match proc_shell {
        Some(value) =>
        {
            proc_shell_return = Some(value);
        }
        _ => { proc_shell_return = get_user_shell_by_env(); }
    }

   return Terminal {
        terminal_name: Some(String::from("test")), // get terminal later im hungry
        session_type: get_session_type_by_env(),
        user_shell: get_user_shell_by_env(),
        proc_shell: proc_shell_return,
   }
}

fn get_proc_shell_by_pid() -> Option<String> {
    let parent_id_read_buf = fs::read_to_string("/proc/self/stat").ok()?; // i think theres a 
    let parent_id = parent_id_read_buf.split_whitespace().nth(3)?;        // better way of doing
                                                                          // this, but just making
                                                                          // read as a buffer so
                                                                          // the value doesnt get
                                                                          // dropped too fast is
                                                                          // wtv 

    let shell = fs::read_to_string(format!("/proc/{}/comm", parent_id)).ok()?;
    return Some(shell.trim().to_string());
}

fn get_user_shell_by_env() -> Option<String> {
    return std::env::var("SHELL").ok();     // only gets user shell not what is running on the
                                            // terminal where program is ran, and may also be
                                            // incorrect if shell is changed within terminal, pid
                                            // for accuracy, acts as fallback
}

fn get_session_type_by_env() -> Option<String> {
    return std::env::var("TERM").ok(); // get terminal capability, shit like xterm-kitty,
                                       // xterm-256, linux, etc, standard types not indicative of
                                       // terminal necessarily
}
