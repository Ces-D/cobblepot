using System;
using System.IO;
using System.CommandLine;
using System.CommandLine.Invocation;
using Config;

namespace Controllers.Commands
{
    public class ViewCommand : Command
    {
        public ViewCommand() : base("view", "Look at the contents of a vault file")
        {
            AddArgument(new Argument<string>("vaultFile", "Name of the Vault File"));
            Handler = CommandHandler.Create<string>(ViewContents);
        }

        public void ViewContents(string vaultFile)
        {
            string[] SupposedVaultFile = Directory.GetFiles(Paths.VaultPath, vaultFile);
            if (SupposedVaultFile.Length == 1)
            {
                try
                {
                    using (StreamReader readFile = File.OpenText(SupposedVaultFile[0]))
                    {
                        string? currentLine;
                        while ((currentLine = readFile.ReadLine()) != null)
                        {
                            Console.WriteLine(currentLine);
                        }
                    }
                }
                catch (System.Exception exp)
                {
                    Console.WriteLine($"{exp.GetType()} thrown while trying to read File");
                    Console.Write(exp.Message);
                }
            }
            else if (SupposedVaultFile.Length > 1)
            {
                Console.WriteLine($"Multiple files exist in {Paths.VaultPath} with the name {vaultFile}.");
            }
            else
            {
                Console.WriteLine($"{vaultFile} is not a valid Vault File");
            }
        }

    }
}

