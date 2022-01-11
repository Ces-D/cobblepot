namespace Infrastructure.EntityFrameworkSqlLite.Accounting;
using Microsoft.EntityFrameworkCore;
using Cobblepot.Domain.Accounting.Journals;
using Cobblepot.Domain.Accounting.Accounts;
using Cobblepot.Domain.Accounting.Entries;
using Microsoft.Extensions.Logging;

public class AccountingContext : DbContext
{
    private readonly ILoggerFactory _loggerFactory;
    public DbSet<Journal> Journals { get; set; }
    public DbSet<Account> Accounts { get; set; }
    public DbSet<Entry> Entries { get; set; }

    public AccountingContext(DbContextOptions dbContextOptions, ILoggerFactory logger) : base(dbContextOptions)
    {
        _loggerFactory = logger;
    }

}
