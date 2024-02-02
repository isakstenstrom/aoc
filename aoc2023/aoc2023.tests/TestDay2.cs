using Xunit;

public class TestDay2Solver
{
    private readonly int _day = 2;
    private readonly ISolver _solver = new Day2Solver();

    [Fact]
    public void TestPart1()
    {
        Assert.Equal("8", _solver.Part1(Runner.GetPuzzleInput(_day, 1, true)));
        Assert.Equal("2348", _solver.Part1(Runner.GetPuzzleInput(_day, 1, false)));
    }

    [Fact]
    public void TestPart2()
    {
        Assert.Equal("2286", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true)));
        Assert.Equal("76008", _solver.Part2(Runner.GetPuzzleInput(_day, 2, false)));
    }
}