using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.Revenues
{
    public class RevenueAccount : IAccount
    {
        internal readonly static int MinCodeValue = 40_000;
        internal readonly static int MaxCodeValue = 49_999;

        public Code IdentificationCode => throw new NotImplementedException();

        public ChartAccountType Group => ChartAccountType.Revenue;

        public string SubGroup => throw new NotImplementedException();

        public EntryType ToIncrease => throw new NotImplementedException();

        public string FinancialStatementId => throw new NotImplementedException();
    }
}