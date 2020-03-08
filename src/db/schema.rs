table! {
    Sessions (sid) {
        sid -> Varchar,
        expires -> Nullable<Timestamptz>,
        data -> Nullable<Text>,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
    }
}

table! {
    chat_members (user_id, chat_id) {
        user_id -> Int4,
        chat_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    chat_messages (id) {
        id -> Int4,
        content -> Varchar,
        chat_id -> Int4,
        author_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    chat_rooms (id) {
        id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    event_participants (user_id, event_id) {
        user_id -> Int4,
        event_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    events (id) {
        id -> Int4,
        name -> Varchar,
        due_date -> Timestamptz,
        group_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    friendships (user_id, friend_id) {
        user_id -> Int4,
        friend_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    group_admins (user_id, group_id) {
        user_id -> Int4,
        group_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    group_members (user_id, group_id) {
        user_id -> Int4,
        group_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    groups (id) {
        id -> Int4,
        name -> Varchar,
        creator_id -> Int4,
        chat_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::{Int4, Timestamptz};
    use crate::db::enums::EnumPostVoteType;

    post_votes (user_id, post_id) {
        vote_type -> EnumPostVoteType,
        user_id -> Int4,
        post_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    posts (id) {
        id -> Int4,
        content -> Varchar,
        author_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::{Int4, Timestamptz};
    use crate::db::enums::EnumRequestType;

    requests (id) {
        id -> Int4,
        request_type -> EnumRequestType,
        sender_id -> Int4,
        receiver_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        handle -> Varchar,
        email -> Varchar,
        password -> Varchar,
        rankpoints -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        frontend_settings -> Json,
        auth_token -> Nullable<Varchar>,
        auth_expire -> Nullable<Timestamptz>,
        is_admin -> Bool,
        profile_picture -> Nullable<Varchar>,
    }
}

joinable!(chat_members -> chat_rooms (chat_id));
joinable!(chat_members -> users (user_id));
joinable!(chat_messages -> chat_rooms (chat_id));
joinable!(chat_messages -> users (author_id));
joinable!(event_participants -> events (event_id));
joinable!(event_participants -> users (user_id));
joinable!(events -> groups (group_id));
joinable!(group_admins -> groups (group_id));
joinable!(group_admins -> users (user_id));
joinable!(group_members -> groups (group_id));
joinable!(group_members -> users (user_id));
joinable!(groups -> chat_rooms (chat_id));
joinable!(groups -> users (creator_id));
joinable!(post_votes -> posts (post_id));
joinable!(post_votes -> users (user_id));
joinable!(posts -> users (author_id));

allow_tables_to_appear_in_same_query!(
    Sessions,
    chat_members,
    chat_messages,
    chat_rooms,
    event_participants,
    events,
    friendships,
    group_admins,
    group_members,
    groups,
    post_votes,
    posts,
    requests,
    users,
);
