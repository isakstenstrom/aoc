public class Day15Solver : ISolver
{
    public struct Lens(string label, int focalLength)
    {
        public string Label { get; } = label;
        public int FocalLength { get; } = focalLength;
    }

    static int CalculateHash(string input)
    {
        int currentValue = 0;
        foreach (char c in input)
        {
            currentValue += c;
            currentValue *= 17;
            currentValue %= 256;
        }
        return currentValue;
    }

    public string Part1(List<string> input)
    {
        return $"{input[0].Split(',').Select(CalculateHash).Sum()}";
    }

    public string Part2(List<string> input)
    {
        List<List<Lens>> boxes = [];
        for (int i = 0; i < 256; i++)
        {
            boxes.Add([]);
        }

        foreach (string lens in input[0].Split(','))
        {
            if (lens[^1] == '-')
            {
                string label = lens[..^1];
                int hash = CalculateHash(label);

                boxes[hash].RemoveAll(x => x.Label == label);
            }
            else
            {
                var splitStr = lens.Split('=');
                string label = splitStr[0];
                int hash = CalculateHash(label);
                int focalLength = int.Parse(splitStr[1]);
                int existingLensIndex = boxes[hash].FindIndex(x => x.Label == label);

                if (existingLensIndex != -1)
                {
                    boxes[hash][existingLensIndex] = new Lens(label, focalLength);
                }
                else
                {
                    boxes[hash].Add(new Lens(label, focalLength));
                }
            }
        }

        int sum = 0;
        for (int b = 0; b < boxes.Count; b++)
        {
            for (int l = 0; l < boxes[b].Count; l++)
            {
                sum += (b + 1) * (l + 1) * boxes[b][l].FocalLength;
            }
        }

        return $"{sum}";
    }
}
