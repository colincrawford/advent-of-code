package colinlcrawford.adventofcode

import scala.io.Source

trait AOCDay {
  def dayNum(): Int

  def part1(input: Array[String]): Option[String]

  def part2(input: Array[String]): Option[String]

  def run(): Unit = {
    val day = dayNum()
    val input = readInput(day)
    printAnswer(day, 1, part1(input))
    printAnswer(day, 2, part2(input))
  }

  def readInput(day: Int): Array[String] = {
    val input = Source.fromResource(s"day$day-input.txt")
    val inputs = input.getLines().toArray
    input.close()
    inputs
  }

  def printAnswer(day: Int, problem: Int, answer: Option[String]) = {
    val prefix = f"Day $day (problem $problem):"
    answer match {
      case None => println(s"$prefix No answer found!")
      case Some(ans) => println(s"$prefix $ans")
    }
  }
}
