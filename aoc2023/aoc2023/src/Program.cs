using System.CommandLine;

public class Program
{
    static void Main(string[] args)
    {
        var dayOption = new Option<int>(
            aliases: ["-d", "--day"],
            description: "Specify the day to run. Defaults to all days.",
            isDefault: true,
            parseArgument: result =>
            {
                int value = -1;
                if (result.Tokens.Count == 0)
                {
                    return value;
                }

                try
                {
                    value = int.Parse(result.Tokens.Single().Value);
                    if (value < 1 || 25 < value)
                    {
                        throw new FormatException();
                    }
                }
                catch (FormatException)
                {
                    result.ErrorMessage = $"Invalid day: \"{result.Tokens.Single().Value}\". Must be an integer between 1 and 25 (inclusive).";
                }
                return value;
            }
        );

        var partOption = new Option<int>(
            aliases: ["-p", "--part"],
            description: "Specify the part to run. Defaults to all parts.",
            isDefault: true,
            parseArgument: result =>
            {
                int value = -1;
                if (result.Tokens.Count == 0)
                {
                    return value;
                }

                try
                {
                    value = int.Parse(result.Tokens.Single().Value);
                    if (value != 1 && value != 2)
                    {
                        throw new FormatException();
                    }
                }
                catch (FormatException)
                {
                    result.ErrorMessage = $"Invalid part: \"{result.Tokens.Single().Value}\". Must be either 1 or 2";
                }
                return value;
            }
        );

        var useSampleInputOption = new Option<bool>(
            aliases: ["-s", "--sample"],
            description: "Use the sample input instead of the real input.",
            getDefaultValue: () => false
        );

        var useAlternateInputOption = new Option<int?>(
            aliases: ["-a", "--alternateInput"],
            description: "Specify alternate input to use.",
            isDefault: true,
            parseArgument: result =>
            {
                int? value = null;
                if (result.Tokens.Count == 0)
                {
                    return value;
                }

                try
                {
                    value = int.Parse(result.Tokens.Single().Value);
                    if (value < 0)
                    {
                        throw new FormatException();
                    }
                }
                catch (FormatException)
                {
                    result.ErrorMessage = $"Invalid input: \"{result.Tokens.Single().Value}\". Must an positive integer";
                }
                return value;
            }
        );

        var rootCommand = new RootCommand("Solutions to Advent of Code 2023");
        rootCommand.AddGlobalOption(dayOption);
        rootCommand.AddGlobalOption(partOption);
        rootCommand.AddGlobalOption(useSampleInputOption);
        rootCommand.AddGlobalOption(useAlternateInputOption);

        rootCommand.SetHandler(Runner.Solve, dayOption, partOption, useSampleInputOption, useAlternateInputOption);

        rootCommand.Invoke(args);
    }
}
