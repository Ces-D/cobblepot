using System;
using System.IO;
using Config;

namespace Controllers.VaultAccess.Journal
{
    public static class JournalFiles
    {

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

        public static void InsertHandler(JournalEntry journalEntry)
        {
            string belongQuarter = QuarterDates.LocatedInQuarterPeriod(journalEntry.Date);
            string entryFilePath = Path.Combine(Paths.JournalPath, $"Entry_{belongQuarter}");
            if (File.Exists(entryFilePath))
            {
                FileAppend(entryFilePath, journalEntry);
                Console.WriteLine($"\nEntry_{belongQuarter} Content Updated");
                Console.WriteLine($"{journalEntry.ToString()}");
                Console.WriteLine($"Path File: {entryFilePath.ToString()}\n");
            }
            else
            {
                CreateFileAppend(entryFilePath, journalEntry);
                Console.WriteLine($"\nEntry_{belongQuarter} Created and Updated");
                Console.WriteLine($"{journalEntry.ToString()}");
                Console.WriteLine($"Path File: {entryFilePath.ToString()}\n");
            }
        }

        public static void InsertHandler(JournalEntry journalEntry, bool note)
        {
            string belongQuarter = QuarterDates.LocatedInQuarterPeriod(journalEntry.Date);
            string entryFilePath = Path.Combine(Paths.JournalPath, $"Entry_{belongQuarter}");
            if (File.Exists(entryFilePath))
            {
                FileAppend(entryFilePath, journalEntry);
                Console.WriteLine($"\nEntry_{belongQuarter} Content Updated");
                Console.WriteLine($"{journalEntry.ToNoteString()}");
                Console.WriteLine($"Path File: {entryFilePath.ToString()}\n");
            }
            else
            {
                CreateFileAppend(entryFilePath, journalEntry);
                Console.WriteLine($"\nEntry_{belongQuarter} Created and Updated");
                Console.WriteLine($"{journalEntry.ToNoteString()}");
                Console.WriteLine($"Path File: {entryFilePath.ToString()}\n");
            }
        }

        private static void FileAppend(string entryFilePath, JournalEntry journalEntry)
        {
            using (StreamWriter sw = File.AppendText(entryFilePath))
            {
                sw.WriteLineAsync(journalEntry.ToString());
                sw.Close();
            }
        }

        private static void CreateFileAppend(string entryFilePath, JournalEntry journalEntry)
        {
            using (StreamWriter fs = File.CreateText(entryFilePath))
            {
                fs.WriteLineAsync(journalEntry.ToString());
                fs.Close();
            }
        }
    }

    internal static class QuarterDates
    {
        private static DateTime firstQuarterStart = new DateTime(DateTime.Today.Year, 1, 1);
        private static DateTime secondQuarterStart = new DateTime(DateTime.Today.Year, 3, 1);
        private static DateTime thirdQuarterStart = new DateTime(DateTime.Today.Year, 6, 1);
        private static DateTime fourthQuarterStart = new DateTime(DateTime.Today.Year, 9, 1);
        private static DateTime previousYearEnd = firstQuarterStart.AddDays(-1);

        public static string LocatedInQuarterPeriod(DateTime entryDate)
        {
            // entryDate is before the start of current year
            if (DateTime.Compare(entryDate, previousYearEnd) <= 0)
            {
                return $"Q4_{fourthQuarterStart.AddYears(-1).Year}";
            }
            // entryDate is before second quarter start and after previous year end
            if (DateTime.Compare(entryDate, secondQuarterStart) < 0 && DateTime.Compare(entryDate, previousYearEnd) > 0)
            {
                return $"Q1_{firstQuarterStart.Year}";
            }
            // entryDate is second quarter start
            if (DateTime.Compare(entryDate, secondQuarterStart) == 0)
            {
                return $"Q2_{secondQuarterStart.Year}";
            }
            // entryDate is after second quarter start and before third quarter start
            if (DateTime.Compare(entryDate, secondQuarterStart) > 0 && DateTime.Compare(entryDate, thirdQuarterStart) < 0)
            {
                return $"Q2_{secondQuarterStart.Year}";
            }
            // entryDate is third quarter start
            if (DateTime.Compare(entryDate, thirdQuarterStart) == 0)
            {
                return $"Q3_{thirdQuarterStart.Year}";
            }
            // entryDate is before fourth quarter start and after third quarter start
            if (DateTime.Compare(entryDate, fourthQuarterStart) < 0 && DateTime.Compare(entryDate, thirdQuarterStart) > 0)
            {
                return $"Q3_{thirdQuarterStart.Year}";
            }
            // entryDate is fourth quarter start
            if (DateTime.Compare(entryDate, fourthQuarterStart) == 0)
            {
                return $"Q4_{fourthQuarterStart.Year}";
            }
            // entryDate is after the fourth quarter start
            else { return $"Q4_{fourthQuarterStart.Year}"; }
        }
    }
}