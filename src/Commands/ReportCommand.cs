using System;
using System.CommandLine;

namespace Commands
{
    public class ReportCommand : Command
    {
        public ReportCommand() : base("report", "create a report for you finances")
        {
            /*
            Going to change the commands from requiring the account details on every request to now only requiring it on the 
            'open' command.

            You will need to open an account for updating the transaction. then the following inputs can be transactions
            related to this account. You will need to close this account in case you want 'open' a new account for 
            transactions. You close this account by calling the 'bal' command. This command will calculate the new balance
            from the transactions and mark the new balance in a new file.

            The directiveCache, may change name, will be the persistance across quarters. We will calculate the new balances
            using the records in this file as the base. The base will be updated after the new balance is calculated.

            This should make it easier to do filtering and maintain the reporting.

            There should be a couple of base accounts. Create a hierarchy to the structure.

            report names:
            balance-sheet * maybe most important
            income-statement
            expenditures * rate of asset changes
            */

        }
    }
}