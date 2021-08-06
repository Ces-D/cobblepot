using System;
using System.Globalization;
namespace Controllers.VaultAccess
{
    public static class JournalDirectives
    {
        public static readonly string Open = "open";
        public static readonly string Close = "close";
        public static readonly string Note = "note";
        public static readonly string Price = "price";
        public static readonly string Balance = "bal";
        public static readonly string Default = "*";
    }

    public class JournalEntry
    {
        private readonly int DATE_MAX_WIDTH = 13;
        private readonly int DIRECTIVE_MAX_WIDTH = 7;
        private readonly int DETAILS_MAX_WIDTH = 55;
        private DateTime entryDate { get; set; }
        private string directive { get; set; }
        private string details { get; set; }
        private double credit { get; set; }
        private CultureInfo currency { get; set; }

        public JournalEntry(DateTime date, string dir, string det, double cred, string cur)
        {
            entryDate = date;
            directive = dir;
            credit = cred;
            details = det;
            currency = new CultureInfo(cur);
        }

        public string DateString
        {
            get
            {

                return entryDate.ToShortDateString();
            }
        }
        public DateTime Date { get { return entryDate; } }

        public string Directive
        {
            get
            {
                return directive;
            }
        }

        public string Details
        {
            get
            {
                return details;
            }
        }

        public string Money
        {
            get
            {
                NumberFormatInfo moneyFormat = currency.NumberFormat;
                moneyFormat.CurrencyPositivePattern = 2;

                return credit.ToString("C", moneyFormat);
            }
        }

        public override string ToString()
        {
            return string.Format("{0}{1}{2}{3}",
            entryDate.ToShortDateString().PadRight(DATE_MAX_WIDTH),
            directive.PadRight(DIRECTIVE_MAX_WIDTH),
            details.PadRight(DETAILS_MAX_WIDTH),
            Money
            );
        }
    }

}