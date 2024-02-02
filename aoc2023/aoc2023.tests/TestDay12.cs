using Xunit;

public class TestDay12Solver
{
    private readonly int _day = 12;
    private readonly ISolver _solver = new Day12Solver();

    [Fact]
    public void TestPart1()
    {
        Assert.Equal("21", _solver.Part1(Runner.GetPuzzleInput(_day, 1, true)));
        Assert.Equal("6488", _solver.Part1(Runner.GetPuzzleInput(_day, 1, false)));
    }

    [Fact]
    public void TestPart2()
    {
        Assert.Equal("525152", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true)));
        Assert.Equal("815364548481", _solver.Part2(Runner.GetPuzzleInput(_day, 2, false)));
    }
}