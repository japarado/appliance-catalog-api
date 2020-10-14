table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        display_name -> Nullable<Varchar>,
        profile_picture -> Nullable<Varchar>,
        bio -> Nullable<Text>,
    }
}
