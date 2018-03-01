table! {
    sessions (id) {
        id -> Int4,
        user_id -> Int4,
        csrf_token -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        display_name -> Nullable<Varchar>,
    }
}

joinable!(sessions -> users (user_id));
