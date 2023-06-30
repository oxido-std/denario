diesel::table! {
    categories (id) {
        id -> Integer,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_deleted -> Bool,
    }
}

diesel::table! {
    records (id) {
        id -> Integer,
        name -> Varchar,
        amount -> Float,
        amount_io -> VarChar,
        comment -> Text,
        record_date -> Date,
        category_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_deleted -> Bool,
        is_mutable -> Bool,
    }
}

diesel::table! {
    credits(id){
        id -> Integer,
        name -> Varchar,
        comment -> Text,
        amount -> Float,
        payments -> Integer,
        started_at -> Date,
        finish_at -> Date,
        category_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_deleted -> Bool,
    }
}