package colinlcrawford.adventofcode.day21

import colinlcrawford.adventofcode.{AOCDay}

case class Entry(ingredients: Array[String], allergens: Array[String])

object Day21 extends AOCDay {
  def dayNum(): Int = 21

  def part1(input: Array[String]): Option[String] = { 
    val entries = parse(input)
    val ingredients = entries.flatMap(_.ingredients).toSet
    var occurs = getOccurences(entries)
    val possible = getPossible(entries)
    val allPossible = possible.values.reduce((acc, next) => acc ++ next)
    val ans = (ingredients -- allPossible).toArray.map(i => occurs(i)).sum
    Some(ans.toString)
  }

  def part2(input: Array[String]): Option[String] = {
    val entries = parse(input)
    var possible = getPossible(entries)
    var doneVals = Set[String]()
    var done = Map[String, String]()
    var updating = true
    while (updating) {
      updating = false
      possible = possible.map({ case (k, v ) => {
        val newV = v.filter(!doneVals.contains(_))
        if (newV.size == 1) {
          done += (k -> newV.head)
          doneVals += newV.head
          updating = true
          (k -> Set[String]())
        } else {
          (k -> newV)
        }
      }}).filter({ case (k, v) => !v.isEmpty })
    }
    Some(done.toArray.sortBy(_._1).map(_._2).mkString(","))
  }

  def parse(input: Array[String]): Array[Entry] = {
    input.map(line => {
      val ingredients = line.takeWhile(_ != '(').split(' ')
      val allergens = line.dropWhile(_ != '(').drop(10).dropRight(1).split(',').map(_.trim)
      Entry(ingredients, allergens)
    })
  }

  def getOccurences(entries: Array[Entry]): Map[String, Int] = {
    entries.flatMap(_.ingredients).groupBy(identity).view.mapValues(_.size).toMap
  }

  def getPossible(entries: Array[Entry]): Map[String, Set[String]] = {
    var possible = Map[String, Set[String]]()
    entries.foreach(entry => {
      entry.allergens.foreach(allergen => {
        if (possible.contains(allergen)) possible += (allergen -> (possible(allergen) & entry.ingredients.toSet))
        else possible += (allergen -> entry.ingredients.toSet)
      })
    })
    possible
  }
}
