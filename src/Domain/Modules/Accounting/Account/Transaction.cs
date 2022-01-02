namespace Cobblepot.Domain.Accounting.Account;

internal record Transaction : ValueObject, IAccountTransaction
{
    private DateTime _transactionDate;
    private string _name;
    private string _memo;
    private bool _isCredit;
    private Money _amount;
    private AccountType _accountType;

    public Transaction(DateTime? transactionDate, string name, string memo, bool isCredit, Money amount, AccountType accountType) : base(DateTime.UtcNow)
    {
        _transactionDate = transactionDate ?? DateTime.UtcNow;
        _name = name;
        _memo = memo;
        _isCredit = isCredit;
        _amount = amount;
        _accountType = accountType;
    }

    public DateTime TransactionDate { get => _transactionDate; }
    public string Name { get => _name; }
    public string Memo { get => _memo; }
    public bool IsCredit { get => _isCredit; }
    public Money Amount { get => _amount; }
    public AccountType AccountType { get => _accountType; }
}
