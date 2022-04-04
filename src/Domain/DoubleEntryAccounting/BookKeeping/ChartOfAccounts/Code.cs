using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts
{
    /// <summary>
    ///  Code specific to this app. Each user of the app gets a unique profile id. In case one accountant is maintaining
    ///  multiple profiles
    /// </summary>
    public record Code
    {
        // The id of the owner of the account 
        public int ProfileCode { get; init; }
        // The code of the account following numbering norms
        public int AccountCode { get; init; }

        internal Code(int profileCode, int accountCode)
        {
            ProfileCode = profileCode;
            AccountCode = accountCode;
        }

        public override string ToString()
        {
            return $"{ProfileCode}-{AccountCode}";
        }
    }
}
