table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        display_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Varchar,
        last_updated -> Nullable<Varchar>,
    }
}
