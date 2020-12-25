package colinlcrawford.adventofcode.day25

import colinlcrawford.adventofcode.{AOCDay}

object Day25 extends AOCDay {
  def dayNum(): Int = 25

  def part1(input: Array[String]): Option[String] = { 
    val (cardPubKey, doorPubKey) = parse(input)
    var loopSize = 1
    var v = 1.toLong
    while (v != cardPubKey) {
      v = (v * 7) % 20201227
      loopSize += 1
    }
    var key = 1.toLong
    while (loopSize > 1) {
      key = (key * doorPubKey) % 20201227
      loopSize -= 1
    }
    Some(key.toString)
  }

  def part2(input: Array[String]): Option[String] = {
    None
  }

  def parse(input: Array[String]): (Long, Long) = (input(0).toLong, input(1).toLong)
}
