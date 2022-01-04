namespace Cobblepot.Domain.Accounting.Accounts;

using Cobblepot.Domain.Accounting.Accounts.AccountTypes;

internal record Transaction : IAccountTransaction
{
    public Transaction(DateTime? transactionDate, string title, string memo, bool isCredit, Money amount, AccountType accountType)
    {
        TransactionDate = transactionDate ?? DateTime.UtcNow;
        Title = title;
        Memo = memo;
        IsCredit = isCredit;
        Amount = amount;
        AccountType = accountType;
    }

    public DateTime TransactionDate { get; }
    public string Title { get; }
    public string Memo { get; }
    public bool IsCredit { get; }
    public Money Amount { get; }
    public AccountType AccountType { get; }
}
