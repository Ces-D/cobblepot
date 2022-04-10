using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.StandardTypes;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts
{
    internal class EquityAccount : IAccount
    {
        private readonly EquityCode _equityCode;
        private readonly Code _identificationCode;

        internal EquityAccount(Code identificationCode)
        {
            _identificationCode = identificationCode;
            _equityCode = EquityCode.Unknown;
        }

        internal EquityAccount(Code identificationCode, EquityCode equityCode)
        {
            _identificationCode = identificationCode;
            _equityCode = equityCode;
        }

        public Code IdentificationCode() => _identificationCode;
        public string Group() => "Equity";
        public string SubGroup() => _equityCode.ToString();
        public EntryType ToIncrease() => EntryType.Credit;
        // TODO: The financial statement is the only value allowed to change
    }
}
