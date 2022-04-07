using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.Services;
using Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.StandardTypes;
using Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.StandardTypes.AccountTypes;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts
{
    public class Account<T> where T : Enum
    {
        public Code IdentificationCode { get; init; }
        public AccountType Type { get; init; }
        public T? SubType { get; private set; }
        public EntryType ToIncrease { get; private set; }

        // TODO: Add property public FinancialStatementId {get;set;} to Account Entity or Aggregate? 

        internal Account(AccountType accountType, T? subAccountGroup)
        {
            Type = accountType;
            ToIncrease = DetermineToIncrease();
            if (subAccountGroup is not null)
            {
                SubType = DetermineSubType(subAccountGroup);
            }

            var idCodeCreator = new IdentificationCodeCreator<T>(Type, SubType);
            IdentificationCode = idCodeCreator.GenerateNew();

        }

        private EntryType DetermineToIncrease()
        {
            return Type switch
            {
                AccountType.Assets => EntryType.Debit,
                AccountType.Liabilities => EntryType.Credit,
                AccountType.ShareholderEquity => EntryType.Credit,
                AccountType.Revenue => EntryType.Credit,
                AccountType.Expenses => EntryType.Debit,
                _ => throw new ArgumentOutOfRangeException(nameof(Type), $"Not expected account value: {Type}"),
            };
        }

        private T DetermineSubType(T subAccountGroup)
        {
            if (Type == AccountType.Assets && subAccountGroup is AssetCode)
            {
                return subAccountGroup;
            }
            else if (Type == AccountType.Liabilities && subAccountGroup is LiabilityCode)
            {
                return subAccountGroup;
            }
            else if (Type == AccountType.ShareholderEquity && subAccountGroup is ShareholderEquityCode)
            {
                return subAccountGroup;
            }
            else if (Type == AccountType.Revenue && subAccountGroup is RevenueCode)
            {
                return subAccountGroup;
            }
            else if (Type == AccountType.Expenses && subAccountGroup is ExpenseCode)
            {
                return subAccountGroup;
            }
            else throw new ArgumentOutOfRangeException(nameof(subAccountGroup), "Is of incorrect type");
        }

    }
}

// TODO: redo this and the Id Code Creator because the T generic code can be shortened. 