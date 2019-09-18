extern crate slack;
use super::slackaction::SlackAction;

pub struct AliasAdd;

/**
 **/
impl SlackAction for AliasAdd {
    fn action(&self, _: &str, user_id: &str, text: &str, _channel: &str) -> Option<String> {
        Some(String::from("alias added"))
    }
}


pub struct AliasRemove;

/**
 **/
impl SlackAction for AliasRemove {
    fn action(&self, _: &str, user_id: &str, text: &str, _channel: &str) -> Option<String> {
        Some(String::from("alias removed"))
    }
}
