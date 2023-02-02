use nix::unistd::close;
use AeroKiTTY::pty::pty;
fn main() {
    let mut pty = pty::new();
    pty.spawn("/bin/sh".to_string());

    let mut read_buffer = vec![];
    loop {
        match pty.read_stdout() {
            Some(mut read_bytes) => {
                read_buffer.append(&mut read_bytes);
            }
            None => {
                println!("{:?}", String::from_utf8(read_buffer).unwrap());
                break;
            }
        }
    }

    close(pty.slave).unwrap();
}
