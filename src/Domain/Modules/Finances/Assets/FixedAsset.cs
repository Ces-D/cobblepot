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
    private Money _totalValue;
    private int _amount;
    private List<Note> _notes;

    public string Title { get { return _title; } set { _title = value; } }
    public string Description { get { return _description; } set { _description = value; } }
    public Money Value { get { return _value; } }
    public Money TotalValue { get { return _totalValue; } }
    public int Amount { get { return _amount; } }
    public bool IsRealEstate { get { return _isRealEstate; } }

    public FixedAsset(Guid id, string title, string? description, Money purchasePrice, TimeSpan usefulLife, int amount, bool? acceleratedDepreciation) : base(id, SystemClock.Now)
    {
        _title = title;
        _description = description ?? "no description";
        _usefulLife = usefulLife;
        _purchasePrice = purchasePrice;
        _isRealEstate = false;
        _value = DetermineValue(acceleratedDepreciation ?? false);
        _amount = amount;
        _totalValue = CalculateTotalValue();
        _notes = new List<Note>();
    }

    public FixedAsset(Guid id, string title, string? description, Money realEstatePrice, TimeSpan usefulLife) : base(id, SystemClock.Now)
    {
        _title = title;
        _description = description ?? "no description";
        _usefulLife = usefulLife;
        _purchasePrice = realEstatePrice;
        _isRealEstate = true;
        _value = realEstatePrice;
        _totalValue = CalculateTotalValue();
        _amount = 1;
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

    public void IncreaseAmount(int amountAdded, Money? newPurchasePrice)
    {
        if (!_isRealEstate)
        {
            _amount += amountAdded;
            _value = newPurchasePrice ?? _value;
            _totalValue = CalculateTotalValue();
        }
    }

    public void DecreaseAmount(int amountRemoved)
    {
        this.CheckRule(new AmountDecreasedIsNotMoreThanOwnedRule(_amount, amountRemoved));

        _amount -= amountRemoved;
        _totalValue = CalculateTotalValue();
    }

    public void UpdateValue(Money newValue)
    {
        this.CheckRule(new UpdateValueMustBePositiveRule(newValue));

        _value = newValue;
        _totalValue = CalculateTotalValue();
    }

    public void AddNote(Note note) => _notes.Add(note);
    private int UsefulLifeInYears() => _usefulLife.Days / 365;
    private Money CalculateTotalValue() => new(_value.Amount * _amount, _value.Currency);

    public override string ToString() => $"Fixed Asset: {_title}, {_created}";
}

// see - https://www.investopedia.com/ask/answers/051215/how-do-you-determine-tangible-assets-useful-life.asp for one method of determing depreciation
// see - https://www.investopedia.com/terms/a/accelerateddepreciation.asp for the other method 
