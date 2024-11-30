namespace Advent_of_Code_2018;

[TestClass]
public class Day01
{
    private static string Part1(IEnumerable<string> input)
    {
        var result = 0;
        foreach (var line in input)
        {
            result += int.Parse(line);
        }
        return result.ToString();
    }
    
    private static string Part2(IEnumerable<string> input)
    {
        var seen = new HashSet<int>();
        var frequency = 0;
        seen.Add(frequency);
        while (true)
        {
            foreach (var line in input)
            {
                var value = int.Parse(line);
                frequency += value;
                if (seen.Contains(frequency))
                {
                    return frequency.ToString();
                }
                seen.Add(frequency);
            }
        }
        throw new Exception("No duplicate found");
    }
    
    [TestMethod]
    public void Day01_Part1()
    {
        var result = Part1(Common.DayInput(nameof(Day01), "2018"));
        Assert.AreEqual("518", result);
    }
    
    [TestMethod]
    public void Day01_Part2()
    {
        var result = Part2(Common.DayInput(nameof(Day01), "2018"));
        Assert.AreEqual("72889", result);
    }
    
}
