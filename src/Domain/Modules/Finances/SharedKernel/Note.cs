namespace Cobblepot.Domain.Modules.Finances.SharedKernel;
public record Note : ValueObject
{
    public string Text { get; init; }
    public DateTime Date { get; init; }

    public Note(string text, DateTime date)
    {
        Text = text;
        Date = date;
    }

    public override string ToString()
    {
        return $"{Date}: {Text}";
    }
}

