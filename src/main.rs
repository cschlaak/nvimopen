#![windows_subsystem = "windows"]

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{thread, time::Duration};
use std::net::{TcpListener, SocketAddr};

use regex::Regex;

const SERVER:    &str = "127.0.0.1:13379";
const NVIM_NAME: &str = "nvim";
const GUI_NAME:  &str = "neovide";

const COMMAND_ERROR_MSG: &str = "Failed to execute commmand";

fn is_server_running() -> bool {
    let addr: SocketAddr = SERVER.parse().unwrap();
    match TcpListener::bind(addr) {
        Ok(_) => false,
        Err(_) => true,
    }
}

fn abs_path(p: &str) -> String {
   let path = Path::new(p);

   let abs: PathBuf = if path.is_absolute() {
       path.to_path_buf()
   } else {
       env::current_dir().unwrap().join(path)
   };

   return abs.to_string_lossy().into_owned()
}

pub fn init_command(program: &str) -> Command {
	let mut cmd = Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
	cmd
}

fn main() {
    let file_line_regex: Regex = Regex::new(r"^(.+?)(?::(\d+))?$").unwrap();

    let args: Vec<(String, Option<String>)> = env::args().skip(1)
        .map(|arg| {
            let captures = file_line_regex.captures(&arg).unwrap();
            let file = abs_path(captures.get(1).unwrap().as_str());
            let line: Option<String> = if let Some(l) = captures.get(2) {
                Some(l.as_str().to_string())
            } else {
                None
            };
            (file, line)
        }).collect();

    println!("args = {:?}", args);

    if !is_server_running() {
        init_command(GUI_NAME).arg("--").arg("--listen").arg(SERVER).spawn().expect(COMMAND_ERROR_MSG);
        thread::sleep(Duration::from_millis(100));
    }

    for (file, line) in args {
        init_command(NVIM_NAME).arg("--server").arg(SERVER).arg("--remote-tab-silent").arg(file).status().expect(COMMAND_ERROR_MSG);

        if let Some(l) = line {
            let vimcmd = format!("<ESC>{}gg", l);
            init_command(NVIM_NAME).arg("--server").arg(SERVER).arg("--remote-send").arg(vimcmd).status().expect(COMMAND_ERROR_MSG);
        }
    }
    init_command(NVIM_NAME).arg("--server").arg(SERVER).arg("--remote-send").arg("<ESC>:NeovideFocus<CR>").status().expect(COMMAND_ERROR_MSG);
}
