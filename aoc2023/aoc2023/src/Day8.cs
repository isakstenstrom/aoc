using System.Text.RegularExpressions;

public class Day8Solver : ISolver
{
    static long LCM(List<long> numbers)
    {
        long result = numbers[0];
        for (int i = 1; i < numbers.Count; i++)
        {
            result = LCM(result, numbers[i]);
        }
        return result;
    }

    static long LCM(long a, long b)
    {
        return a * b / GCD(a, b);
    }

    static long GCD(long a, long b)
    {
        while (b != 0)
        {
            long temp = b;
            b = a % b;
            a = temp;
        }
        return a;
    }

    public long Solve(List<string> input, Regex startPattern, Regex endPattern)
    {
        List<int> directions = input[0].ToCharArray().Select(c => c == 'L' ? 0 : 1).ToList();
        Dictionary<string, string[]> map = input.Skip(2).ToDictionary(line => line.Substring(0, 3),
                                                                      line => new string[] { line.Substring(7, 3), line.Substring(12, 3) });

        List<long> iterationLengths = new();

        foreach (var startLocation in map.Keys.Where(s => startPattern.IsMatch(s)))
        {
            string currentLocation = startLocation;

            long numIterations = 0;
            int directionIndex = 0;

            while (!endPattern.IsMatch(currentLocation))
            {
                currentLocation = map[currentLocation][directions[directionIndex]];

                numIterations++;
                directionIndex = (directionIndex + 1) % directions.Count;
            }
            iterationLengths.Add(numIterations);
        }
        return LCM(iterationLengths);
    }

    public string Part1(List<string> input)
    {
        return $"{Solve(input, new Regex(@"AAA"), new Regex(@"ZZZ"))}";
    }

    public string Part2(List<string> input)
    {
        return $"{Solve(input, new Regex(@"..A"), new Regex(@"..Z"))}";
    }
}
