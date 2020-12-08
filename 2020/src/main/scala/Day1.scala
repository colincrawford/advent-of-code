package colinlcrawford.adventofcode

object Day1 extends AOCDay {
  def dayNum(): Int = 1

  val targetSum = 2020 

  def part1(input: Array[String]): Option[String] = {
    val inputs = input.map(Integer.parseInt(_))
    findTwoSummingTo(targetSum, inputs).map({ case (a, b) => s"${a * b}" })
  }

  def part2(input: Array[String]): Option[String] = {
    val inputs = input.map(Integer.parseInt(_))
    findThreeSummingTo(targetSum, inputs).map({ case (a, b, c) => s"${a * b * c}" })
  }

  def findTwoSummingTo(target: Int, nums: Array[Int]): Option[(Int, Int)] = {
    var neededNums = Set[Int]()
    for (num <- nums) {
      val neededPartner = target - num
      if (neededNums.contains(neededPartner)) return Some(num, neededPartner)
      else neededNums = neededNums + num
    }
    None
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
