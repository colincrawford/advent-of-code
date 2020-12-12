package colinlcrawford.adventofcode.day12

import colinlcrawford.adventofcode.{AOCDay}

sealed trait Direction
case object North extends Direction
case object South extends Direction
case object East extends Direction
case object West extends Direction

sealed trait Instruction
case class Move(direction: Direction, amt: Int) extends Instruction
case class Left(degrees: Int) extends Instruction
case class Right(degrees: Int) extends Instruction
case class Forward(amt: Int) extends Instruction

case class Pos(north: Int, south: Int, east: Int, west: Int, currentDirection: Direction)

object Day12 extends AOCDay {
  def dayNum(): Int = 12

  def part1(input: Array[String]): Option[String] = {
    val finalPos = parse(input).foldLeft(Pos(0, 0, 0, 0, East))(move)
    Some(manhattanDistance(finalPos).toString)
  }

  def part2(input: Array[String]): Option[String] = {
    val startingPos = (Pos(0, 0, 0, 0, East), Pos(1, 0, 10, 0, East))
    val finalPos = parse(input).foldLeft(startingPos)(movePart2)._1
    Some(manhattanDistance(finalPos).toString)
  }

  def manhattanDistance(pos: Pos): Int = (pos.north - pos.south).abs + (pos.east - pos.west).abs

  def move(pos: Pos, instr: Instruction): Pos = {
    instr match {
      case Forward(amt) => move(pos, Move(pos.currentDirection, amt))
      case Left(degrees) => pos.copy(currentDirection = findDirection(pos.currentDirection, -1 * degrees))
      case Right(degrees) => pos.copy(currentDirection = findDirection(pos.currentDirection, degrees))
      case Move(direction, amt) => moveDir(pos, direction, amt)
    }
  }

  def movePart2(pos: (Pos, Pos), instr: Instruction): (Pos, Pos) = {
    val ship = pos._1
    val waypoint = pos._2
    instr match {
      case Forward(amt) => (moveToWaypoint(ship, waypoint, amt), waypoint)
      case Left(degrees) => (ship, rotateWaypoint(waypoint, -1 * degrees))
      case Right(degrees) => (ship, rotateWaypoint(waypoint, degrees))
      case Move(direction, amt) => (ship, moveDir(waypoint, direction, amt))
    }
  }

  def moveToWaypoint(ship: Pos, waypoint: Pos, amt: Int): Pos = {
    ship.copy(
      north = ship.north + (waypoint.north * amt),
      east = ship.east + (waypoint.east * amt),
      south = ship.south + (waypoint.south * amt),
      west = ship.west + (waypoint.west * amt)
    )
  }

  def moveDir(pos: Pos, direction: Direction, amt: Int): Pos = {
    direction match {
      case North => pos.copy(north = pos.north + amt)
      case South => pos.copy(south = pos.south + amt)
      case East => pos.copy(east = pos.east + amt)
      case West => pos.copy(west = pos.west + amt)
    }
  }

  val nextDirs = Map[Direction, Array[Direction]](
    North -> Array(East, South, West), East -> Array(South, West, North),
    South -> Array(West, North, East), West -> Array(North, East, South)
  )

  def rotateWaypoint(pos: Pos, rotateDegrees: Int): Pos = {
    var rotateTimes = ((rotateDegrees / 90) % 4).abs
    (1 to rotateTimes).foldLeft(pos)((newPos, _) => {
      if (rotateDegrees < 0) newPos.copy(north = newPos.east, east = newPos.south, south = newPos.west, west = newPos.north)
      else newPos.copy(north = newPos.west, east = newPos.north, south = newPos.east, west = newPos.south)
    })
  }

  def findDirection(currentDirection: Direction, rotateDegrees: Int): Direction = {
    val rotateTimes = ((rotateDegrees / 90) % 4).abs
    if (rotateTimes == 0) return currentDirection
    var nextDirections = if (rotateDegrees < 0) nextDirs(currentDirection).reverse else nextDirs(currentDirection)
    nextDirections(rotateTimes - 1)
  }

  def parse(input: Array[String]): Array[Instruction] = {
    input.map(line => {
      val amt = line.drop(1).toInt
      line(0) match {
        case 'N' => Move(North, amt)
        case 'S' => Move(South, amt)
        case 'E' => Move(East, amt)
        case 'W' => Move(West, amt)
        case 'L' => Left(amt)
        case 'R' => Right(amt)
        case 'F' => Forward(amt)
      }
    })
  }
}
