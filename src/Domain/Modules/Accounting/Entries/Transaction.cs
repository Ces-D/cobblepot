namespace Cobblepot.Domain.Accounting.Entries;

internal record Transaction
{
    internal Transaction(DateTime transactionDate, string title, string memo, Money amount)
    {
        TransactionDate = transactionDate;
        Title = title;
        Memo = memo;
        Amount = amount;
    }

    internal DateTime TransactionDate { get; }
    internal string Title { get; }
    internal string Memo { get; }
    internal Money Amount { get; }
}
