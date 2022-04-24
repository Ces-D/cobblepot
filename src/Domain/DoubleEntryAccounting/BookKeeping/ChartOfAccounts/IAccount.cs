using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts
{
    public interface IAccount
    {
        Code IdentificationCode { get; }
        ChartAccountType Group { get; }
        string SubGroup { get; }
        EntryType ToIncrease { get; }
        string FinancialStatementId { get; }
    }
}

// TODO: Add property public FinancialStatementId {get;set;} to Account Entity or Aggregate? 
