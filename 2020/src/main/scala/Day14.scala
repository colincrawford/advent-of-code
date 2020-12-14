package colinlcrawford.adventofcode.day14

import colinlcrawford.adventofcode.{AOCDay}

sealed trait Instr
case class Assignment(inx: Long, value: Long) extends Instr
case class BitMask(bits: String) extends Instr

case class MaskV1(andMask: Long, orMask: Long)
case class MaskV2(orMask: Long, bits: String)

sealed trait Transform
case class And(value: Long) extends Transform
case class Or(value: Long) extends Transform

object Day14 extends AOCDay {
  def dayNum(): Int = 14

  def part1(input: Array[String]): Option[String] = { 
    var memory: Map[Long, Long] = Map[Long, Long]()
    var mask: Option[MaskV1] = None
    parse(input).foreach({
      case Assignment(inx, value) => memory += (inx -> applyMask(mask.get, value))
      case BitMask(str) => mask = Some(parseMaskV1(str))
    })
    Some(memory.values.sum.toString)
  }

  def part2(input: Array[String]): Option[String] = {
    var memory = scala.collection.mutable.Map[Long, Long]()
    var mask: Option[MaskV2] = None
    parse(input).foreach({
      case Assignment(inx, value) => {
        val bits = mask.get.bits
        val floaterInxs = bits.zipWithIndex.filter(_._1 == 'X').map(_._2).toList
        applyMask2(List[Transform](), floaterInxs, bits, inx | mask.get.orMask, value, memory) 
      }
      case BitMask(str) => mask = Some(parseMaskV2(str))
    })
    Some(memory.values.sum.toString)
  }

  def parse(input: Array[String]): Array[Instr] = {
    input.map(line => if (line.startsWith("mask")) BitMask(line.drop(7)) else parseAssignment(line))
  }

  def parseAssignment(line: String): Assignment = {
    val inx = line.drop(4).takeWhile(_ != ']').toLong
    val num = line.dropWhile(_ != '=').drop(2).toLong
    Assignment(inx, num)
  }

  def parseMaskV1(bits: String): MaskV1 = {
    var andMask = bits.replace("X", "1")
    var orMask = bits.replace("X", "0")
    MaskV1(java.lang.Long.parseLong(andMask, 2), java.lang.Long.parseLong(orMask, 2))
  }

  def applyMask(mask: MaskV1, value: Long): Long = (value & mask.andMask) | mask.orMask

  def parseMaskV2(bits: String): MaskV2 = {
    var orMask = bits.replace("X", "0")
    MaskV2(java.lang.Long.parseLong(orMask, 2), bits)
  }

  def applyMask2(transforms: List[Transform], floatInxs: List[Int], str: String, inx: Long, value: Long, map: scala.collection.mutable.Map[Long, Long]): Unit = {
    if (floatInxs.isEmpty) {
      val i = transforms.foldLeft(inx)((acc, next) => {
        next match {
          case And(v) => acc & v
          case Or(v) => acc | v
        }
      })
      map(i) = value
    } else {
      val floatInx = floatInxs.head
      val andStr = "1".repeat(floatInx) + "0" + "1".repeat(str.size - floatInx - 1)
      val orStr = "1" + "0".repeat(str.size - floatInx - 1)
      val wAnd = And(java.lang.Long.parseLong(andStr, 2)) :: transforms
      val wOr = Or(java.lang.Long.parseLong(orStr, 2)) :: transforms
      var newS = str.split("")
      newS(floatInx) = "0"
      applyMask2(wAnd, floatInxs.tail, newS.mkString(""), inx, value, map)
      applyMask2(wOr, floatInxs.tail, newS.mkString(""), inx, value, map)
    }
  }
}
