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
    public class Account
    {
        public Code IdentificationCode { get; init; }
        public AccountType Type { get; init; }
        public string SubType { get; private set; }
        public EntryType ToIncrease { get; private set; }

        // TODO: Add property public FinancialStatementId {get;set;} to Account Entity or Aggregate? 

        internal Account(AccountType accountType, Enum? subAccountGroup)
        {
            Type = accountType;
            ToIncrease = DetermineToIncrease();
            SubType = DetermineSubType(subAccountGroup);

            var idCodeCreator = new IdentificationCodeCreator(Type, SubType);
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

        private string DetermineSubType(Enum? subAccountGroup)
        {
            var subGroup = "Unknown";
            if (subAccountGroup is not null)
            {
                if (Type == AccountType.Assets && subAccountGroup is AssetCode)
                {
                    subGroup = subAccountGroup.ToString("G");
                }
                else if (Type == AccountType.Liabilities && subAccountGroup is LiabilityCode)
                {
                    subGroup = subAccountGroup.ToString("G");
                }
                else if (Type == AccountType.ShareholderEquity && subAccountGroup is ShareholderEquityCode)
                {
                    subGroup = subAccountGroup.ToString("G");
                }
                else if (Type == AccountType.Revenue && subAccountGroup is RevenueCode)
                {
                    subGroup = subAccountGroup.ToString("G");
                }
                else if (Type == AccountType.Expenses && subAccountGroup is ExpenseCode)
                {
                    subGroup = subAccountGroup.ToString("G");
                }
            }
            return subGroup;
        }

    }
}
