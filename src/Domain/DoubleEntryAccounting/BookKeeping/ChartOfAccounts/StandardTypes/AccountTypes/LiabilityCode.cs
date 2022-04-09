using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.StandardTypes.AccountTypes
{
    public enum LiabilityCode:int
    {
        Unknown,
        AccountsPayable,
        CharitableContributionsPayable,
        FederalIncomeTaxPayable,
        FederalUnemploymentTaxPayable,
        InsurancePayable,
        InterestPayable,
        LoanPayable,
        MedicareMedicaidPayable,
        NotesPayable,
        ObligationUnderCapitalLease,
        RetirementContributionPayable,
        StateIncomeTaxPayable,
        UnEarnedRevenue,

    }
}
