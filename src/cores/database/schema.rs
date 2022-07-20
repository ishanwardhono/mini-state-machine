table! {
    first_table (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        income -> Nullable<Int4>,
    }
}

table! {
    states (id) {
        id -> Int4,
        code -> Varchar,
        description -> Nullable<Text>,
        webhooks -> Nullable<Array<Text>>,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    first_table,
    states,
);
