table! {
    session (id, line_id) {
        id -> Int4,
        line_id -> Int4,
        data -> Nullable<Jsonb>,
    }
}
