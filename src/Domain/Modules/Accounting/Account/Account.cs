namespace Cobblepot.Domain.Accounting.Account;

public class Account : Entity, IAccount
{
    private string _name;
    private Money _credit;
    private Money _debit;
    private AccountType _accountType;
    private List<IAccountTransaction> _transactions;

    public Account(Guid id, string accountName, AccountType accountType) : base(id, DateTime.UtcNow)
    {
        _name = accountName;
        _credit = new Money(0, Currency.USD);
        _debit = new Money(0, Currency.USD);
        _accountType = accountType;
        _transactions = new List<IAccountTransaction>();
    }

    public string Name { get => _name; }
    public Money Credit { get => _credit; }
    public Money Debit { get => _debit; }
    public AccountType AccountType { get => _accountType; }

    public List<IAccountTransaction> Transactions { get => _transactions; }
}

//TODO: this needs to somehow load the transactions from the journal, maybe through a repository.
// everything could be async