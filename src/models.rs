use super::schema::{slack_users,
                    user_aliases,
                    thxs};

#[derive(Identifiable, Queryable)]
#[table_name = "slack_users"]
pub struct SlackUser {
    pub id: i32,
    pub slack_id: String,
}

#[derive(Identifiable, Associations, Queryable)]
#[belongs_to(SlackUser, foreign_key = "slack_user_id")]
#[table_name = "user_aliases"]
pub struct UserAlias {
    pub id: i32,
    pub name_alias: String,
    pub slack_user_id: i32,
}

#[derive(Identifiable, Associations, Queryable)]
#[belongs_to(SlackUser, foreign_key = "slack_user_id")]
#[table_name = "thxs"]
pub struct Thx {
    pub id: i32,
    pub slack_user_id: i32,
    pub channel_id: String,
}
