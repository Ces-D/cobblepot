fn main() {
    let db_url = cobblepot_core::database_url().expect("Unable to find or create database");
    cobblepot::main(db_url);
}
