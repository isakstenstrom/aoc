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

    public long GetManhattanDistance(LongPoint other)
    {
        return Math.Abs(X - other.X) + Math.Abs(Y - other.Y);
    }

    public override string ToString() => $"({X}, {Y})";
}