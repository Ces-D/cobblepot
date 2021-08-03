using System;
using System.IO;
using System.CommandLine;
using System.CommandLine.Invocation;
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
            AddOption(ValueOption());
            Handler = CommandHandler.Create<string, DateTime, string, int>(JournalEntryHandler);
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

        private Option<int> ValueOption()
        {
            Option<int> value = new Option<int>("--currency");
            value.AddAlias("-c");
            value.IsRequired = true;
            value.Description = "The currency change";

            return value;
        }


        // TODO: create entry pipeline
        /**
         - handle cleaning - ensure proper formatting
         - into vault connection
*/

        private void JournalEntryHandler(string details, DateTime date, string directiveType, int currency)
        {
            Console.WriteLine($"Details: {details}");
            Console.WriteLine($"Date: {date}");
            Console.WriteLine($"Directive: {directiveType}");
            Console.WriteLine($"value: {currency}");
        }
    }

}
