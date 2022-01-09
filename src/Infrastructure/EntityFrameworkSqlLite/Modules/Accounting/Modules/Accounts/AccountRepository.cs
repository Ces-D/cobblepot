namespace Infrastructure.EntityFrameworkSqlLite.Accounting.Modules.Entries;
using Cobblepot.Domain.Accounting.Accounts;
using System.Linq.Expressions;

internal class AccountRepository : IRepository<Account>
{
    private readonly AccountingContext _accountingContext;

    internal AccountRepository(AccountingContext accountingContext)
    {
        _accountingContext = accountingContext;
    }

    public async Task AddAsync(Account entity)
    {
        await _accountingContext.AddAsync(entity);
    }

    public async Task AddRangeAsync(IEnumerable<Account> entities)
    {
        await _accountingContext.AddRangeAsync(entities);
    }

    public async Task<Account> GetAsync(Guid id)
    {
        return await _accountingContext.Accounts.FindAsync(id);
    }

    public IEnumerable<Account> GetWhere(Expression<Func<Account, bool>> predicate)
    {
        var journal = _accountingContext.Accounts.Where(predicate).ToList();
        return journal;
    }

    public void Delete(Account entity)
    {
        _accountingContext.Accounts.Remove(entity);
    }

    public void Update(Account entity)
    {
        _accountingContext.Entry(entity).State = EntityState.Modified;
    }
}

