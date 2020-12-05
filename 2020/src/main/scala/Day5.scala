package colinlcrawford.adventofcode

case class Seat(row: Int, col: Int)

sealed trait Direction
case object Upper extends Direction
case object Lower extends Direction

object Day5 {
  def run(): Unit = {
    val seatScores = readInput().map(getSeat).map(getScore)
    part1(seatScores)
    part2(seatScores)
  }

  def readInput(): Array[String] = {
    Utils.readInputFile("day5-input.txt")
  }

  def part1(seatScores: Array[Int]): Unit = Utils.printAnswer(5, 1, Some(seatScores.max))

  def part2(seatScores: Array[Int]): Unit = {
    val sortedScores = seatScores.sorted
    var prev = sortedScores(0) - 1
    val ans = sortedScores.find(score => {
      val check = score != (prev + 1)
      prev = score
      check
    })
    Utils.printAnswer(5, 2, ans.map(_ - 1))
  }

  def getSeat(binSpec: String): Seat = {
    Seat(getRow(binSpec.take(7)), getCol(binSpec.takeRight(3)))
  }

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

