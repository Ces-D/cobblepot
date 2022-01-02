﻿namespace Cobblepot.Domain.Accounting.Journal;
using Cobblepot.Domain.Accounting.Account;
internal class TranscationAmountsBalanceToNetZeroRule : IBusinessRule
{
    private bool _transactionsBalance;

    public TranscationAmountsBalanceToNetZeroRule(IAccountTransaction initialTransaction, IAccountTransaction secondTransaction)
    {
        _transactionsBalance = initialTransaction.Amount == secondTransaction.Amount;
    }

    public string Message => "Transaction amounts must balance out to net zero";

    public bool IsBroken() => _transactionsBalance;

}
