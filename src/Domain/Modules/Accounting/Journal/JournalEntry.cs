namespace Cobblepot.Domain.Accounting.Journal;
using Cobblepot.Domain.Accounting.Account;

public class JournalEntry : Entity
{
    private List<IAccountTransaction> _transactions;

    public JournalEntry(Guid id) : base(id, DateTime.UtcNow)
    {
        _transactions = new List<IAccountTransaction>();
    }

    public List<IAccountTransaction> Transactions { get => _transactions; }

    public void AddTransactions(List<IAccountTransaction> transactions)
    {
        this.CheckRule(new TransactionsCountIsEvenRule(transactions.Count));

        var step = 0;

        while (step < transactions.Count)
        {
            this.Add(transactions[step], transactions[step++]);
            step += 2;
        }
    }

    private void Add(IAccountTransaction initialTransaction, IAccountTransaction balancingTransaction)
    {
        this.CheckRule(new TransactionCreditsBalanceRule(initialTransaction, balancingTransaction));
        this.CheckRule(new TranscationAmountsBalanceToNetZeroRule(initialTransaction, balancingTransaction));
        this.CheckRule(new TransactionAccountTypesOppositeRule(initialTransaction, balancingTransaction));

        _transactions.Add(initialTransaction);
        _transactions.Add(balancingTransaction);
    }
}
