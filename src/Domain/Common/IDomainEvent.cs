namespace Cobblepot.Domain.Common;
public interface IDomainEvent
{
    Guid Id { get; }
    DateTime OcurredOn { get; }
}
