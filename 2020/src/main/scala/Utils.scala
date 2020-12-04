package colinlcrawford.adventofcode

import scala.io.Source

object Utils {
  def printAnswer[T](day: Int, problem: Int, answer: Option[T]) = {
    val prefix = f"Day $day (problem $problem):"
    answer match {
      case None => println(s"$prefix No answer found!")
      case Some(ans) => println(s"$prefix $ans")
    }
  }

  def readInputFile(fileName: String): Array[String] = {
    val input = Source.fromResource(fileName)
    val inputs = input.getLines().toArray
    input.close()
    inputs
  }
}
