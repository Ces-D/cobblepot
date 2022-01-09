namespace Cobblepot.Domain.Accounting.Entries;
using Cobblepot.Domain.Accounting.Accounts.AccountTypes;

public class Entry : EntityBase, IAggregateRoot, IEntity
{
    private readonly Guid _id;
    private bool _isCredit;
    private AccountType _accountType;
    private Transaction _transaction;

    private Entry(Transaction transaction, AccountType accountType, bool isCredit) : base(DateTime.UtcNow)
    {
        _id = Guid.NewGuid();
        _transaction = transaction;
        _accountType = accountType;
        _isCredit = isCredit;
    }

    public DateTime TransactionDate => _transaction.TransactionDate;
    public Money TransactionAmount => _transaction.Amount;
    public Guid Id => _id;

    public static Entry CreateNew(DateTime transactionDate, string title, string memo, bool isCredit, Money amount, AccountType accountType)
    {
        return new Entry(new Transaction(transactionDate, title, memo, amount), accountType, isCredit);
    }

    public Entry CreateOpposite()
    {
        Transaction oppositeTransaction = new(_transaction.TransactionDate, _transaction.Title, $"Balancing Transaction: {_transaction.Memo}", _transaction.Amount);
        AccountType oppositeAccountType = _accountType switch
        {
            AccountType.Asset => AccountType.Liability,
            AccountType.Liability => AccountType.Asset,
            AccountType.Expense => AccountType.Revenue,
            AccountType.Revenue => AccountType.Expense,
            _ => AccountType.Asset,
        };

        return new Entry(oppositeTransaction, oppositeAccountType, !_isCredit);
    }
}

