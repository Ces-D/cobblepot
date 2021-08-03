using System;
using System.IO;
using System.CommandLine;
using System.CommandLine.Invocation;
using Config;

namespace Controllers.Commands
{
    public class ViewCommand : Command
    // TODO: Update these
    // This is outdated relative to the new  vault file structure
    // ViewVaultFileContents is looking for files in Vault root folder, but everything is now in sub folders
    {
        public ViewCommand() : base("view", "Look at the contents of a vault file")
        {
            AddArgument(VaultFileArgument());
            Handler = CommandHandler.Create<string>(ViewVaultFileContents);
        }

        private Argument<string> VaultFileArgument()
        {
            Argument<string> VaultFile = new Argument<string>();
            VaultFile.Name = "vaultFile";
            VaultFile.Description = "Look at the contents of the chosen vault file";

            ArgumentArity argumentArity = new ArgumentArity(1, 1);
            VaultFile.Arity = argumentArity;

            return VaultFile;
        }

        private void ViewVaultFileContents(string vaultFile)
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

