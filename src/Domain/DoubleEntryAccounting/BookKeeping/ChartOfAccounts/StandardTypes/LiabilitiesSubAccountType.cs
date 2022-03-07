using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.Interfaces;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.StandardTypes
{
    // TODO: replace some types to be more general; also add the other account sub types fake enums
    // see - https://corporatefinanceinstitute.com/resources/knowledge/accounting/types-of-liabilities/
    public record LiabilitiesSubAccountType : ISubAccountType
    {
        public static readonly string AccountsPayable = "Accounts Payable";
        public static readonly string CharitableContributionsPayable = "Charitable Contributions Payable";
        public static readonly string FederalIncomeTaxPayable = "Federal Income Tax Payable";
        public static readonly string FederalUnemploymentTaxPayable = "Federal Unemployment Tax Payable";
        public static readonly string InsurancePayable = "Insurance Payable";
        public static readonly string InterestPayable = "Interest Payable";
        public static readonly string LoanPayable = "Loan Payable";
        public static readonly string MedicareMedicaidPayable = "Medicare/Medicaid Payable";
        public static readonly string NotesPayable = "Notes Payable";
        public static readonly string ObligationUnderCapitalLease = "Obligation under Capital Lease";
        public static readonly string RetirementContributionPayable = "Retirement Contribution Payable";
        public static readonly string StateIncomeTaxPayable = "State Income Tax Payable";
        public static readonly string UnEarnedRevenue = "UnEarned Revenue";

    }
}
