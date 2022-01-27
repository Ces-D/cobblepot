namespace Cobblepot.Domain.Accountant.Rules;
using Cobblepot.Domain.Common;

internal class MoneyAmountIsPositiveRule : IBusinessRule
{
    private readonly bool _amountIsNegative;

    public MoneyAmountIsPositiveRule(decimal amount)
    {
        _amountIsNegative = amount < 0;
    }

    public string Message => "Money amount cannot be negative";

    public bool IsBroken() => _amountIsNegative;
}
