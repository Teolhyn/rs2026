// @generated automatically by Diesel CLI.

diesel::table! {
    reservations (id) {
        id -> Integer,
        room_id -> Integer,
        user_id -> Integer,
        start_time -> Timestamp,
        end_time -> Timestamp,
        status -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    rooms (id) {
        id -> Integer,
        name -> Text,
        capacity -> Integer,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        email -> Text,
        name -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(reservations -> rooms (room_id));
diesel::joinable!(reservations -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(reservations, rooms, users,);
