using System;
using System.Linq;
using System.IO;
using Config;

namespace Controllers.VaultAccess.Directive
{
    public static class DirectiveFile
    {
        public static bool searchForTarget(string searchTerm)
        {
            string? fileLine;
            StreamReader streamReader = new StreamReader(Paths.DirectiveCache);
            while ((fileLine = streamReader.ReadLine()) != null)
            {
                if (fileLine.Contains(searchTerm))
                {
                    streamReader.Close();
                    return true;
                }
            }
            streamReader.Close();
            return false;
        }

        public static void removeTarget(string searchTerm)
        {
            var tempFile = Path.GetTempFileName();
            var linesToKeep = File.ReadLines(Paths.DirectiveCache).Where(line => !line.Contains(searchTerm));

            File.WriteAllLines(tempFile, linesToKeep);

            File.Delete(Paths.DirectiveCache);
            File.Move(tempFile, Paths.DirectiveCache);
        }

        public static void appendTarget(string targetTerm)
        {
            using (StreamWriter directiveFileWriter = File.AppendText(Paths.DirectiveCache))
            {
                directiveFileWriter.WriteLineAsync(targetTerm);
                directiveFileWriter.Close();
            }
        }
    }
}