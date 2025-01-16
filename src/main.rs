use std::{error::Error, net::{TcpListener, TcpStream}};

fn main() -> Result<(), Box<dyn Error>> {
    let server = TcpListener::bind("0.0.0.0")?;
    for stream in server.incoming()  {
        let Ok(mut stream) = stream else {
            continue;
        };
    }

    Ok(())
}
