namespace Cobblepot.Client.Console.Options;
using Cobblepot.Domain.Accounting.Accounts.AccountTypes;

public class AccountTypeOption : Option<AccountType>
{
    public AccountTypeOption() : base("Account Types", "Account of type Asset, Liability, Revenue, Expense")
    {
        this.SetDefaultValue(AccountType.Asset);
    }
}