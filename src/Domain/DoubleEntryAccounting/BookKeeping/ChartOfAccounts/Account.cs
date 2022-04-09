using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.Services;
using Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.StandardTypes;
using Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.Interfaces;
using Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.StandardTypes.AccountTypes;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts
{
    public class Account<T> : IAccountType<T> where T : Enum
    {
        public Code IdentificationCode { get; init; }
        public EntryType ToIncrease { get; private set; }
        // TODO: Add property public FinancialStatementId {get;set;} to Account Entity or Aggregate? 

        internal Account()
        {
            IdentificationCodeCreator<T> identificationCodeCreator = new();
            IdentificationCode = identificationCodeCreator.GenerateIdCode();
            ToIncrease = DetermineToIncrease();
        }

        public AccountType GetAccountType()
        {
            throw new NotImplementedException();
        }
        private EntryType DetermineToIncrease()
        {
            return EntryType.Credit;
        }
    }

}


// TODO: fill out the logic in the un implemented methods