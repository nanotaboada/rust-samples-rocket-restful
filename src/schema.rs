// @generated automatically by Diesel CLI.

diesel::table! {
    players (id) {
        id -> Text,
        first_name -> Text,
        middle_name -> Text,
        last_name -> Text,
        date_of_birth -> Text,
        squad_number -> Integer,
        position -> Text,
        abbr_position -> Text,
        team -> Text,
        league -> Text,
        starting11 -> Integer,
    }
}
