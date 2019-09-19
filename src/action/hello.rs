extern crate slack;
use super::slackaction::SlackAction;
use super::DBConnection;

pub struct Hello;

impl SlackAction for Hello {
    fn action(&self, _: &str, _: &str, _: &str, _: &str, _: &DBConnection) -> Option<String> {
        Some(String::from("Hi!"))
    }
}
