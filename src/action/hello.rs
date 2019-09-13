extern crate slack;

use super::slackaction::SlackAction;

pub struct Hello;

impl SlackAction for Hello {
    fn action(&self, _: &str, _: &str, _: &str) -> Option<&str> {
        println!("responding");
        Some("Hi!")
    }
}
