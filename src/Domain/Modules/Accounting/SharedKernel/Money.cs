namespace Cobblepot.Domain.Accounting.SharedKernel;

using Cobblepot.Domain.Accounting.SharedKernel.Rules;
using System.Globalization;

public record Money
{
    public Money(decimal amount, Currency currency)
    {
        BusinessRuleValidationException.CheckRule(new MoneyAmountIsPositiveRule(amount));

        Amount = amount;
        Currency = currency;
    }

    public decimal Amount { get; init; }
    public Currency Currency { get; init; }

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

    public static Money operator +(Money a, Money b) => new(a.Amount + b.Amount, a.Currency);
    public static Money operator -(Money a, Money b) => new(a.Amount - b.Amount, a.Currency);
}

