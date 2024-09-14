diesel::table! {
    asthobin (id) {
        #[max_length = 10]
        id -> Varchar,
        content -> Longtext,
        time -> BigInt,
    }
}
