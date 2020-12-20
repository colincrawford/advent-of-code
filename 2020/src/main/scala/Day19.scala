package colinlcrawford.adventofcode.day19

import colinlcrawford.adventofcode.{AOCDay}

sealed trait Rule { def id: Int }
case class Depends(ruleIds: List[Int])
case class LetterRule(id: Int, letter: Char) extends Rule
case class DependsRule(id: Int, options: List[Depends]) extends Rule

object Day19 extends AOCDay {
  def dayNum(): Int = 19

  def part1(input: Array[String]): Option[String] = { 
    val (rules, inputs) = parse(input)
    val allowed = ("^" + expand(0, rules) + "$").r
    Some(inputs.count(allowed.matches(_)).toString)
  }

  def part2(input: Array[String]): Option[String] = {
    var (rules, inputs) = parse(input)
    // cache is populated from part 1
    val fortyTwo = cache(42)
    val thirtyOne = cache(31)
    cache.clear()
    // 8: 42 | 42 8
    cache += (8 -> s"($fortyTwo+)")
    // 11: 42 31 | 42 11 31
    // scala regex doesnt seem to support recursive regex, so brute force it
    cache += (11 -> s"($fortyTwo($fortyTwo($fortyTwo($fortyTwo($fortyTwo$thirtyOne)?$thirtyOne)?$thirtyOne)?$thirtyOne)?$thirtyOne)")
    val allowed = ("^" + expand(0, rules) + "$").r
    Some(inputs.count(allowed.matches(_)).toString)
  }

  def parse(input: Array[String]): (Map[Int, Rule], Array[String]) = {
    val sepInx = input.indexOf("")
    val rules = input.take(sepInx).map(parseRule).foldLeft(Map[Int, Rule]())((acc, next) => acc + (next.id -> next))
    val inputStrs = input.drop(sepInx + 1)
    (rules, inputStrs)
  }

  def parseRule(s: String): Rule = {
    val sepInx = s.indexOf(':')
    val id = s.take(sepInx).toInt
    if (s.contains('"')) {
      LetterRule(id, s.dropRight(1).last)
    } else if (s.contains('|')) {
      val groups = s.drop(sepInx + 1).split('|').map(v => v.split(" ").filter(_ != ""))
      DependsRule(id, groups.map(a => Depends(a.map(_.toInt).toList)).toList)
    } else {
      val nums = s.drop(sepInx + 1).split(" ").filter(_ != "").map(_.toInt)
      DependsRule(id, List(Depends(nums.toList)))
    }
  }

  var cache = scala.collection.mutable.Map[Int, String]()
  def expand(ruleId: Int, rules: Map[Int, Rule]): String = {
    if (cache.contains(ruleId)) return cache(ruleId)
    val ans = rules(ruleId) match {
      case LetterRule(id, c) => c.toString
      case DependsRule(id, options) => {
        options.map(d => d.ruleIds.foldLeft("")((acc, rId) => {
          acc + "(" + expand(rId, rules) + ")"
        })).foldLeft("(")((acc, andGroup) => {
          if (acc == "(") acc + andGroup else acc + "|" + andGroup
        }) + ")"
      }
    }
    cache += (ruleId -> ans)
    ans
  }
}
