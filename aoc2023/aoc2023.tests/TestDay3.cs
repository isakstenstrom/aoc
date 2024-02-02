using Xunit;

public class TestDay3Solver
{
    private readonly int _day = 3;
    private readonly ISolver _solver = new Day3Solver();

    [Fact]
    public void TestPart1()
    {
        Assert.Equal("4361", _solver.Part1(Runner.GetPuzzleInput(_day, 1, true)));
        Assert.Equal("514969", _solver.Part1(Runner.GetPuzzleInput(_day, 1, false)));
    }

    [Fact]
    public void TestPart2()
    {
        Assert.Equal("467835", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true)));
        Assert.Equal("78915902", _solver.Part2(Runner.GetPuzzleInput(_day, 2, false)));
    }
}