extern crate slack;
use super::slackaction::SlackAction;

pub struct Thx;

/**
Thx feature for slack bot.
Takes <user_alias>++ and increases that user's points
Returns a message displaying the user's new point count for the channel
 **/
impl SlackAction for Thx {
    fn action(&self, _: &str, user_id: &str, text: &str, channel: &str) -> Option<String> {
        let split: Vec<&str> = text.split("++").collect();
        let points: u32 = 1;
        let user_mention = ["<@", user_id, ">"].join("");
        Some(String::from([split[0],
                           "increased to",
                           &points.to_string(),
                           "points by",
                           &user_mention].join(" ")))
    }
}
