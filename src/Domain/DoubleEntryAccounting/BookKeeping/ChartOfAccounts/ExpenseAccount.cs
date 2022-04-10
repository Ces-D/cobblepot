using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.StandardTypes;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts
{
    public class ExpenseAccount : IAccount
    {
        private readonly ExpenseCode _expenseCode;
        private readonly Code _identificationCode;

        internal ExpenseAccount(Code identificationCode)
        {
            _identificationCode = identificationCode;
            _expenseCode = ExpenseCode.Unknown;
        }

        internal ExpenseAccount(Code identificationCode, ExpenseCode expenseCode)
        {
            _identificationCode = identificationCode;
            _expenseCode = expenseCode;
        }

        public Code IdentificationCode() => _identificationCode;
        public string Group() => "Expense";
        public string SubGroup() => _expenseCode.ToString();
        public EntryType ToIncrease() => EntryType.Debit;
        // TODO: The financial statement is the only value allowed to change
    }
}
