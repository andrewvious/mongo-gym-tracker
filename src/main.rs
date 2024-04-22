use clap::Parser;
use methods::write_log::LogInputs;
use tokio;

pub mod cli;
use crate::cli::{GymTrackerArgs, MethodType};

pub mod methods;
use crate::methods::create_user::create_user;
use crate::methods::read_log::{read_all_logs, read_specific_log};
use crate::methods::write_log::write_log;

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
        .expect("Sorry, something went wrong writing your log. Check your CLI inputs."),
        MethodType::ReadAll { username, password } => read_all_logs(username, password)
            .await
            .expect("Sorry, something went wrong reading your logs. Check your CLI inputs."),
        MethodType::ReadDate {
            username,
            password,
            date,
        } => read_specific_log(username, password, date)
            .await
            .expect("Sorry, something went wrong reading your logs. Check your CLI inputs."),
    }
}

#[tokio::main]
async fn main() {
    let args = GymTrackerArgs::parse();

    run(args).await;
}
