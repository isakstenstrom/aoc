using Xunit;

public class TestDay20Solver
{
    private readonly int _day = 20;
    private readonly ISolver _solver = new Day20Solver();

    [Fact]
    public void TestPart1()
    {
        Assert.Equal("32000000", _solver.Part1(Runner.GetPuzzleInput(_day, 1, true)));
        Assert.Equal("11687500", _solver.Part1(Runner.GetPuzzleInput(_day, 1, true, 1)));
        Assert.Equal("866435264", _solver.Part1(Runner.GetPuzzleInput(_day, 1, false)));
    }

    [Fact]
    public void TestPart2()
    {
        Assert.Equal("229215609826339", _solver.Part2(Runner.GetPuzzleInput(_day, 2, false)));
    }
}