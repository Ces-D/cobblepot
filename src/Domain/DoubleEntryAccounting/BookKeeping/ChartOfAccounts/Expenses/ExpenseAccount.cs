using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.Expenses
{
    public class ExpenseAccount : IAccount
    {
        internal readonly static int MinCodeValue = 50_000;
        internal readonly static int MaxCodeValue = 59_999;

        public Code IdentificationCode => throw new NotImplementedException();

        public ChartAccountType Group => ChartAccountType.Expense;

        public string SubGroup => throw new NotImplementedException();

        public EntryType ToIncrease => throw new NotImplementedException();

        public string FinancialStatementId => throw new NotImplementedException();
    }
}