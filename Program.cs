using System;
using Controllers.Structure;

namespace cobblepot
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Welcome");
            new Ledger();
            new Journal();
            new Account();
        }
    }
}
