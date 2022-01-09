namespace Cobblepot.Domain.Accounting.Journals;
using Cobblepot.Domain.Accounting.Entries;

public class Journal : EntityBase, IAggregateRoot, IEntity
{
    private readonly Guid _id;
    private List<Entry> _entries;
    private DateTime _startDate;
    private DateTime _endDate;

    public Guid Id => _id;

    public Journal() : base(DateTime.UtcNow)
    {
        _id = Guid.NewGuid();
        _entries = new List<Entry>();
        _startDate = DateTime.UtcNow;
        _endDate = DateTime.UtcNow;
    }

    private void SetDates()
    {
        IEnumerable<DateTime> entryDates = _entries.Select(entry => entry.TransactionDate);
        DateTime minDate = entryDates.Min();
        DateTime maxDate = entryDates.Max();
        _startDate = minDate;
        _endDate = maxDate;
    }

    public void Add(Entry entry)
    {
        this.CheckRule(new NoDuplicateJournalEntriesRule(_entries, entry));

        var opposite = entry.CreateOpposite();
        _entries.Add(entry);
        _entries.Add(opposite);
        this.SetDates();
    }
}
// Create Context for Journals and do this but gor Accounts