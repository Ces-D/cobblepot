using System;
using Services.AccountFile;

namespace Services.JournalFile
{
    public class JournalEntry
    {
        private string commandType { get; set; }
        private AccountEntry accountEntry { get; set; }
        private TransactionEntry? transactionEntry { get; set; }
        private string note { get; set; }

        public JournalEntry(string commandType, AccountEntry accountEntry)
        {
            this.commandType = commandType;
            this.accountEntry = accountEntry;
            this.note = "";
        }

        public JournalEntry(string commandType, AccountEntry accountEntry, string note)
        {
            this.commandType = commandType;
            this.accountEntry = accountEntry;
            this.note = note;
        }

        public JournalEntry(TransactionEntry transactionEntry, AccountEntry accountEntry)
        {
            this.commandType = "*";
            this.accountEntry = accountEntry;
            this.transactionEntry = transactionEntry;
            this.note = "";
        }

        public DateTime Date
        {
            get { return this.accountEntry.Date; }
        }

        public override string ToString()
        {
            if (this.commandType == "close" || this.commandType == "open")
            {
                return string.Format("{0}{1}{2}{3}",
                                this.accountEntry.DateString.PadRight(14),
                                this.commandType.PadRight(9),
                                this.accountEntry.Account,
                                this.note.PadLeft(10));
            }
            else if (this.commandType == "note")
            {
                return string.Format("{0}{1}{2}",
                this.accountEntry.DateString.PadRight(14),
                this.commandType.PadRight(9),
                this.note);
            }
            else if (this.commandType == "*")
            {
                if (transactionEntry != null)
                {
                    return string.Format("{0}{1}{2}{3}",
                    this.accountEntry.DateString.PadRight(14),
                    this.commandType.PadRight(9),
                    this.transactionEntry.Details.PadRight(50),
                    this.transactionEntry.Money);
                }
                else { throw new Exception("Internal Error at JournalEntry, commandType '*'"); }
            }
            else { throw new Exception("You need to provide a commandType"); }
        }
    }
}