using System;
using System.CommandLine;
using System.CommandLine.Invocation;
using Services.AccountFile;
using Services.JournalFile;
using Services;

namespace Commands
{
    public class AddNoteCommand : Command
    {
        public AddNoteCommand() : base("note", "Add note to currently active account")
        {
            AddArgument(DetailsArgument());
            AddOption(DateOption());

            AddValidator((cmd) =>
            {
                if (VaultBranch.Account != null)
                {
                    return null;
                }
                else { return "You must activate a vault branch"; }
            });

            Handler = CommandHandler.Create<string, DateTime>((DetailsArgument, DateOption) =>
            {
                if (VaultBranch.Account != null)
                {
                    AccountEntry accountEntry = new AccountEntry(DateOption, VaultBranch.Account);
                    JournalEntry journalEntry = new JournalEntry("note", accountEntry, DetailsArgument);

                    JournalWriter.Append(journalEntry);
                }
            });
        }

        private Argument<string> DetailsArgument()
        {
            Argument<string> details = new Argument<string>("details");
            details.Description = "Descriptive content of this entry";
            ArgumentArity argumentArity = new ArgumentArity(1, 1);
            details.Arity = argumentArity;
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