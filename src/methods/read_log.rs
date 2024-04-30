use std::env;

use bson::{doc, Document};
use futures::TryStreamExt;
use mongodb::{
    options::{AggregateOptions, ClientOptions, FindOptions, ServerApi, ServerApiVersion},
    Client, Collection,
};
use prettytable::{Cell, Row, Table};

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

    let log_db: Collection<Document> = client.database("gym_tracker").collection("logs");
    let user_db: Collection<Document> = client.database("gym_tracker").collection("users");

    // Check if the user exists and the password matches
    let filter_user = doc! {
        "_id": username.trim(),
        "password": password.trim(),
    };

    if let Some(_) = user_db.find_one(filter_user.clone(), None).await? {
        let pipeline = vec![
            // Filter logs by the current user's username
            doc! {
                "$match": {
                    "username": username.trim(),
                }
            },
            doc! {
                "$group": {
                    "_id": {
                        "username": "$username",
                        "date": "$date",
                        "time": "$time",
                        "muscle_group": "$muscle_group",
                        "intensity": "$intensity"
                    }
                }
            },
            doc! {
                "$project": {
                    "_id": 0,
                    "username": "$_id.username",
                    "date": "$_id.date",
                    "time": "$_id.time",
                    "muscle_group": "$_id.muscle_group",
                    "intensity": "$_id.intensity",
                }
            },
            // Sort logs by date in ascending order
            doc! {
                "$sort": {
                    "date": 1
                }
            },
        ];

        let options = AggregateOptions::builder().build();

        let mut cursor = log_db.aggregate(pipeline, options).await?;

        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("User"),
            Cell::new("Date"),
            Cell::new("Time"),
            Cell::new("Muscle Group"),
            Cell::new("Intensity"),
        ]));

        while let Some(doc) = cursor.try_next().await? {
            let username = doc.get_str("username").unwrap().to_string();
            let date = doc.get_str("date").unwrap().to_string();
            let time = doc.get_str("time").unwrap().to_string();
            let muscle_group = doc.get_str("muscle_group").unwrap().to_string();
            let intensity = doc.get_str("intensity").unwrap().to_string();

            table.add_row(Row::new(vec![
                Cell::new(&username),
                Cell::new(&date),
                Cell::new(&time),
                Cell::new(&muscle_group),
                Cell::new(&intensity),
            ]));
        }
        table.printstd();
    } else {
        println!("Sorry, no logs were found for that account. You can create a new account, or if you already have one; write your first log!");
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
