package colinlcrawford.adventofcode.day5

import colinlcrawford.adventofcode.{AOCDay}

case class Seat(row: Int, col: Int)

sealed trait Direction
case object Upper extends Direction
case object Lower extends Direction

object Day5 extends AOCDay {
  def dayNum(): Int = 5

  def part1(input: Array[String]): Option[String] = {
    val ans = seatScores(input).max
    Some(s"$ans")
  }

  def part2(input: Array[String]): Option[String] = {
    val sortedScores = seatScores(input).sorted
    var prev = sortedScores(0) - 1
    val ans = sortedScores.find(score => {
      val check = score != (prev + 1)
      prev = score
      check
    })
    ans.map(_ - 1).map(_.toString)
  }

  def seatScores(input: Array[String]): Array[Int] = input.map(getSeat).map(getScore)

  def getSeat(binSpec: String): Seat = Seat(getRow(binSpec.take(7)), getCol(binSpec.takeRight(3)))

  def getRow(rowSpec: String): Int = getFromSpecRange(rowSpec, 0, 127)

  def getCol(colSpec: String): Int = getFromSpecRange(colSpec, 0, 7)

  def getFromSpecRange(spec: String, min: Int, max: Int): Int = {
    var minn = min
    var maxx = max
    for (letter <- spec) {
      val incr = (((maxx - minn) + 1) / 2)
      getDirection(letter) match {
        case Upper => minn += incr
        case Lower => maxx -= incr
      }
    }
    maxx
  }

  def getScore(seat: Seat) = (seat.row * 8) + seat.col

  def getDirection(c: Char): Direction = {
    if (c == 'F' || c == 'L') return Lower
    if (c == 'B' || c == 'R') return Upper
    throw new Exception("Bad char")
  }
}

