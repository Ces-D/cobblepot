using Services;
using System;
using System.CommandLine;
using System.CommandLine.Invocation;
using System.CommandLine.Hosting;

namespace Commands
{
    public class BranchCommand : Command
    {
        public BranchCommand() : base("branch", "List your current active vault branch")
        {
            Handler = CommandHandler.Create(() =>
            {
                if (VaultBranch.Account != null)
                {
                    Console.WriteLine($"Current Vault Branch: {VaultBranch.Account}");
                }
                else
                {
                    Console.WriteLine("No Vault Branch set");
                }
            });
        }
    }
}