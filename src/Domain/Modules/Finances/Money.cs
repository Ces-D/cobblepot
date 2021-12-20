namespace Cobblepot.Domain.Modules.Finances;
using System.Globalization;
public record Money : ValueObject
{
    public decimal Amount { get; init; }
    public Currency Currency { get; init; }

    public Money(decimal amount, Currency currency)
    {
        Amount = amount;
        Currency = currency;
    }
    public override string ToString()
    {
        string culture = Currency switch
        {
            Currency.USD => "en-US",
            Currency.MXN => "es-MX",
            Currency.EUR => "fr",
            _ => "en-US",
        };
        NumberFormatInfo nfi = new CultureInfo(culture, false).NumberFormat;
        nfi.CurrencyPositivePattern = 2; // see -https://riptutorial.com/csharp/example/4972/currency-formatting

        return string.Format(nfi, "{0:C}", Amount);
    }

}

