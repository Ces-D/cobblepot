namespace Cobblepot.Client.Console.Options;

internal class MemoOption : Option
{
    public MemoOption() : base("--memo", "Memo with transaction details")
    {
        this.AddAlias("-m");
        this.IsRequired = false;
        this.Arity = new ArgumentArity(1, 50);
    }
}

// TODO: figure out why this continues to fail