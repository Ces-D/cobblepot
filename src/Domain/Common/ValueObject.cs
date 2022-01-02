namespace Cobblepot.Domain.Common;

public abstract record ValueObject
{
    protected DateTime DateCreated { get; init; }

    public ValueObject(DateTime dateCreated) { DateCreated = dateCreated; }

    protected void CheckRule(IBusinessRule rule)
    {
        if (rule.IsBroken())
        {
            throw new BusinessRuleValidationException(rule);
        }
    }
}