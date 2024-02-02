public interface ISolver
{
    string Part1(List<string> input);
    string Part2(List<string> input);
}

public class Runner
{
    static string GetFilePath(int day, int? part, bool useSampleInput, int? alternateVersion = null)
    {
        return string.Format(
            "aoc2023/res/{0}/day{1}{2}{3}.txt",
            useSampleInput ? "sample" : "input",
            day,
            part is not null ? $"_part{part.Value}" : "",
            alternateVersion is not null ? $"_alt{alternateVersion.Value}" : "");
    }

    public static List<string> GetPuzzleInput(int day, int part, bool useSampleInput, int? alternateVersion = null)
    {
        string fileName = GetFilePath(day, part, useSampleInput, alternateVersion);
        if (!File.Exists(fileName))
        {
            fileName = GetFilePath(day, null, useSampleInput, alternateVersion);
        }
        var fileLines = File.ReadAllLines(fileName);
        return new List<string>(fileLines);
    }

    public static string FormatDuration(TimeSpan duration)
    {
        if (duration.Days != 0)
        {
            return $"{duration.TotalDays:N2} days";
        }
        if (duration.Hours != 0)
        {
            return $"{duration.TotalHours:N2} h";
        }
        if (duration.Minutes != 0)
        {
            return $"{duration.TotalMinutes:N2} m";
        }
        if (duration.Seconds != 0)
        {
            return $"{duration.TotalSeconds:N2} s";
        }
        if (duration.Milliseconds != 0)
        {
            return $"{duration.TotalMilliseconds:N2} ms";
        }
        if (duration.Microseconds != 0)
        {
            return $"{duration.TotalMicroseconds:N2} us";
        }
        return $"{duration.TotalNanoseconds:N2} ns";
    }

    public static void SolveDay(int day, ISolver solver, int solveDay, int solvePart, bool useSampleInput, int? alternateInput)
    {
        if (day != solveDay && solveDay != -1)
        {
            return;
        }

        List<string> input;

        if (solvePart == 1 || solvePart == -1)
        {
            input = GetPuzzleInput(day, 1, useSampleInput, alternateInput);
            var watch = System.Diagnostics.Stopwatch.StartNew();

            string result = solver.Part1(input);

            watch.Stop();
            var elapsedTime = FormatDuration(watch.Elapsed);
            Console.WriteLine($"Day {day,2}: part 1: {result,16}, in {elapsedTime}");
        }

        if (solvePart == 2 || solvePart == -1)
        {
            input = GetPuzzleInput(day, 2, useSampleInput, alternateInput);
            var watch = System.Diagnostics.Stopwatch.StartNew();

            string result = solver.Part2(input);

            watch.Stop();
            var elapsedTime = FormatDuration(watch.Elapsed);
            Console.WriteLine($"Day {day,2}: part 2: {result,16}, in {elapsedTime}");
        }
    }

    public static void Solve(int solveDay, int solvePart, bool useSampleInput, int? alternateInput)
    {
        var watch = System.Diagnostics.Stopwatch.StartNew();

        SolveDay(1, new Day1Solver(), solveDay, solvePart, useSampleInput, alternateInput);
        SolveDay(2, new Day2Solver(), solveDay, solvePart, useSampleInput, alternateInput);
        SolveDay(3, new Day3Solver(), solveDay, solvePart, useSampleInput, alternateInput);
        SolveDay(4, new Day4Solver(), solveDay, solvePart, useSampleInput, alternateInput);
        SolveDay(5, new Day5Solver(), solveDay, solvePart, useSampleInput, alternateInput);
        SolveDay(6, new Day6Solver(), solveDay, solvePart, useSampleInput, alternateInput);
        SolveDay(7, new Day7Solver(), solveDay, solvePart, useSampleInput, alternateInput);
        SolveDay(8, new Day8Solver(), solveDay, solvePart, useSampleInput, alternateInput);
        SolveDay(9, new Day9Solver(), solveDay, solvePart, useSampleInput, alternateInput);
        SolveDay(11, new Day11Solver(), solveDay, solvePart, useSampleInput, alternateInput);

        var elapsedTime = FormatDuration(watch.Elapsed);

        Console.WriteLine($"Total time elapsed: {elapsedTime}");
    }
}
