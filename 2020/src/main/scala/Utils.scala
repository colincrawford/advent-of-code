package colinlcrawford.adventofcode

object Utils {
  def printAnswer[T](day: Int, problem: Int, answer: Option[T]) = {
    val prefix = f"Day $day (problem $problem):"
    answer match {
      case None => println(s"$prefix No answer found!")
      case Some(ans) => println(s"$prefix $ans")
    }
  }
}
