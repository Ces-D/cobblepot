namespace Cobblepot.Domain.Modules.Finances.Assets;

// see - https://www.investopedia.com/terms/c/currentassets.asp
public class ShortTermAsset : Entity, IAsset, IDescribable
{
    private string _title;
    private string _description;
    private ShortTermAssetType _type;
    private Money _value;
    private Money _totalValue;
    private int _amount;
    private List<Note> _notes;


    public string Title { get { return _title; } set { _title = value; } }
    public string Description { get { return _description; } set { _description = value; } }
    public Money Value { get { return _value; } }
    public Money TotalValue { get { return _totalValue; } }
    public int Amount { get { return _amount; } }
    public string AssetType { get { return _type.ToString(); } }

    public ShortTermAsset(Guid id, string title, string? description, ShortTermAssetType assetType, Money value, int? amount) : base(id, SystemClock.Now)
    {
        _title = title;
        _description = description ?? "no description";
        _type = assetType;
        _value = value;
        _amount = amount ?? 1;
        _totalValue = CalculateTotalValue();
        _notes = new List<Note>();
    }

    public void IncreaseAmount(int amountAdded, Money? newPurchasePrice)
    {
        _amount += amountAdded;
        _value = newPurchasePrice ?? _value;
        _totalValue = CalculateTotalValue();
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

    private Money CalculateTotalValue() => new(_value.Amount * _amount, _value.Currency);
    public void AddNote(Note note) => _notes.Add(note);
    public override string ToString() => $"Short-Term Asset: {_title}, {_created}";
}