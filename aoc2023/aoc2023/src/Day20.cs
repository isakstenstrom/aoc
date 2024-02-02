using System.Text.RegularExpressions;

public class Day20Solver : ISolver
{
    enum PulseType
    {
        LOW,
        HIGH,
    }

    struct Pulse(string from, string to, PulseType pulseType)
    {
        public string From { get; } = from;
        public string To { get; } = to;
        public PulseType PulseType { get; } = pulseType;

        public override string ToString() => $"{From} -{PulseType}-> {To}";
    }

    abstract class Module(string name, string[] recipients)
    {
        public readonly string Name = name;
        public readonly string[] Recipients = recipients;

        public abstract Pulse[] SendPulse(Pulse pulse);
    }

    class Broadcaster(string name, string[] recipients) : Module(name, recipients)
    {
        public override Pulse[] SendPulse(Pulse pulse)
        {
            return Recipients.Select(x => new Pulse(Name, x, pulse.PulseType)).ToArray();
        }
    }

    class FlipFlop(string name, string[] recipients) : Module(name, recipients)
    {
        private PulseType state = PulseType.LOW;
        public override Pulse[] SendPulse(Pulse pulse)
        {
            if (pulse.PulseType == PulseType.HIGH)
            {
                return [];
            }
            state = state == PulseType.LOW ? PulseType.HIGH : PulseType.LOW;

            return Recipients.Select(x => new Pulse(Name, x, state)).ToArray();
        }
    }

    class Conjunction(string name, string[] recipients) : Module(name, recipients)
    {
        public Dictionary<string, PulseType> senders = new();

        public void AddSender(string sender)
        {
            senders[sender] = PulseType.LOW;
        }

        public override Pulse[] SendPulse(Pulse pulse)
        {
            senders[pulse.From] = pulse.PulseType;

            PulseType pulseToSend = PulseType.HIGH;
            if (senders.All(x => x.Value == PulseType.HIGH))
            {
                pulseToSend = PulseType.LOW;
            }
            return Recipients.Select(x => new Pulse(Name, x, pulseToSend)).ToArray();
        }
    }

    static Dictionary<string, Module> ParseModules(List<string> input)
    {
        Dictionary<string, Module> modules = new();
        foreach (var line in input)
        {
            string name = Regex.Match(line, @"(?<=^%|^&|^)[a-z]+").Value;
            string[] recipients = Regex.Matches(line, @"(?<= )[a-z]+(?=,|$)").Select(x => x.Value).ToArray();

            modules[name] = line.First() switch
            {
                '%' => new FlipFlop(name, recipients),
                '&' => new Conjunction(name, recipients),
                _ => new Broadcaster(name, recipients),
            };

            foreach (var recipient in recipients)
            {
                modules.TryAdd(recipient, new Broadcaster(recipient, []));
            }
        }

        foreach (var module in modules.Values)
        {
            foreach (var recipient in module.Recipients)
            {
                if (modules[recipient] is Conjunction c)
                {
                    c.AddSender(module.Name);
                }
            }
        }
        return modules;
    }

    public string Part1(List<string> input)
    {
        Dictionary<string, Module> modules = ParseModules(input);

        long numLowPulses = 0;
        long numHighPulses = 0;
        for (int i = 0; i < 1000; i++)
        {
            Queue<Pulse> pulses = new();
            pulses.Enqueue(new Pulse("button", "broadcaster", PulseType.LOW));

            while (pulses.Count > 0)
            {
                var pulse = pulses.Dequeue();
                if (pulse.PulseType == PulseType.LOW)
                {
                    numLowPulses++;
                }
                else
                {
                    numHighPulses++;
                }

                var newPulses = modules[pulse.To].SendPulse(pulse);
                foreach (var newPulse in newPulses)
                {
                    pulses.Enqueue(newPulse);
                }
            }
        }

        return $"{numLowPulses * numHighPulses}";
    }

    public string Part2(List<string> input)
    {
        Dictionary<string, Module> modules = ParseModules(input);

        HashSet<string> modulesToTrack = new();
        foreach (var module in modules.Values)
        {
            if (module.Recipients.Contains("rx"))
            {
                modulesToTrack.UnionWith(((Conjunction)modules[module.Name]).senders.Keys);
            }
        }
        List<long> lcmList = [];

        for (int numButtonPresses = 1; true; numButtonPresses++)
        {
            Queue<Pulse> pulses = new();
            pulses.Enqueue(new Pulse("button", "broadcaster", PulseType.LOW));

            while (pulses.Count > 0)
            {
                var pulse = pulses.Dequeue();
                if (pulse.PulseType == PulseType.HIGH && modulesToTrack.Contains(pulse.From))
                {
                    modulesToTrack.Remove(pulse.From);
                    lcmList.Add(numButtonPresses);
                    if (modulesToTrack.Count == 0)
                    {
                        return $"{MathUtils.LCM(lcmList)}";
                    }
                }

                var newPulses = modules[pulse.To].SendPulse(pulse);
                foreach (var newPulse in newPulses)
                {
                    pulses.Enqueue(newPulse);
                }
            }
        }
    }
}