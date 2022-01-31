namespace Cobblepot.Domain.Accountant;
using Cobblepot.Domain.Accountant.Rules;
using Cobblepot.Domain.Common;
using System.Globalization;

public record Money
{
    public decimal Amount
    {
        get { return Amount; }
        set
        {
            BusinessRuleValidationException.CheckRule(new MoneyAmountIsPositiveRule(value));
            Amount = value;
        }
    }
    public Currency Currency { get; set; }

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

    // public static Money operator +(Money a, Money b) => new(a.Amount + b.Amount, a.Currency);
    public static Money operator +(Money a, Money b)
    {
        return new Money()
        {
            Amount = a.Amount + b.Amount,
            Currency = a.Currency
        };
    }
    public static Money operator -(Money a, Money b)
    {
        return new Money()
        {
            Amount = a.Amount - b.Amount,
            Currency = a.Currency
        };
    }
}