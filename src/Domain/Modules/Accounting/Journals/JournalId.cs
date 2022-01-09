namespace Cobblepot.Domain.Accounting.Journals;

public record JournalId : IEntityId
{
    public JournalId(Guid? id) => Id = id ?? Guid.NewGuid();
    public Guid Id { get; }
}
