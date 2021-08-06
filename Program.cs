using System.CommandLine;
using System.CommandLine.Builder;
using Controllers.Commands;
using Controllers.VaultAccess.Middleware;
using Config;
namespace cobblepot
{
    class Program
    {
        static void Main(string[] args)
        {
            new Vault();

            Command newSubCommand = new NewCommand();
            Command viewSubCommand = new ViewCommand();

            CommandLineBuilder newCommandBuilder = new CommandLineBuilder(newSubCommand);
            // TODO: figure out how to get the middleware working
            newCommandBuilder.UseMiddleware(async (context, next) =>
            {
                System.Console.WriteLine("Hello");
                await next(context);
            }
            );


            RootCommand rootCommand = new RootCommand("cobblepot")
            {
                newCommandBuilder.Command,
                viewSubCommand
            };

            rootCommand.Description = "A finance tool for the poor";
            rootCommand.TreatUnmatchedTokensAsErrors = true;

            rootCommand.InvokeAsync(args).Wait();
        }
    }
}