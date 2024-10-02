// @generated automatically by Diesel CLI.

diesel::table! {
    notes (uuid) {
        uuid -> Binary,
        title -> Text,
        body -> Text,
        created_at -> BigInt,
        updated_at -> BigInt,
    }
}

diesel::table! {
  settings (key) {
    key -> Text,
    value -> Text
  }
}
