// @generated automatically by Diesel CLI.

diesel::table! {
    note_tags (note_uuid, tag_uuid) {
        note_uuid -> Binary,
        tag_uuid -> Binary,
    }
}

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
    schemas (uuid) {
        uuid -> Binary,
        name -> Text,
    }
}

diesel::table! {
    settings (key) {
        key -> Text,
        value -> Text,
    }
}

diesel::table! {
    tag_parents (parent_uuid, child_uuid) {
        parent_uuid -> Binary,
        child_uuid -> Binary,
    }
}

diesel::table! {
    tags (uuid) {
        uuid -> Binary,
        title -> Text,
    }
}

diesel::joinable!(note_tags -> notes (note_uuid));
diesel::joinable!(note_tags -> tags (tag_uuid));

diesel::allow_tables_to_appear_in_same_query!(
    note_tags,
    notes,
    schemas,
    settings,
    tag_parents,
    tags,
);
