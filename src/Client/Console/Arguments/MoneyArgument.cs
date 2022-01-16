namespace Cobblepot.Client.Console.Arguments;
using Cobblepot.Domain.Accounting.SharedKernel;
using System.CommandLine.Parsing;

internal class MoneyArgument : Argument<decimal>
{
    public MoneyArgument()
    {
        this.Name = "Money";
        this.Description = "Value of money in your currency";
        this.SetDefaultValue(0);
        this.AddValidator(val =>
        {
            if (Convert.ToDecimal(val.GetValueOrDefault()) >= 0)
            {
                return null;
            }
            else { return "Money value cannot be less than zero"; }
        });
    }
}