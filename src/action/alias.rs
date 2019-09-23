extern crate slack;

use super::slackaction::SlackAction;
use super::DBConnection;
use super::models::{SlackUser, UserAlias};
use super::schema::slack_users::dsl::*;
use super::schema::slack_users::dsl::id as slack_users_id; // for unamibiguous query on "ID"
use super::schema::user_aliases::dsl::*;
use super::diesel::prelude::*;

pub struct AliasConnect;

impl SlackAction for AliasConnect {
    fn action(&self, _: &str, user_id: &str, text: &str, _channel: &str, conn: &DBConnection) -> Option<String> {
        let message_split: Vec<&str> = text.split(" connect ").collect();
        let alias: &str = match message_split.get(1) {
            Some(alias) => alias,
            _ => return Some(String::from("Alias not provided"))
        };
        
        // Find existing user or create
        let existing_user = slack_users
            .filter(slack_id.eq(&user_id))
            .first::<SlackUser>(conn);
        
        let user_record: SlackUser = match existing_user {
            Ok(user) => { user },
            _ => {
                // Insert new user and return that record.
                diesel::insert_into(slack_users)
                    .values(slack_id.eq(user_id))
                    .execute(conn)
                    .expect("failed to save user!");
                
                slack_users.filter(slack_id.eq(&user_id))
                    .first::<SlackUser>(conn)
                    .expect("failed to add user!")
            }
        };

        // Check if alias exists
        match user_aliases
            .filter(name_alias.eq(&alias))
            .first::<UserAlias>(conn) {
                Ok(UserAlias { id: ref alias_id, name_alias: _, slack_user_id: ref suid }) if suid == user_record.id => {
                    Some(String::from("You already own that alias")),
                },
                Ok(alias_record) if alias_record.slack_user_id == None => {
                    // update record here
                    diesel::update(&alias_record)
                        .set(slack_user_id.eq(user_record.id))
                        .execute(conn)
                        .expect("Failed to update alias");
                    Some("Attached alias to you!")
                },
                Ok(_) => Some(String::from("That doesn't belong to you")),
                _ => {
                    diesel::insert_into(user_aliases)
                        .values((name_alias.eq(&alias),
                                 slack_user_id.eq(user_record)))
                        .execute(conn)
                        .expect("failed to create alias");
                    Some(String::from("Created alias and attached it to you!"))
                }
            }
    }
}

pub struct AliasAdd;

/**
 **/
impl SlackAction for AliasAdd {
    fn action(&self, _: &str, user_id: &str, text: &str, _channel: &str, conn: &DBConnection) -> Option<String> {
        // Pull alias from message string
        let message_split: Vec<&str> = text.split(" add ").collect();
        let alias: &str = match message_split.get(1) {
            Some(alias) => { alias },
            _ => { return Some(String::from("Alias not provided")) }
        };

        // Search for existing alias & error if exists
        let existing_alias_count = user_aliases
            .filter(name_alias.eq(&alias))
            .count()
            .first::<i64>(conn);
        match existing_alias_count {
            Ok(x) if x > 0 => { return Some(String::from(["alias", alias, "already exists!"].join(" "))) },
            Ok(_) => { () },
            Err(_) => { return Some(String::from("something went wrong! could not find existing aliases")) }
        };
            

        // Find existing user or create
        let existing_user = slack_users
            .filter(slack_id.eq(&user_id))
            .first::<SlackUser>(conn);
        
        let user_record: SlackUser = match existing_user {
            Ok(user) => { user },
            _ => {
                // Insert new user and return that record.
                diesel::insert_into(slack_users)
                    .values(slack_id.eq(user_id))
                    .execute(conn)
                    .expect("failed to save user!");
                
                slack_users.filter(slack_id.eq(&user_id))
                    .first::<SlackUser>(conn)
                    .expect("failed to add user!")
            }
        };

        // Insert alias
        match diesel::insert_into(user_aliases)
            .values((name_alias.eq(alias), slack_user_id.eq(user_record.id)))
            .execute(conn) {
                Ok(_) => { Some(String::from(["alias `", alias, "` added for <@", user_id, ">"].join(""))) },
                Err(_) => { Some(String::from("Something went wrong! could not add alias.")) }
            }
    }
}


pub struct AliasRemove;

/**
 **/
impl SlackAction for AliasRemove {
    fn action(&self, _: &str, user_id: &str, text: &str, _channel: &str, conn: &DBConnection) -> Option<String> {
        let message_split: Vec<&str> = text.split(" remove ").collect();
        let alias: &str = match message_split.get(1) {
            Some(alias) => { alias },
            _ => { return Some(String::from("Alias not provided")) }
        };
        // Check if alias exists
        let existing_alias = user_aliases
            .filter(name_alias.eq(&alias))
            .first::<UserAlias>(conn);

        let dne = Some(String::from(["Alias", alias, "doesn't exist"].join(" ")));
        match existing_alias {
            Ok(alias_record) => {
                // Check if alias is owned by user
                match slack_users.filter(slack_users_id.eq(&alias_record.slack_user_id)).first::<SlackUser>(conn) {
                    // TODO:: This statement probably own't work
                    Ok(SlackUser { id: _, slack_id: ref sid }) if sid == user_id | sid == None => {
                        // Delete alias if owned by user
                        match diesel::delete(&alias_record).execute(conn) {
                            Ok(_) => Some(String::from(["Alias `", &alias_record.name_alias, "` deleted."].join(""))),
                            Err(_) => Some(String::from("Failed to detele alias."))
                        }
                    },
                    Ok(_) => { Some(String::from("Alias is owned by another user")) },
                    _ => { dne }
                }
            },
            _ => dne 
        }
    }
}


pub struct AliasHelp;

impl SlackAction for AliasHelp {
    fn action(&self, _: &str, _: &str, _: &str, _: &str, _: &DBConnection) -> Option<String> {
        Some(String::from([
            "Alias Commands",
            "Aliases allow users to interact with you through aliases you have set. They allow you to be linked to other features, such as thx",
            "",
            "`!alias add <word>`: add an alias to yourself.",
            "`!alias remove <word>`: remove an alias from yourself."
        ].join("\n")))
    }
}
