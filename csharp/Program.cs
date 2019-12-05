using System;
using System.IO;
using System.Linq;

namespace cs
{
    class Program
    {
        static int fuelRequired(int mass)
        {
            var fuel = (int)Math.Floor(mass / 3.0) - 2;
            if (fuel <= 0)
            {
                return 0;
            }
            return fuel + fuelRequired(fuel);
        }

        static void day1_2()
        {
            var path = Path.Combine(Directory.GetCurrentDirectory(), "day1.txt");
            var file = File.OpenText(path);

            int total = 0;
            string? line = null;
            while ((line = file.ReadLine()) != null)
            { 
                var trimmed = line.Trim();
                if (trimmed == "") {
                    continue;
                }
                total += fuelRequired(System.Int32.Parse(trimmed));
            }

            Console.WriteLine(total);
        }
        static void Main()
        {
        }
    }
}
