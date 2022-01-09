namespace Cobblepot.Domain.Accounting.Accounts;
internal class AccountId : IEntityId
{
    public AccountId() { Id = Guid.NewGuid(); }
    public AccountId(Guid id) { Id = id; }

    public Guid Id { get; }
}
