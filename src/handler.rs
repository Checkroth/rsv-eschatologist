extern crate slack;
extern crate diesel;

use self::diesel::mysql::MysqlConnection;
use self::slack::{Event, EventHandler, Message, RtmClient};
use super::action::patterns;
use super::action::slackaction::SlackAction;


pub struct Handler {
    pub db_connection: MysqlConnection,
}

/**
slack::EventHandler implementation for our local handler.
Allows slack-rs api to call our code when messages are received.
**/
impl EventHandler for Handler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        match event.clone() {
            Event::Message(message) => self.handle_message(*message, cli),
            _ => return
        };
    }

    fn on_close(&mut self, _cli: &RtmClient) {
        println!("closed");
    }

    fn on_connect(&mut self, _cli: &RtmClient) {
        println!("connected");
    }
}

/**
Custom handler for slack API.
Dictates how to handle messages received from slack by unwrwapping message contents and passing to the action module.
**/
impl Handler {
    fn handle_message(&mut self, message: Message, cli: &RtmClient) {
        let message_standard = match message {
            Message::Standard(message_standard) => message_standard,
            _ => return
        };
        let bot_id: &str = match cli.start_response().slf.as_ref() {
            Some(bot_user) => {
                match bot_user.id.as_ref() {
                    Some(bot_id) => { &bot_id },
                    _ => return
                }
            },
            _ => return
        };
        // TODO:: Refactor - unwrap is unsafe.
        let user_id: String = message_standard.user.unwrap();
        let channel: String = message_standard.channel.unwrap();
        let text: String = message_standard.text.unwrap();
        let slack_action: Box<dyn SlackAction> = patterns::determine_action(&text, &bot_id);

        if user_id == bot_id {
            return
        };
        slack_action.action(&bot_id, &user_id, &text, &channel, &self.db_connection).and_then(
            |response_text| cli.sender().send_message(&channel, &response_text).ok()
        );
    }
}
