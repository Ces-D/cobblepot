use diesel::RunQueryDsl;
use diesel::{insert_into, ExpressionMethods, QueryDsl, SqliteConnection};

use crate::models::journal;
use crate::schema;

pub fn create_journal_entry(
    mut new_journal_entry: journal::NewJournalEntry,
    new_transaction: journal::NewJournalTransaction,
    connection: &mut SqliteConnection,
) -> journal::Journal {
    match insert_into(schema::transaction_event::table)
        .values(new_transaction)
        .get_result::<journal::JournalTransaction>(connection)
    {
        Ok(transaction) => {
            new_journal_entry.transaction_code = transaction.transaction_code();
            insert_into(schema::journal::table)
                .values(new_journal_entry)
                .get_result::<journal::Journal>(connection)
                .expect("Unable to insert Journal entry")
        },
        Err(_) => panic!("Unable to insert transaction_event"),
    }
}

pub fn read_journal_entry(entry_code: i32, connection: &mut SqliteConnection) -> journal::Journal {
    schema::journal::dsl::journal
        .filter(schema::journal::dsl::entry_code.eq(entry_code))
        .first::<journal::Journal>(connection)
        .expect("Unable to retrieve journal")
}
