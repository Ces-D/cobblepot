namespace Cobblepot.Client.Console.Commands.Accounting;
using Cobblepot.Client.Console.Arguments;
using Cobblepot.Client.Console.Options;
using Cobblepot.Domain.Accounting.Accounts.AccountTypes;
using Cobblepot.Domain.Accounting.SharedKernel;
using Cobblepot.Domain.Accounting.Journals;
using Cobblepot.Domain.Accounting.Entries;
using Microsoft.Extensions.Configuration;

internal class ListJournalEntryCommand : Command
{
    private Journal _workingJournal;
    private readonly IConfiguration? _config;
    public ListJournalEntryCommand(Journal journal, IConfiguration? config) : base("list", "List the entries in the workings accounting journal")
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
