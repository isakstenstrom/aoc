using Xunit;

public class TestDay11Solver
{
    private readonly int _day = 11;
    private readonly Day11Solver _solver = new Day11Solver();

    [Fact]
    public void TestPart1()
    {
        Assert.Equal("374", _solver.Part1(Runner.GetPuzzleInput(_day, 1, true)));
        Assert.Equal("9543156", _solver.Part1(Runner.GetPuzzleInput(_day, 1, false)));
    }

    [Fact]
    public void TestPart2()
    {
        Assert.Equal("1030", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true), 10 - 1));
        Assert.Equal("8410", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true), 100 - 1));

        Assert.Equal("82000210", _solver.Part2(Runner.GetPuzzleInput(_day, 2, true)));
        Assert.Equal("625243292686", _solver.Part2(Runner.GetPuzzleInput(_day, 2, false)));
    }
}