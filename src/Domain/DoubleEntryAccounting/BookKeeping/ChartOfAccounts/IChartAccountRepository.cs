using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts
{
    public interface IChartAccountRepository<T> where T : IAccount
    {

        Task<T> GetByIdentificationCodeAsync(Code code);
        Task<List<T>> GetAccountsByAccountCodeRangeAsync(int minAccountCode, int maxAccountCode);
        Task<T> CreateAsync(T chartAccount);
        Task<List<T>> GetAccountsAsync();
        Task<T> SaveAsync(T chartAccount, bool upsert = true);
    }
}