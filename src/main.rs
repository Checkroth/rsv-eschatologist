extern crate slack;
mod action;
mod handler;
use slack::RtmClient;
use std::env;

fn main() {
    match env::var("SLACK_API_TOKEN") {
        Ok(api_token) => {
            println!("OK!: {}", api_token);
            let mut handler = handler::Handler;
            let r = RtmClient::login_and_run(&api_token, &mut handler);
            match r {
                Ok(_) => {},
                Err(err) => panic!("error: {}", err)
            }
        },
        Err(_) => {
            println!("Please set SLACK_API_TOKEN env var");
        }
    }
}
