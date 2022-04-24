using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.Liabilities
{
    public class LiabilityAccount : IAccount
    {
        internal readonly static int MinCodeValue = 20_000;
        internal readonly static int MaxCodeValue = 29_999;

        public Code IdentificationCode => throw new NotImplementedException();

        public ChartAccountType Group => ChartAccountType.Liability;

        public string SubGroup => throw new NotImplementedException();

        public EntryType ToIncrease => throw new NotImplementedException();

        public string FinancialStatementId => throw new NotImplementedException();
    }
}