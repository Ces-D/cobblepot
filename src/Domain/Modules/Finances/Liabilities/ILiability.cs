namespace Cobblepot.Domain.Modules.Finances.Liabilities;

// see - https://www.investopedia.com/terms/l/liability.asp
public interface ILiability
{
    public Money Cost { get; }
    public DateTime Maturity { get; }
    public void AddCredit(Money credit);
}