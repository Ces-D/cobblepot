namespace Cobblepot.Client.Console;
using System.CommandLine.Builder;
using System.CommandLine.Parsing;
using Microsoft.Extensions.Hosting;
using System.IO;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;

using Cobblepot.Client.Console.Commands.Accounting;
using Cobblepot.Domain.Accounting.Journals;

public class Program
{

    public static Parser BuildParser(ServiceProvider serviceProvider)
    {
        var rootCommand = new RootCommand("The easiest personal finance tool for command line junkies.");
        rootCommand.Name = "Cobblepot";

        var instanceJournal = new Journal();
        rootCommand.AddCommand(new AddJournalEntryCommand(instanceJournal, serviceProvider.GetService<IConfiguration>()));
        rootCommand.AddCommand(new ListJournalEntryCommand(instanceJournal, serviceProvider.GetService<IConfiguration>()));

        var commandLineBuilder = new CommandLineBuilder(rootCommand);

        return commandLineBuilder.UseDefaults().Build();
    }

    public static ServiceProvider BuildServiceProvider()
    {
        var servicesCollection = new ServiceCollection();
        IConfigurationRoot config = new ConfigurationBuilder()
            .SetBasePath(Directory.GetCurrentDirectory())
            .AddJsonFile("appSettings.json", optional: false, reloadOnChange: true)
            .Build();
        servicesCollection.AddSingleton<IConfiguration>(config);

        return servicesCollection.BuildServiceProvider();
    }


    public static async Task Main(string[] args)
    {
        ServiceProvider services = BuildServiceProvider();
        Parser parser = BuildParser(services);

        await parser.InvokeAsync(args);
    }
}