table! {
    permissions (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    role_permissions (role_id, permission_id) {
        role_id -> Int4,
        permission_id -> Int4,
    }
}

table! {
    roles (id) {
        id -> Int4,
        name -> Text,
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
    user_roles (user_id, role_id) {
        user_id -> Int4,
        role_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        hash -> Text,
        created_at -> Timestamp,
    }
}

joinable!(role_permissions -> permissions (permission_id));
joinable!(role_permissions -> roles (role_id));
joinable!(sessions -> users (user_id));
joinable!(user_roles -> roles (role_id));
joinable!(user_roles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    permissions,
    role_permissions,
    roles,
    sessions,
    user_roles,
    users,
);
