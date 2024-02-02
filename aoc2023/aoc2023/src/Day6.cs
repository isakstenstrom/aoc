public class Day6Solver : ISolver
{
    public long GetDistance(long totalTime, long pressTime)
    {
        return pressTime * (totalTime - pressTime);
    }

    public long Solve(List<long> times, List<long> distances)
    {
        long res = 1;
        for (int raceIndex = 0; raceIndex < times.Count(); raceIndex++)
        {
            double D = times[raceIndex] * times[raceIndex] - 4 * (-1) * (-distances[raceIndex]);
            long root = Convert.ToInt64(
                Math.Ceiling(
                    ((-times[raceIndex] + Math.Sqrt(D)) / (2 * (-1))) + 0.0000000000001
                )
            );

            res *= (times[raceIndex] - root) - (root - 1);
        }
        return res;
    }

    public string Part1(List<string> input)
    {
        List<long> times = input[0].Split(" ", StringSplitOptions.RemoveEmptyEntries).Skip(1).Select(long.Parse).ToList();
        List<long> distances = input[1].Split(" ", StringSplitOptions.RemoveEmptyEntries).Skip(1).Select(long.Parse).ToList();

        return $"{Solve(times, distances)}";
    }

    public string Part2(List<string> input)
    {
        List<long> times = [long.Parse(string.Join("", input[0].Split(" ", StringSplitOptions.RemoveEmptyEntries).Skip(1)))];
        List<long> distances = [long.Parse(string.Join("", input[1].Split(" ", StringSplitOptions.RemoveEmptyEntries).Skip(1)))];

        return $"{Solve(times, distances)}";
    }
}
