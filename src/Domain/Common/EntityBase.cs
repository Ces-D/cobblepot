namespace Cobblepot.Domain.Common;

public abstract class EntityBase
{
    private List<IDomainEvent>? _domainEvents;
    private readonly DateTime _createdAt;

    protected EntityBase(DateTime createdAt)
    {
        _createdAt = createdAt;
    }

    protected DateTime CreatedAt { get => _createdAt; }

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