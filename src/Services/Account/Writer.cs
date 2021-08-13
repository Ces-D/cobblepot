using System;
using System.Linq;
using System.IO;
using Config;

namespace Services.Account
{
    public static class AccountWriter
    {
        private static bool Find(string accountName)
        {
            string? fileLine;

            StreamReader streamReader = new StreamReader(Paths.Account_Records);
            while ((fileLine = streamReader.ReadLine()) != null)
            {
                if (fileLine.Contains(accountName))
                {
                    streamReader.Close();
                    return true;
                }
            }
            streamReader.Close();
            return false;
        }

        public static string ToEntryString(string accountName, DateTime entryDate)
        {
            return string.Format("{0}{1}",
                          arg0: entryDate.ToShortDateString(),
                          arg1: accountName.PadLeft(30));
        }

        public static void Append(string accountName, DateTime entryDate)
        {
            if (AccountWriter.Find(accountName))
            {
                Console.WriteLine($"{accountName} has previously been opened");
            }
            else
            {
                string fileLine = AccountWriter.ToEntryString(accountName, entryDate);

                using (StreamWriter directiveFileWriter = File.AppendText(Paths.Account_Records))
                {
                    directiveFileWriter.WriteLineAsync(fileLine);
                    directiveFileWriter.Close();
                }

                Console.WriteLine($"{accountName} has been opened");
            }

        }

        public static void Remove(string accountName)
        {
            var tempFile = Path.GetTempFileName();
            var linesToKeep = File.ReadLines(Paths.Account_Records).Where(line => !line.Contains(accountName));

            File.WriteAllLines(tempFile, linesToKeep);

            File.Delete(Paths.Account_Records);
            File.Move(tempFile, Paths.Account_Records);

            Console.WriteLine($"{accountName} has been closed");
        }

        public static void List()
        {
            string? fileLine;
            StreamReader streamReader = new StreamReader(Paths.Account_Records);
            while ((fileLine = streamReader.ReadLine()) != null)
            {
                Console.WriteLine(fileLine);
            }
            streamReader.Close();
        }
    }
}