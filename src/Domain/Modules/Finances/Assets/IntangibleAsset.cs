namespace Cobblepot.Domain.Modules.Finances.Assets;

// see - https://www.investopedia.com/terms/i/intangibleasset.asp
public class IntangibleAsset : Entity, IAsset, IDescribable
{
    private string _title;
    private string _description;
    private Money _value;
    private List<Note> _notes;

    public string Title { get { return _title; } set { _title = value; } }
    public string Description { get { return _description; } set { _description = value; } }
    public Money Value { get { return _value; } }

    public IntangibleAsset(Guid id, string title, string? description) : base(id, SystemClock.Now)
    {
        _title = title;
        _description = description ?? "no description";
        _value = new Money(0, Currency.USD);
        _notes = new List<Note>();
    }

    public void AddNote(Note note) => _notes.Add(note);

    public override string ToString() => $"Intangible Asset: {_title}, {_created}";

}

// Accounting for intangible assets differs depending on the type of asset, and they can be either amortized or tested for impairment each year.