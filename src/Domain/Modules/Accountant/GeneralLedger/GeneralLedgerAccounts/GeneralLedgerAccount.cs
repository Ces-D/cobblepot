namespace Cobblepot.Domain.Accountant.GeneralLedger.GeneralLedgerAccounts;
using Cobblepot.Domain.Common;
using Cobblepot.Domain.Accountant.Entries;

public class GeneralLedgerAccount : EntityBase
{
    public List<Entry> History { get; private set; }
    public AccountType AccountType { get; private set; }
    public Currency Currency { get; set; }
    /// <summary>
    /// Primary way the user will identify this account
    /// </summary> 
    public string Tag { get; set; }
    /// <summary>
    /// In case this particular account is associated with a specific institution
    /// </summary>
    public string? Institution { get; set; }
    /// <summary>
    /// General used for describing the reason for creating this account
    /// </summary>
    public string? Description { get; set; }

    public GeneralLedgerAccount(string tag, Currency currency, AccountType accountType) : base(Guid.NewGuid(), DateTime.UtcNow)
    {
        History = new List<Entry>();
        Currency = currency;
        Tag = tag;
        AccountType = accountType;
    }

    public void AddTransaction(Entry transaction)
    {
        // TODO: does this method need validation
        /**
        It most likely does. Although this kind of logic probably needs to go through the Trial Balance
        This means that The General Ledger - which is where the trial balance will exist - also needs a Transaction
        method. That method is going to need more logic than this one. So possibly this one doesnt need anything
        */
        History.Add(transaction);
    }

}

/**
 * TODO: This and GL Account Categoriesis a many to many relationship and this is the key. see - https://stackoverflow.com/questions/4366102/many-to-many-relationship-in-ddd
 */