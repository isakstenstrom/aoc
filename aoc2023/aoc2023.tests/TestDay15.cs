using Xunit;

public class TestDay15Solver
{
    private readonly int _day = 15;
    private readonly ISolver _solver = new Day15Solver();

    [Fact]
    public void TestPart1()
    {
        Assert.Equal("1320", _solver.Part1(Runner.GetPuzzleInput(_day, 1, true)));
        Assert.Equal("503487", _solver.Part1(Runner.GetPuzzleInput(_day, 1, false)));
    }

    [Fact]
    public void TestPart2()
    {
        Assert.Equal("145", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true)));
        Assert.Equal("261505", _solver.Part2(Runner.GetPuzzleInput(_day, 2, false)));
    }
}