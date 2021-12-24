namespace Cobblepot.Domain.Modules.Finances.Assets;

internal class AmountDecreasedIsNotMoreThanOwnedRule : IBusinessRule
{
    private bool _amountLessThanOrEqualToOwned;

    public AmountDecreasedIsNotMoreThanOwnedRule(int amountOwned, int amountDecreasing)
    {
        _amountLessThanOrEqualToOwned = amountOwned >= amountDecreasing;
    }

    public string Message => "Decrease amount cannot be more than the amount owned";

    public bool IsBroken() => _amountLessThanOrEqualToOwned;
}