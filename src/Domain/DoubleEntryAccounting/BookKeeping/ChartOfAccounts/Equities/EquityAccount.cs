using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.Equities
{
    public class EquityAccount : IAccount
    {
        internal int MinCodeValue = 30_000;
        internal int MaxCodeValue = 39_999;

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