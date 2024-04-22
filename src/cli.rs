use clap::*;

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
