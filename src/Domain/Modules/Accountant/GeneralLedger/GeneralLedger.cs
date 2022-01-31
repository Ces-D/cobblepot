namespace Cobblepot.Domain.Accountant.GeneralLedger;
using Cobblepot.Domain.Common;
using Cobblepot.Domain.Accountant.GeneralLedger.GeneralLedgerAccounts;
public class GeneralLedger : EntityBase, IAggregateRoot
{
    public List<GeneralLedgerAccount> ChartOfAccounts { get; private set; }

    public GeneralLedger():base(Guid.NewGuid(), DateTime.UtcNow)
    {
        ChartOfAccounts = new List<GeneralLedgerAccount>();
    }

    // this needs an addEntry method. the method should take variety of params. Sghould it take 
    /**
     * a AccountCategory - i.e accounts payavle etc
     * right now the GeneralLedgerAccount is meant to hold only the qualitative information about this account. It should hold history as well. Maybe it shouldnt, but then again I think it would be nice to.
     * With the GLAccount holding only qualitative, I need something to hold the quantitative stuff
     */
}
