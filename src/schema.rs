table! {
    roles (id) {
        id -> Int4,
        name -> Text,
        admin -> Bool,
        download -> Bool,
        edit -> Bool,
        upload -> Bool,
        view -> Bool,
    }
}

table! {
    sessions (id) {
        id -> Text,
        user_id -> Int4,
        ip_address -> Inet,
        user_agent -> Text,
        last_activity -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        hash -> Text,
        created_at -> Timestamp,
        role_id -> Int4,
    }
}

joinable!(sessions -> users (user_id));
joinable!(users -> roles (role_id));

allow_tables_to_appear_in_same_query!(roles, sessions, users,);
