using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.StandardTypes.AccountTypes;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.Services
{
    /// <summary>
    /// Generates the identification code of each account in the Chart of Accounts. The code should be consistent across the entire
    /// application 
    /// </summary>
    internal class IdentificationCodeCreator<T> where T : Enum
    {

        public IdentificationCodeCreator()
        {
        }

        public Code GenerateIdCode()
        {
            int profileCode = GenerateProfileCode();
            int accountCode = GenerateAccountCode();
            return new Code(profileCode, accountCode);
        }

        private int GenerateAccountCode()
        {
            return 0;
        }

        private int GenerateProfileCode()
        {
            return 0;
        }
    }
}

// TODO: complete the code creation for account and profile.