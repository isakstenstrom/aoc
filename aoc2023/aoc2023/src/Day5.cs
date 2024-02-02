public struct LongPoint
{
    public LongPoint(long x, long y)
    {
        X = x;
        Y = y;
    }

    public long X { get; }
    public long Y { get; }

    public LongPoint? Intersection(LongPoint other)
    {
        long low = Math.Max(X, other.X);
        long high = Math.Min(Y, other.Y);


        if (low <= high)
        {
            return new LongPoint(low, high);
        }
        return null;
    }

    public (LongPoint? under, LongPoint? over) Difference(LongPoint other)
    {
        LongPoint? under = null;
        LongPoint? over = null;

        if (X < other.X)
        {
            under = new LongPoint(X, Math.Min(Y, other.X - 1));
        }
        if (Y > other.Y)
        {
            over = new LongPoint(Math.Max(other.Y + 1, X), Y);
        }
        return (under, over);
    }

    public override string ToString() => $"({X}, {Y})";
}

public struct Transform
{
    public Transform(LongPoint range, long offset)
    {
        Range = range;
        Offset = offset;
    }

    public Transform(string input)
    {
        var splitStr = input.Split(" ");
        long dest = long.Parse(splitStr[0]);
        long source = long.Parse(splitStr[1]);
        long length = long.Parse(splitStr[2]);

        Range = new LongPoint(source, source + length);
        Offset = dest - source;
    }

    public LongPoint Range { get; }
    public long Offset { get; }

}

public class Day5Solver : ISolver
{
    public long Solve(List<string> input, List<LongPoint> ranges)
    {
        List<List<Transform>> transformStage = new();

        int mapIndex = 0;
        transformStage.Add(new List<Transform>());
        foreach (var line in input)
        {
            if (line.Length == 0)
            {
                mapIndex++;
                transformStage.Add(new List<Transform>());
                continue;
            }
            if (line.EndsWith("map:"))
            {
                continue;
            }

            transformStage[mapIndex].Add(new Transform(line));
        }

        var currentRanges = ranges;
        foreach (var transforms in transformStage)
        {
            var nextRanges = new List<LongPoint>();

            for (int rangeIndex = 0; rangeIndex < currentRanges.Count(); rangeIndex++)
            {
                bool foundIntersection = false;
                foreach (Transform transform in transforms)
                {
                    LongPoint? intersection = currentRanges[rangeIndex].Intersection(transform.Range);
                    if (intersection is not null)
                    {
                        foundIntersection = true;
                        nextRanges.Add(new LongPoint(intersection.Value.X + transform.Offset, intersection.Value.Y + transform.Offset));

                        var (over, under) = currentRanges[rangeIndex].Difference(transform.Range);
                        if (over is not null)
                        {
                            currentRanges.Add(over.Value);
                        }
                        if (under is not null)
                        {
                            currentRanges.Add(under.Value);
                        }
                        break;
                    }
                }

                if (!foundIntersection)
                {
                    nextRanges.Add(currentRanges[rangeIndex]);
                }
            }
            currentRanges = nextRanges;
        }

        return currentRanges.Min(lp => lp.X);
    }

    public string Part1(List<string> input)
    {
        List<LongPoint> staringRanges = new List<LongPoint>();
        foreach (var startIndex in input[0].Substring(7).Split(" ").Select(long.Parse))
        {
            staringRanges.Add(new LongPoint(startIndex, startIndex));
        }
        long res = Solve(input.GetRange(3, input.Count() - 3), staringRanges);
        return $"{res}";
    }

    public string Part2(List<string> input)
    {
        List<LongPoint> staringRanges = new List<LongPoint>();
        foreach (var ranges in input[0].Substring(7).Split(" ").Chunk(2))
        {
            long startingIndex = long.Parse(ranges[0]);
            long length = long.Parse(ranges[1]);

            staringRanges.Add(new LongPoint(startingIndex, startingIndex + length));
        }
        long res = Solve(input.GetRange(3, input.Count() - 3), staringRanges);
        return $"{res}";
    }
}
