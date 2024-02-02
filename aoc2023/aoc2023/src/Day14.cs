using System.Text;

public class Day14Solver : ISolver
{
    private const int TOTAL_NUM_ITERATIONS = 1000000000;

    enum Rock
    {
        Empty,
        Round,
        Square,
    }

    static Rock[,] CreatePlatform(List<string> input)
    {
        Rock[,] rocks = new Rock[input.Count, input[0].Length];

        for (int y = 0; y < input.Count; y++)
        {
            for (int x = 0; x < input[0].Length; x++)
            {
                rocks[y, x] = input[y][x] switch
                {
                    '.' => Rock.Empty,
                    'O' => Rock.Round,
                    '#' => Rock.Square,
                    _ => throw new Exception("Unknown rock type")
                };
            }
        }
        return rocks;
    }

    static string GetPlatformHash(Rock[,] rocks)
    {
        StringBuilder sb = new StringBuilder();
        for (int y = 0; y < rocks.GetLength(0); y++)
        {
            for (int x = 0; x < rocks.GetLength(1); x++)
            {
                switch (rocks[y, x])
                {
                    case Rock.Empty:
                        sb.Append('.');
                        break;
                    case Rock.Round:
                        sb.Append('O');
                        break;
                    case Rock.Square:
                        sb.Append('#');
                        break;
                }
            }
        }
        return sb.ToString();
    }

    static void TiltPlatformNorth(Rock[,] rocks)
    {
        for (int y = 0; y < rocks.GetLength(0); y++)
        {
            for (int x = 0; x < rocks.GetLength(1); x++)
            {
                if (rocks[y, x] == Rock.Round)
                {
                    int newY = y;
                    while (newY - 1 >= 0 && rocks[newY - 1, x] == Rock.Empty)
                    {
                        newY--;
                    }
                    rocks[y, x] = Rock.Empty;
                    rocks[newY, x] = Rock.Round;
                }
            }
        }
    }

    static void TiltPlatformSouth(Rock[,] rocks)
    {
        for (int y = rocks.GetLength(0) - 1; y >= 0; y--)
        {
            for (int x = 0; x < rocks.GetLength(1); x++)
            {
                if (rocks[y, x] == Rock.Round)
                {
                    int newY = y;
                    while (newY + 1 < rocks.GetLength(0) && rocks[newY + 1, x] == Rock.Empty)
                    {
                        newY++;
                    }
                    rocks[y, x] = Rock.Empty;
                    rocks[newY, x] = Rock.Round;
                }
            }
        }
    }

    static void TiltPlatformWest(Rock[,] rocks)
    {
        for (int x = 0; x < rocks.GetLength(1); x++)
        {
            for (int y = 0; y < rocks.GetLength(0); y++)
            {
                if (rocks[y, x] == Rock.Round)
                {
                    int newX = x;
                    while (newX - 1 >= 0 && rocks[y, newX - 1] == Rock.Empty)
                    {
                        newX--;
                    }
                    rocks[y, x] = Rock.Empty;
                    rocks[y, newX] = Rock.Round;
                }
            }
        }
    }

    static void TiltPlatformEast(Rock[,] rocks)
    {
        for (int x = rocks.GetLength(1) - 1; x >= 0; x--)
        {
            for (int y = 0; y < rocks.GetLength(0); y++)
            {
                if (rocks[y, x] == Rock.Round)
                {
                    int newX = x;
                    while (newX + 1 < rocks.GetLength(1) && rocks[y, newX + 1] == Rock.Empty)
                    {
                        newX++;
                    }
                    rocks[y, x] = Rock.Empty;
                    rocks[y, newX] = Rock.Round;
                }
            }
        }
    }

    static int CalculateScore(Rock[,] rocks)
    {
        int sum = 0;
        for (int y = 0; y < rocks.GetLength(0); y++)
        {
            for (int x = 0; x < rocks.GetLength(1); x++)
            {
                if (rocks[y, x] == Rock.Round)
                {
                    sum += rocks.GetLength(0) - y;
                }
            }
        }
        return sum;
    }

    public string Part1(List<string> input)
    {
        Rock[,] rocks = CreatePlatform(input);
        TiltPlatformNorth(rocks);
        return $"{CalculateScore(rocks)}";
    }

    public string Part2(List<string> input)
    {
        Rock[,] rocks = CreatePlatform(input);

        Dictionary<string, int> seen = [];
        bool loopFound = false;
        for (int iter = 1; iter <= TOTAL_NUM_ITERATIONS; iter++)
        {
            TiltPlatformNorth(rocks);
            TiltPlatformWest(rocks);
            TiltPlatformSouth(rocks);
            TiltPlatformEast(rocks);
            if (!loopFound)
            {
                string platformHash = GetPlatformHash(rocks);
                if (seen.TryGetValue(platformHash, out int cycleStart))
                {
                    loopFound = true;
                    int cycleLength = iter - cycleStart;
                    int iterRemaining = (TOTAL_NUM_ITERATIONS - iter) % cycleLength;
                    iter = TOTAL_NUM_ITERATIONS - iterRemaining;
                }
                else
                {
                    seen.Add(platformHash, iter);
                }
            }
        }

        return $"{CalculateScore(rocks)}";
    }
}
