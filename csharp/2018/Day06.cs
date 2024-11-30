namespace Advent_of_Code_2018;

[TestClass]
public class Day06
{
    record Coord(int id, Pos<int> pos);

    private static string Part1(IEnumerable<string> input)
    {
        var result = new StringBuilder();
        var points = new List<Coord>();
        var box = new Box<int>(0, 0);
        int idCount = 1;
        foreach (var line in input)
        {
            var parts = line.Split(", ");
            Pos<int> pos = new Pos<int>(int.Parse(parts[0]), int.Parse(parts[1]));
            points.Add(new Coord(idCount, pos));
            idCount++;

            box.IncreaseToPoint(pos);
        }

        var infinity = box.Width * box.Height;

        // increase the box in all directions so that we can detect infinite areas
        box.Min.x -= box.Width;
        box.Min.y -= box.Height;
        box.Max.x += box.Width;
        box.Max.y += box.Height;

        Console.WriteLine($"Box: {box}");

        var grid = new List<List<int>>();
        for (int y = box.Min.y; y <= box.Max.y; y++)
        {
            var row = new List<int>();
            for (int x = box.Min.x; x <= box.Max.x; x++)
            {
                var pos = new Pos<int>(x, y);
                var distances = new List<int>();

                // find the closest point
                foreach (var point in points)
                {
                    distances.Add(point.pos.Manhattan(pos));
                }
                var min = distances.Min();
                var closest = distances.IndexOf(min);
                if (distances.Count(d => d == min) > 1)
                {
                    row.Add(0);
                }
                else
                {
                    row.Add(points[closest].id);
                }
            }
        }
        // count the number of points in each area
        var areas = new Dictionary<int, int>();
        foreach (var point in points) {
            areas[point.id] = 0;
        }
        for (int y = box.Min.y; y <= box.Max.y; y++)
        {
            for (int x = box.Min.x; x <= box.Max.x; x++)
            {
                var id = grid[y - box.Min.y][x - box.Min.x];
                if (id != 0)
                {
                    areas[id]++;
                }
            }
        }
        // print the ten largest areas
        var sorted = areas.OrderByDescending(a => a.Value);
        foreach (var area in sorted.Take(10))
        {
            Console.WriteLine($"Area {area.Key}: {area.Value}");
        }

        return result.ToString();
    }
    
    private static string Part2(IEnumerable<string> input)
    {
        var result = new StringBuilder();
        foreach (var line in input)
        {
        }
        return result.ToString();
    }
    
    [TestMethod]
    public void Day06_Part1_Example01()
    {
        var input = """
            <TODO>
            """;
        var result = Part1(Common.GetLines(input));
        Assert.AreEqual("", result);
    }
    
    [TestMethod]
    public void Day06_Part1_Example02()
    {
        var input = """
            <TODO>
            """;
        var result = Part1(Common.GetLines(input));
        Assert.AreEqual("", result);
    }
    
    [TestMethod]
    public void Day06_Part1()
    {
        var result = Part1(Common.DayInput(nameof(Day06), "2018"));
        Assert.AreEqual("", result);
    }
    
    [TestMethod]
    public void Day06_Part2_Example01()
    {
        var input = """
            <TODO>
            """;
        var result = Part2(Common.GetLines(input));
        Assert.AreEqual("", result);
    }
    
    [TestMethod]
    public void Day06_Part2_Example02()
    {
        var input = """
            <TODO>
            """;
        var result = Part2(Common.GetLines(input));
        Assert.AreEqual("", result);
    }
    
    [TestMethod]
    public void Day06_Part2()
    {
        var result = Part2(Common.DayInput(nameof(Day06), "2018"));
        Assert.AreEqual("", result);
    }
    
}
