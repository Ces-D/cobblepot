using System;

namespace Services
{
    public class VaultBranch
    {
        private const string VARIABLE_NAME = "Vault_Account";
        public VaultBranch(string account)
        {
            Environment.SetEnvironmentVariable(VARIABLE_NAME, account, EnvironmentVariableTarget.Machine);
        }

        public static string? Account
        {
            get
            {
                if (Environment.GetEnvironmentVariable(VARIABLE_NAME) != null)
                {
                    return Environment.GetEnvironmentVariable(VARIABLE_NAME);
                }
                else
                {
                    return null;
                }
            }
        }
    }
}

