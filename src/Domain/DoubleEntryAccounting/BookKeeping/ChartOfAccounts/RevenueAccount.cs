using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.StandardTypes;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts
{
    public class RevenueAccount : IAccount
    {
        private readonly RevenueCode _revenueCode;
        private readonly Code _identificationCode;

        internal RevenueAccount(Code identificationCode)
        {
            _identificationCode = identificationCode;
            _revenueCode = RevenueCode.Unknown;
        }

        internal RevenueAccount(Code identificationCode, RevenueCode revenueCode)
        {
            _identificationCode = identificationCode;
            _revenueCode = revenueCode;
        }

        public Code IdentificationCode() => _identificationCode;
        public string Group() => "Revenue";
        public string SubGroup() => _revenueCode.ToString();
        public EntryType ToIncrease() => EntryType.Credit;
        // TODO: The financial statement is the only value allowed to change
    }
}
