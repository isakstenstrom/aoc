using Xunit;

public class TestDay18Solver
{
    private readonly int _day = 18;
    private readonly ISolver _solver = new Day18Solver();

    [Fact]
    public void TestPart1()
    {
        Assert.Equal("62", _solver.Part1(Runner.GetPuzzleInput(_day, 1, true)));
        Assert.Equal("62500", _solver.Part1(Runner.GetPuzzleInput(_day, 1, false)));
    }

    [Fact]
    public void TestPart2()
    {
        Assert.Equal("952408144115", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true)));
        Assert.Equal("122109860712709", _solver.Part2(Runner.GetPuzzleInput(_day, 2, false)));
    }
}