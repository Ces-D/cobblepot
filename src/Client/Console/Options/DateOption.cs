namespace Cobblepot.Client.Console.Options;

public class DateOption : Option<DateTime>
{
    public DateOption(string? description = null) : base("Date", description)
    {
        this.SetDefaultValue(DateTime.UtcNow);
    }
}