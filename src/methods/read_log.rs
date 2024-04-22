use std::env;

use bson::{doc, Document};
use futures::TryStreamExt;
use mongodb::{
    options::{ClientOptions, FindOptions, ServerApi, ServerApiVersion},
    Client, Collection,
};

pub async fn read_all_logs(username: String, password: String) -> mongodb::error::Result<()> {
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

    // Create connection to Log database.
    let log_db: Collection<Document> = client.database("gym_tracker").collection("logs");

    // Create connection to user database.
    let user_db: Collection<Document> = client.database("gym_tracker").collection("users");

    // Create filter for user
    let filter_user = doc! {
        "_id": username.clone().trim(),
        "password": password.clone().trim(),
    };
    let find_user = FindOptions::builder()
        .sort(doc! {
        "_id": 1,
        "password": 1})
        .build();

    let mut user_cursor = user_db.find(filter_user, find_user).await?;

    // Create filter and cursor for logs
    let filter_logs = doc! {
        "username": username.clone().trim(),
    };
    let find_logs = FindOptions::builder()
        .sort(doc! {
        "username": 1})
        .build();

    let mut log_cursor = log_db.find(filter_logs, find_logs).await?;

    // Use both the user and the log cursor to find relevant logs.
    if let Some(_user) = user_cursor.try_next().await? {
        while let Some(logs) = log_cursor.try_next().await? {
            println!(
                "Log recieved!: 
                {:#?}",
                logs
            )
        }
    } else {
        println!("Sorry, no logs were found for that account. You can create a new account, or if you already have one; write your first log!")
    }
    Ok(())
}

pub async fn read_specific_log(
    username: String,
    password: String,
    date: String,
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

    // Create connection to Log database.
    let log_db: Collection<Document> = client.database("gym_tracker").collection("logs");

    // Create connection to user database.
    let user_db: Collection<Document> = client.database("gym_tracker").collection("users");

    // Create filter for user
    let filter_user = doc! {
        "_id": username.clone().trim(),
        "password": password.clone().trim(),
    };
    let find_user = FindOptions::builder()
        .sort(doc! {
        "_id": 1,
        "password": 1})
        .build();

    let mut user_cursor = user_db.find(filter_user, find_user).await?;

    // Create filter for logs
    let filter_log = doc! {
        "username": username.clone().trim(),
        "date": date.clone().trim()
    };
    let find_logs = FindOptions::builder()
        .sort(doc! {"username": 1, "date": 1})
        .build();

    let mut log_cursor = log_db.find(filter_log, find_logs).await?;

    // Use both the user and the log cursor to find relevant logs.
    if let Some(_user) = user_cursor.try_next().await? {
        while let Some(logs) = log_cursor.try_next().await? {
            println!(
                "Found logs for user {} with date the date of {}: 
            {:#?}",
                username, date, logs
            )
        }
    } else {
        println!("Sorry, no logs were found for that account. You can create a new account, or if you already have one; write your first log!")
    }

    Ok(())
}
