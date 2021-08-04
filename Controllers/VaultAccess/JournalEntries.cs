using System;
using System.Data;
using System.Globalization;

namespace Controllers.VaultAccess
{

    public class JournalEntry
    {
        private readonly int DATE_MAX_WIDTH = 13;
        private readonly int DIRECTIVE_MAX_WIDTH = 7;
        private readonly int DETAILS_MAX_WIDTH = 45;
        private DateTime entryDate { get; set; }
        private string directive { get; set; }
        private string details { get; set; }
        private double currency { get; set; }
        public JournalEntry(DateTime date, string dir, string det, double cur)
        {
            entryDate = date;
            directive = dir;
            details = det;
            currency = cur;
        }

        // TODO: add the currency symbols $ 
        public override string ToString()
        {
            return string.Format("{0}{1}{2}{3}",
            entryDate.ToShortDateString().PadRight(DATE_MAX_WIDTH),
            directive.PadRight(DIRECTIVE_MAX_WIDTH),
            details.PadRight(DETAILS_MAX_WIDTH),
            currency);
        }


    }

}