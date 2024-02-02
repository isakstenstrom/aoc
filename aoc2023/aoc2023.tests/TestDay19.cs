using Xunit;

public class TestDay19Solver
{
    private readonly int _day = 19;
    private readonly ISolver _solver = new Day19Solver();

    [Fact]
    public void TestPart1()
    {
        Assert.Equal("19114", _solver.Part1(Runner.GetPuzzleInput(_day, 1, true)));
        Assert.Equal("421983", _solver.Part1(Runner.GetPuzzleInput(_day, 1, false)));
    }

    [Fact]
    public void TestPart2()
    {
        Assert.Equal("167409079868000", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true)));
        Assert.Equal("129249871135292", _solver.Part2(Runner.GetPuzzleInput(_day, 2, false)));
    }
}