package colinlcrawford.adventofcode

import scala.io.Source
import util.control.Breaks._

object Day1 {
  def run(): Unit = {
    val targetSum = 2020
    val inputs = readInput()
    part1(inputs, targetSum)
    part2(inputs, targetSum)
  }

  def readInput(): Array[Int] = {
    val input = Source.fromResource("day1-input.txt")
    val inputs = input.getLines().toArray.map(Integer.parseInt(_))
    input.close()
    inputs
  }

  def part1(inputs: Array[Int], targetSum: Int): Unit = {
    val answer = findTwoSummingTo(targetSum, inputs).map({
      case (a, b) => a * b
    })
    Utils.printAnswer(1, 1, answer)
  }

  def findTwoSummingTo(target: Int, nums: Array[Int]): Option[(Int, Int)] = {
    var neededNums = Set[Int]()
    for (num <- nums) {
      val neededPartner = target - num
      neededNums.contains(neededPartner) match {
        case false => neededNums = neededNums + num
        case true => return Some(num, neededPartner)
      }
    }
    None
  }

  def part2(inputs: Array[Int], targetSum: Int): Unit = {
    val answer = findThreeSummingTo(targetSum, inputs).map({
      case (a, b, c) => a * b * c
    })
    Utils.printAnswer(1, 2, answer)
  }

  def findThreeSummingTo(target: Int, nums: Array[Int]): Option[(Int, Int, Int)] = {
    val numbers = nums.toSet
    for (i <- 0 to nums.size - 1) {
      for (j <- i+1 to nums.size - 1) {
        val lookingFor = target - (nums(i) + nums(j))
        if (numbers.contains(lookingFor)) {
          return Some((nums(i), nums(j), lookingFor))
        }
      }
    }
    None
  }
}
