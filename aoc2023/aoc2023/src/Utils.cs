public enum Direction
{
    North,
    East,
    South,
    West,
};

public static class DirectionExtensions
{
    public static Direction RotateLeft(this Direction direction)
    {
        return direction switch
        {
            Direction.North => Direction.West,
            Direction.East => Direction.North,
            Direction.South => Direction.East,
            Direction.West => Direction.South,
            _ => throw new ArgumentException("Invalid direction"),
        };
    }

    public static Direction RotateRight(this Direction direction)
    {
        return direction switch
        {
            Direction.North => Direction.East,
            Direction.East => Direction.South,
            Direction.South => Direction.West,
            Direction.West => Direction.North,
            _ => throw new ArgumentException("Invalid direction"),
        };
    }
}

public struct LongPoint(long x, long y)
{
    public long X { get; } = x;
    public long Y { get; } = y;


    public override bool Equals(object? obj) => Equals(obj as Nullable<LongPoint>);

    public bool Equals(LongPoint? p)
    {
        if (p is null)
        {
            return false;
        }

        if (GetType() != p.GetType())
        {
            return false;
        }

        return X == p.Value.X && Y == p.Value.Y;
    }

    public override int GetHashCode() => (X, Y).GetHashCode();

    public static bool operator ==(LongPoint p1, LongPoint p2)
    {
        return p1.Equals(p2);
    }

    public static bool operator !=(LongPoint p1, LongPoint p2)
    {
        return !(p1 == p2);
    }

    public static LongPoint operator +(LongPoint a) => a;

    public static LongPoint operator -(LongPoint a) => new LongPoint(-a.X, -a.Y);

    public static LongPoint operator +(LongPoint a, LongPoint b)
        => new LongPoint(a.X + b.X, a.Y + b.Y);

    public static LongPoint operator -(LongPoint a, LongPoint b)
        => a + (-b);

    public static LongPoint operator *(LongPoint a, long b)
        => new LongPoint(a.X * b, a.Y * b);


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

    public long GetManhattanDistance(LongPoint other)
    {
        return Math.Abs(X - other.X) + Math.Abs(Y - other.Y);
    }

    public LongPoint Step(Direction dir)
    {
        return dir switch
        {
            Direction.North => new LongPoint(X, Y - 1),
            Direction.East => new LongPoint(X + 1, Y),
            Direction.South => new LongPoint(X, Y + 1),
            Direction.West => new LongPoint(X - 1, Y),
            _ => throw new ArgumentException("Invalid direction"),
        };
    }

    public IEnumerable<LongPoint> Neighbors()
    {
        yield return Step(Direction.North);
        yield return Step(Direction.East);
        yield return Step(Direction.South);
        yield return Step(Direction.West);
    }

    public override string ToString() => $"({X}, {Y})";
}

