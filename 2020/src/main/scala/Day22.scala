package colinlcrawford.adventofcode.day22

import scala.collection.mutable.{Queue}

import colinlcrawford.adventofcode.{AOCDay}

case class Deck(player: Int, cards: List[Int])

class Game(val deck1: Deck, val deck2: Deck) {
  var d1 = new Queue[Int]()
  var d2 = new Queue[Int]()

  def play(): Deck = {
    d1.addAll(deck1.cards)
    d2.addAll(deck2.cards)
    while (!d1.isEmpty && !d2.isEmpty) {
      val card1 = d1.dequeue
      val card2 = d2.dequeue
      if (card1 > card2) {
        d1.enqueue(card1)
        d1.enqueue(card2)
      } else {
        d2.enqueue(card2)
        d2.enqueue(card1)
      }
    }
    if (d1.isEmpty) Deck(2, d2.toList) else Deck(1, d1.toList)
  }

  def playR(): Deck = {
    var seen = Set[String]()
    d1.addAll(deck1.cards)
    d2.addAll(deck2.cards)
    while (!d1.isEmpty && !d2.isEmpty) {
      val gameState = d1.mkString + "-" + d2.mkString
      if (seen.contains(gameState)) return Deck(1, d1.toList)
      seen += gameState

      var p1Wins = true
      val card1 = d1.dequeue
      val card2 = d2.dequeue

      if (d1.size >= card1 && d2.size >= card2) {
        // play recursive
        var g = new Game(Deck(1, d1.toList.take(card1)), Deck(2, d2.toList.take(card2)))
        val winningDeck = g.playR
        p1Wins = (winningDeck.player == 1)
      } else {
        p1Wins = card1 > card2
      }

      if (p1Wins) {
        d1.enqueue(card1)
        d1.enqueue(card2)
      } else {
        d2.enqueue(card2)
        d2.enqueue(card1)
      }
    }
    if (d1.isEmpty) Deck(2, d2.toList) else Deck(1, d1.toList)
  }
}

object Day22 extends AOCDay {
  def dayNum(): Int = 22

  def part1(input: Array[String]): Option[String] = { 
    val (d1, d2) = parse(input)
    val winningDeck = new Game(d1, d2).play
    Some(score(winningDeck).toString)
  }

  def part2(input: Array[String]): Option[String] = {
    val (d1, d2) = parse(input)
    val winningDeck = new Game(d1, d2).playR
    Some(score(winningDeck).toString)
  }

  def score(deck: Deck): Int = deck.cards.reverse.zipWithIndex.map({ case (v, i) => v * (i + 1) }).sum

  def parse(input: Array[String]): (Deck, Deck) = {
    val deck1 = input.drop(1).takeWhile(_ != "").map(_.toInt).toList
    val deck2 = input.dropWhile(_ != "Player 2:").drop(1).map(_.toInt).toList
    (Deck(1, deck1), Deck(2, deck2))
  }
}
