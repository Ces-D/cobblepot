using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.Services
{
    /// <summary>
    /// Generates the identification code of each account in the Chart of Accounts. The code should be consistent across the entire
    /// application 
    /// </summary>
    internal class IdentificationCodeCreator
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

// TODO: complete the code creation for account and profile. This service should be montired by the chart of acccounts. Only a single instance of this 
// in the entrire app. The single instance makes sure that no duplicate code are created. When the user makes the account, either they provide the sub group
// or it is unknown.
// Decide on a system for creating the account codes.