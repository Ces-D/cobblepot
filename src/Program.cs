using System.CommandLine;
using System.CommandLine.Builder;
using Controllers.Commands;
using System.CommandLine.Parsing;
using System.Threading.Tasks;
using Config;

namespace Cobblepot
{
    class Program
    {
        static CommandLineBuilder BuildCommandLine()
        {
            RootCommand root = new RootCommand("cobblepot");
            root.Description = "A finance tool for the poor";
            root.TreatUnmatchedTokensAsErrors = true;
            root.AddAlias("cp");

            root.AddCommand(new NewEntryCommand());
            // root.AddCommand(new ViewCommand());

            return new CommandLineBuilder(root);
        }

        static async Task Main(string[] args)
        {
            Vault.Build();

            await BuildCommandLine()
            .UseDefaults()
            .Build()
            .InvokeAsync(args);
        }
    }
}

// https://dev.to/nikiforovall/develop-clean-command-line-applications-with-system-commandline-clean-cli-1lfp
// TODO: See if you can switch over to this kind of method.