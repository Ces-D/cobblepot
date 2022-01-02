namespace Cobblepot.Domain.Modules.Finances.Revenues;

// see - https://www.investopedia.com/terms/o/operating-revenue.asp
public class OperatingRevenue : Entity, IRevenue, IDescribable
{
    private string _title;
    private string _description;
    private Money _revenue;
    private RevenueType _revenueType;
    private List<Note> _notes;

    public Money Revenue { get { return _revenue; } }
    public RevenueType RevenueType { get { return _revenueType; } }
    public string Title { get { return _title; } set { _title = value; } }
    public string Description { get { return _description; } set { _description = value; } }

    public OperatingRevenue(Guid id, string title, string? description, Money revenue, RevenueType revenueType) : base(id, SystemClock.Now)
    {
        _title = title;
        _description = description ?? "no description";
        _revenue = revenue;
        _revenueType = revenueType;
        _notes = new List<Note>();
    }

    public void AddNote(Note note) => _notes.Add(note);

    public override string ToString() => $"Operating Revenue: {_title}, {_created}";
}

// TODO: review for any possible changes to revenues, including adding rules