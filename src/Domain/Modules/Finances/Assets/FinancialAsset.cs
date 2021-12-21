namespace Cobblepot.Domain.Modules.Finances.Assets;

// see - https://www.investopedia.com/terms/f/financialasset.asp
public class FinancialAsset : Entity, IAsset, IDescribable
{
    private string _title;
    private string _description;
    private Money _value;
    private List<Note> _notes;
    private Money _purchasePrice;
    private TimeSpan _timeOwned;

    public string Title { get { return _title; } set { _title = value; } }
    public string Description { get { return _description; } set { _description = value; } }
    public Money Value { get { return _value; } }
    public Money PurchasePrice { get { return _purchasePrice; } }
    public TimeSpan TimeOwned { get { return _timeOwned; } }

    public FinancialAsset(Guid id, string title, string? description, Money value, Money purchasePrice, TimeSpan? timeOwned) : base(id, SystemClock.Now)
    {
        _title = title;
        _description = description ?? "no description";
        _value = value;
        _purchasePrice = purchasePrice;
        _timeOwned = timeOwned ?? SystemClock.Now.Subtract(_created);
        _notes = new List<Note>();
    }

    public void AddNote(Note note) => _notes.Add(note);
    public override string ToString() => $"Financial Asset: {_title}, {_created}";
}

// TODO: Determine a method for gaining insights of value
// Financial assets are valued depending on how the investment is categorized and the motive behind it.