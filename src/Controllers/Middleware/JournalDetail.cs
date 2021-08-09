using System.Text.RegularExpressions;
using System.CommandLine.Parsing;

namespace Controllers.Middleware
{
    public static class JournalDetail
    {
        public static string FORMAT_ERROR_RESPONSE = "The detail of this entry do not match the format <assetType>:<country?>:<institution?>:<account>";
        public static bool MatchesFormatConvention(string potential)
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