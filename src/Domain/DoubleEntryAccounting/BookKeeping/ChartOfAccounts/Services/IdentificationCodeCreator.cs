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
    internal class IdentificationCodeCreator
    {
        private AccountType _accountType;
        private string _subAccounttype;

        public IdentificationCodeCreator(AccountType accountType, string subAccountType)
        {
            _accountType = accountType;
            _subAccounttype = subAccountType;
        }

        public Code GenerateNew()
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

// TODO: complete the code creation for account and profile.This class is called during initilization of an account and after the other account
// properties where generated