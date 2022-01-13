namespace Cobblepot.Client.Console.Arguments;

internal class MemoArgument : Argument<string>
{

    public MemoArgument()
    {
        Name = "Memo";
        Description = "Details about this transaction";
        this.Arity = new ArgumentArity(5, 40);
    }
}
