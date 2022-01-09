namespace Cobblepot.Domain.Accounting.Entries;

public record EntryId : IEntityId
{
    public EntryId(Guid? id) => Id = id ?? Guid.NewGuid();
    public Guid Id { get; }
}