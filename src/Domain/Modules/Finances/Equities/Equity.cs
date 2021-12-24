namespace Cobblepot.Domain.Modules.Finances.Equities;

// see - https://www.investopedia.com/terms/e/equity.asp 

public abstract class Equity : Entity
{
    private decimal _totalAssets;
    private decimal _totalLiabilities;
    public Equity(Guid id, decimal totalAssetValue, decimal totalLiabilityValue) : base(id, SystemClock.Now)
    {
        _totalAssets = totalAssetValue;
        _totalLiabilities = totalLiabilityValue;
    }

    public decimal TotalEquity() { return _totalAssets - _totalLiabilities; }
}

// TODO: review because this might be a value type