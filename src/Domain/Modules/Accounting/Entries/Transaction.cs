namespace Cobblepot.Domain.Accounting.Entries;

public record Transaction
{
    internal Transaction(DateTime transactionDate, string title, string memo, Money amount)
    {
        TransactionDate = transactionDate;
        Title = title;
        Memo = memo;
        Amount = amount;
    }

    public DateTime TransactionDate { get; }
    public string Title { get; }
    public string Memo { get; }
    public Money Amount { get; }
}
