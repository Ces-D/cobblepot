using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.Assets
{
    public class AssetAccount : IAccount
    {
        internal int MinCodeValue = 10000;
        internal int MaxCodeValue = 19_999;

        public Code IdentificationCode => throw new NotImplementedException();

        public ChartAccountType Group => ChartAccountType.Asset;

        public string SubGroup => throw new NotImplementedException();

        public EntryType ToIncrease => throw new NotImplementedException();

        public string FinancialStatementId => throw new NotImplementedException();
    }
}