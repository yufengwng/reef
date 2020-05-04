use std::io;
use std::io::Write;
use std::process::Command;

fn main() {
    let prompt = "$ ";
    let mut buffer = String::new();

    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        buffer = buffer.trim().to_owned();
        if buffer.is_empty() {
            continue;
        }

        let line = buffer.split_whitespace().collect::<Vec<_>>();
        match Command::new(line[0]).args(&line[1..]).output() {
            Ok(output) => {
                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
            }
            Err(_) => println!("reef: failed to execute command"),
        }
    }
}
