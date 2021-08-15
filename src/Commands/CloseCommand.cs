using System;
using System.CommandLine;
using System.CommandLine.Invocation;
using Services.AccountFile;
using Services.JournalFile;


namespace Commands
{
    //same as git remove
    public class CloseCommand : Command
    {
        public CloseCommand() : base("close", "Close and Account")
        {
            AddArgument(AccountName());
            AddOption(AccountCloseDate());

            Handler = CommandHandler.Create<DateTime, string>((AccountCloseDate, AccountName) =>
            {
                AccountEntry accountEntry = new AccountEntry(AccountCloseDate, AccountName);
                JournalEntry journalEntry = new JournalEntry("close", accountEntry);
                AccountWriter.Remove(accountEntry);
                JournalWriter.Append(journalEntry);
            });
        }

        private Argument<string> AccountName()
        {
            Argument<string> account = new Argument<string>();
            account.Name = "Account";
            account.Description = "Name of the Account";

            account.AddValidator(acc =>
            {
                if (!AccountEntry.MatchesConvention(acc.ToString()))
                {
                    return AccountEntry.FORMAT_ERROR_RESPONSE;
                }
                else { return null; }
            });

            return account;
        }
        private Option<DateTime> AccountCloseDate()
        {
            Option<DateTime> date = new Option<DateTime>("--date");
            date.AddAlias("-d");
            date.Description = "Date of entry";
            date.SetDefaultValue(DateTime.Now);

            return date;
        }
    }
}