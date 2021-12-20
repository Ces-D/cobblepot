namespace Cobblepot.Domain.Modules.Finances.Assets;

// see - https://www.investopedia.com/terms/c/asset.asp
public class FixedAsset : Entity, IAsset
{
    private string _title;
    private string? _description;
    private DateTime _createDate;
    private TimeSpan _usefulLife;
    private Money _purchasePrice;
    private bool _isRealEstate;
    private Money _value;

    public FixedAsset(Guid id, string title, string description, Money purchasePrice, TimeSpan usefulLife, bool? acceleratedDepreciation, bool isRealEstate) : base(id)
    {
        _title = title;
        _description = description;
        _createDate = SystemClock.Now;
        _usefulLife = usefulLife;
        _purchasePrice = purchasePrice;
        _isRealEstate = isRealEstate;
        _value = DetermineValue(acceleratedDepreciation ?? false);
    }


    public Money Value() => _value;
    public override string ToString() => $"Fixed Asset: {_title}";
    public string Title() => _title;
    public string Description() => $"{_description ?? "no description"}: {_createDate}";

    private int UsefulLifeInYears()
    {
        return _usefulLife.Days / 365;
    }

    private Money DetermineValue(bool acceleratedDepreciation)
    {
        var timeInPossession = SystemClock.Now.Subtract(_createDate);
        if (acceleratedDepreciation)
        {
            var rateOfDepreciation = (1 / UsefulLifeInYears()) * 2;
            var valueAmount = _purchasePrice.Amount / (_purchasePrice.Amount / rateOfDepreciation);
            return new Money(valueAmount, _purchasePrice.Currency);
        }
        else
        {
            var remainingLife = _usefulLife.Days - timeInPossession.Days;
            var rateOfDepreciation = _purchasePrice.Amount / (UsefulLifeInYears() * 100);
            var valueAmount = _purchasePrice.Amount - (_purchasePrice.Amount * (remainingLife * rateOfDepreciation));
            return new Money(valueAmount, _purchasePrice.Currency);
        }
    }
}

// see - https://www.investopedia.com/ask/answers/051215/how-do-you-determine-tangible-assets-useful-life.asp for one method of determing depreciation
// see - https://www.investopedia.com/terms/a/accelerateddepreciation.asp for the other method 

// TODO: determine value of fixed real estate assets