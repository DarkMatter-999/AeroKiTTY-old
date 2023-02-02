use libc::execle;
use nix::{
    fcntl::OFlag,
    pty::{grantpt, posix_openpt, ptsname, unlockpt, PtyMaster},
    unistd::{close, fork, read, ForkResult},
};
use std::os::fd::IntoRawFd;
use std::{
    fs::File,
    os::fd::RawFd,
    process::{Command, ExitStatus},
};
pub struct pty {
    pub master: PtyMaster,
    pub slave: RawFd,
}

impl pty {
    pub fn new() -> pty {
        let master: PtyMaster = match posix_openpt(OFlag::O_RDWR | OFlag::O_NOCTTY) {
            Ok(n) => n,
            Err(n) => panic!("posix_openpt {}", n),
        };

        match grantpt(&master) {
            Ok(_) => {}
            Err(n) => panic!("grantpt {}", n),
        }

        match unlockpt(&master) {
            Ok(_) => {}
            Err(n) => panic!("unlockpt {}", n),
        }

        let slave_name;
        match unsafe { ptsname(&master) } {
            Ok(s) => slave_name = s,
            Err(n) => panic!("ptsname {}", n),
        }

        println!("{}", slave_name);
        let slave = match File::open(slave_name) {
            Ok(n) => n.into_raw_fd(),
            Err(n) => panic!("{}", n),
        };

        pty {
            master: master,
            slave: slave,
        }
    }

    pub fn spawn(&mut self, command: String) {
        let p = match unsafe { fork() } {
            Ok(n) => n,
            Err(n) => {
                let e = close(self.slave);
                match e {
                    Ok(_) => (),
                    Err(n) => panic!("spawn {}", n),
                }
                panic!("Failed to fork {}", n)
            }
        };

        let _status;
        if let ForkResult::Child = p {
            _status = Command::new(command)
                .status()
                .expect("Could not spawn the shell");
            std::thread::sleep(std::time::Duration::from_millis(500));
            std::process::exit(0);
        }
    }

    pub fn read_stdout(&mut self) -> Option<Vec<u8>> {
        let mut read_buffer = [0; 65536];
        let read_result = read(self.slave, &mut read_buffer);
        match read_result {
            Ok(bytes_read) => Some(read_buffer[..bytes_read].to_vec()),
            Err(_e) => None,
        }
    }
}
