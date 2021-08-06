using System.Text.RegularExpressions;
using System.CommandLine;
using System.CommandLine.Invocation;
using Controllers.Commands;

namespace Controllers.VaultAccess.Middleware
{
    public static class JournalDetail
    {
        public static void checkDetailFormat(InvocationContext context, Argument<string> detailArgument)
        {
            var argument = context.ParseResult.FindResultFor(detailArgument);


        }

        private static bool MatchFormatConvention(string potential, string directive)
        {
            string pattern;
            if (directive == JournalDirectives.Note)
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