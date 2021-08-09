using System;
using System.CommandLine;
using Controllers.Middleware;
using System.CommandLine.Invocation;
using Controllers.VaultAccess.Journal;

namespace Commands
{
    public class NewNoteCommand : Command
    {
        public NewNoteCommand() : base("new-note", "For creating a note on activity. Create a new note directive entry in Journal.")
        {
            AddArgument(DetailArgument());
            AddOption(DateOption());

            TreatUnmatchedTokensAsErrors = true;

            Handler = CommandHandler.Create(
              (string DetailArgument, DateTime DateOption, double NumberOption, string CurrencyOption) =>
              {
                  JournalEntry entry = new JournalEntry(DateOption, "note", DetailArgument, NumberOption, CurrencyOption);
                  JournalFiles.InsertHandler(entry, true);
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
    }
}