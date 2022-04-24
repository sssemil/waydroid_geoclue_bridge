mod manager_gen;
mod client_gen;
mod location_gen;
mod client;


pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    client::run()
}