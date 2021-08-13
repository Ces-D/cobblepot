using System;
using System.CommandLine;
using System.CommandLine.Invocation;
using Services.Account;


namespace Commands
{
    //same as git remove
    public class CloseCommand : Command
    {
        public CloseCommand() : base("close", "Close and Account")
        {
            AddArgument(AccountName());

            Handler = CommandHandler.Create((string AccountName) =>
            {
                AccountWriter.Remove(accountName: AccountName);
            });
        }

        private Argument<string> AccountName()
        {
            Argument<string> account = new Argument<string>();
            account.Name = "Account";
            account.Description = "Name of the Account";

            account.AddValidator(acc =>
            {
                if (!AccountFormat.MatchesConvention(acc.ToString()))
                {
                    return AccountFormat.FORMAT_ERROR_RESPONSE;
                }
                else { return null; }
            });

            return account;
        }
    }
}