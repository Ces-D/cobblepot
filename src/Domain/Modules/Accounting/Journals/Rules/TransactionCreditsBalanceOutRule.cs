namespace Cobblepot.Domain.Accounting.Journals.Rules;

using Cobblepot.Domain.Accounting.Accounts;

internal class TransactionCreditsBalanceRule : IBusinessRule
{
    private bool _transactionsBalance;

    public TransactionCreditsBalanceRule(IAccountTransaction initialTransaction, IAccountTransaction secondTransaction)
    {
        _transactionsBalance = initialTransaction.IsCredit != secondTransaction.IsCredit;
    }

    public string Message => "Transaction credits must balance out by being opposites";

    public bool IsBroken() => _transactionsBalance;

}
