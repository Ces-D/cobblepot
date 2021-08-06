using System;
using System.IO;
using Config;

namespace Controllers.VaultAccess
{
    public static class JournalFiles
    {

        internal static void AppendExistingFile(string existingFileName) { }

        internal static void CreateThenAppendFile() { }

        public static void JournalEntryIntoVaultHandler(JournalEntry entry)
        {
            
        }

        // enter date for entry into cli
        // check date for the quarter it is located in
        // create the file name
        // check if the file name exists
        // if exists 
        // open 
        // write line at bottom
        // close
        // if not exists
        // create file
        // add line to bottom
        public static string NameJournalFile(string previousName)
        {
            string name = "";
            return name;
        }
    }

    internal static class QuarterDates
    {
        private static DateTime firstQuarterStart = new DateTime(DateTime.Today.Year, 1, 1);
        private static DateTime secondQuarterStart = new DateTime(DateTime.Today.Year, 3, 1);
        private static DateTime thirdQuarterStart = new DateTime(DateTime.Today.Year, 6, 1);
        private static DateTime fourthQuarterStart = new DateTime(DateTime.Today.Year, 9, 1);

        public static string LocatedInQuarterPeriod(DateTime entryDate)
        {
            // entryDate is before second quarter start
            if (DateTime.Compare(entryDate, secondQuarterStart) < 0)
            {
                return $"Q1_{firstQuarterStart.ToShortDateString()}";
            }
            // entryDate is second quarter start
            if (DateTime.Compare(entryDate, secondQuarterStart) == 0)
            {
                return $"Q2_{secondQuarterStart.ToShortDateString()}";
            }
            // entryDate is after second quarter start and before third quarter start
            if (DateTime.Compare(entryDate, secondQuarterStart) > 0 && DateTime.Compare(entryDate, thirdQuarterStart) < 0)
            {
                return $"Q2_{secondQuarterStart.ToShortDateString()}";
            }
            // entryDate is third quarter start
            if (DateTime.Compare(entryDate, thirdQuarterStart) == 0)
            {
                return $"Q3_{thirdQuarterStart.ToShortDateString()}";
            }
            // entryDate is before fourth quarter start and after third quarter start
            if (DateTime.Compare(entryDate, fourthQuarterStart) < 0 && DateTime.Compare(entryDate, thirdQuarterStart) > 0)
            {
                return $"Q3_{thirdQuarterStart.ToShortDateString()}";
            }
            // entryDate is fourth quarter start
            if (DateTime.Compare(entryDate, fourthQuarterStart) == 0)
            {
                return $"Q4_{fourthQuarterStart.ToShortDateString()}";
            }
            // entryDate is after the fourth quarter start
            else { return $"Q4_{fourthQuarterStart.ToShortDateString()}"; }
        }
    }
}