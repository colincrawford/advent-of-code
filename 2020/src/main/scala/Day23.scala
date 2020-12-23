package colinlcrawford.adventofcode.day23

import colinlcrawford.adventofcode.{AOCDay}

class CupCircle(var cups: Array[Int]) {
  var minCup = cups.min
  var maxCup = cups.max
  var cupToNext = Map[Int, Int]()

  var count = 0
  while (count < (cups.size - 1)) {
    cupToNext += (cups(count) -> cups(count + 1))
    count += 1
  }
  cupToNext += (cups.last -> cups.head)

  var curr = cups.head

  def playRounds(n: Int) = { (1 to n).foreach(_ => playRound() ) }

  def playRound() = {
    val a = cupToNext(curr)
    val b = cupToNext(a)
    val c = cupToNext(b)
    val newCurr = cupToNext(c)

    var taken = Set[Int](a, b, c)
    var next = getReverseNextId(curr)
    while (!cupToNext.contains(next) || taken.contains(next)) next = getReverseNextId(next)

    val prev = cupToNext(next)
    cupToNext += (next -> a)
    cupToNext += (c -> prev)
    cupToNext += (curr -> newCurr)

    curr = cupToNext(curr)
  }

  def getReverseNextId(id: Int): Int = if (id == minCup) maxCup else id - 1

  def addCupsToMillion() = {
    cupToNext += (cups.last -> (maxCup + 1))
    ((maxCup + 1) to 1000000).foreach(i => { cupToNext += (i -> (i + 1)) })
    cupToNext += (1000000 -> cups.head)
    maxCup = 1000000
  }

  def ans1(): String = {
    var vals = List[Int]()
    var start = cupToNext(1)
    while (start != 1) {
      vals = start :: vals
      start = cupToNext(start)
    }
    vals.reverse.mkString
  }

  def ans2(): String = {
    var a = cupToNext(1)
    var b = cupToNext(a)
    (a.toLong * b.toLong).toString
  }
}

object Day23 extends AOCDay {
  def dayNum(): Int = 23

  def part1(input: Array[String]): Option[String] = { 
    var cupCircle = parse(input)
    cupCircle.playRounds(100)
    Some(cupCircle.ans1)
  }

  def part2(input: Array[String]): Option[String] = {
    var cupCircle = parse(input)
    cupCircle.addCupsToMillion()
    cupCircle.playRounds(10000000)
    Some(cupCircle.ans2)
  }

  def parse(input: Array[String]): CupCircle = {
    new CupCircle(input(0).split("").map(_.toInt))
  }
}
