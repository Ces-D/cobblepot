using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.Expenses
{
    public class ExpenseAccount : IAccount
    {
        internal readonly int MinCodeValue = 50_000;
        internal readonly int MaxCodeValue = 59_999;

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