using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.StandardTypes;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts
{
    public class LiabilityAccount : IAccount
    {
        private readonly LiabilityCode _liabilityCode;
        private readonly Code _identificationCode;

        internal LiabilityAccount(Code identificationCode)
        {
            _identificationCode = identificationCode;
            _liabilityCode = LiabilityCode.Unknown;
        }

        internal LiabilityAccount(Code identificationCode, LiabilityCode liabilityCode)
        {
            _identificationCode = identificationCode;
            _liabilityCode = liabilityCode;
        }

        public Code IdentificationCode() => _identificationCode;
        public string Group() => "Liability";
        public string SubGroup() => _liabilityCode.ToString();
        public EntryType ToIncrease() => EntryType.Credit;
        // TODO: The financial statement is the only value allowed to change
    }
}
