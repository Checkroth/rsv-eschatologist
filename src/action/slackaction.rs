extern crate slack;

pub trait SlackAction {
    fn action(&self, bot_id: &str, text: &str, channel: &str) -> Option<&str>;
}

pub struct Invalid;

impl SlackAction for Invalid {
    fn action(&self, _: &str, _: &str, _: &str) -> Option<&str> {
        // Todo:: Just make this do nothing instead of printing
        println!("Not a valid slack command");
        None
    }
}
