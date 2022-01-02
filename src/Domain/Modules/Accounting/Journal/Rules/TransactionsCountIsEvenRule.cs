namespace Cobblepot.Domain.Accounting.Journal;

internal record TransactionsCountIsEvenRule : IBusinessRule
{
    private bool _isEven;
    public TransactionsCountIsEvenRule(int transactionsLength)
    {
        _isEven = transactionsLength % 2 == 0;
    }

    public string Message => "Transactions must be an even count if they balance";
    public bool IsBroken() => _isEven;
}