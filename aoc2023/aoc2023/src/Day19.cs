using System.Text.RegularExpressions;

public class Day19Solver : ISolver
{
    struct Rule
    {
        public int Category { get; }
        public char Type { get; }
        public long Value { get; }
        public string Target { get; }

        public Rule(string s)
        {
            string[] splitStr = Regex.Split(s, @"([:<>])");

            if (splitStr.Count() == 1)
            {
                Category = 0;
                Type = '|';
                Value = 0;
                Target = splitStr[0];
            }
            else
            {
                Category = splitStr[0][0] switch
                {
                    'x' => 0,
                    'm' => 1,
                    'a' => 2,
                    's' => 3,
                    _ => throw new Exception("Invalid category"),
                };
                Type = splitStr[1][0];
                Value = long.Parse(splitStr[2]);
                Target = splitStr[4];
            }
        }

        public override string ToString() => $"({Category}, {Type}, {Value}, {Target})";
    }

    bool Filter(Dictionary<string, List<Rule>> workflows, long[] mp)
    {
        string currentWorkflow = "in";

        while (currentWorkflow != "A" && currentWorkflow != "R")
        {
            foreach (var rule in workflows[currentWorkflow])
            {
                if (rule.Type == '<' && mp[rule.Category] < rule.Value)
                {
                    currentWorkflow = rule.Target;
                    break;
                }
                if (rule.Type == '>' && mp[rule.Category] > rule.Value)
                {
                    currentWorkflow = rule.Target;
                    break;
                }
                if (rule.Type == '|')
                {
                    currentWorkflow = rule.Target;
                    break;
                }
            }
        }
        return currentWorkflow == "A";
    }

    long GetNumAccepted(Dictionary<string, List<Rule>> workflows, string workflow, LongPoint[] p)
    {
        string currentWorkflow = workflow;
        long numAccepted = 0;

        while (currentWorkflow != "A" && currentWorkflow != "R")
        {
            foreach (var rule in workflows[currentWorkflow])
            {
                if (rule.Type == '<')
                {
                    LongPoint? intersection = p[rule.Category].Intersection(new LongPoint(rule.Value, 4000));
                    (LongPoint? under, LongPoint? _) = p[rule.Category].Difference(new LongPoint(rule.Value, 4000));

                    if (under.HasValue)
                    {
                        LongPoint[] newP = (LongPoint[])p.Clone();
                        newP[rule.Category] = under.Value;
                        numAccepted += GetNumAccepted(workflows, rule.Target, newP);
                    }

                    if (!intersection.HasValue)
                    {
                        return numAccepted;
                    }
                    p[rule.Category] = intersection.Value;
                }
                if (rule.Type == '>')
                {

                    LongPoint? intersection = p[rule.Category].Intersection(new LongPoint(0, rule.Value));
                    (LongPoint? _, LongPoint? over) = p[rule.Category].Difference(new LongPoint(0, rule.Value));

                    if (over.HasValue)
                    {
                        LongPoint[] newP = (LongPoint[])p.Clone();
                        newP[rule.Category] = over.Value;
                        numAccepted += GetNumAccepted(workflows, rule.Target, newP);
                    }

                    if (!intersection.HasValue)
                    {
                        return numAccepted;
                    }
                    p[rule.Category] = intersection.Value;
                }
                if (rule.Type == '|')
                {
                    currentWorkflow = rule.Target;
                }
            }
        }

        if (currentWorkflow == "A")
        {
            numAccepted += (p[0].Y - p[0].X + 1)
                         * (p[1].Y - p[1].X + 1)
                         * (p[2].Y - p[2].X + 1)
                         * (p[3].Y - p[3].X + 1);
        }

        return numAccepted;
    }

    public string Part1(List<string> input)
    {
        Dictionary<string, List<Rule>> workflows = new();

        int index = 0;
        for (; index < input.Count; index++)
        {
            if (input[index] == "")
            {
                break;
            }

            var splitStr = input[index].Split(new char[] { '{', '}', ',' }, StringSplitOptions.RemoveEmptyEntries);
            workflows.Add(splitStr.First(), []);
            foreach (var rule in splitStr.Skip(1))
            {
                workflows.Last().Value.Add(new Rule(rule));
            }
        }

        index++;

        long sum = 0;
        for (; index < input.Count; index++)
        {
            long[] mp = Regex.Matches(input[index], @"(\d+)").Select(m => long.Parse(m.Value)).ToArray();
            if (Filter(workflows, mp))
            {
                sum += mp.Sum();
            }
        }
        return $"{sum}";
    }

    public string Part2(List<string> input)
    {
        Dictionary<string, List<Rule>> workflows = new();

        int index = 0;
        for (; index < input.Count; index++)
        {
            if (input[index] == "")
            {
                break;
            }

            var splitStr = input[index].Split(new char[] { '{', '}', ',' }, StringSplitOptions.RemoveEmptyEntries);
            workflows.Add(splitStr.First(), []);
            foreach (var rule in splitStr.Skip(1))
            {
                workflows.Last().Value.Add(new Rule(rule));
            }
        }


        return $"{GetNumAccepted(workflows, "in", [new(1, 4000), new(1, 4000), new(1, 4000), new(1, 4000)])}";
    }
}