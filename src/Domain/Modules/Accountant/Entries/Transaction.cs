namespace Cobblepot.Domain.Accountant.Entries;

public record Transaction
{
    public DateTie Date { get; set; }
    public string Title { get; set; }
    public string Memo { get; set; }
    public Money Amount { get; set; }
}