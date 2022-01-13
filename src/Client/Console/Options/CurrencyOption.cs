namespace Cobblepot.Client.Console.Options;
using Cobblepot.Domain.Accounting.SharedKernel;

public class CurrencyOption : Option<Currency>
{
    public CurrencyOption() : base("Currency", "Currency of type USD, MXN, EUR")
    {
        this.SetDefaultValue(Currency.USD);
    }
}