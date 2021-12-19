namespace Cobblepot.Domain.Modules.Finances.Assets;

// see - https://www.investopedia.com/terms/c/asset.asp
public class FinancialAsset : Entity, IAsset
{
    private string _title;
    private string? _description;
    private DateTime _createDate;
    private Money _value;

    public FinancialAsset(Guid id, string title, string description, , Money value) : base(id)
    {
        _title = title;
        _description = description;
        _createDate = SystemClock.Now;
        _value = value;
    }
    public FinancialAsset(Guid id, string title, ShortTermAssetType assetType, Money value) : base(id)
    {
        _title = title;
        _value = value;
        _createDate = SystemClock.Now;
    }


    public Money Value() => _value;
    public override string ToString() => $"Financial Asset: {_title}";
    public string Title() => _title;
    public string Description() => $"{_description ?? "no description"}: {_createDate}";
}

// TODO: Determine a method for gaining insights of value
// Financial assets are valued depending on how the investment is categorized and the motive behind it.