using System;
using System.Text.RegularExpressions;

namespace Services.AccountFile
{
    public class AccountEntry
    {
        public const string FORMAT_ERROR_RESPONSE = "The detail of this entry do not match the format <assetType>:<country?>:<institution?>:<account>";
        private DateTime date { get; set; }
        private string account { get; set; }
        public AccountEntry(DateTime entryDate, string account)
        {
            this.date = entryDate;
            this.account = account;
        }

        public DateTime Date
        {
            get { return this.date; }
        }

        public string DateString
        {
            get
            {
                return this.date.ToShortDateString();
            }
        }

        public string Account
        {
            get
            {
                return this.account;
            }
        }

        public override string ToString()
        {
            return string.Format("{0}{1}",
                          arg0: this.DateString.PadRight(30),
                          arg1: this.Account);
        }
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