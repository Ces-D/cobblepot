using System;
using System.Globalization;
using System.CommandLine;
using System.CommandLine.Invocation;
using Controllers.VaultAccess;
using Config;

namespace Controllers.Commands
{

    public class NewCommand : Command
    {

        public NewCommand() : base("new", "Create a new vault entry")
        {
            AddArgument(DetailsArgument());
            AddOption(DateOption());
            AddOption(DirectiveOption());
            AddOption(CreditOption());
            AddOption(CurrencyOption());
            Handler = CommandHandler.Create((DateTime date, string directiveType, string details, double credit, string currency) =>
            {
                JournalEntry entry = new JournalEntry(date, directiveType, details, credit, currency);
                Console.WriteLine(entry.ToString());
            });
        }

        private Argument<string> DetailsArgument()
        {
            Argument<string> details = new Argument<string>("details");
            details.Description = "The details of this journal entry";

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

        private Option<string> DirectiveOption()
        {
            Option<string> directive = new Option<string>("--directive-type");
            directive.AddAlias("-t");
            directive.Description = "The kind of entry from available options";

            directive.FromAmong(DirectiveOptions.Balance, DirectiveOptions.Close,
            DirectiveOptions.Note, DirectiveOptions.Open,
            DirectiveOptions.Price, DirectiveOptions.Default);

            directive.SetDefaultValue(DirectiveOptions.Default);

            return directive;
        }

        private Option<double> CreditOption()
        {
            Option<double> value = new Option<double>("--credit");
            value.AddAlias("-cred");
            value.IsRequired = true;
            value.Description = "The credit or debit";

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
