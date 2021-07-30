using System.CommandLine;

namespace Controllers.Commands
{

    public class AccountCommand
    {
        Command account_cmd;
        public AccountCommand()
        {
          account_cmd = new Argument<string>("account", "")
        }
    }
}