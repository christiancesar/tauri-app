use dotenv::dotenv;
use std::env;
use std::error::Error;

use mongodb::{options::ClientOptions, Client};

pub async fn db_connection() -> Result<Client, Box<dyn Error>> {
   dotenv().ok();
   let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

   let options = ClientOptions::parse(&url).await?;

   let client = Client::with_options(options).unwrap();
   
   print!("Connected to database", );

   Ok(client)
}
