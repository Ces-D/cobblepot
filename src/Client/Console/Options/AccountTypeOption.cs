namespace Cobblepot.Client.Console.Options;
using Cobblepot.Domain.Accounting.Accounts.AccountTypes;

internal class AccountTypeOption : Option<AccountType>
{
    public AccountTypeOption() : base("--accountType", "Type of account this transaction affects")
    {
        this.AddAlias("-acc");
        this.SetDefaultValue(AccountType.Asset);
        this.IsRequired = false;
    }
}