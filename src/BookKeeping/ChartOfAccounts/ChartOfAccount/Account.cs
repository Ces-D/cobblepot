using System;
using System.ComponentModel.DataAnnotations;
using Microsoft.EntityFrameworkCore;
using Cobblepot.BuildingBlocks.Domain;

namespace Cobblepot.BookKeeping.ChartOfAccounts.ChartOfAccount;

public class Account : Aggregate
{

    [Key]
    public AccountCode Code { get; private set; }
    public string Name { get; private set; }
    public AccountGroup AccountGroup { get; private set; }
    public string SubGroup { get; private set; }
    public TransactionType ToIncrease { get; private set; }
    public string FinancialStatementId { get; private set; }
}