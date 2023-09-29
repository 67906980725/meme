diesel::table! {
    style (id) {
        id -> Nullable<Integer>,
        name -> Text,
        sort -> Integer,
    }
}

diesel::table! {
    folder (id) {
        id -> Nullable<Integer>,
        name -> Text,
        click -> Integer,
        style_id -> Integer,
        style_name -> Text,
    }
}

diesel::table! {
    keyword (id) {
        id -> Nullable<Integer>,
        name -> Text,
        style_id -> Integer,
    }
}

diesel::table! {
    keyword_folder_r (id) {
        id -> Nullable<Integer>,
        keyword_id -> Integer,
        folder_id -> Integer,
        style_id -> Integer,
    }
}

diesel::table! {
    img (id) {
        id -> Nullable<Integer>,
        path -> Text,
        click -> Integer,
    }
}

diesel::table! {
    img_folder_r (id) {
        id -> Nullable<Integer>,
        img_id -> Integer,
        folder_id -> Integer,
    }
}

// allow_tables_to_appear_in_same_query!(folder, keyword, keyword_folder_r);
