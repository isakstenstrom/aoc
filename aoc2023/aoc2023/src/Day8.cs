using System.Text.RegularExpressions;

public class Day8Solver : ISolver
{
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
        return MathUtils.LCM(iterationLengths);
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
