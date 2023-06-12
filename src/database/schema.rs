// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Int8,
        name -> Text,
        display_name -> Text,
        created -> Timestamptz,
    }
}

diesel::table! {
    authors_of_books (id) {
        id -> Int8,
        author_id -> Int8,
        book_id -> Int8,
    }
}

diesel::table! {
    books (id) {
        id -> Int8,
        name -> Text,
        description -> Nullable<Text>,
    }
}

diesel::joinable!(authors_of_books -> authors (author_id));
diesel::joinable!(authors_of_books -> books (book_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    authors_of_books,
    books,
);
