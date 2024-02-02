public class Day18Solver : ISolver
{
    public long CalculateArea(List<LongPoint> p, long circumference)
    {
        long sum = 0;
        for (int i = 0; i < p.Count - 1; i++)
        {
            sum += p[i].X * p[i + 1].Y - p[i + 1].X * p[i].Y;
        }
        return Math.Abs(sum) / 2 + circumference / 2 + 1;
    }
    public string Solve(List<string> input, bool useComplicatedInput)
    {
        List<LongPoint> points = new();
        LongPoint currentPoint = new LongPoint(0, 0);
        long circumference = 0;
        foreach (var line in input)
        {
            char directionChar;
            long lineLength;
            if (useComplicatedInput)
            {
                directionChar = line[^2];
                lineLength = Convert.ToInt64(line[^7..^2], 16);
            }
            else
            {
                directionChar = line[0];
                lineLength = long.Parse(line.Split(' ')[1]);
            }

            Direction dir = directionChar switch
            {
                'R' or '0' => Direction.East,
                'D' or '1' => Direction.South,
                'L' or '2' => Direction.West,
                'U' or '3' => Direction.North,
                _ => throw new Exception($"Invalid direction: {directionChar}"),
            };
            currentPoint += new LongPoint(0, 0).Step(dir) * lineLength;
            circumference += lineLength;

            points.Add(currentPoint);
        }
        return $"{CalculateArea(points, circumference)}";
    }


    public string Part1(List<string> input)
    {
        return Solve(input, false);
    }

    public string Part2(List<string> input)
    {
        return Solve(input, true);
    }
}