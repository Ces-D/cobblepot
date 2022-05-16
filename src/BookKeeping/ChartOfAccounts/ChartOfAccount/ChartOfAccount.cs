using System;
using System.ComponentModel.DataAnnotations;
using Microsoft.EntityFrameworkCore;
using Cobblepot.BuildingBlocks.Domain;

namespace Cobblepot.BookKeeping.ChartOfAccounts.ChartOfAccount;

public class ChartOfAccount : Aggregate
{
    public ChartOfAccount() { }

    [Key]
    public AccountCode AccountCode { get; private set; }
    public string AccountName { get; private set; }
    public TransactionType ToIncrease { get; private set; }
    public string FinancialStatementId { get; private set; }
    public DateTime LastUpdated { get; set; }
}