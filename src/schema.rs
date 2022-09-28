// @generated automatically by Diesel CLI.

diesel::table! {
    bit_image (id) {
        id -> Int4,
        title -> Varchar,
        description -> Varchar,
        create_at -> Timestamp,
        update_at -> Timestamp,
        user_id -> Int4,
    }
}

diesel::table! {
    bit_image_like (user_id, bit_image_id) {
        bit_image_id -> Int4,
        user_id -> Int4,
        score -> Int2,
        create_at -> Timestamp,
    }
}

diesel::table! {
    user_ (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(bit_image -> user_ (user_id));
diesel::joinable!(bit_image_like -> bit_image (bit_image_id));
diesel::joinable!(bit_image_like -> user_ (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    bit_image,
    bit_image_like,
    user_,
);
