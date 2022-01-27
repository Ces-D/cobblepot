namespace Cobblepot.Domain.Accountant.Entries;
using Cobblepot.Domain.Common;
using Cobblepot.Domain.Accountant.Accounts;

public class Entry : EntityBase, IAggregateRoot
{
    public Transaction Transaction { get; private set; }
    public bool IsCredit { get; private set; }
    public Guid AccountId { get; private set; }

    public Entry(Guid id, DateTime createdDate, Transaction transacion, bool isCredit, Guid accountId) : base(id, createdDate)
    {
        Transaction = transacion;
        IsCredit = isCredit;
        AccountId = accountId;
    }

    public Entry()
    {
        Transaction = new Transaction();
    }

    public void UpdateTransaction(Transaction transaction)
    {
        Transaction = transaction;
    }

    public void UpdateAssociatedAccount(Account account)
    {
        AccountId = account.Id;
    }

    public Entry CreateDoubleEntry(Guid accountId)
    {
        var transaction = new Transaction()
        {
            Amount = Transaction.Amount,
            Date = Transaction.Date,
            Memo = $"Balancing transaction: {Transaction.Memo}",
            Title = Transaction.Title
        };
        return new Entry(Guid.NewGuid(), DateTime.UtcNow, transaction, !IsCredit, accountId);
    }

}