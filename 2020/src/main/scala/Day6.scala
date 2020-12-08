package colinlcrawford.adventofcode

object Day6 extends AOCDay {
  type GroupAnswers = List[Set[Char]]
  type GroupAnswerCombiner = (Set[Char], Set[Char]) => Set[Char]

  def dayNum(): Int = 6

  def part1(input: Array[String]): Option[String] = {
    val ans = combineAnswersWith(parseInput(input), (_ ++ _))
    Some(s"$ans")
  }

  def part2(input: Array[String]): Option[String] = {
    val ans = combineAnswersWith(parseInput(input), (_ & _))
    Some(s"$ans")
  }

  def combineAnswersWith(answers: List[GroupAnswers], combiner: GroupAnswerCombiner): Int = {
    answers.map(_.reduce(combiner)).map(_.size).sum
  }

  def parseInput(input: Array[String]): List[GroupAnswers] = {
    input.foldRight(List[GroupAnswers]())({
      case ("", acc) => List[Set[Char]]() :: acc
      case (line, Nil) => List(line.toSet) :: Nil
      case (line, h :: t) => (line.toSet :: h) :: t
    })
  }
}

