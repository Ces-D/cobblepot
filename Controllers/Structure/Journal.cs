using System;
using System.IO;

namespace Controllers.Structure
{
    public class Journal : LedgerStructure
    {
        protected readonly static string Journal_Path = Path.Combine(Ledger_Path, "Journal");
        public Journal()
        {
            if (!Directory.Exists(Journal_Path))
            {
                Directory.CreateDirectory(Journal_Path);
                Console.WriteLine("Journal Directory Created Successfully");
            }
        }
        public class Entry
        {
            public Entry(string date, string directive, string description, double amount)
            {

            }
        }

    }

}