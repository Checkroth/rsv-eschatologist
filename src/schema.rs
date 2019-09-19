table! {
    slack_users (id) {
        id -> Integer,
        slack_id -> Varchar,
    }
}

table! {
    thxs (id) {
        id -> Integer,
        slack_user_id -> Integer,
        channel_id -> Varchar,
    }
}

table! {
    user_aliases (id) {
        id -> Integer,
        name_alias -> Varchar,
        slack_user_id -> Integer,
    }
}

joinable!(thxs -> slack_users (slack_user_id));
joinable!(user_aliases -> slack_users (slack_user_id));

allow_tables_to_appear_in_same_query!(
    slack_users,
    thxs,
    user_aliases,
);
