namespace Cobblepot.Domain.Accountant.Accounts.AccountType;

public struct Cash : IAccountType
{
    public string Name => "Cash";
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