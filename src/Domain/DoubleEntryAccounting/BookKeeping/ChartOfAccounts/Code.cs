using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts
{
    public record Code
    {
        /// <summary>
        ///  Code specific to this app. Each user of the app gets a unique profile id. In case one accountant is maintaining
        ///  multiple profiles
        /// </summary>
        public int ProfileCode { get; init; }
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
