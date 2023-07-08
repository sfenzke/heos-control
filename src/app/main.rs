use heos::Connection;

#[tokio::main]
async fn main() {
    match Connection::connect() {
        Ok(_) => {println!("Connected to heos Device")},
        Err(error) => {println!("Error while connecting to HEOS device: {}", &error)}
    }
}
