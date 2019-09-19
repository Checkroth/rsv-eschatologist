extern crate slack;
use super::DBConnection;
use super::slackaction::SlackAction;
use super::models::{SlackUser, UserAlias, Thx};
use super::schema::slack_users::dsl::*;
use super::schema::thxs::dsl::*;
use super::schema::user_aliases::dsl::*;
use super::schema::slack_users::dsl::id as slack_users_pk;
use super::schema::thxs::dsl::slack_user_id as thx_slack_user_id;
use super::diesel::prelude::*;

pub fn get_user_channel_thx(user: &SlackUser, channel: &str, conn: &DBConnection) -> i64 {
    match Thx::belonging_to(user)
        .filter(channel_id.eq(&channel))
        .count()
        .first::<i64>(conn) {
            Ok(i) => i,
            _ => -1
        }
}

pub struct ThxAdd;

/**
AddThx feature for slack bot.
Takes <user_alias>++ and increases that user's points
Returns a message displaying the user's new point count for the channel
 **/
impl SlackAction for ThxAdd {
    fn action(&self, _: &str, user_id: &str, text: &str, channel: &str, conn: &DBConnection) -> Option<String> {
        let split: Vec<&str> = text.split("++").collect();
        let target_alias: &str = match split.get(0) {
            Some(alias) => alias,
            _ => return Some(String::from("No alias found"))
        };

        let alias_record = match user_aliases
            .filter(name_alias.eq(&target_alias))
            .first::<UserAlias>(conn) {
                Ok(alias_record) => alias_record,
                _ => return Some(String::from(["Couldn't find user", target_alias].join(" ")))
            };

        let user_record =  match slack_users
            .filter(slack_users_pk.eq(&alias_record.slack_user_id))
            .first::<SlackUser>(conn) {
                Ok(SlackUser { id: _, slack_id: ref sid}) if sid == user_id => return None,
                Ok(user_record) => user_record,
                _ => return Some(String::from(["Couldn't find user", target_alias].join(" ")))
            };

        match diesel::insert_into(thxs)
            .values((thx_slack_user_id.eq(user_record.id), channel_id.eq(channel)))
            .execute(conn) {
                Ok(_) => {
                    Some(String::from(["<@",
                                       &user_record.slack_id,
                                       "> increased to ",
                                       &get_user_channel_thx(&user_record, channel, conn).to_string(),
                                       " thx points in this channel"].join("")))
                },
                _ => Some(String::from("Couldn't process request!"))
            }
                    
    }
}
