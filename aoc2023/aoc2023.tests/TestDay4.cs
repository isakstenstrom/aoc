using Xunit;

public class TestDay4Solver
{
    private readonly int _day;
    private readonly ISolver _solver;

    public TestDay4Solver()
    {
        _day = 4;
        _solver = new Day4Solver();
    }

    [Fact]
    public void TestPart1()
    {
        Assert.Equal("13", _solver.Part1(Runner.GetPuzzleInput(_day, 1, true)));
        Assert.Equal("24175", _solver.Part1(Runner.GetPuzzleInput(_day, 1, false)));
    }


    [Fact]
    public void TestPart2()
    {
        Assert.Equal("30", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true)));
        Assert.Equal("18846301", _solver.Part2(Runner.GetPuzzleInput(_day, 2, false)));
    }
}