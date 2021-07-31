using System;
using System.IO;

namespace Config
{
    public class Vault
    {
        public Vault()
        {
            if (!Directory.Exists(Paths.VaultPath))
            {
                Directory.CreateDirectory(Paths.VaultPath);
                Console.WriteLine("Vault Created");
            }
            if (!Directory.Exists(Paths.JournalPath))
            {
                Directory.CreateDirectory(Paths.JournalPath);
                Console.WriteLine("Journal Created");
            }
            if (!Directory.Exists(Paths.AccountsPath))
            {
                Directory.CreateDirectory(Paths.AccountsPath);
                Console.WriteLine("Account Created");
            }
            if (!Directory.Exists(Paths.ReportsPath))
            {
                Directory.CreateDirectory(Paths.ReportsPath);
                Console.WriteLine("Reports Created");
            }
        }

    }
}