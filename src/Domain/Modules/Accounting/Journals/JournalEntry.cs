namespace Cobblepot.Domain.Accounting.Journals;

using Cobblepot.Domain.Accounting.Accounts;
using Cobblepot.Domain.Accounting.Journals.Rules;

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
        CheckRule(new TransactionsCountIsEvenRule(transactions.Count));

        var step = 0;

        while (step < transactions.Count)
        {
            Add(transactions[step], transactions[step++]);
            step += 2;
        }
    }

    private void Add(IAccountTransaction initialTransaction, IAccountTransaction balancingTransaction)
    {
        CheckRule(new TransactionCreditsBalanceRule(initialTransaction, balancingTransaction));
        CheckRule(new TranscationAmountsBalanceToNetZeroRule(initialTransaction, balancingTransaction));
        CheckRule(new TransactionAccountTypesOppositeRule(initialTransaction, balancingTransaction));

        _transactions.Add(initialTransaction);
        _transactions.Add(balancingTransaction);
    }
}
