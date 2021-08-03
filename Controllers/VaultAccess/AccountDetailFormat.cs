using System.Text.RegularExpressions;

namespace Controllers.VaultAccess
{
    public static class AccountDetailFormat
    {
        public enum AccountTypes
        {
            Assets,
            Liabilities,
            Income,
            Expenses,
            Equity
        }

        public static bool matchesNamingConvention(string potential)
        {
            string pattern = @"(\w{4,7})(:([\w\d]{2,20})){1,6}";
            return Regex.IsMatch(potential, pattern);
        }

    }
}