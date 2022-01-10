namespace Cobblepot.Domain.Accounting.Accounts;
using Cobblepot.Domain.Accounting.Accounts.AccountTypes;

internal record EntryTypeMatchesAccountValueTypeRule : IBusinessRule
{
    private bool _typesMatch;

    public EntryTypeMatchesAccountValueTypeRule(AccountType accountType, AccountType entryType)
    {
        _typesMatch = accountType == entryType;
    }

    public string Message => "Entry type must match the account type";
    public bool IsBroken() => _typesMatch;
}