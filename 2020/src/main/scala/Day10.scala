package colinlcrawford.adventofcode

object Day10 extends AOCDay {
  def dayNum(): Int = 10

  def part1(input: Array[String]): Option[String] = {
    // start with 3 having 1 b/c of the built in laptop adapter at the end
    var diffs = Map[Int, Int](3 -> 1)
    input.map(_.toInt).sorted.scan(0)((prev, next) => {
      val diff = next - prev
      diffs += (diff -> (diffs.getOrElse(diff, 0) + 1))
      next
    })
    Some(s"${diffs.getOrElse(1, 0) * diffs.getOrElse(3, 0)}")
  }

  var cache = Map[Int, Long]()
  def part2(input: Array[String]): Option[String] = {
    val inputs = input.map(_.toInt).sorted
    val terminalAdapter = inputs.last
    // memoize partial answers to speed things up
    cache += (terminalAdapter -> 1)
    Some(s"${calcCombos(0, inputs.toSet)}")
  }

  def calcCombos(prev: Int, adapters: Set[Int]): Long = {
    if (cache.contains(prev)) return cache(prev)
    val nextPossible = (Set(prev + 1, prev + 2, prev + 3) & adapters)
    val ans = nextPossible.toList.map(x => calcCombos(x, adapters)).sum
    cache += (prev -> ans)
    ans
  }
}
