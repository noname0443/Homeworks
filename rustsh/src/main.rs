#![cfg_attr(debug_assertions, allow(unused_imports))]
use libc::{self, STDOUT_FILENO};
use nix::{fcntl::{open, OFlag}, sys::{stat::Mode, wait::waitpid}, unistd::{self, close, dup2, execvp, fork, ForkResult}};
use std::{self, default, ffi::CString};

fn main() {
    let fork_num = unsafe {
        fork().unwrap()
    };
    match fork_num {
        ForkResult::Child => {
            let fd = open("hi.txt", OFlag::O_WRONLY|OFlag::O_CREAT, Mode::from_bits_truncate(0o664)).unwrap();
            dup2(fd, STDOUT_FILENO).unwrap();
            close(fd).unwrap();
            let cmd = CString::new("echo").unwrap();
            let args = vec![cmd.clone(), CString::new("hi!").unwrap()];
            execvp(&cmd, args.as_slice()).unwrap();
        },
        ForkResult::Parent { child } => {
            waitpid(child, None).unwrap();
        }
    }
}
