package colinlcrawford.adventofcode

object Day6 {
  type GroupAnswers = List[Set[Char]]

  def run(): Unit = {
    val input = parseInput(readInput())
    part1(input)
    part2(input)
  }

  def readInput(): Array[String] = Utils.readInputFile("day6-input.txt")

  def part1(input: List[GroupAnswers]): Unit = Utils.printAnswer(6, 1, Some(combineAnswersWith(input, (_ ++ _))))

  def part2(input: List[GroupAnswers]): Unit = Utils.printAnswer(6, 2, Some(combineAnswersWith(input, (_ & _))))

  def combineAnswersWith(answers: List[GroupAnswers], combiner: (Set[Char], Set[Char]) => Set[Char]): Int =
    answers.map(_.reduce(combiner)).map(_.size).sum

  def parseInput(input: Array[String]): List[GroupAnswers] = {
    input.foldRight(List[GroupAnswers]())({
      case ("", acc) => List[Set[Char]]() :: acc
      case (line, Nil) => List(line.toSet) :: Nil
      case (line, h :: t) => (line.toSet :: h) :: t
    })
  }
}

