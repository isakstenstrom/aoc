using Xunit;

public class TestDay10Solver
{
    private readonly int _day = 10;
    private readonly ISolver _solver = new Day10Solver();

    [Fact]
    public void TestPart1()
    {
        Assert.Equal("4", _solver.Part1(Runner.GetPuzzleInput(_day, 1, true)));
        Assert.Equal("8", _solver.Part1(Runner.GetPuzzleInput(_day, 1, true, 1)));
        Assert.Equal("6812", _solver.Part1(Runner.GetPuzzleInput(_day, 1, false)));
    }

    [Fact]
    public void TestPart2()
    {
        Assert.Equal("1", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true)));
        Assert.Equal("1", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true, 1)));
        Assert.Equal("4", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true, 2)));
        Assert.Equal("4", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true, 3)));
        Assert.Equal("8", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true, 4)));
        Assert.Equal("10", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true, 5)));
        Assert.Equal("527", _solver.Part2(Runner.GetPuzzleInput(_day, 2, false)));
    }
}