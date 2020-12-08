package colinlcrawford.adventofcode

object Day6 {
  type GroupAnswers = List[Set[Char]]

  def run(): Unit = {
    val input = parseInput(readInput())
    part1(input)
    part2(input)
  }

  def readInput(): Array[String] = {
    Utils.readInputFile("day6-input.txt")
  }

  def part1(input: List[GroupAnswers]): Unit = {
    val ans = input.map(_.reduce(_ ++ _)).map(_.size).sum
    Utils.printAnswer(6, 1, Some(ans))
  }

  def part2(input: List[GroupAnswers]): Unit = {
    val ans = input.map(_.reduce(_ & _)).map(_.size).sum
    Utils.printAnswer(6, 2, Some(ans))
  }

  def parseInput(input: Array[String]): List[GroupAnswers] = {
    input.foldRight(List[GroupAnswers]())({
      case ("", acc) => List[Set[Char]]() :: acc
      case (line, Nil) => List(line.toSet) :: Nil
      case (line, h :: t) => (line.toSet :: h) :: t
    })
  }
}

