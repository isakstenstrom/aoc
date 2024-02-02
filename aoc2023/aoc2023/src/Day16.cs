public class Day16Solver : ISolver
{
    enum TileType
    {
        Empty,
        HorizontalSplitter,
        VerticalSplitter,
        Slash,
        Backslash,
    }

    struct Tile(char c)
    {
        public TileType Type = c switch
        {
            '.' => TileType.Empty,
            '-' => TileType.HorizontalSplitter,
            '|' => TileType.VerticalSplitter,
            '/' => TileType.Slash,
            '\\' => TileType.Backslash,
            _ => throw new Exception($"Unknown tile type: {c}"),
        };

        public bool[] Energized = [false, false, false, false];
    }

    private static List<List<Tile>> ParseInput(List<string> input)
    {
        List<List<Tile>> tiles = [];
        foreach (string line in input)
        {
            tiles.Add([]);
            foreach (char c in line)
            {
                tiles.Last().Add(new Tile(c));
            }
        }
        return tiles;
    }

    private static int SimulateGrid(List<List<Tile>> tiles, LongPoint startPos, Direction startDir)
    {
        foreach (List<Tile> row in tiles)
        {
            foreach (Tile tile in row)
            {
                for (int i = 0; i < 4; i++)
                {
                    tile.Energized[i] = false;
                }
            }
        }

        Queue<(LongPoint, Direction)> lightBeams = new();
        lightBeams.Enqueue((startPos, startDir));

        while (lightBeams.Count > 0)
        {
            (LongPoint pos, Direction dir) = lightBeams.Dequeue();
            if (pos.X < 0 || tiles[0].Count <= pos.X || pos.Y < 0 || tiles.Count <= pos.Y)
            {
                continue;
            }
            Tile tile = tiles[(int)pos.Y][(int)pos.X];

            if (tile.Energized[(int)dir])
            {
                continue;
            }
            tile.Energized[(int)dir] = true;

            switch (tile.Type)
            {
                case TileType.Empty:
                    lightBeams.Enqueue((pos.Step(dir), dir));
                    break;
                case TileType.HorizontalSplitter:
                    lightBeams.Enqueue((pos.Step(Direction.East), Direction.East));
                    lightBeams.Enqueue((pos.Step(Direction.West), Direction.West));
                    break;
                case TileType.VerticalSplitter:
                    lightBeams.Enqueue((pos.Step(Direction.North), Direction.North));
                    lightBeams.Enqueue((pos.Step(Direction.South), Direction.South));
                    break;
                case TileType.Slash:
                    {
                        Direction newDir = dir switch
                        {
                            Direction.North => Direction.East,
                            Direction.East => Direction.North,
                            Direction.South => Direction.West,
                            Direction.West => Direction.South,
                            _ => throw new Exception($"Invalid direction: {dir}"),
                        };
                        lightBeams.Enqueue((pos.Step(newDir), newDir));
                    }
                    break;
                case TileType.Backslash:
                    {
                        Direction newDir = dir switch
                        {
                            Direction.North => Direction.West,
                            Direction.West => Direction.North,
                            Direction.South => Direction.East,
                            Direction.East => Direction.South,
                            _ => throw new Exception($"Invalid direction: {dir}"),
                        };
                        lightBeams.Enqueue((pos.Step(newDir), newDir));
                    }
                    break;
                default:
                    throw new Exception($"Unknown tile type: {tile.Type}");
            }
        }

        int sum = 0;
        foreach (List<Tile> row in tiles)
        {
            foreach (Tile tile in row)
            {
                if (tile.Energized.Any(energized => energized))
                {
                    sum++;
                }
            }
        }

        return sum;
    }


    public string Part1(List<string> input)
    {
        List<List<Tile>> tiles = ParseInput(input);

        return $"{SimulateGrid(tiles, new LongPoint(0, 0), Direction.East)}";
    }

    public string Part2(List<string> input)
    {
        List<List<Tile>> tiles = ParseInput(input);

        int max = 0;
        for (int x = 0; x < tiles[0].Count; x++)
        {
            max = Math.Max(max, SimulateGrid(tiles, new LongPoint(x, 0), Direction.South));
        }
        for (int y = 0; y < tiles.Count; y++)
        {
            max = Math.Max(max, SimulateGrid(tiles, new LongPoint(0, y), Direction.East));
        }
        for (int x = 0; x < tiles[0].Count; x++)
        {
            max = Math.Max(max, SimulateGrid(tiles, new LongPoint(x, tiles.Count - 1), Direction.North));
        }
        for (int y = 0; y < tiles.Count; y++)
        {
            max = Math.Max(max, SimulateGrid(tiles, new LongPoint(tiles[0].Count - 1, y), Direction.West));
        }

        return $"{max}";
    }
}
