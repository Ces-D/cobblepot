namespace Cobblepot.Client.Console.Options;

public class IsCreditOption : Option<bool>
{
    public IsCreditOption() : base("credit", "Specify if this transaction is being credited to or debited from")
    {
        this.SetDefaultValue(false);
    }
}