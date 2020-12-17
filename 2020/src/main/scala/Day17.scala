package colinlcrawford.adventofcode.day17

import colinlcrawford.adventofcode.{AOCDay}

case class Pos(x: Int, y: Int, z: Int, w: Int)

object Day17 extends AOCDay {
  def dayNum(): Int = 17

  def part1(input: Array[String]): Option[String] = Some(run(parse(input), getNeighbors3d).size.toString)

  def part2(input: Array[String]): Option[String] = Some(run(parse(input), getNeighbors4d).size.toString)

  def run(start: Set[Pos], getNeighbors: Pos => List[Pos]): Set[Pos] = {
    (1 to 6).foldLeft(start)((active, _) => {
      active.toList.flatMap(getNeighbors).groupBy(identity).map(e => (e._1, e._2.size))
        .filter({ case (pos, num) => (num == 2 && active.contains(pos)) || num == 3 })
        .map(_._1).toSet
    })
  }

  def parse(input: Array[String]): Set[Pos] = {
    input.zipWithIndex.flatMap({
      case (line, inx) => line.split("").zipWithIndex.map({
        case (c, jnx) => if (c == "#") Some(Pos(jnx, inx, 0, 0)) else None
      })
    }).filter(_.isDefined).map(_.get).toSet
  }

  val cords3d: List[(Int, Int, Int)] = (-1 to 1).flatMap(i => (-1 to 1).flatMap(j => (-1 to 1).map(k => (i, j, k)))).filter(_ != (0, 0, 0)).toList
  def getNeighbors3d(pos: Pos): List[Pos] = cords3d.map(c => applyCord(pos, (c._1, c._2, c._3, 0)))

  val cords4d: List[(Int, Int, Int, Int)] = (-1 to 1).flatMap(i => (-1 to 1).flatMap(j => (-1 to 1).flatMap(k => (-1 to 1).map(w => (i, j, k, w))))).filter(_ != (0, 0, 0, 0)).toList
  def getNeighbors4d(pos: Pos): List[Pos] = cords4d.map(c => applyCord(pos, c))

  def applyCord(pos: Pos, c: (Int, Int, Int, Int)): Pos = pos.copy(x=pos.x + c._1, y=pos.y + c._2, z=pos.z + c._3, w=pos.w + c._4)

}
