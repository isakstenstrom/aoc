public interface ISolver
{
    string Part1(List<string> input);
    string Part2(List<string> input);
}

public class Runner
{
    public static List<string> GetPuzzleInput(int day, int part, bool useSampleInput)
    {
        string fileName = String.Format(
            "aoc2023/res/{0}/day{1}_part{2}.txt",
            useSampleInput ? "sample" : "input",
            day,
            part);

        if (!File.Exists(fileName))
        {
            fileName = String.Format(
                "aoc2023/res/{0}/day{1}.txt",
                useSampleInput ? "sample" : "input",
                day);
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

    public static void SolveDay(int day, ISolver solver, int solveDay, int solvePart, bool useSampleInput)
    {
        if (day != solveDay && solveDay != -1)
        {
            return;
        }

        List<string> input;

        if (solvePart == 1 || solvePart == -1)
        {
            input = GetPuzzleInput(day, 1, useSampleInput);
            var watch = System.Diagnostics.Stopwatch.StartNew();

            string result = solver.Part1(input);

            watch.Stop();
            var elapsedTime = FormatDuration(watch.Elapsed);
            Console.WriteLine($"Day {day,2}: part 1: {result,16}, in {elapsedTime}");
        }

        if (solvePart == 2 || solvePart == -1)
        {
            input = GetPuzzleInput(day, 2, useSampleInput);
            var watch = System.Diagnostics.Stopwatch.StartNew();

            string result = solver.Part2(input);

            watch.Stop();
            var elapsedTime = FormatDuration(watch.Elapsed);
            Console.WriteLine($"Day {day,2}: part 2: {result,16}, in {elapsedTime}");
        }
    }

    public static void Solve(int solveDay, int solvePart, bool useSampleInput)
    {
        var watch = System.Diagnostics.Stopwatch.StartNew();

        SolveDay(1, new Day1Solver(), solveDay, solvePart, useSampleInput);
        SolveDay(2, new Day2Solver(), solveDay, solvePart, useSampleInput);
        SolveDay(3, new Day3Solver(), solveDay, solvePart, useSampleInput);
        SolveDay(4, new Day4Solver(), solveDay, solvePart, useSampleInput);
        SolveDay(5, new Day5Solver(), solveDay, solvePart, useSampleInput);
        SolveDay(6, new Day6Solver(), solveDay, solvePart, useSampleInput);

        var elapsedTime = FormatDuration(watch.Elapsed);

        Console.WriteLine($"Total time elapsed: {elapsedTime}");
    }
}
