using System;
using System.CommandLine;
using System.CommandLine.Builder;
using System.CommandLine.Parsing;
using System.CommandLine.Hosting;
using System.Threading.Tasks;

using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Configuration;

using Config;
using Commands;

namespace Cobblepot
{
    // TODO: Test all commands
    class Program
    {

        private static CommandLineBuilder BuildCommandLine()
        {
            RootCommand root = new RootCommand("cobblepot");
            root.Description = "A finance tool for the poor";
            root.TreatUnmatchedTokensAsErrors = true;
            root.AddAlias("cp");
            root.AddCommand(new OpenCommand());
            root.AddCommand(new CloseCommand());
            root.AddCommand(new BranchCommand());
            root.AddCommand(new CheckoutCommand());
            root.AddCommand(new AddTransactionCommand());
            root.AddCommand(new AddNoteCommand());

            return new CommandLineBuilder(root);
        }

        public static async Task Main(string[] args)
        {
            await BuildCommandLine()
            .UseHost(hostBuilderFactory =>
            {
                hostBuilderFactory.ConfigureAppConfiguration((hostBuilderContext, configurationBuilder) =>
                {
                    IConfigurationRoot config = new ConfigurationBuilder()
                    .SetBasePath(AppDomain.CurrentDomain.BaseDirectory)
                    .AddJsonFile("cobblepot_settings.json")
                    .Build();

                    configurationBuilder.AddConfiguration(config);
                });
            })
            .UseDefaults()
            .Build()
            .InvokeAsync(args);
        }
    }
}
