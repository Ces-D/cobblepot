using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.Revenues
{
    public class RevenueAccount : IAccount
    {
        internal int MinCodeValue = 40_000;
        internal int MaxCodeValue = 49_999;

        public string FinancialStatementId()
        {
            throw new NotImplementedException();
        }

        public string Group()
        {
            throw new NotImplementedException();
        }

        public Code IdentificationCode()
        {
            throw new NotImplementedException();
        }

        public string SubGroup()
        {
            throw new NotImplementedException();
        }

        public EntryType ToIncrease()
        {
            throw new NotImplementedException();
        }
    }
}