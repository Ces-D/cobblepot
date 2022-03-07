using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.Interfaces;
using Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.StandardTypes;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts
{
    public class Account
    {
        public Code IdentificationCode { get; private set; }
        internal AccountType Group { get; private set; }
        internal ISubAccountType? SubGroup { get; private set; }
        public EntryType ToIncrease { get; private set; }
        // TODO: Add property public FinancialStatementId {get;set;} to Account Entity or Aggregate? 
        internal Account()
        {
            // TODO: add the initializers for the properties. This is going to require the creation of IDCodeService 
        }


    }
}
