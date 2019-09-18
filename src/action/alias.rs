extern crate slack;
use super::slackaction::SlackAction;

pub struct AliasAdd;

/**
 **/
impl SlackAction for AliasAdd {
    fn action(&self, _: &str, user_id: &str, text: &str, channel: &str) -> Option<String> {
        Some(String::from("alias added"))
    }
}
