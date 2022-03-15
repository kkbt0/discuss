table! {
    discuss_main (id) {
        id -> Integer,
        author -> Text,
        content -> Text,
        created_at -> Text,
        up -> Integer,
        down -> Integer,
        read_number -> Integer,
        sharded_number -> Integer,
        father_nodes -> Nullable<Integer>,
        son_nodes -> Text,
    }
}
