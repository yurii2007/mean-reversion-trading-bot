use strategy::Strategy;

mod logger;
mod strategy;

#[tokio::main]
async fn main() -> Result<(), String> {
    logger::init_logger();
    
    let strategy = Strategy::new();

    Ok(())
}
