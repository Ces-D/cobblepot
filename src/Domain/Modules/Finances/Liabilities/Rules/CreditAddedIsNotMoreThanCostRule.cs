namespace Cobblepot.Domain.Modules.Finances.Liabilities;

internal class CreditAddedIsNotMoreThanCostRule : IBusinessRule
{
    private bool _lessThanOrEqualToCost;

    public CreditAddedIsNotMoreThanCostRule(Money liabilityCost, Money creditBeingAdded)
    {
        _lessThanOrEqualToCost = liabilityCost.Amount >= creditBeingAdded.Amount;
    }
    public string Message => "The credit added to your liability cannot be more than the cost of the liability";
    public bool IsBroken() => _lessThanOrEqualToCost;
}