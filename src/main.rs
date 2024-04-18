use std::env;

use chrono::offset::Local;
use clap::*;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    options::{ClientOptions, FindOptions, ServerApi, ServerApiVersion},
    Client, Collection,
};
use serde::{Deserialize, Serialize};
use tokio;

#[derive(Debug, Parser, PartialEq)]
#[clap(
    name = "gymtracker",
    version = "2.0",
    about = "A simple application to track workout's"
)]
pub struct GymTrackerArgs {
    #[clap(subcommand)]
    pub user_method: MethodType,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum MethodType {
    /// Create a new user Account
    CreateUser {
        /// Username for Account
        username: String,
        /// Users full name: i.e, First\ Last
        name: String,
        /// Password for Account
        password: String,
    },
    /// Print workout logs for user defined.
    ReadAll { username: String, password: String },
    /// Print a workout log for date specified.
    ReadDate {
        username: String,
        password: String,
        date: String,
    },
    /// Create workout log in database.
    Write {
        /// Account username
        username: String,
        /// Account password
        password: String,
        /// Date of training session, i.e 00-00-0000
        date: String,
        /// Time of training session, i.e 00:00-00:00
        time: String,
        /// Muscle's trained during session, i.e Back,\ Biceps
        muscle_group: String,
        /// Intensity of training session, range from 1-10
        intensity: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInputs {
    username: String,
    name: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogInputs {
    username: String,
    password: String,
    date: String,
    time: String,
    muscle_group: String,
    intensity: String,
}

async fn create_user(
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

async fn write_log(
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

async fn read_all_logs(username: String, password: String) -> mongodb::error::Result<()> {
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

async fn read_specific_log(
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

async fn run(args: GymTrackerArgs) {
    match args.user_method {
        MethodType::CreateUser {
            username,
            name,
            password,
        } => create_user(username, name, password)
            .await
            .expect("Sorry, something went wrong creating your Account."),
        MethodType::Write {
            username,
            password,
            date,
            time,
            muscle_group,
            intensity,
        } => write_log(LogInputs {
            username,
            password,
            date,
            time,
            muscle_group,
            intensity,
        })
        .await
        .expect("Sorry, something went wrong writing your log."),
        MethodType::ReadAll { username, password } => read_all_logs(username, password)
            .await
            .expect("Sorry, something went wrong reading your logs."),
        MethodType::ReadDate {
            username,
            password,
            date,
        } => read_specific_log(username, password, date)
            .await
            .expect("Sorry, something went wrong reading your logs."),
    }
}

#[tokio::main]
async fn main() {
    let args = GymTrackerArgs::parse();

    run(args).await;
}
