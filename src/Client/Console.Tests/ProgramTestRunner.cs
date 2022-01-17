using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.CommandLine;
using System.CommandLine.Parsing;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;

namespace Cobblepot.Client.Console.Tests
{
    public class ProgramTestRunner
    {
        public static async Task Main(string[] args)
        {

            ServiceProvider services = Program.BuildServiceProvider();
            Parser parser = Program.BuildParser(services);

            var addJournalEntryCommandTests = new AddJournalEntryCommandTests(run: true);

            if (addJournalEntryCommandTests.Run)
            {
                foreach (var command in addJournalEntryCommandTests.Tests)
                {
                    System.Console.WriteLine($"Command: {command}");
                    await parser.InvokeAsync(command);
                    System.Console.WriteLine();
                }
            }
        }

        // Figure out how to run these tests
    }
}
