using System;
using System.Linq;
using System.IO;
using Config;

namespace Services.AccountFile
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

        public static void Append(AccountEntry accountEntry)
        {
            if (AccountWriter.Find(accountEntry.Account))
            {
                Console.WriteLine($"{accountEntry.Account} has previously been opened");
            }
            else
            {
                using (StreamWriter directiveFileWriter = File.AppendText(Paths.Account_Records))
                {
                    directiveFileWriter.WriteLineAsync(accountEntry.ToString());
                    directiveFileWriter.Close();
                }

                Console.WriteLine($"{accountEntry.Account} has been opened");
            }

        }

        public static void Remove(AccountEntry accountEntry)
        {
            var tempFile = Path.GetTempFileName();
            var linesToKeep = File.ReadLines(Paths.Account_Records).Where(line => !line.Contains(accountEntry.Account));

            File.WriteAllLines(tempFile, linesToKeep);

            File.Delete(Paths.Account_Records);
            File.Move(tempFile, Paths.Account_Records);

            Console.WriteLine($"{accountEntry.Account} has been closed");
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