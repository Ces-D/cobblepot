using System;
using System.CommandLine;
using System.CommandLine.Invocation;

namespace Commands
{
    public class AddNoteCommand : Command
    {
        public AddNoteCommand() : base("note", "Add note to currently active account")
        {
            // TODO: Create handlers
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
    }
}