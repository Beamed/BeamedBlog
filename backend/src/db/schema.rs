table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        display_name -> Nullable<Varchar>,
    }
}