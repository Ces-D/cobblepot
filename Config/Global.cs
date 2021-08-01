using System.IO;

namespace Config
{

    public static class VaultFeatures
    {
        public static readonly string Journal = "journal";
        public static readonly string Account = "account";
        public static readonly string Report = "report";
    }

    public static class DirectiveOptions
    {
        public static readonly string Open = "open";
        public static readonly string Close = "close";
        public static readonly string Note = "note";
        public static readonly string Price = "price";
        public static readonly string Balance = "bal";
        public static readonly string Default = "*";
    }


    public static class Paths
    {
        public readonly static string BasePath = Directory.GetCurrentDirectory();
        public readonly static string VaultPath = Path.Combine(BasePath, "Vault");
        public readonly static string JournalPath = Path.Combine(VaultPath, "Journal");
        public readonly static string AccountsPath = Path.Combine(VaultPath, "Accounts");
        public readonly static string ReportsPath = Path.Combine(VaultPath, "Reports");
    }

}