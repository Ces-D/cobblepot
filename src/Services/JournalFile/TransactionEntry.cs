using System;
using System.Globalization;
using Services.AccountFile;

namespace Services.JournalFile
{
    public class TransactionEntry
    {
        private string details { get; set; }
        private double number { get; set; }
        private AccountEntry accountEntry { get; set; }
        private CultureInfo currency { get; set; }
        public TransactionEntry(AccountEntry accountEntry, string transactionDetails, double transactionNumber, string transactionCurrency)
        {
            this.accountEntry = accountEntry;
            this.details = transactionDetails;
            this.number = transactionNumber;
            this.currency = new CultureInfo(transactionCurrency);
        }

        public string Details { get { return this.details; } }

        public string DateString { get { return this.accountEntry.DateString; } }

        public DateTime Date { get { return this.accountEntry.Date; } }

        public string Money
        {
            get
            {
                NumberFormatInfo moneyFormat = currency.NumberFormat;
                moneyFormat.CurrencyPositivePattern = 2;
                return this.number.ToString("C", moneyFormat);
            }
        }
    }
}