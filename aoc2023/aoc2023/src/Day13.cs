public class Day13Solver : ISolver
{
    static bool IsReflection(List<uint> line, bool hasSmudge)
    {
        // Only reflections of even length are valid
        if (line.Count % 2 == 1)
        {
            return false;
        }

        bool foundSmudge = false;

        int start = 0;
        int end = line.Count - 1;
        while (start < end)
        {
            if (line[start] != line[end])
            {
                if (!hasSmudge) return false;
                if (foundSmudge) return false;
                if (uint.PopCount(line[start] ^ line[end]) != 1) return false;
                foundSmudge = true;
            }

            start++;
            end--;
        }

        if (hasSmudge)
        {
            return foundSmudge;
        }
        return true;
    }

    static int CalculateReflection(List<uint> line, bool hasSmudge)
    {
        for (int i = 1; i < line.Count - 1; i++)
        {
            if (IsReflection(line[..(i + 1)], hasSmudge))
            {
                return (0 + i + 1) / 2;
            }
            if (IsReflection(line[i..], hasSmudge))
            {
                return (i + line.Count) / 2;
            }
        }

        return 0;
    }

    static int Solve(List<string> input, bool hasSmudge)
    {
        int sum = 0;
        foreach (var patternStr in string.Join('\n', input.ToArray()).Split("\n\n"))
        {
            var lines = patternStr.Split('\n').ToArray();

            List<uint> horizontalLines = new uint[lines[0].Length].ToList();
            List<uint> verticalLines = new uint[lines.Length].ToList();
            for (int y = 0; y < lines.Length; y++)
            {
                for (int x = 0; x < lines[0].Length; x++)
                {
                    if (lines[y][x] == '#')
                    {
                        horizontalLines[x] |= (uint)1 << y;
                        verticalLines[y] |= (uint)1 << x;
                    }
                }
            }
            sum += CalculateReflection(horizontalLines, hasSmudge);
            sum += 100 * CalculateReflection(verticalLines, hasSmudge);
        }
        return sum;
    }

    public string Part1(List<string> input)
    {
        return $"{Solve(input, false)}";
    }

    public string Part2(List<string> input)
    {
        return $"{Solve(input, true)}";
    }
}
