namespace Cobblepot.Domain.Modules.Finances.SharedKernel;

internal interface IDescribable
{
    public string Title { get; set; }
    public string Description { get; set; }
    public string ToString();
    public void AddNote(Note note);
}
