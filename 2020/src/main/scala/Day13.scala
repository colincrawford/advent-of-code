package colinlcrawford.adventofcode.day13

import colinlcrawford.adventofcode.{AOCDay}

object Day13 extends AOCDay {
  def dayNum(): Int = 13

  def part1(input: Array[String]): Option[String] = { 
    val (startingTime, busIds) = parse(input)
    val (busId, time) = busIds.map(_._1)
      .map(busId => (busId, findClosestTime(startingTime, busId)))
      .reduceLeft((min, next) => if (next._2 < min._2) next else min)
    val ans = (time - startingTime) * busId
    Some(ans.toString)
  }

  def part2(input: Array[String]): Option[String] = {
    val (_, busIds) = parse(input)
    var time: Long = 0
    var step: Long = 1
    for ((bId, inx) <- busIds) {
      while (((time + inx) % bId) != 0) time += step
      step *= bId
    }
    Some(time.toString)
  }

  def parse(input: Array[String]): (Int, Array[(Int, Int)]) = {
    val startTime = input(0).toInt
    val busIds = input(1).split(",").zipWithIndex.filter(_._1 != "x").map({ case (bId, inx) => (bId.toInt, inx) })
    (startTime, busIds)
  }

  def findClosestTime(startingTime: Int, busId: Int): Int = {
    val (quotient, remainder) = (startingTime / busId, startingTime % busId)
    if (remainder == 0) startingTime else busId * (quotient + 1)
  }
}
