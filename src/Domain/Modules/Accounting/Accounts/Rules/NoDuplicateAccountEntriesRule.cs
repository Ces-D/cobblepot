namespace Cobblepot.Domain.Accounting.Accounts;
using Cobblepot.Domain.Accounting.Entries;

internal class NoDuplicateAccountEntriesRule : IBusinessRule
{
    private bool _duplicateEntryFound;

    public NoDuplicateAccountEntriesRule(List<Entry> entries, Entry beingAdded)
    {
        _duplicateEntryFound = entries.FindIndex(x => x.Id == beingAdded.Id) != -1 ? true : false;
    }

    public string Message => "No duplicate account entries can be entered";

    public bool IsBroken() => _duplicateEntryFound;
}