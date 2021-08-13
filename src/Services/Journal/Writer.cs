using System;
using System.IO;
using Config;

namespace Services.Journal
{
    public static class JournalWriter
    {
        public static void Append(string entryText)
        {
            using (StreamWriter directiveFileWriter = File.AppendText(Paths.Account_Records))
            {
                directiveFileWriter.WriteLineAsync(entryText);
                directiveFileWriter.Close();
            }
            // TODO: add how the console output will look
        }

    }

}
