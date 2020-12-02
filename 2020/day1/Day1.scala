import scala.io.Source
import util.control.Breaks._

object Day1 {
  def main(args: Array[String]): Unit = {
    val targetSum = 2020
    val inputs = readInput()
    part1(inputs, targetSum)
    part2(inputs, targetSum)
  }

  def readInput(): Array[Int] = {
    Source.fromFile("./day1/input.txt").getLines().toArray.map(Integer.parseInt(_))
  }

  def part2(inputs: Array[Int], targetSum: Int): Unit = {
    val msg = findThreeSummingTo(targetSum, inputs) match {
      case None => "No answer found for part 2!"
      case Some((num1, num2, num3)) => s"Day 1: Part 2 Answer -> ${num1 * num2 * num3}"
    }
    println(msg)
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

  def part1(inputs: Array[Int], targetSum: Int): Unit = {
    val msg = findTwoSummingTo(targetSum, inputs) match {
      case None => "No answer found for part 1!"
      case Some((num1, num2)) => s"Day 1: Part 1 Answer -> ${num1 * num2}"
    }
    println(msg)
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
}
