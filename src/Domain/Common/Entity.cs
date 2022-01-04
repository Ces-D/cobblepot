namespace Cobblepot.Domain.Common;
using System.Reflection;

public abstract class Entity
{
    private List<IDomainEvent>? _domainEvents;
    private readonly IEntityId _entityId;
    private readonly DateTime _createdAt;

    protected Entity(IEntityId id, DateTime createdAt)
    {
        _entityId = id;
        _createdAt = createdAt;
    }

    protected DateTime CreatedAt { get => _createdAt; }
    public IEntityId Id { get => _entityId; }

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