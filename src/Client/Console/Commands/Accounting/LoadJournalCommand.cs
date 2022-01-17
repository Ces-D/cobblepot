namespace Cobblepot.Client.Console.Commands.Accounting;
using Cobblepot.Client.Console.Arguments;
using Cobblepot.Client.Console.Options;
using Cobblepot.Domain.Accounting.Accounts.AccountTypes;
using Cobblepot.Domain.Accounting.SharedKernel;
using Cobblepot.Domain.Accounting.Journals;
using Cobblepot.Domain.Accounting.Entries;
using Microsoft.Extensions.Configuration;

internal class LoadJournalCommand : Command
{
    private Journal _workingJournal;
    private readonly IConfiguration? _config;
    public LoadJournalCommand(Journal journal, IConfiguration? config) : base("load", "Load a new journal populated with your entries between two dates")
    {
        _workingJournal = journal;
        _config = config;

        this.SetHandler(() =>
        {

            foreach (Entry item in _workingJournal.Entries)
            {
                System.Console.WriteLine(string.Format("{0}{1}", "Journal Entry Added".PadRight(25), item.Id));
                System.Console.WriteLine(string.Format("{0}{1}", "Date".PadRight(25), item.TransactionDate));
                System.Console.WriteLine(string.Format("{0}{1}", "Title".PadRight(25), item.TransactionTitle));
                if (item.TransactionMemo != null) { System.Console.WriteLine(string.Format("{0}{1}", "Memo".PadRight(25), item.TransactionMemo)); }
                System.Console.WriteLine(string.Format("{0}{1}", $"Amount - {(item.IsCredit ? "Credit" : "Debit")}".PadRight(25), item.TransactionAmount));
                System.Console.WriteLine();
            }
        }
         );
    }
}

// TODO: Create a journal Entry Formatter class for detailed and excerpted entries - beneficial for this command which will write possibly hundreds of entries to command line
// TODO: create session by writing a JSON file with the journal entries adn reading/writin to it
// TODO: The commit command which still needs to be written should erase this JSON session and save transaction to the db
// TODO: connect a db to the application - url should be located in the appSettings.json file