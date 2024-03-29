extern crate slack;
#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

mod action;
mod handler;

use slack::RtmClient;
use std::env;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;


pub fn establish_db_connection(db_url: &str) -> MysqlConnection {
    MysqlConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}
    

fn main() {
    match (env::var("SLACK_API_TOKEN"), env::var("SLACK_DATABASE_URL")) {
        (Ok(api_token), Ok(db_url)) => {
            let mut handler = handler::Handler { db_connection: establish_db_connection(&db_url) };
            let r = RtmClient::login_and_run(&api_token, &mut handler);
            match r {
                Ok(_) => {},
                Err(err) => panic!("error: {}", err)
            }
        },
        _ => {
            println!("Please set SLACK_API_TOKEN and SLACK_DATABASE_URL env vars");
        }
    }
}
