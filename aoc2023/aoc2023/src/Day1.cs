using System.Text.RegularExpressions;

public class Day1Solver : ISolver
{
    static int StringToInt(string input)
    {
        return input switch
        {
            "one" or "1" => 1,
            "two" or "2" => 2,
            "three" or "3" => 3,
            "four" or "4" => 4,
            "five" or "5" => 5,
            "six" or "6" => 6,
            "seven" or "7" => 7,
            "eight" or "8" => 8,
            "nine" or "9" => 9,
            _ => throw new Exception("Invalid string"),
        };
    }

    static string Solve(List<string> input, string pattern)
    {
        int sum = 0;
        foreach (var line in input)
        {
            int first = StringToInt(Regex.Match(line, pattern).Value);
            int last = StringToInt(Regex.Match(line, pattern, RegexOptions.RightToLeft).Value);

            sum += first * 10 + last;
        }
        return sum.ToString();
    }

    public string Part1(List<string> input)
    {
        return Solve(input, @"(\d)");
    }

    public string Part2(List<string> input)
    {
        return Solve(input, @"(\d|one|two|three|four|five|six|seven|eight|nine)");
    }
}
