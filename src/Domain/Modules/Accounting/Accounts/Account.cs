namespace Cobblepot.Domain.Accounting.Accounts;
using Cobblepot.Domain.Accounting.Entries;
using Cobblepot.Domain.Accounting.Accounts.AccountTypes;

public class Account : EntityBase, IAggregateRoot, IEntity
{
    private readonly Guid _id;
    private readonly IAccountValueCalculator _accountValueCalculator;
    private string _name;
    private Money _value;
    private List<Entry> _entries;

    public Guid Id { get { return _id; } }
    public string Name { get { return _name; } }
    public Money Value { get { return _value; } }

    internal Account(string name, Currency currency, IAccountValueCalculator accountValueCalculator) : base(DateTime.UtcNow)
    {
        _id = Guid.NewGuid();
        _name = name;
        _value = new Money(0, currency);
        _accountValueCalculator = accountValueCalculator;
        _entries = new List<Entry>();
    }

    internal Account(Guid id, DateTime createdDate, string name, Money value, IAccountValueCalculator accountValueCalculator, List<Entry> entries) : base(createdDate)
    {
        _id = id;
        _name = name;
        _value = value;
        _accountValueCalculator = accountValueCalculator;
        _entries = entries;
    }

    public void CreditAccount(Money money)
    {
        _value = _accountValueCalculator.Credit(_value, money);
    }

    public void DebitAccount(Money money)
    {
        _value = _accountValueCalculator.Debit(_value, money);
    }

    public void Add(Entry entry)
    {
        this.CheckRule(new NoDuplicateAccountEntriesRule(_entries, entry));
        this.CheckRule(new EntryTypeMatchesAccountValueTypeRule(_accountValueCalculator.AccountType, entry.AccountType));

        var opposite = entry.CreateOpposite();
        _entries.Add(entry);
        _entries.Add(opposite);
    }

}