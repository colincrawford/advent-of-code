package colinlcrawford.adventofcode.day18

import colinlcrawford.adventofcode.{AOCDay}

sealed trait Expr
case class Num(n: Int) extends Expr
case class Mul(left: Expr, right: Expr) extends Expr
case class Add(left: Expr, right: Expr) extends Expr
case class Paren(expr: Expr) extends Expr

object ExprEval {
  def eval(expr: Expr): Long = {
    expr match {
      case Num(n) => n.toLong
      case Mul(left, right) => eval(left) * eval(right)
      case Add(left, right) => eval(left) + eval(right)
      case Paren(e) => eval(e)
    }
  }

  def addPriority(expr: Expr): Expr = {
    expr match {
      case Num(n) => expr
      case Mul(left, right) => Mul(addPriority(left), addPriority(right))
      case Paren(e) => Paren(addPriority(e))
      case Add(left, right) => addPriority(right) match {
        case Mul(a, b) => Mul(Add(addPriority(left), a), b)
        case r => Add(addPriority(left), r)
      }
    }
  }

  def parse(raw: String): Expr = parser(raw.toList.filter(_ != ' ').reverse.map({
    case '(' => ')' case ')' => '(' case c => c
  }))

  def parser(tokens: List[Char]): Expr = {
    var ts = List[Char]()
    val prev: Expr = if (tokens.head.isDigit) {
      Num(tokens.head.toString.toInt)
    } else {
      // we have an open paren
      var seen = 1
      ts = tokens.tail.takeWhile(c => {
        if (c == ')') seen -= 1
        else if (c == '(') seen += 1
        seen != 0
      })
      Paren(parser(ts))
    }
    val rest = if (ts.isEmpty) tokens.tail else tokens.tail.drop(ts.size + 1)
    if (rest.size > 0) {
      val right = parser(rest.tail)
      rest.head match { case '*' => Mul(prev, right) case '+' => Add(prev, right) }
    } else {
      prev
    }
  }
}

object Day18 extends AOCDay {
  def dayNum(): Int = 18

  def part1(input: Array[String]): Option[String] = { 
    val ans = input.map(ExprEval.parse).map(ExprEval.eval).sum
    Some(ans.toString)
  }

  def part2(input: Array[String]): Option[String] = {
    val ans = input.map(ExprEval.parse).map(ExprEval.addPriority).map(ExprEval.eval).sum
    Some(ans.toString)
  }
}
