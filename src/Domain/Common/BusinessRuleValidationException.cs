namespace Cobblepot.Domain.Common;

public class BusinessRuleValidationException : Exception
{
    protected IBusinessRule BrokenRule { get; init; }
    protected string Details { get; init; }
    public BusinessRuleValidationException(IBusinessRule brokenRule)
    {
        BrokenRule = brokenRule;
        this.Details = brokenRule.Message;
    }
    public override string ToString()
    {
        return $"{BrokenRule.GetType().FullName}: {BrokenRule.Message}";
    }
}

