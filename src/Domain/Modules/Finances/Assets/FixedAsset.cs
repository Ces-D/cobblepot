namespace Cobblepot.Domain.Modules.Finances.Assets;

// see - https://www.investopedia.com/terms/f/fixedasset.asp
public class FixedAsset : Entity, IAsset, IDescribable
{
    private string _title;
    private string _description;
    private TimeSpan _usefulLife;
    private Money _purchasePrice;
    private bool _isRealEstate;
    private Money _value;
    private List<Note> _notes;

    public string Title { get { return _title; } set { _title = value; } }
    public string Description { get { return _description; } set { _description = value; } }
    public Money Value { get { return _value; } }
    public bool IsRealEstate { get { return _isRealEstate; } }

    public FixedAsset(Guid id, string title, string? description, Money purchasePrice, TimeSpan usefulLife, bool? acceleratedDepreciation) : base(id, SystemClock.Now)
    {
        _title = title;
        _description = description ?? "no description";
        _usefulLife = usefulLife;
        _purchasePrice = purchasePrice;
        _isRealEstate = false;
        _value = DetermineValue(acceleratedDepreciation ?? false);
        _notes = new List<Note>();
    }

    public FixedAsset(Guid id, string title, string? description, Money purchasePrice, TimeSpan usefulLife, Money realEstateValue) : base(id, SystemClock.Now)
    {
        _title = title;
        _description = description ?? "no description";
        _usefulLife = usefulLife;
        _purchasePrice = purchasePrice;
        _isRealEstate = true;
        _value = realEstateValue;
        _notes = new List<Note>();
    }

    private Money DetermineValue(bool acceleratedDepreciation)
    {
        var timeInPossession = SystemClock.Now.Subtract(_created);
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

    private int UsefulLifeInYears() => _usefulLife.Days / 365;

    public void AddNote(Note note) => _notes.Add(note);

    public override string ToString() => $"Fixed Asset: {_title}, {_created}";
}

// see - https://www.investopedia.com/ask/answers/051215/how-do-you-determine-tangible-assets-useful-life.asp for one method of determing depreciation
// see - https://www.investopedia.com/terms/a/accelerateddepreciation.asp for the other method 
