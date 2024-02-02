public class Day17Solver : ISolver
{
    public static int DirToInt(Direction direction)
    {
        return direction switch
        {
            Direction.North => 0,
            Direction.East => 1,
            Direction.South => 2,
            Direction.West => 3,
            _ => throw new ArgumentException("Invalid direction"),
        };
    }

    public string Solve(List<string> input, int minLineLength, int maxLineLength)
    {
        LongPoint goal = new(input[0].Length - 1, input.Count - 1);

        // Keeps track of visited based on coordinates, direction and line length.
        bool[,,,] visited = new bool[input.Count, input[0].Length, 4, maxLineLength];

        PriorityQueue<(LongPoint, Direction, int), int> q = new();
        q.Enqueue((new LongPoint(1, 0), Direction.East, 1), input[0][1] - '0');
        q.Enqueue((new LongPoint(0, 1), Direction.South, 1), input[1][0] - '0');

        while (q.TryDequeue(out (LongPoint, Direction, int) curr, out int currCost))
        {
            (LongPoint currPos, Direction currDir, int currLineLength) = curr;
            if (visited[currPos.Y, currPos.X, DirToInt(currDir), currLineLength])
            {
                continue;
            }
            visited[currPos.Y, currPos.X, DirToInt(currDir), currLineLength] = true;
            if (currPos == goal && currLineLength >= minLineLength)
            {
                return $"{currCost}";
            }

            Direction newDir;
            LongPoint newPos;

            newDir = currDir.RotateLeft();
            newPos = currPos.Step(newDir);
            if (newPos.X >= 0
                && newPos.Y >= 0
                && newPos.X < input[0].Length
                && newPos.Y < input.Count
                && currLineLength >= minLineLength
                )
            {
                q.Enqueue((newPos, newDir, 0), currCost + input[(int)newPos.Y][(int)newPos.X] - '0');
            }

            newDir = currDir.RotateRight();
            newPos = currPos.Step(newDir);
            if (newPos.X >= 0
                && newPos.Y >= 0
                && newPos.X < input[0].Length
                && newPos.Y < input.Count
                && currLineLength >= minLineLength
                )
            {
                q.Enqueue((newPos, newDir, 0), currCost + input[(int)newPos.Y][(int)newPos.X] - '0');
            }

            newDir = currDir;
            newPos = currPos.Step(newDir);
            if (newPos.X >= 0
                && newPos.Y >= 0
                && newPos.X < input[0].Length
                && newPos.Y < input.Count
                && currLineLength + 1 < maxLineLength
                )
            {
                q.Enqueue((newPos, newDir, currLineLength + 1), currCost + input[(int)newPos.Y][(int)newPos.X] - '0');
            }
        }
        throw new Exception("No path found");
    }

    public string Part1(List<string> input)
    {
        return Solve(input, 0, 3);
    }

    public string Part2(List<string> input)
    {
        return Solve(input, 3, 10);
    }
}