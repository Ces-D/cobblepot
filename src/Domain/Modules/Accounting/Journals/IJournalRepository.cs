namespace Cobblepot.Domain.Accounting.Journals;
using System.Threading.Tasks;

public interface IJournalRepository
{
    Task AddEntryAsync(JournalEntry journalEntry);
    Task<JournalEntry> GetByIdAsync(Guid id);
}
