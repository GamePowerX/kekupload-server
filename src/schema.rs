// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Bpchar,
        ext -> Varchar,
        hash -> Bpchar,
    }
}
