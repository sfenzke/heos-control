use std::error::Error;

use heos::Connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    match Connection::connect() {
        Ok(mut connection) => {
            println!("Connected to heos Device");

            connection.get_devices()?;
        },
        Err(error) => {println!("Error while connecting to HEOS device: {}", &error)}
    }

    Ok(())
}
