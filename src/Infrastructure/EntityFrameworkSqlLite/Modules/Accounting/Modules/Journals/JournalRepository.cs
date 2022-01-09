namespace Infrastructure.EntityFrameworkSqlLite.Accounting.Modules.Journals;
using Cobblepot.Domain.Accounting.Journals;
using System.Linq.Expressions;

internal class JournalRepository : IRepository<Journal>
{
    private readonly AccountingContext _accountingContext;

    internal JournalRepository(AccountingContext accountingContext)
    {
        _accountingContext = accountingContext;
    }

    public async Task AddAsync(Journal entity)
    {
        await _accountingContext.AddAsync(entity);
    }

    public async Task AddRangeAsync(IEnumerable<Journal> entities)
    {
        await _accountingContext.AddRangeAsync(entities);
    }

    public async Task<Journal> GetAsync(Guid id)
    {
        return await _accountingContext.Journals.FindAsync(id);
    }

    public IEnumerable<Journal> GetWhere(Expression<Func<Journal, bool>> predicate)
    {
        var journal = _accountingContext.Journals.Where(predicate).ToList();
        return journal;
    }

    public void Delete(Journal entity)
    {
        _accountingContext.Journals.Remove(entity);
    }

    public void Update(Journal entity)
    {
        _accountingContext.Entry(entity).State = EntityState.Modified;
    }
}
