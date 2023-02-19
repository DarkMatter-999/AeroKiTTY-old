use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, Command, Stdio},
};

pub struct Term {
    process: Child,
}

impl Term {
    pub fn new(ps: String, args: String) -> Child {
        let process = Command::new(ps)
            .arg(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("failed to execute process");

        process
    }
}

pub fn run(ps: String, args: String) {
    let mut process = Command::new(ps)
        .arg(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to execute process");

    {
        let stdin = process.stdin.as_mut().unwrap();
        // let mut stdin_writer = BufWriter::new(stdin);
        stdin.write_all(b"ls -la\r\n").unwrap();

        drop(stdin);

        let stdout = process.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            println!("{:?}", line);
        }
        println!("Here");
    }

    // process.wait().unwrap();
}
