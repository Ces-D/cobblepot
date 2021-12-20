namespace Cobblepot.Domain.Modules.Finances.Assets;

// see - https://www.investopedia.com/terms/c/currentassets.asp
public class ShortTermAsset : Entity, IAsset
{
    private string _title;
    private string? _description;
    private DateTime _createDate;
    private ShortTermAssetType _type;
    private Money _value;

    public ShortTermAsset(Guid id, string title, string description, ShortTermAssetType assetType, Money value) : base(id)
    {
        _title = title;
        _description = description;
        _type = assetType;
        _createDate = SystemClock.Now;
        _value = value;
    }
    public ShortTermAsset(Guid id, string title, ShortTermAssetType assetType, Money value) : base(id)
    {
        _title = title;
        _type = assetType;
        _value = value;
        _createDate = SystemClock.Now;
    }


    public Money Value() => _value;
    public override string ToString() => $"Short-Term Asset: {_title}";
    public string Title() => _title;
    public string Description() => $"{_description ?? "no description"}: {_createDate}";
    public string AssetType() => _type.ToString();
}

// TODO: add functions for updating these values