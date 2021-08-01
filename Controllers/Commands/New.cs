using System;
using System.IO;
using System.CommandLine;
using Config;

namespace Controllers.Commands
{
    public class NewCommand : Command
    {
        public NewCommand() : base("new", "Creating a new vault feature entry")
        {
            AddArgument(VaultFeatureArgument());
        }

        private Argument VaultFeatureArgument()
        {
            Argument<string> vFeature = new Argument<string>();
            vFeature.Name = "feature";
            vFeature.Description = "Creates a new vault feature file";

            vFeature.FromAmong(VaultFeatures.Account, VaultFeatures.Journal, VaultFeatures.Report);

            ArgumentArity argumentArity = new ArgumentArity(1, 1);
            vFeature.Arity = argumentArity;

            return vFeature;
        }

        private void CreateVaultFeatureFile(string feature)
        {
            // needs to check if feature already has an existing feature file
            // yes ? check if that file expired : create new file 
            // file expired ? create new file : add the 
        }

        // consider adding another layer of commands
        // one for each of the vault features features 
        // otherwise you'll have to create a lot of options and option requirements may become messy
    }
    // TODO: descide on the above
}