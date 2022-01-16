namespace Cobblepot.Client.Console.Options;

internal class DateOption : Option<DateTime>
{
    public DateOption(string? description = null) : base("--date", description)
    {
        this.SetDefaultValue(DateTime.UtcNow);
        this.IsRequired = false;
    }
}