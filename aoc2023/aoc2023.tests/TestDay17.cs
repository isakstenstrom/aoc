using Xunit;

public class TestDay17Solver
{
    private readonly int _day = 17;
    private readonly ISolver _solver = new Day17Solver();

    [Fact]
    public void TestPart1()
    {
        Assert.Equal("102", _solver.Part1(Runner.GetPuzzleInput(_day, 1, true)));
        Assert.Equal("791", _solver.Part1(Runner.GetPuzzleInput(_day, 1, false)));
    }

    [Fact]
    public void TestPart2()
    {
        Assert.Equal("94", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true)));
        Assert.Equal("71", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true, 1)));
        Assert.Equal("900", _solver.Part2(Runner.GetPuzzleInput(_day, 2, false)));
    }
}