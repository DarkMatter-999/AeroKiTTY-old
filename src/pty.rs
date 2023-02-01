use nix::{
    fcntl::OFlag,
    pty::{grantpt, posix_openpt, ptsname, unlockpt, PtyMaster},
};
pub struct pty {
    master: PtyMaster,
    slave: u64,
}

impl pty {
    pub fn new() -> pty {
        let mut master: PtyMaster;
        match posix_openpt(OFlag::O_RDWR | OFlag::O_NOCTTY) {
            Ok(n) => master = n,
            Err(n) => panic!("posix_openpt"),
        }

        match grantpt(&master) {
            Ok(n) => {}
            Err(n) => panic!("grantpt"),
        }

        match unlockpt(&master) {
            Ok(n) => {}
            Err(n) => panic!("unlockpt"),
        }

        let mut slave_name;
        match unsafe { ptsname(&master) } {
            Ok(s) => slave_name = s,
            Err(s) => panic!("ptsname"),
        }

        println!("{}", slave_name);
        let slave = 0;

        pty {
            master: master,
            slave: slave,
        }
    }
}
