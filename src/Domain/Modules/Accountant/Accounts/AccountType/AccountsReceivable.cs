namespace Cobblepot.Domain.Accountant.Accounts.AccountType;

public struct AccountsReceivable : IAccountType
{
    public string Name => "Accounts Receivable";
    public AccountType AccountType => AccountType.Asset;

    public Money Credit()
    {
        throw new NotImplementedException();
    }

    public Money Debit()
    {
        throw new NotImplementedException();
    }
}