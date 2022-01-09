namespace Cobblepot.Domain.Accounting.Accounts.AccountTypes;

public interface IAccountValueCalculator
{
    Money Credit(Money currentValue, Money credit);
    Money Debit(Money currentValue, Money debit);
}