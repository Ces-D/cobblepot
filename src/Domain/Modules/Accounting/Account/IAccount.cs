namespace Cobblepot.Domain.Accounting.Account;
public interface IAccount
{
    string Name { get; }
    Money Credit { get; }
    Money Debit { get; }
    AccountType AccountType { get; }
    List<IAccountTransaction> Transactions { get; }
}
