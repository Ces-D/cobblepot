namespace Cobblepot.Domain.Accounting.SharedKernel.Rules;

internal record MoneyAmountIsPositiveRule : IBusinessRule
{
    private readonly bool _amountIsPositive;

    public MoneyAmountIsPositiveRule(decimal amount)
    {
        _amountIsPositive = amount >= 0;
    }

    public string Message => "Money amount cannot be negative";

    public bool IsBroken() => !_amountIsPositive;
}