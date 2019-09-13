extern crate slack;
use self::slack::{Event, EventHandler, Message, RtmClient};
use super::action::patterns;
use super::action::slackaction::SlackAction;

pub struct Handler;

impl EventHandler for Handler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        println!("event triggered: {:?}", event);

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

impl Handler {
    fn handle_message(&mut self, message: Message, cli: &RtmClient) {
        let message_standard = match message {
            Message::Standard(message_standard) => message_standard,
            _ => return
        };
        let bot_id: &str = match cli.start_response().slf.as_ref() {
            Some(user) => {
                match user.id.as_ref() {
                    Some(bot_id) => { &bot_id },
                    _ => return
                }
            },
            _ => return
        };

        match &message_standard.user {
            Some(user_id) if user_id == bot_id => { return },
            _ => { () }
        }

        let channel: String = message_standard.channel.unwrap();
        let text: String = message_standard.text.unwrap();
        let slack_action: Box<dyn SlackAction> = patterns::determine_action(&text, &bot_id);
        slack_action.action(&bot_id, &text, &channel).and_then(
            |response_text| cli.sender().send_message(&channel, response_text).ok()
        );
    }
}
