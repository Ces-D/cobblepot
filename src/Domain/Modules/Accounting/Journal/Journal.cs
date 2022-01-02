namespace Cobblepot.Domain.Accounting.Journal;

public class Journal : Entity
{
    private IJournalRepository _journalRepository;
    private List<JournalEntry> _entries;

    public Journal(Guid id, IJournalRepository journalRepository) : base(id, DateTime.UtcNow)
    {
        _entries = new List<JournalEntry>();
        _journalRepository = journalRepository;
    }

    public void AddEntry(JournalEntry journalEntry) { _entries.Add(journalEntry); }
}

// TODO: figure out how the repositories fit in