using System;
using System.IO;
namespace Controllers.Structure
{
    public class AccountClean : LedgerStructure
    {
        protected readonly static string Account_Path = Path.Combine(Ledger_Path, "Account");

        public AccountClean()
        {
            if (!Directory.Exists(Account_Path))
            {
                Directory.CreateDirectory(Account_Path);
                Console.WriteLine("Account Directory Created Successfully");
            }
        }

        // Returns true if the string matches the form Assets:US:BofA:Checking
        public bool matchesAssetChart(string asset)
        {
            string pattern = @"(Asset):([A-Z]{2}):\w{2,20}:\w{2,20}";
            return isRegexMatch(pattern, asset);
        }

        // Returns true if the string matches the form Expenses:Taxes:Y2012:US:Federal:PreTax401k
        public bool matchesExpenseChart(string expense)
        {
            string pattern = @"(Expense):(\w{2,10})(:{0,1}([\w\d]{2,20})){0,4}";
            return isRegexMatch(pattern, expense);
        }

        // Returns true if the string matches the form Income:US:ETrade:Gains
        public bool matchesIncomeChart(string income)
        {
            string pattern = @"(Income):(\w{2,10})(:{0,1}([\w\d]{2,20})){0,4}";
            return isRegexMatch(pattern, income);
        }

        // Returns true if the string matches the form Liability:US:Chase:Slate 
        public bool matchesLiabilityChart(string liability)
        {
            string pattern = @"(Liability):(\w{2,10})(:{0,1}([\w\d]{2,20})){0,4}";
            return isRegexMatch(pattern, liability);
        }

        // Returns true if the string matches the form Equity:Opening-Balances
        public bool matchesEquityChart(string equity)
        {
            string pattern = @"(Equity):[\s\S]{0,20}";
            return isRegexMatch(pattern, equity);
        }

    }
}