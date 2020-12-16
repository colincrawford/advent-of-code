package colinlcrawford.adventofcode.day16

import colinlcrawford.adventofcode.{AOCDay}

case class Range(min: Int, max: Int)
case class Rule(name: String, range1: Range, range2: Range)

object Day16 extends AOCDay {
  def dayNum(): Int = 16

  type Ticket = Array[Int]

  def part1(input: Array[String]): Option[String] = { 
    val (rules, myTicket, tickets) = parse(input)
    val ans = tickets.flatten.filter(n => !rules.exists(r => check(n, r))).sum
    Some(ans.toString)
  }

  def part2(input: Array[String]): Option[String] = {
    val (rules, myTicket, tickets) = parse(input)
    val validTickets = tickets.filter(t => t.forall(i => rules.exists(r => check(i, r))))
    val entries = Array.fill(tickets(0).size){Array.fill(validTickets.size){0}}
    validTickets.zipWithIndex.foreach({ case (t, inx) => t.zipWithIndex.foreach({ case (i, jnx) => entries(jnx)(inx) = i }) })
    var ruleToEntries = rules.map(r => (r, entries.zipWithIndex.filter({ case (entries, inx) => entries.forall(e => check(e, r)) }).map(_._2))).toMap
    val inxToRule = for ((k, v) <- search(ruleToEntries, Set[Int]()).get) yield (v, k)
    val ans = myTicket.zipWithIndex.map({ case (n, i) => (n, inxToRule(i)) }).filter({ case (n, r) => r.name.contains("departure") }).map(_._1.toLong).product
    Some(ans.toString)
  }

  def search(ruleToEntries: Map[Rule, Array[Int]], used: Set[Int]): Option[Map[Rule, Int]] = {
    if (ruleToEntries.isEmpty) return Some(Map())
    val (rule, inxs) = ruleToEntries.head
    var filtered = inxs.filter(!used.contains(_)).toList
    while (!filtered.isEmpty) {
      val next = filtered.head
      filtered = filtered.tail

      val attempt = search(ruleToEntries - rule, used + next)
      if (!attempt.isEmpty) return Some(attempt.get + (rule -> next))
    }
    None
  }

  def check(x: Int, rule: Rule): Boolean = {
    (x <= rule.range1.max && x >= rule.range1.min) || (x <= rule.range2.max && x >= rule.range2.min)
  }

  def parse(input: Array[String]): (Array[Rule], Ticket, Array[Ticket]) = {
    val myTicketInx = input.indexOf("") + 1
    val ruleStrs = input.slice(0, myTicketInx - 1)
    val myTicket = input(myTicketInx + 1)
    val tickets = input.slice(myTicketInx + 4, input.size)
    (ruleStrs.map(parseRule), parseTicket(myTicket), tickets.map(parseTicket))
  }

  def parseTicket(ticket: String): Ticket = ticket.split(",").map(_.toInt)

  def parseRule(rule: String): Rule = {
    val name = rule.takeWhile(_ != ':')
    val ranges = rule.dropWhile(_ != ':').drop(2)
    val orInx = ranges.indexOf(" or ")
    val range1 = ranges.take(orInx).split("-").map(_.toInt)
    val range2 = ranges.drop(orInx + 4).split("-").map(_.toInt)
    Rule(name, Range(range1(0), range1(1)), Range(range2(0), range2(1)))
  }
}
