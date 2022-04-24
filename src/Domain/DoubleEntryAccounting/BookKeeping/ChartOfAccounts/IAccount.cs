using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts
{
    internal interface IAccount
    {
        Code IdentificationCode();
        string Group();
        string SubGroup();
        EntryType ToIncrease();
        string FinancialStatementId();
    }
}

// TODO: Add property public FinancialStatementId {get;set;} to Account Entity or Aggregate? 
