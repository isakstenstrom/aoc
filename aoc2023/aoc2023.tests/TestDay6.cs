using Xunit;

public class TestDay6Solver
{
    private readonly int _day;
    private readonly ISolver _solver;

    public TestDay6Solver()
    {
        _day = 6;
        _solver = new Day6Solver();
    }

    [Fact]
    public void TestPart1()
    {
        Assert.Equal("288", _solver.Part1(Runner.GetPuzzleInput(_day, 1, true)));
        Assert.Equal("1108800", _solver.Part1(Runner.GetPuzzleInput(_day, 1, false)));
    }


    [Fact]
    public void TestPart2()
    {
        Assert.Equal("71503", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true)));
        Assert.Equal("36919753", _solver.Part2(Runner.GetPuzzleInput(_day, 2, false)));
    }
}