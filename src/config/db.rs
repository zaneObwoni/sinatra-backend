use mongodb::{options::ClientOptions, Client, bson::doc};


pub async fn get_mongo_client() -> Result<Client, mongodb::error::Error> {
    let mongo_uri = std::env::var("MONGO_URI")
        .expect("😱 MONGO_URI environment must be set");

    let mut client_options = ClientOptions::parse(&mongo_uri).await.unwrap();

    client_options.app_name = Some("sinatra".to_string());
    let client = Client::with_options(client_options)?;

    // Ping the server to see if you can connect to the cluster
    client
        .database("sinatra")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("🤯 CONNECTED TO DB SUCCESSFULLY!!!");

    Ok(client)
}
