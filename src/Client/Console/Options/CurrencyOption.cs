namespace Cobblepot.Client.Console.Options;
using Cobblepot.Domain.Accounting.SharedKernel;

internal class CurrencyOption : Option<Currency>
{
    public CurrencyOption(Currency? baseCurrency = null) : base("--currency", "Currency of type USD, MXN, EUR")
    {
        this.SetDefaultValue(baseCurrency ?? Currency.USD);
        this.IsRequired = false;
    }
}