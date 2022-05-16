using System;
using Microsoft.EntityFrameworkCore;
using Cobblepot.BookKeeping.ChartOfAccounts.ChartOfAccount;

namespace Cobblepot.BookKeeping.ChartOfAccounts.Data;

internal class ChartOfAccountsContext : DbContext
{
    private readonly string _connectionString;
    public DbSet<ChartOfAccount.ChartOfAccount> ChartOfAccounts { get; set; }

    public ChartOfAccountsContext(string connectionString)
    {
        _connectionString = connectionString;
    }

    protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder)
    {
        optionsBuilder.UseSqlServer(_connectionString);
    }
}