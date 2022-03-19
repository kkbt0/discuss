table! {
    discuss_main (id) {
        id -> Integer,
        author -> Integer,
        content -> Text,
        created_at -> Text,
        reply_to -> Integer,
        up -> Integer,
        down -> Integer,
        read_number -> Integer,
        sharded_number -> Integer,
        father_nodes -> Nullable<Integer>,
        son_nodes -> Text,
    }
}

table! {
    main_nodes_list (id) {
        id -> Integer,
        nodes_list -> Text,
    }
}

table! {
    user_list (id) {
        id -> Integer,
        key_id -> Text,
        show_name -> Text,
        key_status -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(discuss_main, main_nodes_list, user_list,);
