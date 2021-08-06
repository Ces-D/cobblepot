using System;
using System.IO;

namespace Config
{
    /** Project Structure
        Vault
            |
            |_Report
            |       |_Accounts
            |       |_Balance
            |       |_Assets
            |
            |_Journal
    */
    public static class Paths
    {
        public readonly static string BasePath = Directory.GetCurrentDirectory();
        public readonly static string VaultPath = Path.Combine(BasePath, "Vault");
        public readonly static string JournalPath = Path.Combine(VaultPath, "Journal");
        public readonly static string ReportsPath = Path.Combine(VaultPath, "Reports");
        public readonly static string AccountsPath = Path.Combine(ReportsPath, "Accounts");
        public readonly static string BalancePath = Path.Combine(ReportsPath, "Balance");
        public readonly static string AssetsPath = Path.Combine(ReportsPath, "Assets");

    }


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
            if (!Directory.Exists(Paths.ReportsPath))
            {
                Directory.CreateDirectory(Paths.ReportsPath);
                Console.WriteLine("Reports Created");
            }
            if (!Directory.Exists(Paths.AccountsPath))
            {
                Directory.CreateDirectory(Paths.AccountsPath);
                Console.WriteLine("Accounts Created");
            }
            if (!Directory.Exists(Paths.BalancePath))
            {
                Directory.CreateDirectory(Paths.BalancePath);
                Console.WriteLine("Balance Created");
            }
            if (!Directory.Exists(Paths.AssetsPath))
            {
                Directory.CreateDirectory(Paths.AssetsPath);
                Console.WriteLine("Assets Created");
            }
        }
    }
}
