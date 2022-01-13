namespace Cobblepot.Domain.Accounting.Journals;
using Cobblepot.Domain.Accounting.Entries;

public class Journal : EntityBase, IAggregateRoot, IEntity
{
    private readonly Guid _id;
    private List<Entry> _entries;

    public Guid Id { get { return _id; } }
    public List<Entry> Entries { get { return _entries; } }

    public Journal() : base(DateTime.UtcNow)
    {
        _id = Guid.NewGuid();
        _entries = new List<Entry>();
    }

    internal Journal(Guid id, DateTime createdDate, List<Entry> entries) : base(createdDate)
    {
        _id = id;
        _entries = entries;
    }

    public void Add(Entry entry)
    {
        this.CheckRule(new NoDuplicateJournalEntriesRule(_entries, entry));

        var opposite = entry.CreateOpposite();
        _entries.Add(entry);
        _entries.Add(opposite);
    }

    public IEnumerable<Entry> Add(Entry entry, bool includeOpposite)
    {
        this.CheckRule(new NoDuplicateJournalEntriesRule(_entries, entry));

        var opposite = entry.CreateOpposite();
        _entries.Add(entry);
        _entries.Add(opposite);

        return includeOpposite ? new List<Entry>(2) { entry, opposite } : new List<Entry>(1) { entry };
    }
}
// Create Context for Journals and do this but gor Accounts