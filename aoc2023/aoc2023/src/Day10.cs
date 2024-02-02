public class Day10Solver : ISolver
{
    public Direction? GetDirectionFromPipe(char pipe, Direction prevDir)
    {
        return (pipe, prevDir) switch
        {
            ('|', Direction.North) => Direction.North,
            ('|', Direction.South) => Direction.South,
            ('-', Direction.West) => Direction.West,
            ('-', Direction.East) => Direction.East,
            ('L', Direction.South) => Direction.East,
            ('L', Direction.West) => Direction.North,
            ('J', Direction.South) => Direction.West,
            ('J', Direction.East) => Direction.North,
            ('7', Direction.East) => Direction.South,
            ('7', Direction.North) => Direction.West,
            ('F', Direction.West) => Direction.South,
            ('F', Direction.North) => Direction.East,
            _ => null,
        };
    }

    public List<LongPoint>? GetLoopPoints(List<string> input, LongPoint pos, Direction dir)
    {
        List<LongPoint> loopPoints = [];
        loopPoints.Add(pos);

        pos = pos.Step(dir);

        while (0 <= pos.X && 0 <= pos.Y && pos.X < input[0].Length && pos.Y < input.Count)
        {
            char pipe = input[(int)pos.Y][(int)pos.X];

            if (pipe == 'S')
            {
                return loopPoints;
            }

            Direction? newDir = GetDirectionFromPipe(pipe, dir);
            if (newDir == null)
            {
                break;
            }

            dir = newDir.Value;
            pos = pos.Step(dir);
            loopPoints.Add(pos);
        }

        return null;
    }


    public List<LongPoint> CalculateLoop(List<string> input)
    {
        LongPoint start = new LongPoint(-1, -1);
        for (int y = 0; y < input.Count; y++)
        {
            for (int x = 0; x < input[0].Length; x++)
            {
                if (input[y][x] == 'S')
                {
                    start = new LongPoint(x, y);
                }
            }
        }

        if (start == new LongPoint(-1, -1))
        {
            throw new Exception("No start found");
        }

        List<LongPoint>? loopPoints;
        loopPoints = GetLoopPoints(input, start, Direction.North);
        if (loopPoints is not null) return loopPoints;
        loopPoints = GetLoopPoints(input, start, Direction.East);
        if (loopPoints is not null) return loopPoints;
        loopPoints = GetLoopPoints(input, start, Direction.South);
        if (loopPoints is not null) return loopPoints;
        loopPoints = GetLoopPoints(input, start, Direction.West);
        if (loopPoints is not null) return loopPoints;

        throw new Exception("No loop found");
    }

    public long CalculateArea(List<LongPoint> loopPoints)
    {
        long sum = 0;
        for (int i = 0; i < loopPoints.Count - 1; i++)
        {
            sum += loopPoints[i].X * loopPoints[i + 1].Y - loopPoints[i + 1].X * loopPoints[i].Y;
        }
        return Math.Abs(sum) / 2 - loopPoints.Count / 2 + 1;
    }

    public string Part1(List<string> input)
    {
        return $"{CalculateLoop(input).Count / 2}";
    }

    public string Part2(List<string> input)
    {
        return $"{CalculateArea(CalculateLoop(input))}";
    }
}
