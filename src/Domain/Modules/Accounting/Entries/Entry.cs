namespace Cobblepot.Domain.Accounting.Entries;
using Cobblepot.Domain.Accounting.Accounts.AccountTypes;

public class Entry : Entity
{
    private bool _isCredit;
    private AccountType _accountType;
    private Transaction _transaction;

    private Entry(Transaction transaction, AccountType accountType, bool isCredit) : base(new EntryId(Guid.NewGuid()), DateTime.UtcNow)
    {
        _transaction = transaction;
        _accountType = accountType;
        _isCredit = isCredit;
    }

    public DateTime TransactionDate => _transaction.TransactionDate;
    public Money TransactionAmount => _transaction.Amount;

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

