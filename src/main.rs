use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();

    stdin.read_to_string(&mut buffer)?;
    println!("{}", buffer);

    match jsp::jsp_parse_json(&buffer) {
        Ok(x) => {
            println!("===JSON START====");
            println!("{}", x);
            println!("===JSON END======");
            Ok(())
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(io::Error::other("Invalid JSON"))
        }
    }
}
