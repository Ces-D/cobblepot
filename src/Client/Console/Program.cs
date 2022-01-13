namespace Cobblepot.Client.Console;
using Cobblepot.Client.Console.Commands.Accounting;
using Cobblepot.Domain.Accounting.Journals;
using System.CommandLine.Builder;
using System.CommandLine.Parsing;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;

public class Program
{
    private static CommandLineBuilder BuildCommandLine()
    {
        var journal = new Journal();
        var rootCommand = new RootCommand("cobblepot");
        rootCommand.AddCommand(new AddJournalEntryCommand(journal));

        var commandLineBuilder = new CommandLineBuilder(rootCommand);
        return commandLineBuilder;
    }


    public static async Task Main(string[] args)
    {
        await BuildCommandLine().UseDefaults().Build().InvokeAsync(args);
        // TODO: figure out if hosting is going to be necessary for this app
    }
}
