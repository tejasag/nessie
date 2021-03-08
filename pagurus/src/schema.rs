table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        display_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        last_updated -> Nullable<Timestamp>,
    }
}
