using Xunit;

public class TestDay14Solver
{
    private readonly int _day = 14;
    private readonly ISolver _solver = new Day14Solver();

    [Fact]
    public void TestPart1()
    {
        Assert.Equal("136", _solver.Part1(Runner.GetPuzzleInput(_day, 1, true)));
        Assert.Equal("110779", _solver.Part1(Runner.GetPuzzleInput(_day, 1, false)));
    }

    [Fact]
    public void TestPart2()
    {
        Assert.Equal("64", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true)));
        Assert.Equal("86069", _solver.Part2(Runner.GetPuzzleInput(_day, 2, false)));
    }
}