using System;
using System.CommandLine;
using System.CommandLine.Invocation;
using Services.AccountFile;
using Services.JournalFile;

namespace Commands
{
    // same as git branch
    public class OpenCommand : Command
    {
        public OpenCommand() : base("open", "Open an account")
        {
            AddArgument(AccountName());
            AddOption(AccountEntryDate());

            Handler = CommandHandler.Create<DateTime, string>((DateTime AccountEntryDate, string AccountName) =>
            {
                AccountEntry accountEntry = new AccountEntry(AccountEntryDate, AccountName);
                JournalEntry journalEntry = new JournalEntry("open", accountEntry);
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

        private Option<DateTime> AccountEntryDate()
        {
            Option<DateTime> date = new Option<DateTime>("--date");
            date.AddAlias("-d");
            date.Description = "Date of entry";
            date.SetDefaultValue(DateTime.Now);

            return date;
        }
    }
}