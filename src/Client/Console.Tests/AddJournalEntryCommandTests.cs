using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Cobblepot.Client.Console.Tests
{
    internal class AddJournalEntryCommandTests
    {
        public readonly List<string> Tests;
        public readonly bool Run;

        public AddJournalEntryCommandTests(bool? run = null)
        {
            Run = run ?? false;
            Tests = new List<string>();
            AddTestQueries();
        }

        private void AddTestQueries()
        {
            Tests.AddRange(new List<string>()
            {
                "add -h",
                "add One",
                "add Two 10",
                "add Three 10 --accountType Liability",
                "add Four 20 --currency EUR",
                $"add Five 40 --accountType Asset --memo {@"""Words go here that test the memo"""}",
                "add Six 0 --date 1/17/2020",
            });
        }
    }
}