namespace Cobblepot.Domain.Common;

public class BusinessRuleValidationException : Exception
{
    protected IBusinessRule BrokenRule { get; init; }
    protected string Details { get; init; }
    public BusinessRuleValidationException(IBusinessRule brokenRule)
    {
        BrokenRule = brokenRule;
        Details = brokenRule.Message;
    }
    public override string ToString()
    {
        return $"{BrokenRule.GetType().FullName}: {BrokenRule.Message}";
    }

    public static void CheckRule(IBusinessRule rule)
    {
        if (rule.IsBroken())
        {
            throw new BusinessRuleValidationException(rule);
        }
    }
}

