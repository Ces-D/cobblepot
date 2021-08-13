using System;
using System.CommandLine;
using System.CommandLine.Invocation;
using Services.Account;

namespace Commands
{
    // same as git branch
    public class OpenCommand : Command
    {
        public OpenCommand() : base("open", "Open an account")
        {
            AddArgument(AccountName());
            AddOption(AccountEntryDate());

            Handler = CommandHandler.Create((string AccountName, DateTime AccountEntryDate) =>
            {
                AccountWriter.Append(accountName: AccountName, entryDate: AccountEntryDate);
            });
        }

        private Argument<string> AccountName()
        {
            Argument<string> account = new Argument<string>();
            account.Name = "Account";
            account.Description = "Name of the Account";

            account.AddValidator(acc =>
            {
                if (!AccountFormat.MatchesConvention(acc.ToString()))
                {
                    return AccountFormat.FORMAT_ERROR_RESPONSE;
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