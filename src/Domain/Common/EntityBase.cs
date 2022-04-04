namespace Cobblepot.Domain.Common;

public abstract class EntityBase : IEntity
{
    private List<IDomainEvent>? _domainEvents;
    public DateTime CreatedAt { get; init; }
    public Guid Id { get; init; }

    protected EntityBase()
    {
        CreatedAt = DateTime.UtcNow;
        Id = Guid.NewGuid();
    }


    protected EntityBase(Guid id, DateTime createdDate)
    {
        CreatedAt = createdDate.ToUniversalTime();
        Id = id;
    }

    public void ClearDomainEvents()
    {
        _domainEvents?.Clear();
    }

    protected void AddDomainEvent(IDomainEvent domainEvent)
    {
        _domainEvents ??= new List<IDomainEvent>();
        _domainEvents.Add(domainEvent);
    }

    protected void CheckRule(IBusinessRule rule)
    {
        BusinessRuleValidationException.CheckRule(rule);
    }

}