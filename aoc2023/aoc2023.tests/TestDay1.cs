using Xunit;

public class TestDay1Solver
{
    private readonly int _day;
    private readonly ISolver _solver;

    public TestDay1Solver()
    {
        _day = 1;
        _solver = new Day1Solver();
    }

    [Fact]
    public void TestPart1()
    {
        Assert.Equal("142", _solver.Part1(Runner.GetPuzzleInput(_day, 1, true)));
        Assert.Equal("55447", _solver.Part1(Runner.GetPuzzleInput(_day, 1, false)));
    }


    [Fact]
    public void TestPart2()
    {
        Assert.Equal("281", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true)));
        Assert.Equal("54706", _solver.Part2(Runner.GetPuzzleInput(_day, 2, false)));
    }
}