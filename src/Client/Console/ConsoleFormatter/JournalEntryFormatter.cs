namespace Cobblepot.Client.Console.ConsoleFormatter;
using System;
using Cobblepot.Domain.Accounting.Entries;

internal struct JournalEntryFormatter
{
    public void DetailedOutput(Entry entry, string? heading = null)
    {
        Console.WriteLine(String.Format("{0}{1}", heading ?? "Entry".PadRight(25), entry.Id));
        Console.WriteLine(String.Format("{0}{1}", "Date".PadRight(25), entry.Transaction.TransactionDate));
        Console.WriteLine(String.Format("{0}{1}", "Title".PadRight(25), entry.Transaction.Title));
        Console.WriteLine(String.Format("{0}{1}", "Memo".PadRight(25), entry.Transaction.Memo));
        Console.WriteLine(String.Format("{0}{1}", $"{(entry.IsCredit ? "Credit" : "Debit")} Amount".PadRight(25), entry.Transaction.Amount.ToString()));
        Console.WriteLine(String.Format("{0}{1}", "Account Type".PadRight(25), entry.AccountType));
    }

    public void ShortOutput(Entry entry, string? heading = null)
    {
        Console.WriteLine(String.Format("{0}{1}", heading ?? "Entry".PadRight(25), entry.Id));
        const string Format = "{0}{1}{2}{3}";
        Console.WriteLine(String.Format(Format,
            $"{entry.Transaction.TransactionDate}".PadRight(4),
            $"{entry.Transaction.Title}".PadRight(4),
            $"{(entry.IsCredit ? entry.Transaction.Amount.ToString() : $"( {entry.Transaction.Amount} )")}".PadRight(4)));
    }
}
