using System.Text.RegularExpressions;

namespace Controllers.Middleware
{
    public static class JournalDetail
    {
        public static bool MatchesFormatConvention(string potential)
        {
            string notePattern = @"(Asset|Liability|Income|Expense|Equity):([\w\d,./!&]{1,45}){1,2}";
            string generalPattern = @"(Asset|Liability|Income|Expense|Equity)(:([\w\d]{2,20})){1,6}";
            if (Regex.IsMatch(potential, notePattern) || Regex.IsMatch(potential, generalPattern))
            {
                return true;
            }
            else { return false; }
        }

    }
}