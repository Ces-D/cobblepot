namespace Cobblepot.Domain.Accounting.Accounts;
public interface IAccount
{
    string Name { get; }
    IAccountType AccountType { get; }
}
