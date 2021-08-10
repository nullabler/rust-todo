table! {
    category (id) {
        id -> Integer,
        name -> Varchar,
        created_at -> Datetime,
    }
}

table! {
    task (id) {
        id -> Integer,
        category_id -> Integer,
        name -> Varchar,
        status -> Varchar,
        created_at -> Datetime,
    }
}

joinable!(task -> category (category_id));

allow_tables_to_appear_in_same_query!(
    category,
    task,
);
