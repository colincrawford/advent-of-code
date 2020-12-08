package colinlcrawford.adventofcode

import scala.collection.mutable.{Queue, HashMap}

case class RuleEdge(color: String, weight: Int)
case class RuleNode(color: String, edges: List[RuleEdge])

object Day7 extends AOCDay {
  def dayNum(): Int = 7

  type RuleGraph = Map[String, List[RuleEdge]]

  def part1(input: Array[String]): Option[String] = {
    val ans = findAllThatCanHold("shiny gold", parseInput(input)).size
    Some(s"$ans")
  }

  def part2(input: Array[String]): Option[String] = {
    // subtract by 1 not to count the shiny gold bag that contains everything
    val ans = findBagsNeededWithin("shiny gold", parseInput(input), HashMap[String, Int]()) - 1
    Some(s"$ans")
  }

  def findBagsNeededWithin(color: String, ruleGraph: RuleGraph, cache: HashMap[String, Int]): Int = {
    // speed things up by memoizing results
    if (cache.contains(color)) return cache(color)

    val totalBags = ruleGraph(color).map(edge => findBagsNeededWithin(edge.color, ruleGraph, cache) * edge.weight).sum + 1
    cache += (color -> totalBags)
    totalBags
  }

  def findAllThatCanHold(color: String, ruleGraph: RuleGraph): Set[String] = {
    val reverseInx = inverseRuleGraph(ruleGraph)
    // start with the bags containing our target
    var bagsContaining = reverseInx(color)
    // traverse the graph outward from those bags seeing what points to them
    var needSearching = Queue[String]()
    needSearching.addAll(bagsContaining)

    while (!needSearching.isEmpty) {
      val next = needSearching.dequeue
      bagsContaining = bagsContaining + next

      if (reverseInx.contains(next)) needSearching.addAll(reverseInx(next))
    }

    bagsContaining
  }

  // flip the graph - so, for a given node, what nodes point to it? 
  def inverseRuleGraph(ruleGraph: RuleGraph): Map[String, Set[String]] = {
    ruleGraph.foldRight(Map[String, Set[String]]())((entry, acc) => {
      entry._2.foldRight(acc)((edge, acc2) => {
        if (acc2.contains(edge.color)) acc2 + (edge.color -> (acc2(edge.color) + entry._1))
        else acc2 + (edge.color -> Set[String](entry._1))
      })
    })
  }

  def parseInput(input: Array[String]): RuleGraph = input.map(parseRule).map(rule => rule.color -> rule.edges).toMap

  val colorSpecSep = " bags contain "
  // ex: "posh crimson bags contain 2 mirrored tan bags, 1 faded red bag, 1 striped gray bag."
  def parseRule(line: String): RuleNode = {
    val colorRuleSepInx = line.indexOf(colorSpecSep)
    val color = line.take(colorRuleSepInx)
    val specStrings = line.drop(colorRuleSepInx + colorSpecSep.size).dropRight(1).split(", ")
    val edges = specStrings.filter(_ != "no other bags").map(parseRuleStr).toList
    RuleNode(color, edges)
  }

  // ex: "2 mirrored tan bags"
  def parseRuleStr(ruleStr: String): RuleEdge = {
    val spec = ruleStr.take(ruleStr.indexOf(" bag"))
    val numColorSep = spec.indexOf(" ")
    val color = spec.drop(numColorSep + 1)
    val weight = Integer.parseInt(spec.take(numColorSep))
    RuleEdge(color, weight)
  }
}

