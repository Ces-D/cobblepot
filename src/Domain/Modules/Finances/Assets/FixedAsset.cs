namespace Cobblepot.Domain.Modules.Finances.Assets;

// see - https://www.investopedia.com/terms/c/asset.asp
public class FixedAsset : Entity, IAsset
{
    private string _title;
    private string? _description;
    private DateTime _createDate;
    private Money _value;

    public FixedAsset(Guid id, string title, string description, , Money value) : base(id)
    {
        _title = title;
        _description = description;
        _createDate = SystemClock.Now;
        _value = value;
    }
    public FixedAsset(Guid id, string title, ShortTermAssetType assetType, Money value) : base(id)
    {
        _title = title;
        _value = value;
        _createDate = SystemClock.Now;
    }


    public Money Value() => _value;
    public override string ToString() => $"Fixed Asset: {_title}";
    public string Title() => _title;
    public string Description() => $"{_description ?? "no description"}: {_createDate}";
}

// TODO: incorporate depreciation of long term assets
// see - https://www.investopedia.com/ask/answers/051215/how-do-you-determine-tangible-assets-useful-life.asp for one method of determing depreciation
// see - https://www.investopedia.com/terms/a/accelerateddepreciation.asp for the other method 