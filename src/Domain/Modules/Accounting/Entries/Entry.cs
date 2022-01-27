﻿namespace Cobblepot.Domain.Accounting.Entries;
using Cobblepot.Domain.Accounting.Accounts.AccountTypes;

public class Entry : EntityBase, IAggregateRoot, IEntity
{
    private readonly Guid _id;
    private bool _isCredit;
    private AccountType _accountType;
    private readonly Transaction _transaction;

    public Guid Id { get { return _id; } }
    public AccountType AccountType { get { return _accountType; } } // TODO: change this to use the Account entity 
    /// reason being that the there is right no way to connect an account to a accounts payable for example 
    // Some of these properties should be removed from being private because they do not require any kind of special modification
    public Transaction Transaction { get { return _transaction; } }
    public bool IsCredit { get { return _isCredit; } }

    public Entry(DateTime transactionDate, string title, string memo, Money amount, AccountType accountType, bool isCredit) : base(DateTime.UtcNow)
    {
        _id = Guid.NewGuid();
        _transaction = new Transaction(transactionDate, title, memo, amount);
        _accountType = accountType;
        _isCredit = isCredit;
    }

    internal Entry(Transaction transaction, AccountType accountType, bool isCredit) : base(DateTime.UtcNow)
    {
        _id = Guid.NewGuid();
        _transaction = transaction;
        _accountType = accountType;
        _isCredit = isCredit;
    }

    internal Entry(Guid id, DateTime createdDate, Transaction transaction, AccountType accountType, bool isCredit) : base(createdDate)
    {
        _id = id;
        _transaction = transaction;
        _accountType = accountType;
        _isCredit = isCredit;
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

