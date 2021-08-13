using System.CommandLine;
using System.CommandLine.Builder;
using System.CommandLine.Parsing;
using System.Threading.Tasks;
using Config;
using Commands;

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

            root.AddCommand(new OpenCommand());
            root.AddCommand(new CloseCommand());
            root.AddCommand(new AddTransactionCommand());
            root.AddCommand(new AddNoteCommand());
            root.AddCommand(new CheckoutCommand());

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
