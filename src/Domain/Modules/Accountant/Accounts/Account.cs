namespace Cobblepot.Domain.Accountant.Accounts;
using Cobblepot.Domain.Common;
using Cobblepot.Domain.Accountant.Entries;
using Cobblepot.Domain.Accountant.Accounts.AccountType;

public class Account : EntityBase
{
    public string Name { get; set; }
    public List<Entry> History { get; private set; }
    public IAccountType Category { get; private set; }

    public Account(Guid id, DateTime createdDate, string name, List<Entry> history, IAccountType category) : base(id, createdDate)
    {
        Name = name;
        History = history;
        Category = category;
    }

    public Account(){}
}

// TODO: Create the repositories for this namespace
// Start working on the client side and we'll figure out what is missing from there. The account should be created before any entry so start there
// The console app should suggest or only allow entries with the account Id of an already created account