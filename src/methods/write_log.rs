use std::env;

use bson::{doc, Document};
use chrono::Local;
use futures::TryStreamExt;
use mongodb::{
    options::{ClientOptions, FindOptions, ServerApi, ServerApiVersion},
    Client, Collection,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogInputs {
    pub username: String,
    pub password: String,
    pub date: String,
    pub time: String,
    pub muscle_group: String,
    pub intensity: String,
}

pub async fn write_log(
    LogInputs {
        username,
        password,
        date,
        time,
        muscle_group,
        intensity,
    }: LogInputs,
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

    let new_log_doc = doc! {
        "username": username.clone(),
        "date": date,
        "time": time,
        "muscle_group": muscle_group,
        "intensity": intensity,
    };

    let filter_user = doc! {
        "_id": username.clone().trim(),
        "password": password.clone().trim(),
    };

    let find_user = FindOptions::builder()
        .sort(doc! {
        "_id": 1,
        "password": 1})
        .build();
    let user_db: Collection<Document> = client.database("gym_tracker").collection("users");

    let mut cursor = user_db.find(filter_user, find_user).await?;

    if let Some(_user) = cursor.try_next().await? {
        log_db.insert_one(new_log_doc.clone(), None).await?;

        println!(
            "New log inserted for: {},
            at
            {}!",
            username.clone(),
            Local::now(),
        );
    } else {
        println!("Sorry, you need to make an account first.")
    }

    Ok(())
}
