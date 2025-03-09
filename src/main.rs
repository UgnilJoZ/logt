use std::{
    io::{stderr, stdout, BufRead, BufReader, Read, Result}, process::{Command, Stdio}, sync::mpsc::{channel, Sender}, thread,
};

fn handle_output(stdio: impl Read, id: &'static str, sender: Sender<(&str, Result<String>)>) {
    let reader = BufReader::new(stdio);
    for line in reader.lines() {
        sender.send((id, line));
    }
}

fn main() {
    let mut subprocess = Command::new("seq")
        .arg("4")
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Launching child process");
    let child_stdout = subprocess
        .stdout
        .take()
        .expect("Connect to stdout of child process");
    let child_stderr = subprocess
        .stderr
        .take()
        .expect("Connect to stderr of child process");
    let stdout = stdout().lock();
    let stderr = stderr().lock();

    let (stdout_sender, receiver) = channel();
    let stderr_sender = stdout_sender.clone();

    let stdout_thread = thread::spawn(move || handle_output(child_stdout, "stdout", stdout_sender));
    let stderr_thread = thread::spawn(move || handle_output(child_stderr, "stderr", stderr_sender));

    for (id, line) in receiver.iter() {
        if id == "stderr" {
            match line {
                Ok(line) => eprintln!("[{id}] {line}"),
                Err(e) => eprint!("Err reading {id}: {e}"),
            }
        } else {
            match line {
                Ok(line) => println!("[{id}] {line}"),
                Err(e) => print!("Err reading {id}: {e}"),
            }
        }
    }

    stdout_thread
        .join()
        .expect("Join stdout catching thread");
    stderr_thread
        .join()
        .expect("Join stderr catching thread");
}
