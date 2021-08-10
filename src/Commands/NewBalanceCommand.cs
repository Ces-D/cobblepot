using System;
using System.CommandLine;
using Controllers.Middleware;
using System.CommandLine.Invocation;
using Controllers.VaultAccess.Journal;
using Controllers.VaultAccess.Directive;

namespace Commands
{
    public class NewBalanceCommand : Command
    {
        public NewBalanceCommand() : base("new-balance", "For updating account balance. Create a new balance directive entry in Journal.")
        {
            AddArgument(DetailArgument());
            AddOption(DateOption());
            AddOption(NumberOption());
            AddOption(CurrencyOption());
            TreatUnmatchedTokensAsErrors = true;


            Handler = CommandHandler.Create(
              (string DetailArgument, DateTime DateOption, double NumberOption, string CurrencyOption) =>
              {
                  JournalEntry entry = new JournalEntry(DateOption, "bal", DetailArgument, NumberOption, CurrencyOption);
                  JournalFiles.InsertHandler(entry);
                  if (!DirectiveFile.findTarget(DetailArgument))
                  {
                      DirectiveFile.appendTarget(DirectiveFile.toEntryString(DateOption, DetailArgument));
                  }
              });
        }

        private Argument<string> DetailArgument()
        {
            Argument<string> detail = new Argument<string>("detail");
            detail.Description = "Descriptive content of this entry";

            detail.AddValidator(det =>
            {
                if (!JournalDetail.MatchesFormatConvention(det.ToString()))
                {
                    return JournalDetail.FORMAT_ERROR_RESPONSE;
                }
                else { return null; }
            });

            return detail;
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