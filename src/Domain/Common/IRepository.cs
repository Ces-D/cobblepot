namespace Cobblepot.Domain.Common;
using System.Linq.Expressions;

public interface IRepository<T> where T : IAggregateRoot
{
    Task<T> GetAsync(Guid id);
    IEnumerable<T> GetWhere(Expression<Func<T, bool>> predicate);
    Task AddAsync(T entity);
    Task AddRangeAsync(IEnumerable<T> entities);
    void Update(T entity);
    void Delete(T entity);
}
