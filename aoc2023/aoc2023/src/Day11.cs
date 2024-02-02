public class Day11Solver : ISolver
{
    public long Solve(List<string> input, long spaceToAdd)
    {
        long numColsWithoutGalaxy = 0;
        List<long> colOffset = new();
        for (int x = 0; x < input[0].Length; x++)
        {
            bool foundGalaxy = false;
            for (int y = 0; y < input.Count; y++)
            {
                if (input[y][x] == '#')
                {
                    foundGalaxy = true;
                }
            }
            if (!foundGalaxy)
            {
                numColsWithoutGalaxy += spaceToAdd;
            }
            colOffset.Add(numColsWithoutGalaxy);
        }

        long numRowsWithoutGalaxy = 0;
        List<long> rowOffset = new();
        for (int y = 0; y < input.Count; y++)
        {
            bool foundGalaxy = false;
            for (int x = 0; x < input[y].Length; x++)
            {
                if (input[y][x] == '#')
                {
                    foundGalaxy = true;
                }
            }
            if (!foundGalaxy)
            {
                numRowsWithoutGalaxy += spaceToAdd;
            }
            rowOffset.Add(numRowsWithoutGalaxy);
        }

        List<LongPoint> galaxies = new();
        for (int y = 0; y < input.Count; y++)
        {
            for (int x = 0; x < input[y].Length; x++)
            {
                if (input[y][x] == '#')
                {
                    galaxies.Add(new LongPoint(x + colOffset[x], y + rowOffset[y]));
                }
            }
        }

        long sum = 0;
        for (int from = 0; from < galaxies.Count; from++)
        {
            for (int to = from + 1; to < galaxies.Count; to++)
            {
                sum += galaxies[from].GetManhattanDistance(galaxies[to]);
            }
        }

        return sum;
    }

    public string Part1(List<string> input)
    {
        return $"{Solve(input, 1)}";
    }

    public string Part2(List<string> input)
    {
        return $"{Solve(input, 1000000 - 1)}";
    }

    public string Part2(List<string> input, long spaceToAdd)
    {
        return $"{Solve(input, spaceToAdd)}";
    }
}
