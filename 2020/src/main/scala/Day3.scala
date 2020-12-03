package colinlcrawford.adventofcode

import scala.io.Source

case class Rate(x: Int, y: Int)
case class Position(x: Int, y: Int)
case class MoveResult(hitTree: Boolean, newPosition: Position)

class TreeGrid(val rows: Array[String]) {
  def move(position: Position, rate: Rate): MoveResult = {
    val newPosition = Position(position.x + rate.x, position.y + rate.y)
    return MoveResult(locationIsTree(newPosition), newPosition)
  }

  def canMove(position: Position, rate: Rate): Boolean = {
    (position.y + rate.y) < rows.size
  }

  def locationIsTree(position: Position): Boolean = {
    val row = rows(position.y)
    row(position.x % row.length) == '#'
  }
}

object Day3 {
  def run(): Unit = {
    val input = readInput()
    part1(input)
    part2(input)
  }

  def readInput(): TreeGrid = {
    val input = Source.fromResource("day3-input.txt")
    val inputs = input.getLines().toArray
    input.close()
    new TreeGrid(inputs)
  }

  def part1(input: TreeGrid) = {
    Utils.printAnswer(3, 1, Some(countTreesHit(input, Rate(3, 1))))
  }

  def part2(input: TreeGrid) = {
    val rates = List(Rate(1, 1), Rate(3, 1), Rate(5, 1), Rate(7, 1), Rate(1, 2))
    val ans = rates.map(countTreesHit(input, _)).reduce((a, b) => a * b)
    Utils.printAnswer(3, 2, Some(ans))
  }

  def countTreesHit(grid: TreeGrid, rate: Rate): Int = {
    var treesHit = 0
    var position = Position(0, 0)
    while (grid.canMove(position, rate)) {
      val result = grid.move(position, rate)
      if (result.hitTree) {
        treesHit += 1
      }
      position = result.newPosition
    }
    treesHit
  }
}
