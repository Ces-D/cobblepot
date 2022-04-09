using Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts;
using Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.StandardTypes.AccountTypes;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.Interfaces
{
    public interface IAccountType<T>
    {
        Code IdentificationCode { get; }
        AccountType GetAccountType();
        // TODO: find a way to return the subAccount Type, might be from the account code
    }
}
