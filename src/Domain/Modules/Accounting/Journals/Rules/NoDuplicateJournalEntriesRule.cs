namespace Cobblepot.Domain.Accounting.Journals;
using Cobblepot.Domain.Accounting.Entries;

internal class NoDuplicateJournalEntriesRule : IBusinessRule
{
    private bool _duplicateEntryFound;

    public NoDuplicateJournalEntriesRule(List<Entry> entries, Entry beingAdded)
    {
        _duplicateEntryFound = entries.FindIndex(x => x.Id == beingAdded.Id) != -1 ? true : false;
    }

    public string Message => "No duplicate journal entries can be entered";

    public bool IsBroken() => _duplicateEntryFound;
}