namespace Cobblepot.Domain.Accounting.Journal;
using System.Threading.Tasks;

public interface IJournalRepository
{
    Task AddAsync(Journal journal);
    Task<Journal> GetByIdAsync(Guid id);
}
