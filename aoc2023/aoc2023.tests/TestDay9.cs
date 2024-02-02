using Xunit;

public class TestDay9Solver
{
    private readonly int _day;
    private readonly ISolver _solver;

    public TestDay9Solver()
    {
        _day = 9;
        _solver = new Day9Solver();
    }

    [Fact]
    public void TestPart1()
    {
        Assert.Equal("114", _solver.Part1(Runner.GetPuzzleInput(_day, 1, true)));
        Assert.Equal("1995001648", _solver.Part1(Runner.GetPuzzleInput(_day, 1, false)));
    }


    [Fact]
    public void TestPart2()
    {
        Assert.Equal("2", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true)));
        Assert.Equal("988", _solver.Part2(Runner.GetPuzzleInput(_day, 2, false)));
    }
}