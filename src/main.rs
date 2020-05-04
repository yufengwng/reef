use std::io;
use std::io::Write;

fn main() {
    let prompt = "$ ";
    let mut buffer = String::new();
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();

        buffer = buffer.trim().to_owned();
        if !buffer.is_empty() {
            println!("{}", buffer);
        }
    }
}
