fn main() {
    let db_url = cobblepot_core::database_url().expect("Unable to find or create database");
    cli::main(db_url);
}
