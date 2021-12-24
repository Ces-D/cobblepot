namespace Cobblepot.Domain.Modules.Finances.Assets;

// see - https://www.investopedia.com/terms/a/asset.asp 
public interface IAsset
{
    public Money Value { get; }
    public Money TotalValue { get; }
    public int Amount { get; }
    public void IncreaseAmount(int amountAdded, Money? newPurchasePrice);
    public void DecreaseAmount(int amountRemoved);
    public void UpdateValue(Money newValue);
}