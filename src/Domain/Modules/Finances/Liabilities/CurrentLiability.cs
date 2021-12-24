﻿namespace Cobblepot.Domain.Modules.Finances.Liabilities;

// see - https://www.investopedia.com/terms/c/currentliabilities.asp

public class CurrentLiability : Entity, ILiability, IDescribable
{
    private string _title;
    private string _description;
    private Money _cost;
    private DateTime _maturity;
    private List<Note> _notes;

    public string Title { get { return _title; } set { _title = value; } }
    public string Description { get { return _description; } set { _description = value; } }
    public Money Cost { get { return _cost; } }
    public DateTime Maturity { get { return _maturity; } }

    public CurrentLiability(Guid id, string title, string? description, Money cost, DateTime maturityDate) : base(id, SystemClock.Now)
    {
        _title = title;
        _description = description ?? "no description";
        _cost = cost;
        _maturity = maturityDate;
        _notes = new List<Note>();
    }

    public void AddCredit(Money credit)
    {
        this.CheckRule(new CreditAddedIsNotMoreThanCostRule(_cost, credit));

        var newCostBalance = _cost.Amount - credit.Amount;
        _cost = new Money(newCostBalance, _cost.Currency);
    }

    public void AddNote(Note note) => _notes.Add(note);

    public override string ToString() => $"Current Liability: {_title}, {_created}";
}
