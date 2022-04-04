using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.StandardTypes;
using Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.StandardTypes.AccountTypes;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts
{
    public record AccountGroup
    {
        public AccountType Type { get; init; }
        public string SubType { get; init; }
        public EntryType ToIncrease { get; init; }

        public AccountGroup(AccountType accountType, string subGroup, EntryType toIncrease)
        {
            Type = accountType;
            SubType = subGroup;
            ToIncrease = toIncrease;
        }
    }
}
