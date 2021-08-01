using System;
using System.IO;
using System.CommandLine;
using Config;

namespace Controllers.Commands
{
    public class NewCommand : Command
    {
        public NewCommand() : base("new", "Create a new vault entry")
        {
            AddOption(DateOption());
            AddOption(DirectiveOption());
            AddOption(ArgumentOption());
            AddOption(GainLossOption());
            AddOption(ValueOption());
        }

        private Option<string> DateOption()
        {
            Option<string> date = new Option<string>("--date");
            date.AddAlias("-d");
            date.Description = "Date associated, default to today";
            date.SetDefaultValue(DateTime.Now);

            return date;
        }

        private Option<string> DirectiveOption()
        {
            Option<string> directive = new Option<string>("--directive");
            directive.AddAlias("-dir");
            directive.Description = "The kind of entry from available options";

            directive.FromAmong(DirectiveOptions.Balance, DirectiveOptions.Close,
            DirectiveOptions.Note, DirectiveOptions.Open,
            DirectiveOptions.Price, DirectiveOptions.Default);

            directive.SetDefaultValue(DirectiveOptions.Default);

            return directive;
        }

        private Option<string> ArgumentOption()
        {
            Option<string> argument = new Option<string>("--argument");
            argument.AddAlias("-a");
            argument.Description = "The details of this journal entry";

            argument.IsRequired = true;

            return argument;
        }

        private Option<bool> GainLossOption()
        {
            Option<bool> gainLoss = new Option<bool>("--gain");
            gainLoss.AddAlias("-g");
            gainLoss.Description = "Include if the following price is a gain, Omit if the following is a loss";

            return gainLoss;
        }

        private Option<string> ValueOption()
        {
            Option<string> value = new Option<string>("--value");
            value.AddAlias("-v");
            value.Description = "The value of this entry, either change in value or current value";

            value.IsRequired = true;
            return value;
        }

        private void NewHandler()
        {
            // TODO: handle the various cases
        }
    }
}

/**
Everything is going to go through the journal
It will add the item into a journal file, this will be the most complete record
From the journal depending on the directive
We will be able to generate the reports. 

// TODO: redo the schema and folder structure of the vault. Journal and Reports are the only folders
*/