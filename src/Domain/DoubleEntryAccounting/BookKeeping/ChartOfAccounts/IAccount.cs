using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.StandardTypes;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts
{
    internal interface IAccount
    {
        Code IdentificationCode();
        string Group();
        string SubGroup();
        EntryType ToIncrease();

    }
}

// TODO: Add property public FinancialStatementId {get;set;} to Account Entity or Aggregate? 
