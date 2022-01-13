namespace Cobblepot.Client.Console.Arguments;

internal class TitleArgument : Argument<string>
{

    public TitleArgument(string? description = null)
    {
        Name = "Title";
        Description = description ?? "Heading descriptor for this transaction";
    }
}
