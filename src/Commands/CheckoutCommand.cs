using System;
using System.CommandLine;
using System.CommandLine.Invocation;
using Services.AccountFile;
using Services;

namespace Commands
{
    public class CheckoutCommand : Command
    {
        public CheckoutCommand() : base("checkout", "Activate this account")
        {
            AddArgument(AccountName());

            Handler = CommandHandler.Create((string AccountName) =>
            {
                new VaultBranch(AccountName);
                Console.WriteLine($"Current working Account Branch: {AccountName}");
            });
        }

        private Argument<string> AccountName()
        {
            Argument<string> account = new Argument<string>();
            account.Name = "Account";
            account.Description = "Name of the Account";

            account.AddValidator(acc =>
            {
                if (!AccountEntry.MatchesConvention(acc.ToString()))
                {
                    return AccountEntry.FORMAT_ERROR_RESPONSE;
                }
                else { return null; }
            });

            return account;
        }
    }
}