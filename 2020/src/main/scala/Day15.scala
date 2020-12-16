package colinlcrawford.adventofcode.day15

import colinlcrawford.adventofcode.{AOCDay}

object Day15 extends AOCDay {
  def dayNum(): Int = 15

  def part1(input: Array[String]): Option[String] = { 
    val ans = playGameUntilTurn(input(0).split(",").map(_.toInt).zipWithIndex, 2020)
    Some(ans.toString)
  }

  def part2(input: Array[String]): Option[String] = {
    val ans = playGameUntilTurn(input(0).split(",").map(_.toInt).zipWithIndex, 30000000)
    Some(ans.toString)
  }

  def playGameUntilTurn(inputs: Array[(Int, Int)], turnsToPlay: Int): Int = {
    var seen: Map[Int, List[Int]] = inputs.map({ case (num, turn) => (num -> List(turn + 1)) }).toMap
    var currentTurn = seen.size + 1
    var prevNum = inputs.last._1

    while (currentTurn <= turnsToPlay) {
      val turnsSaid = seen(prevNum)
      prevNum = if (turnsSaid.size > 1) turnsSaid.head - turnsSaid.tail.head else 0
      seen += (if (seen.contains(prevNum)) (prevNum -> List(currentTurn, seen(prevNum).head)) else (prevNum -> List(currentTurn)))
      currentTurn += 1
    }

    prevNum
  }
}
