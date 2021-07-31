using System;
using System.Text.RegularExpressions;

namespace Schemas
{
    interface IFormatting
    {
        bool matchesNamingConvention(string potential);
    }

    public class AccountSchema : IFormatting
    {
        public enum AccountTypes
        {
            Assets,
            Liabilities,
            Income,
            Expenses,
            Equity
        }
        public bool matchesNamingConvention(string potential)
        {
            string pattern = @"(\w{4,7})(:([\w\d]{2,20})){1,6}";
            return Regex.IsMatch(potential, pattern);
        }
    }
}