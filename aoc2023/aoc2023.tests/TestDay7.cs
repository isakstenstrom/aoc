using Xunit;

public class TestDay7Solver
{
    private readonly int _day;
    private readonly ISolver _solver;

    public TestDay7Solver()
    {
        _day = 7;
        _solver = new Day7Solver();
    }

    [Fact]
    public void TestPart1()
    {
        Assert.Equal("6440", _solver.Part1(Runner.GetPuzzleInput(_day, 1, true)));
        Assert.Equal("250898830", _solver.Part1(Runner.GetPuzzleInput(_day, 1, false)));
    }


    [Fact]
    public void TestPart2()
    {
        Assert.Equal("5905", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true)));
        Assert.Equal("252127335", _solver.Part2(Runner.GetPuzzleInput(_day, 2, false)));
    }
}