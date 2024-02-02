public class Day4Solver : ISolver
{
    int CalculateCardWinnings(string input)
    {
        HashSet<int> winningNumbers = input.Split("|")[0].Split(" ", StringSplitOptions.RemoveEmptyEntries).Skip(2).Select(Int32.Parse).ToHashSet();
        var myNumbers = input.Split("|")[1].Split(" ", StringSplitOptions.RemoveEmptyEntries).Select(Int32.Parse);

        int numMatches = 0;
        foreach (var num in myNumbers)
        {
            if (winningNumbers.Contains(num))
            {
                numMatches += 1;
            }
        }
        return numMatches;
    }

    public string Part1(List<string> input)
    {
        int sum = 0;
        foreach (string line in input)
        {
            int numMatches = CalculateCardWinnings(line);

            if (numMatches != 0)
            {
                sum += 1 << (numMatches - 1);
            }
        }
        return $"{sum}";
    }

    public string Part2(List<string> input)
    {
        List<int> cardList = Enumerable.Repeat(1, input.Count()).ToList();

        for (int lineId = 0; lineId < input.Count; lineId++)
        {
            int numMatches = CalculateCardWinnings(input[lineId]);

            for (int i = lineId + 1; i <= lineId + numMatches; i++)
            {
                cardList[i] += cardList[lineId];
            }
        }
        return $"{cardList.Sum()}";
    }
}
