using System.Text.RegularExpressions;

public class Day2Solver : ISolver
{
    (int maxRed, int maxGreen, int maxBlue) MaxColors(string input)
    {
        int maxRed = 0;
        int maxGreen = 0;
        int maxBlue = 0;

        foreach (Match match in Regex.Matches(input, @"\d+ (red|green|blue)"))
        {
            string[] splitStr = match.Value.Split(" ");
            int numCubes = int.Parse(splitStr[0]);
            string cubeColor = splitStr[1];

            switch (cubeColor)
            {
                case "red":
                    maxRed = Math.Max(maxRed, numCubes);
                    break;
                case "green":
                    maxGreen = Math.Max(maxGreen, numCubes);
                    break;
                case "blue":
                    maxBlue = Math.Max(maxBlue, numCubes);
                    break;
            }
        }
        return (maxRed, maxGreen, maxBlue);
    }

    public string Part1(List<string> input)
    {
        int sum = 0;
        int lineNum = 1;
        foreach (string line in input)
        {
            (int maxRed, int maxGreen, int maxBlue) = MaxColors(line);

            if (maxRed <= 12 && maxGreen <= 13 && maxBlue <= 14)
            {
                sum += lineNum;
            }

            lineNum++;
        }
        return $"{sum}";
    }

    public string Part2(List<string> input)
    {
        int sum = 0;
        foreach (string line in input)
        {
            (int maxRed, int maxGreen, int maxBlue) = MaxColors(line);
            sum += maxRed * maxGreen * maxBlue;
        }
        return $"{sum}";
    }
}
