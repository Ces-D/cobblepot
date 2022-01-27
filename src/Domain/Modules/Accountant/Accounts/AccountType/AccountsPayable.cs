namespace Cobblepot.Domain.Accountant.Accounts.AccountType;

public struct AccountsPayable : IAccountType
{
    public string Name => "Accounts Payable";
    public AccountType AccountType => AccountType.Liability;

    public Money Credit()
    {
        throw new NotImplementedException();
    }

    public Money Debit()
    {
        throw new NotImplementedException();
    }
}