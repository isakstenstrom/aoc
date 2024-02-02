public enum Direction
{
    North,
    East,
    South,
    West,
};

public struct LongPoint
{
    public LongPoint(long x, long y)
    {
        X = x;
        Y = y;
    }

    public long X { get; }
    public long Y { get; }


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

    public override string ToString() => $"({X}, {Y})";
}

