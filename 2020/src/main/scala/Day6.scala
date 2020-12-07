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
    var groupAnswers = List[GroupAnswers]()
    var currentGroup = List[Set[Char]]()
    for (line <- input) {
      if (line == "") {
        groupAnswers = currentGroup :: groupAnswers
        currentGroup = List[Set[Char]]()
      } else {
        currentGroup = line.toSet :: currentGroup
      }
    }
    if (!currentGroup.isEmpty) groupAnswers = currentGroup :: groupAnswers
    groupAnswers
  }
}

