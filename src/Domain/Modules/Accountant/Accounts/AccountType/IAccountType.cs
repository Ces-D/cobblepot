namespace Cobblepot.Domain.Accountant.Accounts.AccountType;

public interface IAccountType
{
    public string Name { get; }    
    public AccountType AccountType { get; }
    public Money Debit();
    public Money Credit();
}