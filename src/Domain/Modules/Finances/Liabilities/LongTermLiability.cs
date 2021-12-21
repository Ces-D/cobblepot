namespace Cobblepot.Domain.Modules.Finances.Liabilities;

// see - https://www.investopedia.com/terms/c/currentassets.asp

public class LongTermLiability : Entity, ILiability, IDescribable
{
    private string _title;
    private string _description;
    private Money _cost;
    private DateTime _maturity;
    private List<Note> _notes;

    public string Title { get { return _title; } set { _title = value; } }
    public string Description { get { return _description; } set { _description = value; } }
    public Money Cost { get { return _cost; } }
    public DateTime Maturity { get { return _maturity; } }

    public LongTermLiability(Guid id, string title, string? description, Money cost, DateTime maturityDate) : base(id, SystemClock.Now)
    {
        _title = title;
        _description = description ?? "no description";
        _cost = cost;
        _maturity = maturityDate;
        _notes = new List<Note>();
    }

    public void AddNote(Note note) => _notes.Add(note);

    public override string ToString() => $"Long-Term Liability: {_title}, {_created}";
}
