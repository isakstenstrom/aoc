using Xunit;

public class TestDay5Solver
{
    private readonly int _day;
    private readonly ISolver _solver;

    public TestDay5Solver()
    {
        _day = 5;
        _solver = new Day5Solver();
    }

    [Fact]
    public void TestLongPointIntersection()
    {
        Assert.Null(new LongPoint(4, 8).Intersection(new LongPoint(1, 2)));
        Assert.Equal(new LongPoint(4, 4), new LongPoint(4, 8).Intersection(new LongPoint(1, 4)));
        Assert.Equal(new LongPoint(4, 6), new LongPoint(4, 8).Intersection(new LongPoint(1, 6)));
        Assert.Equal(new LongPoint(5, 7), new LongPoint(4, 8).Intersection(new LongPoint(5, 7)));
        Assert.Equal(new LongPoint(4, 8), new LongPoint(4, 8).Intersection(new LongPoint(3, 9)));
        Assert.Equal(new LongPoint(6, 8), new LongPoint(4, 8).Intersection(new LongPoint(6, 10)));
        Assert.Equal(new LongPoint(8, 8), new LongPoint(4, 8).Intersection(new LongPoint(8, 10)));
        Assert.Null(new LongPoint(4, 8).Intersection(new LongPoint(10, 12)));
    }

    [Fact]
    public void TestLongPointDifference()
    {
        Assert.Equal((null, new LongPoint(4, 8)), new LongPoint(4, 8).Difference(new LongPoint(1, 2)));
        Assert.Equal((null, new LongPoint(5, 8)), new LongPoint(4, 8).Difference(new LongPoint(1, 4)));
        Assert.Equal((null, new LongPoint(7, 8)), new LongPoint(4, 8).Difference(new LongPoint(1, 6)));
        Assert.Equal((new LongPoint(4, 4), new LongPoint(8, 8)), new LongPoint(4, 8).Difference(new LongPoint(5, 7)));
        Assert.Equal((null, null), new LongPoint(4, 8).Difference(new LongPoint(3, 9)));
        Assert.Equal((new LongPoint(4, 5), null), new LongPoint(4, 8).Difference(new LongPoint(6, 10)));
        Assert.Equal((new LongPoint(4, 7), null), new LongPoint(4, 8).Difference(new LongPoint(8, 10)));
        Assert.Equal((new LongPoint(4, 8), null), new LongPoint(4, 8).Difference(new LongPoint(10, 12)));

    }

    [Fact]
    public void TestPart1()
    {
        Assert.Equal("35", _solver.Part1(Runner.GetPuzzleInput(_day, 1, true)));
        Assert.Equal("251346198", _solver.Part1(Runner.GetPuzzleInput(_day, 1, false)));
    }


    [Fact]
    public void TestPart2()
    {
        Assert.Equal("46", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true)));
        Assert.Equal("72263011", _solver.Part2(Runner.GetPuzzleInput(_day, 2, false)));
    }
}