using Xunit;

public class TestDay13Solver
{
    private readonly int _day = 13;
    private readonly ISolver _solver = new Day13Solver();

    [Fact]
    public void TestPart1()
    {
        Assert.Equal("405", _solver.Part1(Runner.GetPuzzleInput(_day, 1, true)));
        Assert.Equal("27202", _solver.Part1(Runner.GetPuzzleInput(_day, 1, false)));
    }

    [Fact]
    public void TestPart2()
    {
        Assert.Equal("400", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true)));
        Assert.Equal("41566", _solver.Part2(Runner.GetPuzzleInput(_day, 2, false)));
    }
}