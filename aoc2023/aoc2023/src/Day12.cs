public class Day12Solver : ISolver
{
    public long Rec(string springs, List<int> groups, int springIndex, int groupIndex, long[,] dp)
    {
        if (dp[springIndex, groupIndex] != -1)
        {
            return dp[springIndex, groupIndex];
        }

        int firstPossiblyDamaged = springs[springIndex..].IndexOfAny(['?', '#']);

        if (groupIndex == groups.Count)
        {
            if (firstPossiblyDamaged == -1 || !springs[springIndex..].Contains('#'))
            {
                return 1;
            }
            return 0;
        }

        if (firstPossiblyDamaged == -1)
        {
            return 0;
        }

        int newSpringIndex = springIndex + firstPossiblyDamaged;
        long numCombinations = 0;

        if (groups[groupIndex] < springs.Length - newSpringIndex
            && !springs.Substring(newSpringIndex, groups[groupIndex]).Contains('.')
            && springs[newSpringIndex + groups[groupIndex]] != '#')
        {
            numCombinations += Rec(springs, groups, newSpringIndex + groups[groupIndex] + 1, groupIndex + 1, dp);
        }

        if (springs[newSpringIndex] == '?')
        {
            numCombinations += Rec(springs, groups, newSpringIndex + 1, groupIndex, dp);
        }

        dp[springIndex, groupIndex] = numCombinations;
        return numCombinations;
    }

    public long Solve(List<string> input, int numRepeats)
    {
        long sum = 0;
        foreach (var line in input)
        {
            var splitStr = line.Split(" ");
            string springs = string.Join("?", Enumerable.Repeat(splitStr[0], numRepeats)) + '.';
            List<int> groups = string.Join(",", Enumerable.Repeat(splitStr[1], numRepeats)).Split(",").Select(int.Parse).ToList();

            long[,] dp = new long[springs.Length + 1, groups.Count + 1];
            for (int i = 0; i < dp.GetLength(0); i++)
            {
                for (int j = 0; j < dp.GetLength(1); j++)
                {
                    dp[i, j] = -1;
                }
            }

            sum += Rec(springs, groups, 0, 0, dp);
        }

        return sum;
    }

    public string Part1(List<string> input)
    {
        return $"{Solve(input, 1)}";
    }

    public string Part2(List<string> input)
    {
        return $"{Solve(input, 5)}";
    }
}
