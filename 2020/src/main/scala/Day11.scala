package colinlcrawford.adventofcode.day11

import colinlcrawford.adventofcode.{AOCDay}

sealed trait Spot
case object Filled extends Spot
case object Empty extends Spot
case object Floor extends Spot

case class Pos(row: Int, col: Int)

object Day11 extends AOCDay {
  type Grid = Array[Array[Spot]]

  def dayNum(): Int = 11

  def part1(input: Array[String]): Option[String] = {
    val ans = runRulesUntilNotChanging(parseGrid(input), 4, countFilledAdjacent).flatten.count(_ == Filled)
    Some(ans.toString)
  }

  def part2(input: Array[String]): Option[String] = {
    val ans = runRulesUntilNotChanging(parseGrid(input), 5, countFilledWithinSight).flatten.count(_ == Filled)
    Some(ans.toString)
  }

  def parseGrid(input: Array[String]): Grid = input.map(_.split("").map({
    case "#" => Filled case "L" => Empty case "." => Floor
  }))

  def runRulesUntilNotChanging(grid: Grid, adjSeatCap: Int, getAdjSeats: (Grid, Pos) => Int): Grid = {
    var keepRunning = true
    var newGrid = grid
    while (keepRunning) {
      keepRunning = false
      newGrid = newGrid.zipWithIndex.map({ case (row, rowInx) => {
        row.zipWithIndex.map({ case (spot, colInx) => {
          val adjFilled = getAdjSeats(newGrid, Pos(rowInx, colInx))
          spot match {
            case Empty if (adjFilled == 0) => {
              keepRunning = true
              Filled
            }
            case Filled if (adjFilled >= adjSeatCap) => {
              keepRunning = true
              Empty
            }
            case _ => spot
          }
        }})
      }})
    }
    newGrid
  }

  def get(grid: Grid, p: Pos): Option[Spot] = {
    if (p.row >= 0 && p.row < grid.size && p.col >= 0 && p.col < grid(0).size) Some(grid(p.row)(p.col))
    else None
  }

  def add(pos: Pos, row: Int, col: Int): Pos = Pos(pos.row + row, pos.col + col)

  val adjInx = List((-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1))
  def getAdjPos(pos: Pos): List[Pos] = adjInx.map({ case (r, c) => add(pos, r, c)})

  def isFilled(grid: Grid, spot: Option[Spot]): Boolean = spot.map(_ == Filled).getOrElse(false)

  def countFilledAdjacent(grid: Grid, pos: Pos): Int = {
    getAdjPos(pos).map(get(grid, _)).count(isFilled(grid, _))
  }

  def countFilledWithinSight(grid: Grid, pos: Pos): Int = {
    adjInx.count({ case(r, c) => {
      var newPos = add(pos, r, c)
      var spot = get(grid, newPos)
      while (spot.isDefined && spot.get == Floor) {
        newPos = add(newPos, r, c)
        spot = get(grid, newPos)
      }
      isFilled(grid, spot)
    }})
  }
}
