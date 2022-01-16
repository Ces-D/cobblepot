namespace Cobblepot.Client.Console.Arguments;

internal class TitleArgument : Argument
{

    public TitleArgument(string? description = null)
    {
        Name = "Title";
        Description = description ?? "Heading descriptor for this transaction";
        this.Arity = new ArgumentArity(1, 10);
    }
}
