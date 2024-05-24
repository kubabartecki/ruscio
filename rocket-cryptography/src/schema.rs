diesel::table! {
    hashes (id) {
        id -> Integer,
        time_of_computing -> Integer,
        number_of_digits -> Integer,
    }
}
