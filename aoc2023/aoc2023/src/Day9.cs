public class Day9Solver : ISolver
{
    public int Solve(List<string> input, Func<List<int>, int> saveNumberFunc, Func<int, int, int> reducerFunc)
    {
        int sum = 0;
        foreach (string line in input)
        {
            Stack<int> interestingNumbers = new();

            List<int> currentLine = line.Split(" ").Select(int.Parse).ToList();

            while (currentLine.Any(x => x != 0))
            {
                interestingNumbers.Push(saveNumberFunc(currentLine));

                List<int> newLine = new();

                for (int i = 1; i < currentLine.Count; i++)
                {
                    newLine.Add(currentLine[i] - currentLine[i - 1]);
                }
                currentLine = newLine;
            }

            sum += interestingNumbers.Aggregate(0, reducerFunc);
        }

        return sum;
    }

    public string Part1(List<string> input)
    {
        return $"{Solve(input, l => l.Last(), (acc, x) => x + acc)}";
    }

    public string Part2(List<string> input)
    {
        return $"{Solve(input, l => l.First(), (acc, x) => x - acc)}";
    }
}
