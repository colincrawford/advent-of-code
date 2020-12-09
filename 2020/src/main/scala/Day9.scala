package colinlcrawford.adventofcode

object Day9 extends AOCDay {
  def dayNum(): Int = 9

  def part1(input: Array[String]): Option[String] = {
    val inputs = input.map(_.toLong)
    var seen = inputs.take(25).toSet
    val prev = inputs(0)
    inputs.sliding(26, 1)
      .find(group => {
        val result = group.find(next => seen.contains(group.last - next)).isEmpty
        seen = seen - prev + group.last
        result
      })
      .map(_.last.toString)
  }

  def part2(input: Array[String]): Option[String] = {
    val inputs = input.map(_.toLong)
    val targetNum = part1(input).get.toLong
    var start = 0
    var end = start + 2
    while (start < inputs.size - 1 && end < inputs.length) {
      val range = inputs.slice(start, end)
      val sum = range.sum
      if (sum == targetNum) return Some(s"${range.min + range.max}")
      else if (sum < targetNum) end += 1
      else if (sum > targetNum) start += 1
    }
    None
 }
}
