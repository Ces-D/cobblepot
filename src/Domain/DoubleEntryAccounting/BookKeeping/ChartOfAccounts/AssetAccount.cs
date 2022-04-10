using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.StandardTypes;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts
{
    public class AssetAccount : IAccount
    {
        private readonly AssetCode _assetCode;
        private readonly Code _identificationCode;

        internal AssetAccount(Code identificationCode)
        {
            _identificationCode = identificationCode;
            _assetCode = AssetCode.Unknown;
        }

        internal AssetAccount(Code identificationCode, AssetCode assetCode)
        {
            _identificationCode = identificationCode;
            _assetCode = assetCode;
        }

        public Code IdentificationCode() => _identificationCode;
        public string Group() => "Asset";
        public string SubGroup() => _assetCode.ToString();
        public EntryType ToIncrease() => EntryType.Debit;
        // TODO: The financial statement is the only value allowed to change
    }
}
