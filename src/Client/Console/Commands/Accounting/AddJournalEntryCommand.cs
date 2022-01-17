namespace Cobblepot.Client.Console.Commands.Accounting;
using System;
using Cobblepot.Client.Console.Arguments;
using Cobblepot.Client.Console.Options;
using Cobblepot.Client.Console.ConsoleFormatter;
using Cobblepot.Domain.Accounting.Accounts.AccountTypes;
using Cobblepot.Domain.Accounting.SharedKernel;
using Cobblepot.Domain.Accounting.Journals;
using Cobblepot.Domain.Accounting.Entries;
using Microsoft.Extensions.Configuration;

internal class AddJournalEntryCommand : Command
{
    private Journal _workingJournal;
    private readonly IConfiguration? _config;
    private readonly JournalEntryFormatter _entryFormatter;
    public AddJournalEntryCommand(Journal journal, IConfiguration? config) : base("add", "Add new entry to your working accounting journal")
    {
        _workingJournal = journal;
        _config = config;
        _entryFormatter = new JournalEntryFormatter();

        var titleArgument = new TitleArgument("Title of this transaction");
        var moneyArgument = new MoneyArgument();
        var memoOption = new MemoOption();
        var accountTypeOption = new AccountTypeOption();
        var dateOption = new DateOption();
        var isCreditOption = new IsCreditOption();
        var currencyOption = new CurrencyOption(_config.GetValue<Currency>("Currency"));

        this.AddArgument(titleArgument);
        this.AddArgument(moneyArgument);
        this.AddOption(memoOption);
        this.AddOption(accountTypeOption);
        this.AddOption(dateOption);
        this.AddOption(isCreditOption);
        this.AddOption(currencyOption);

        this.SetHandler(
            (string title, decimal money, string[] memo, AccountType accountType, DateTime date, bool isCredit, Currency currency) =>
        {
            var entry = new Entry(date, title, String.Join(" ", memo), new Money(money, currency), accountType, isCredit);
            var entries = _workingJournal.Add(entry, true);

            foreach (Entry item in entries)
            {
                _entryFormatter.DetailedOutput(item);
            }
        },
         titleArgument, moneyArgument, memoOption, accountTypeOption, dateOption, isCreditOption, currencyOption);
    }
}

// TODO: add a commit command that saves the journal to the dbContext, 
// should also trigger the addition to the ledgers
// TODO: add a command for loading entries between certain dates
// TODO: add option for detailed or short outputs that is set in appSettings