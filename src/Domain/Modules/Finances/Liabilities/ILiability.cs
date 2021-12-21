namespace Cobblepot.Domain.Modules.Finances.Liabilities;

// see - https://www.investopedia.com/terms/l/liability.asp
internal interface ILiability
{
    public Money Cost { get; }
    public DateTime Maturity { get; }

}