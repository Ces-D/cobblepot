using System;
using System.CommandLine;

namespace Commands
{
    public class ReportCommand : Command
    {
        public ReportCommand() : base("report", "create a report for you finances")
        {
            // the directive should keep a record of open and close for assets 
            // keep it in a separate file

            // type of record - Asset|Liability|Income|Expense|Equity
            // TODO: look through the reports of beancount

            // balance sheet - balance matters at a point in time | assets positive + liabilities negative
            // income statement - change in balance matters over a period of time | expenses increase ovt + income decreases ovt

            // In addition to journal. There is also a sheet that holds the various open and closes. Possibly
            // two sheets. This should add the opens into a file and remove them when closed. So possibly parser
            // that opens sheet on every input. 

            // cmd:cobblepot cmd:new-open|close|note|balance <args> <options>
            // this could make it easier to add logic. Without those subcommands added then we are in the * 
            // scenario. if it is an expense or income that matches the details and it is an asset or liability 

            // new-balance can act as an open but for existing accounts. 

            // Going to need a parser that doesnt allow inputs if directive * to non opened.
            // could be folled by a suggest of the open detail closest to what they mean.   
        }
    }
}