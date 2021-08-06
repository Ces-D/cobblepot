using System.Text.RegularExpressions;
using Config;

namespace Controllers.VaultAccess
{
    public class JournalDetail
    {
        public enum AccountTypes
        {
            Assets,
            Liabilities,
            Income,
            Expenses,
            Equity
        }

        public bool DetailsMatchFormatConvention(string potential, string directive)
        {
            string pattern;
            if (directive == DirectiveOptions.Note)
            {
                pattern = @"([\w\d,./!&]{1,30})";
            }
            else
            {
                pattern = @"(Asset|Liability|Income|Expense|Equity)(:([\w\d]{2,20})){1,6}";
            }
            return Regex.IsMatch(potential, pattern);
        }

    }
}