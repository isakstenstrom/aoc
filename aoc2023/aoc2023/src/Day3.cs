using System.Drawing;
using System.Text.RegularExpressions;

public class Day3Solver : ISolver
{
    public string Part1(List<string> input)
    {
        List<Point> symbols = new List<Point>();

        for (int lineNum = 0; lineNum < input.Count(); lineNum++)
        {
            foreach (Match match in Regex.Matches(input[lineNum], @"[^\d.]"))
            {
                symbols.Add(new Point(match.Index, lineNum));
            }
        }

        int sum = 0;

        for (int lineNum = 0; lineNum < input.Count(); lineNum++)
        {
            foreach (Match match in Regex.Matches(input[lineNum], @"\d+"))
            {
                Rectangle rect = new Rectangle(match.Index, lineNum, match.Length, 1);
                rect.Inflate(1, 1);

                foreach (Point s in symbols)
                {
                    if (rect.Contains(s))
                    {
                        sum += int.Parse(match.Value);
                        break;
                    }
                }
            }
        }
        return $"{sum}";
    }

    public string Part2(List<string> input)
    {
        List<(Point, List<int>)> gears = new List<(Point, List<int>)>();

        for (int lineNum = 0; lineNum < input.Count(); lineNum++)
        {
            foreach (Match match in Regex.Matches(input[lineNum], @"[^\d.]"))
            {
                gears.Add((new Point(match.Index, lineNum), new List<int>()));
            }
        }

        for (int lineNum = 0; lineNum < input.Count(); lineNum++)
        {
            foreach (Match match in Regex.Matches(input[lineNum], @"\d+"))
            {
                Rectangle rect = new Rectangle(match.Index, lineNum, match.Length, 1);
                rect.Inflate(1, 1);

                foreach ((Point, List<int>) g in gears)
                {
                    if (rect.Contains(g.Item1))
                    {
                        g.Item2.Add(int.Parse(match.Value));
                    }
                }
            }
        }

        int sum = 0;
        foreach ((Point, List<int>) g in gears)
        {
            if (g.Item2.Count == 2)
            {
                sum += g.Item2[0] * g.Item2[1];
            }
        }

        return $"{sum}";
    }
}
