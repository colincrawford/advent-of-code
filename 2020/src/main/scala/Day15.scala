package colinlcrawford.adventofcode.day15

import colinlcrawford.adventofcode.{AOCDay}

object Day15 extends AOCDay {
  def dayNum(): Int = 15

  def part1(input: Array[String]): Option[String] = { 
    val ans = playGameUntilTurn(input(0).split(",").map(_.toInt).zipWithIndex, 2020, 100000)
    Some(ans.toString)
  }

  def part2(input: Array[String]): Option[String] = {
    val ans = playGameUntilTurn(input(0).split(",").map(_.toInt).zipWithIndex, 30000000, 30000000)
    Some(ans.toString)
  }

  def playGameUntilTurn(inputs: Array[(Int, Int)], turnsToPlay: Int, answerArrSize: Int): Int = {
    var seen: Array[Array[Int]] = Array.fill(answerArrSize){ Array(-1, -1) }
    inputs.foreach({ case (num, turn) => seen(num)(0) = turn + 1 })
    var currentTurn = inputs.size + 1
    var prevNum = inputs.last._1

    while (currentTurn <= turnsToPlay) {
      val turnsSaid = seen(prevNum)
      prevNum = if (turnsSaid(1) != -1) turnsSaid(0) - turnsSaid(1) else 0
      val prevSeen = seen(prevNum)
      if (prevSeen(0) != -1) {
        seen(prevNum)(1) = seen(prevNum)(0)
      }
      seen(prevNum)(0) = currentTurn
      currentTurn += 1
    }

    prevNum
  }
}
