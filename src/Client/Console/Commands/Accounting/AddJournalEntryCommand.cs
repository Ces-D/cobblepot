namespace Cobblepot.Client.Console.Commands.Accounting;
using Cobblepot.Client.Console.Arguments;
using Cobblepot.Client.Console.Options;
using Cobblepot.Domain.Accounting.Accounts.AccountTypes;
using Cobblepot.Domain.Accounting.SharedKernel;
using Cobblepot.Domain.Accounting.Journals;
using Cobblepot.Domain.Accounting.Entries;

internal class AddJournalEntryCommand : Command
{
    private Journal _workingJournal;
    public AddJournalEntryCommand(Journal journal) : base("add", "Add new entry to your accounting journal")
    {
        _workingJournal = journal;
        this.AddArgument(new TitleArgument());
        this.AddArgument(new MemoArgument());
        this.AddArgument(new MoneyArgument());
        this.AddOption(new AccountTypeOption());
        this.AddOption(new DateOption());
        this.AddOption(new IsCreditOption());
        this.AddGlobalOption(new CurrencyOption());

        this.SetHandler((string title, string memo, decimal money, AccountType accountType, DateTime date, bool isCredit, Currency currency) =>
        {
            var entry = new Entry(date, title, memo, new Money(money, currency), accountType, isCredit);
            var entries = _workingJournal.Add(entry, true);

            foreach (Entry item in entries)
            {
                System.Console.WriteLine(string.Format("{0}{1}", "Journal Entry Added".PadRight(25), item.Id));
                System.Console.WriteLine(string.Format("{0}{1}", "Date:".PadRight(25), item.TransactionDate));
                System.Console.WriteLine(string.Format("{0}{1}", "Title".PadRight(25), item.TransactionTitle));
                System.Console.WriteLine(string.Format("{0}{1}", "Amount".PadRight(25), item.TransactionAmount));
                System.Console.WriteLine();
            }
        });
    }
}

// TODO: test this command for both inputs and outputs
// TODO: add command for listing working Journal entries
// TODO: add a commit command that saves the journal to the dbContext, 
// should also trigger the addition to the ledgers
// TODO: add a command for loading entries between certain dates