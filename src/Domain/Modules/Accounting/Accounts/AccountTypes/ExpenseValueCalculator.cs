namespace Cobblepot.Domain.Accounting.Accounts.AccountTypes;

public struct ExpenseValueCalculator : IAccountValueCalculator
{
    public AccountType AccountType => AccountType.Expense;

    public Money Credit(Money currentValue, Money credit) => currentValue -= credit;

    public Money Debit(Money currentValue, Money debit) => currentValue += debit;
}