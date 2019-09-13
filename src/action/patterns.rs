extern crate slack;
use super::hello;
use super::slackaction;

pub fn determine_action(message_text: &str, bot_id: &str) -> Box<dyn slackaction::SlackAction> {
    match message_text {
        "Hi!" => { Box::new(hello::Hello) },
        _ => { Box::new(slackaction::Invalid) }
    }
}
