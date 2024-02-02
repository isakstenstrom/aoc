public struct Hand : IComparable
{
    public Hand(string cards, int handType, int bid)
    {
        Cards = cards;
        HandType = handType;
        Bid = bid;
    }

    public string Cards { get; }
    public int HandType { get; }
    public int Bid { get; }

    public int CompareTo(object? obj)
    {
        if (obj is null) return 1;

        Hand? other = obj as Hand?;
        if (other is not null)
        {
            if (HandType == other.Value.HandType)
            {
                return Cards.CompareTo(other.Value.Cards);
            }
            return HandType.CompareTo(other.Value.HandType);
        }
        else
        {
            throw new ArgumentException("Object is not a Temperature");
        }
    }
}

public class Day7Solver : ISolver
{
    public string Part1(List<string> input)
    {
        List<Hand> hands = new();

        foreach (var line in input)
        {
            var splitStr = line.Split(" ");
            string cards = splitStr[0];
            int bet = int.Parse(splitStr[1]);

            var charCounts = cards.GroupBy(x => x)
                                  .OrderByDescending(x => x.Count())
                                  .Select(x => x.Count());

            int handType = (charCounts.FirstOrDefault(), charCounts.Skip(1).FirstOrDefault()) switch
            {
                (5, 0) => 6,
                (4, 1) => 5,
                (3, 2) => 4,
                (3, 1) => 3,
                (2, 2) => 2,
                (2, 1) => 1,
                _ => 0,
            };

            cards = cards.Replace("A", "e")
                         .Replace("K", "d")
                         .Replace("Q", "c")
                         .Replace("J", "b")
                         .Replace("T", "a");

            hands.Add(new Hand(cards, handType, bet));
        }

        hands.Sort();

        int sum = 0;
        for (int i = 0; i < hands.Count(); i++)
        {
            sum += hands[i].Bid * (i + 1);
        }
        return $"{sum}";
    }

    public string Part2(List<string> input)
    {
        List<Hand> hands = new();

        foreach (var line in input)
        {
            var splitStr = line.Split(" ");
            string cards = splitStr[0];
            int bet = int.Parse(splitStr[1]);

            var charCounts = cards.Where(c => c != 'J')
                                  .GroupBy(x => x)
                                  .OrderByDescending(x => x.Count())
                                  .Select(x => x.Count());

            int numJs = cards.Where(c => c == 'J').Count();

            int handType = (charCounts.FirstOrDefault() + numJs, charCounts.Skip(1).FirstOrDefault()) switch
            {
                (5, 0) => 6,
                (4, 1) => 5,
                (3, 2) => 4,
                (3, 1) => 3,
                (2, 2) => 2,
                (2, 1) => 1,
                _ => 0,
            };

            cards = cards.Replace("A", "e")
                         .Replace("K", "d")
                         .Replace("Q", "c")
                         .Replace("J", "1")
                         .Replace("T", "a");

            hands.Add(new Hand(cards, handType, bet));
        }

        hands.Sort();

        int sum = 0;
        for (int i = 0; i < hands.Count(); i++)
        {
            sum += hands[i].Bid * (i + 1);
        }
        return $"{sum}";
    }
}
