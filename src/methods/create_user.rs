use std::env;

use bson::doc;
use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};

pub async fn create_user(
    username: String,
    name: String,
    password: String,
) -> mongodb::error::Result<()> {
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let mut client_options = ClientOptions::parse(client_uri)
        .await
        .expect("Failure to connect to Atlas cluster.");

    // Set the server_api field of the client_options object to set the version of the Stable API on the client
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Get a handle to the cluster
    let client = Client::with_options(client_options).expect("Failure to retrieve client options.");

    // Create connection to User database.
    let user_db = client.database("gym_tracker").collection("users");

    // Create User template for CLI
    let new_user_doc = doc! {
        "_id": username,
        "user": name,
        "password": password,
    };

    // Inserts user into database, and establishes unique _id for logs.
    let insert_user_result = user_db
        .insert_one(new_user_doc.clone(), None)
        .await
        .expect("Failure to insert new document...");
    println!(
        "New user created! username: {}",
        insert_user_result.inserted_id
    );
    Ok(())
}
