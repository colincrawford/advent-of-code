package colinlcrawford.adventofcode.day3

import colinlcrawford.adventofcode.{AOCDay}

case class Rate(x: Int, y: Int)
case class Position(x: Int, y: Int)
case class MoveResult(hitTree: Boolean, newPosition: Position)

class TreeGrid(val rows: Array[String]) {
  def move(position: Position, rate: Rate): MoveResult = {
    val newPosition = Position(position.x + rate.x, position.y + rate.y)
    return MoveResult(locationIsTree(newPosition), newPosition)
  }

  def canMove(position: Position, rate: Rate): Boolean = (position.y + rate.y) < rows.size

  def locationIsTree(position: Position): Boolean = {
    val row = rows(position.y)
    row(position.x % row.length) == '#'
  }
}

object Day3 extends AOCDay {
  def dayNum(): Int = 3

  def part1(input: Array[String]) = {
    val ans = countTreesHit(new TreeGrid(input), Rate(3, 1))
    Some(s"$ans")
  }

  def part2(input: Array[String]) = {
    val rates = List(Rate(1, 1), Rate(3, 1), Rate(5, 1), Rate(7, 1), Rate(1, 2))
    val ans = rates.map(countTreesHit(new TreeGrid(input), _)).reduce((a, b) => a * b)
    Some(s"$ans")
  }

  def countTreesHit(grid: TreeGrid, rate: Rate): Int = {
    var treesHit = 0
    var position = Position(0, 0)
    while (grid.canMove(position, rate)) {
      val result = grid.move(position, rate)
      if (result.hitTree) treesHit += 1
      position = result.newPosition
    }
    treesHit
  }
}
