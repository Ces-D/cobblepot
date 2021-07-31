using System;
using System.CommandLine;
using System.CommandLine.Invocation;
using System.Linq;
using System.Collections.Generic;

namespace Controllers.Commands
{
    public class ConfigCommand : Command
    {
        public ConfigCommand() : base("config", "Create a configuration file for your Vault")
        {
            AddOption(FileCreationFrequencyOption());
        }

        private enum FileCreationFrequencyValues
        {
            Yearly,
            BiYearly,
            Quarterly,
            Monthly,
        }
        private Option FileCreationFrequencyOption()
        {
            Option FCFOption = new Option<string>("--file_frequency");
            FCFOption.AddAlias("-fcf");
            FCFOption.Description = "The frequency which new dated files will be auto paginated/generated";
            FCFOption.SetDefaultValue(FileCreationFrequencyValues.Quarterly);

            return FCFOption;
        }
        private void FileCreationFrequencyHandler(string file_frequency)
        {
// TODO: complete 
        }
    }
}