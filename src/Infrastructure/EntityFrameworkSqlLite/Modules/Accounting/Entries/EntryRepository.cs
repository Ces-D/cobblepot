namespace Infrastructure.EntityFrameworkSqlLite.Accounting.Entries;
using Cobblepot.Domain.Accounting.Entries;
using System.Linq.Expressions;

internal class EntryRepository : IRepository<Entry>
{
    private readonly AccountingContext _accountingContext;

    internal EntryRepository(AccountingContext accountingContext)
    {
        _accountingContext = accountingContext;
    }

    public async Task AddAsync(Entry entity)
    {
        await _accountingContext.AddAsync(entity);
    }

    public async Task AddRangeAsync(IEnumerable<Entry> entities)
    {
        await _accountingContext.AddRangeAsync(entities);
    }

    public async Task<Entry> GetAsync(Guid id)
    {
        return await _accountingContext.Entries.FindAsync(id);
    }

    public IEnumerable<Entry> GetWhere(Expression<Func<Entry, bool>> predicate)
    {
        var journal = _accountingContext.Entries.Where(predicate).ToList();
        return journal;
    }

    public void Delete(Entry entity)
    {
        _accountingContext.Entries.Remove(entity);
    }

    public void Update(Entry entity)
    {
        _accountingContext.Entry(entity).State = EntityState.Modified;
    }

        public void Save()
    {
        _accountingContext.SaveChanges();
    }

}

