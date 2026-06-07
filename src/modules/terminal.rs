use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
pub struct Terminal {
    terminal_name: Option<String>,
    terminal_type: Option<String>,
    terminal_ver: Option<String>,
    user_shell: Option<String>,
    proc_shell: Option<String>,
}

struct TerminalInfo {
    terminal_name: Option<String>,
    terminal_ver: Option<String>,
}

pub fn get_terminal_info() -> Terminal {
    let proc_shell = get_proc_shell_by_pid()
        .or_else(|| get_user_shell_by_env());

    let terminal_info = attempt_get_terminal()
        .unwrap_or(TerminalInfo {
            terminal_name: None,
            terminal_ver: None,
        });

    Terminal {
        terminal_name: terminal_info.terminal_name,
        terminal_type: get_terminal_type_by_env(),
        terminal_ver: terminal_info.terminal_ver,
        user_shell: get_user_shell_by_env(),
        proc_shell: proc_shell,
   }
}

fn get_parent_process_id(process_id: u32) -> Option<u32> {
    let stat = fs::read_to_string(format!("/proc/{process_id}/stat")).ok()?;
    let format = stat.split_once(") ")?.1;
    format.split_whitespace().nth(1)?.parse().ok()
}

fn get_comm_out(process_id: u32) -> Option<String> {
    fs::read_to_string(format!("/proc/{process_id}/comm"))
        .ok()
        .map(|s| s.trim().to_string())
}

fn get_version(process_id: u32) -> Option<String> {                 // this needs to be adjusted
                                                                    // for different terminal cmds
    let version = Command::new(format!("/proc/{process_id}/exe"))
        .arg("--version")
        .output()           // this might be unorthodox..
        .ok()
        .map(|out| String::from_utf8_lossy(&out.stdout).to_string());

    version?.split_whitespace().find(|part| {
        part.chars().any(|c| c.is_ascii_digit())
            && part.chars().all(|c| c.is_ascii_digit() || c == '.')
    })
    .map(|s| s.to_string())
}



fn attempt_get_terminal() -> Option<TerminalInfo> {

    const KNOWN_TERMINALS: &[&str] = &[  // adjust or use different method for actually storing
                                         // these
        "kitty",
        "wezterm-gui",
        "alacritty",
        "ghostty",
        "konsole",
        "gnome-terminal-server",
        "xfce4-terminal",
        "foot",
        "xterm",
        "terminator",
        "tilix",
    ];
    let mut process_id = std::process::id();

    while let Some(parent_id) = get_parent_process_id(process_id) {
        let comm = get_comm_out(parent_id)?;

        if KNOWN_TERMINALS.contains(&comm.as_str()) {
            return Some(TerminalInfo {
                terminal_name: Some(comm),
                terminal_ver: Some(get_version(parent_id)?),
            })
            //return Some(comm);
        }

        if parent_id <= 1 {
            break;
        }
        process_id = parent_id;
    }
    None
}

fn get_proc_shell_by_pid() -> Option<String> {
    let process_id = std::process::id();
    let parent_id = get_parent_process_id(process_id)?;

    get_comm_out(parent_id)
}

fn get_user_shell_by_env() -> Option<String> {
    let shell = std::env::var("SHELL").ok()?;

    Some(Path::new(&shell)
        .file_name()?
        .to_string_lossy()
        .into_owned()           
    )
}

fn get_terminal_type_by_env() -> Option<String> {
    return std::env::var("TERM")
        .ok()
        .filter(|v| !v.is_empty());    // get terminal capability, shit like xterm-kitty,
                                       // xterm-256, linux, etc, standard types not indicative of
                                       // terminal necessarily
}
