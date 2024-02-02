using Xunit;

public class TestDay16Solver
{
    private readonly int _day = 16;
    private readonly ISolver _solver = new Day16Solver();

    [Fact]
    public void TestPart1()
    {
        Assert.Equal("46", _solver.Part1(Runner.GetPuzzleInput(_day, 1, true)));
        Assert.Equal("7927", _solver.Part1(Runner.GetPuzzleInput(_day, 1, false)));
    }

    [Fact]
    public void TestPart2()
    {
        Assert.Equal("51", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true)));
        Assert.Equal("8246", _solver.Part2(Runner.GetPuzzleInput(_day, 2, false)));
    }
}