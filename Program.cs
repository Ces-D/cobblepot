using System;
using System.CommandLine;
using System.CommandLine.Invocation;
using 
namespace cobblepot
{
    class Program
    {
        static int Main(string[] args)
        {
            new Ledger();

            RootCommand rootCommand = new RootCommand("cobblepot"){
            };
            rootCommand.Description = "A finance tool for the poor";

            return rootCommand.Invoke(args);
        }
    }
}