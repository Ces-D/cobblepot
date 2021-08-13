using System.Text.RegularExpressions;

namespace Services.Account
{
    public static class AccountFormat
    {
        public static string FORMAT_ERROR_RESPONSE = "The detail of this entry do not match the format <assetType>:<country?>:<institution?>:<account>";
        public static bool MatchesConvention(string potential)
        {
            string notePattern = @"(Asset|Liability|Income|Expense|Equity):([\w\d,./!&]{1,65}){1,2}";
            string generalPattern = @"(Asset|Liability|Income|Expense|Equity)(:([\w\d]{2,20})){1,6}";
            if (Regex.IsMatch(potential, notePattern) || Regex.IsMatch(potential, generalPattern))
            {
                return true;
            }
            else { return false; }
        }

    }
}