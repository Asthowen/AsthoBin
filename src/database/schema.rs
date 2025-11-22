diesel::table! {
    asthobin (id) {
        #[max_length = 10]
        id -> Varchar,
        content -> Longtext,
        language -> Varchar,
        time -> BigInt,
    }
}
