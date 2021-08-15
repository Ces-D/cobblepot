using System;
using System.CommandLine;
using System.CommandLine.Invocation;
using Services.AccountFile;
using Services.JournalFile;
using Services;
namespace Commands
{
    public class AddTransactionCommand : Command
    {
        public AddTransactionCommand() : base("add", "Add transaction to currently active account")
        {
            AddArgument(DetailsArgument());
            AddOption(NumberOption());
            AddOption(CurrencyOption());

            AddValidator((cmd) =>
            {
                if (VaultBranch.Account != null)
                {
                    return null;
                }
                else { return "You must activate a vault branch"; }
            });

            Handler = CommandHandler.Create<string, DateTime, double, string>((DetailsArgument, DateOption, NumberOption, CurrencyOption) =>
            {
                if (VaultBranch.Account != null)
                {
                    AccountEntry accountEntry = new AccountEntry(DateOption, VaultBranch.Account);
                    TransactionEntry transactionEntry = new TransactionEntry(accountEntry, DetailsArgument, NumberOption, CurrencyOption);
                    JournalEntry journalEntry = new JournalEntry(transactionEntry, accountEntry);

                    JournalWriter.Append(journalEntry);
                }
            });
        }

        private Argument<string> DetailsArgument()
        {
            Argument<string> details = new Argument<string>("details");
            details.Description = "Descriptive content of this entry";

            return details;
        }

        private Option<DateTime> DateOption()
        {
            Option<DateTime> date = new Option<DateTime>("--date");
            date.AddAlias("-d");
            date.Description = "Date of entry";
            date.SetDefaultValue(DateTime.Now);

            return date;
        }

        private Option<double> NumberOption()
        {
            Option<double> value = new Option<double>("--number");
            value.AddAlias("-n");
            value.IsRequired = true;
            value.Description = "The number of credit or debit";

            return value;
        }

        private Option<string> CurrencyOption()
        {
            Option<string> currency = new Option<string>("--currency");
            currency.AddAlias("-c");
            currency.Description = "The currency of your entry";

            currency.SetDefaultValue("en-US");

            return currency;
        }
    }
}
