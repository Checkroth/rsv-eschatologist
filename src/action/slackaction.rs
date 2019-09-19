extern crate slack;

use super::DBConnection;

pub trait SlackAction {
    fn action(&self,
              bot_id: &str,
              user_id: &str,
              text: &str,
              channel: &str,
              conn: &DBConnection) -> Option<String>;
}

pub struct Invalid;

impl SlackAction for Invalid {
    fn action(&self, _: &str, _: &str, _: &str, _: &str, _: &DBConnection) -> Option<String> {
        // Todo:: Just make this do nothing instead of printing
        println!("Not a valid slack command");
        None
    }
}
