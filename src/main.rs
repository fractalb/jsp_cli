use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();

    stdin.read_to_string(&mut buffer)?;
    println!("{}", buffer);

    let mut p = buffer.chars().peekable();
    if jsp::jsp_consume_value(&mut p) {
        if p.peek() == None {
            println!("Valid JSON");
            return Ok(());
        }
    }
    println!("Invalid JSON");
    Err(io::Error::new(io::ErrorKind::Other, "Invalid JSON"))
}
