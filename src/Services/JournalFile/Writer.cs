using System;
using System.IO;
using Config;

namespace Services.JournalFile
{
    public static class JournalWriter
    {
        public static void Append(JournalEntry journalEntry)
        {
            string quarterName = QuarterDates.LocatedInQuarterPeriod(journalEntry.Date);
            string entryFilePath = Path.Combine(Paths.JournalPath, $"Entry_{quarterName}");
            JournalWriter.LocateOrCreateFileThenAppend(entryFilePath, journalEntry.ToString());
            Console.WriteLine($"Journal Content Updated");
            Console.WriteLine($"{journalEntry.ToString()}");
            Console.WriteLine($"Path File: {entryFilePath.ToString()}");
        }
        private static void LocateOrCreateFileThenAppend(string entryFilePath, string entryText)
        {
            if (File.Exists(entryFilePath))
            {
                using (StreamWriter streamWriter = File.AppendText(entryFilePath))
                {
                    streamWriter.WriteLineAsync(entryText);
                    streamWriter.Close();
                }
            }
            else
            {

                using (StreamWriter streamWriter = File.CreateText(entryFilePath))
                {
                    streamWriter.WriteLineAsync(entryText);
                    streamWriter.Close();
                }
            }

            // create class JournalEntry
            // shift logic into that file
        }

    }

}
