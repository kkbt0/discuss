table! {
    discuss_main (id) {
        id -> Integer,
        author -> Text,
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

allow_tables_to_appear_in_same_query!(
    discuss_main,
    main_nodes_list,
);
