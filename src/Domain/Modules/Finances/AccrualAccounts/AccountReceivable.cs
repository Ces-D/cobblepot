namespace Cobblepot.Domain.Modules.Finances.AccrualAccounts;
using Cobblepot.Domain.Modules.Finances.Liabilities;
using Cobblepot.Domain.Modules.Finances.Assets;

public class AccountReceivable : Entity
{
    private CurrentLiability _receivableAccount;
    private IAsset _asset;
    private bool _isPayed;
    public AccountReceivable(Guid id, CurrentLiability currentLiability, IAsset asset) : base(id, SystemClock.Now)
    {
        _receivableAccount = currentLiability;
        _asset = asset;
        _isPayed = false;
    }
}