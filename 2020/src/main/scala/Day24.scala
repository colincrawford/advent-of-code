package colinlcrawford.adventofcode.day24

import colinlcrawford.adventofcode.{AOCDay}

sealed trait Direction
case object NorthWest extends Direction
case object NorthEast extends Direction
case object East extends Direction
case object SouthEast extends Direction
case object SouthWest extends Direction
case object West extends Direction

object Direction {
  val dirs = Set[String]("nw", "ne", "e", "se", "sw", "w")
  def isDir(s: String): Boolean = dirs.contains(s)

  def parse(s: String): Direction = {
    s match {
      case "nw" => NorthWest
      case "ne" => NorthEast
      case "e" => East
      case "se" => SouthEast
      case "sw" => SouthWest
      case "w" => West
    }
  }
}

case class Path(moves: List[Direction])

sealed trait Color
case object Black extends Color
case object White extends Color
case class Coord(north: Int, east: Int, color: Color)

object Coord {
  def update(coord: Coord, dir: Direction): Coord = {
    dir match {
      case NorthWest => coord.copy(north = coord.north + 1, east = coord.east - 1)
      case NorthEast => coord.copy(north = coord.north + 1, east = coord.east + 1)
      case East => coord.copy(east = coord.east + 2)
      case SouthEast => coord.copy(north = coord.north - 1, east = coord.east + 1)
      case SouthWest => coord.copy(north = coord.north - 1, east = coord.east - 1)
      case West => coord.copy(east = coord.east - 2)
    }
  }

  def fromPath(path: Path): Coord = path.moves.foldLeft(Coord(0, 0, White))((acc, next) => update(acc, next))

  def flip(coord: Coord): Coord = coord.copy(color=(if (coord.color == White) Black else White))
}

object Day24 extends AOCDay {
  def dayNum(): Int = 24

  def printPath(path: Path) = println(path.moves.mkString(","))

  var flippedTiles = Array[Coord]()

  def part1(input: Array[String]): Option[String] = { 
    val tiles = parse(input)
    flippedTiles = tiles.groupBy(identity).view.mapValues(_.size).toArray
      .map({ case (k, v) => if (v % 2 == 0) k else Coord.flip(k)})
      .filter(_.color == Black)
    Some(flippedTiles.size.toString)
  }

  def part2(input: Array[String]): Option[String] = {
    val blackTiles = (1 to 100).foldLeft(flippedTiles)((acc, _) => passDay(acc))
    Some(blackTiles.size.toString)
  }

  def passDay(blackTiles: Array[Coord]): Array[Coord] = {
    var neighbors = blackTiles.flatMap(getNeighbors(_)).groupBy(t => (t.north, t.east)).view.mapValues(_.size).toMap
    val keep = blackTiles.filter(t => neighbors.contains((t.north, t.east)) && neighbors((t.north, t.east)) < 3).toSet
    val flip = neighbors.filter({ case (k, count) => count == 2}).map({ case (k, count) => Coord(k._1, k._2, Black) }).toSet
    (keep ++ flip).toArray
  }

  val dirs = Array[Direction](NorthWest, NorthEast, East, SouthEast, SouthWest, West)
  def getNeighbors(coord: Coord): Array[Coord] = dirs.map(d => Coord.update(coord, d).copy(color=White))

  def parse(input: Array[String]): Array[Coord] = input.map(parsePath).map(Coord.fromPath)

  def parsePath(line: String): Path = {
    var dirs = List[Direction]()
    var dir = ""
    for (c <- line) {
      dir += c.toString
      if (Direction.isDir(dir)) {
        dirs = Direction.parse(dir) :: dirs
        dir = ""
      }
    }
    Path(dirs.reverse)
  }
}
