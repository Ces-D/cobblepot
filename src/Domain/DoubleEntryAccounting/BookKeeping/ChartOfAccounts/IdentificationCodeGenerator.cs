using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts
{
    internal class IdentificationCodeGenerator
    {
        protected IChartAccountRepository<IAccount> _accountRepository;

        internal IdentificationCodeGenerator(IChartAccountRepository<IAccount> accountRepository)
        {
            _accountRepository = accountRepository;
        }

        public async Task<Code> CreateNew(int minAccountCode, int maxAccountCode)
        {
            var existingAccountsInAccountCodeRange = await _accountRepository.GetAccountsByAccountCodeRangeAsync(minAccountCode, maxAccountCode);
            int lastAccountCodeInsert = 0;
            foreach (IAccount account in existingAccountsInAccountCodeRange)
            {
                if (account.IdentificationCode.AccountCode > lastAccountCodeInsert)
                {
                    lastAccountCodeInsert = account.IdentificationCode.AccountCode;
                }
            }

            return new Code(0, lastAccountCodeInsert++);
        }
    }
}
// TODO: this should also be passed the profile code. And the repository should also check for account code range that has this profile code