using Xunit;

public class TestDay8Solver
{
    private readonly int _day;
    private readonly ISolver _solver;

    public TestDay8Solver()
    {
        _day = 8;
        _solver = new Day8Solver();
    }

    [Fact]
    public void TestPart1()
    {
        Assert.Equal("2", _solver.Part1(Runner.GetPuzzleInput(_day, 1, true)));
        Assert.Equal("18673", _solver.Part1(Runner.GetPuzzleInput(_day, 1, false)));
    }


    [Fact]
    public void TestPart2()
    {
        Assert.Equal("6", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true)));
        Assert.Equal("17972669116327", _solver.Part2(Runner.GetPuzzleInput(_day, 2, false)));
    }
}