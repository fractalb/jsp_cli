use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();

    stdin.read_to_string(&mut buffer)?;
    println!("{}", buffer);

    if let Some(x) = jsp::jsp_parse_json(&mut buffer) {
        println!("===JSON START====");
        println!("{}", x);
        println!("===JSON END======");
        Ok(())
    } else {
        println!("Invalid JSON");
        Err(io::Error::new(io::ErrorKind::Other, "Invalid JSON"))
    }
}
