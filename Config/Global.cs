using System.IO;

namespace Config
{

    public static class VaultFeatures
    {
        public static readonly string Journal = "journal";
        public static readonly string Report = "report";
    }

    public static class DirectiveOptions
    {
        public static readonly string Open = "open";
        public static readonly string Close = "close";
        public static readonly string Note = "note";
        public static readonly string Price = "price";
        public static readonly string Balance = "balance";
        public static readonly string Default = "*";
    }
}