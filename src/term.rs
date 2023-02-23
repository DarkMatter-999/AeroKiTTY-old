use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, Command, Stdio},
};

pub struct Term {
    process: Child,
}

impl Term {
    pub fn new(ps: String, args: String) -> Term {
        let process = Command::new(ps)
            .arg(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("failed to execute process");

        Term { process }
    }

    pub fn read_stdio(&mut self) {
        let stdout = self.process.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        print!("{:?}", stdout_lines);

        // for line in stdout_lines {
        //     if let Ok(line) = line {
        //         println!("{:?}", line);
        //     }
        // }
    }

    pub fn write_stdin(&mut self) {
        let stdin = self.process.stdin.as_mut().unwrap();
        stdin.write_all(b"ls -la\r\n").unwrap();
    }
}
