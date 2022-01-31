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

    public Account() { }
}

// TODO: Create the repositories for this namespace
// Start working on the client side and we'll figure out what is missing from there. The account should be
// created before any entry so start there
// The console app should suggest or only allow entries with the account Id of an already created account

/**
 * I think the entry should have both the account and the account type.
 * For example the account of Capital One can both fall under the accounts Recievable and accounts Payable
 * Or would this be a Loan account under Capital One (account recieve)and then a bank account 
 * under Capital one (account pay)
 * But in a different instance this same Loan account became a (account pay) and the bank account 
 * became a (account reviece)
 * 
 * so
 * 
 * yes. Initial idea that the entry should contain both the account and accoutType as seperate params
 * Does this mean that I should separate Account Type from Accout aka remove Category. Yes. But then
 * what identification information should go into the Account
 * - maybe an enum of basic identifiers (including other)
 * - this can be where I include the country of this account, hence all money related to it should be in that currency
 * - instiution - optional ex. Capital One
 * - description - reason for creating this account irl - debit account or some shit
 * - change Name to Tag because this should now be how we as the user are going to identify this account as
 * - 
 * 
 * side note - in the future docs or help text. I should include a lot of examples of potential accounts - at least 10
 * 
 * moving on
 * 
 * not that the accountType is separate from the Account
 * Maybe there should be an accountType base class where all the accountTypes inherit from. I would rather this be an interface so I can reference it in the Entry
 * I guess I can reference the base calss 
 * Are there any other things that I am going to want to include in this base class
 * maybe this should also have the transaction history
 * create a general ledger
 * 
 * 
 * after a quick read I see that I am going to have to change the accountType class or interface
 * into a 'General Ledger' Account. This means that I am going to have to create a General Ledger class as well
 * IMPORTANT : There is word here of a 'Trial Balance' which confirmas that the sum of all debit accounts 
 * are equal to the sum of all credit accounts
 * 
 * The Chart of Accounts holds the information for all Accounts. In other words this is the repo for all accounts
 * see - https://www.investopedia.com/terms/c/chart-accounts.asp for some examples of basic accounts to create
 * Maybe include a syntax for all the supported accounts and then create them as needed
 * Typically contains name, description, and id
 */