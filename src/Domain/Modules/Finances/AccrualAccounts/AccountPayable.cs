namespace Cobblepot.Domain.Modules.Finances.AccrualAccounts;
using Cobblepot.Domain.Modules.Finances.Liabilities;
using Cobblepot.Domain.Modules.Finances.Assets;

// see - https://www.investopedia.com/terms/a/accountspayable.asp

public class AccountPayable : Entity
{
    private CurrentLiability _payableAccount;
    private IAsset _asset;
    private Money _amountPaid;
    private Money _remainingBalance;
    private TimeSpan _daysOverDue;


    public AccountPayable(Guid id, CurrentLiability payableAccount, IAsset obtainedAsset) : base(id, SystemClock.Now)
    {
        _payableAccount = payableAccount;
        _asset = obtainedAsset;
    }


}

//TODO: complete the account payable