using System.IO;
using System.CommandLine;
using Controllers.Commands;
using Config;
namespace cobblepot
{
    class Program
    {
        static void Main(string[] args)
        {
            new Vault();

            RootCommand rootCommand = new RootCommand("cobblepot")
            {
                new ViewCommand(),
                new NewCommand(),
            };
            rootCommand.Description = "A finance tool for the poor";
            rootCommand.TreatUnmatchedTokensAsErrors = true;

            rootCommand.InvokeAsync(args).Wait();
        }
    }
}