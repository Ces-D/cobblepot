namespace Cobblepot.Domain.Modules.Finances.Assets;

internal class UpdateValueMustBePositiveRule : IBusinessRule
{
    private bool _newValueIsPositiveWholeNumber;
    public UpdateValueMustBePositiveRule(Money newValue)
    {
        _newValueIsPositiveWholeNumber = newValue.Amount >= 0;
    }

    public string Message => "New value must be a positive whole number";

    public bool IsBroken() => _newValueIsPositiveWholeNumber;
}