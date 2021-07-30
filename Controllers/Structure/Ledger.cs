using System;
using System.IO;
using System.Globalization;
using System.Text.RegularExpressions;

namespace Controllers.Structure
{
    public class LedgerStructure
    {
        protected readonly static string Ledger_Path = Path.Combine(Directory.GetCurrentDirectory(), "Ledger");

        protected static DateTime toDate(string dString)
        {
            DateTime date;
            CultureInfo culture = new CultureInfo("en-US");
            if (DateTime.TryParse(dString, culture, DateTimeStyles.None, out date))
            {
                return date;
            }
            else
            {
                throw new FormatException($"{dString} is not a proper format");
            }
        }

        protected bool isRegexMatch(string regexPattern, string argument)
        {
            if (argument != "" && Regex.IsMatch(regexPattern, argument))
            {
                return true;
            }
            return false;
        }
    }


    public class Ledger : LedgerStructure
    {
        public Ledger()
        {
            if (!Directory.Exists(Ledger_Path))
            {
                Directory.CreateDirectory(Ledger_Path);
                Console.WriteLine("Ledger Directory Created Successfully");
            }
        }
    }
}