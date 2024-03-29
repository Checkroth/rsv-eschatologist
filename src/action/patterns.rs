extern crate slack;
extern crate regex;

use lazy_static::lazy_static;
use regex::Regex;
use super::slackaction;
use super::hello;
use super::thx;
use super::alias;

lazy_static! {
    // Lazy Static constructs of regex patterns for slack message parsing
    static ref PATTERN_HELLO: Regex = Regex::new(r"^Hi!$").unwrap();
    static ref PATTERN_THX: Regex = Regex::new(r"^\S*(\+\+)$").unwrap();
    static ref PATTERN_ALIAS_ADD: Regex = Regex::new(r"^!alias add \S*$").unwrap();
    static ref PATTERN_ALIAS_REMOVE: Regex = Regex::new(r"^!alias remove \S*$").unwrap();
    static ref PATTERN_ALIAS_HELP: Regex = Regex::new(r"^!alias help$").unwrap();
}



/**
Determines the action to execute from a slack message.
References lazy static Regex patterns and returns an implementation of SlackAction
 **/
pub fn determine_action(message_text: &str, _bot_id: &str) -> Box<dyn slackaction::SlackAction> {
    match message_text {
        hi if PATTERN_HELLO.is_match(hi) => { Box::new(hello::Hello) },
        thx if PATTERN_THX.is_match(thx) => { Box::new(thx::ThxAdd) },
        alias_add if PATTERN_ALIAS_ADD.is_match(alias_add) => { Box::new(alias::AliasAdd) },
        alias_remove if PATTERN_ALIAS_REMOVE.is_match(alias_remove) => { Box::new(alias::AliasRemove) },
        alias_help if PATTERN_ALIAS_HELP.is_match(alias_help) => { Box::new(alias::AliasHelp) },
        _ => { Box::new(slackaction::Invalid) }
    }
}
