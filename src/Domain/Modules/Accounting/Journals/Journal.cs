namespace Cobblepot.Domain.Accounting.Journals;

public class Journal : Entity
{
    private IJournalRepository _journalRepository;

    public Journal(Guid id, IJournalRepository journalRepository) : base(id, DateTime.UtcNow)
    {
        _journalRepository = journalRepository;
    }

    public void AddEntry(JournalEntry journalEntry)
    {
        _journalRepository.AddEntryAsync(journalEntry);
    }
}