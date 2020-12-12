package colinlcrawford.adventofcode.day8

import colinlcrawford.adventofcode.{AOCDay}

sealed trait Instruction
case class Nop(amount: Int) extends Instruction
case class Acc(amount: Int) extends Instruction
case class Jmp(amount: Int) extends Instruction

class Program(val instructions: Array[Instruction]) {
  var programCounter = 0
  var acc = 0

  def step() = instructions(programCounter) match {
    case Nop(amt) => programCounter += 1
    case Acc(amt) => {
      acc += amt
      programCounter += 1
    }
    case Jmp(amt) => programCounter += amt
  }


  def runUntilRepeatingInstrOrFinishing(): Program = {
    var runLines = Set[Int]()
    while(!isFinished && !runLines.contains(programCounter)) {
      runLines += programCounter
      step()
    }
    this
  }

  def isFinished(): Boolean = programCounter == instructions.length
}

object Day8 extends AOCDay {
  def dayNum(): Int = 8

  def part1(input: Array[String]): Option[String] = {
    val instructions = input.map(parseInstruction)
    val acc = new Program(instructions).runUntilRepeatingInstrOrFinishing().acc
    Some(s"$acc")
  }

  def part2(input: Array[String]): Option[String] = {
    val instructions = input.map(parseInstruction)
    var ans: Option[Int] = None
    instructions.zipWithIndex.find({
      case (instr, inx) => {
        val prog = instr match {
          case Acc(amt) => None
          case Nop(amt) => checkForFinishingProgram(instructions, inx, Jmp(amt))
          case Jmp(amt) => checkForFinishingProgram(instructions, inx, Nop(amt))
        }
        prog match {
          case None => false
          case Some(p) => {
            ans = Some(p.acc)
            true
          }
        }
      }
    })
    ans.map(_.toString)
  }

  def checkForFinishingProgram(instructions: Array[Instruction], inx: Int, to: Instruction): Option[Program] = {
    val old = instructions(inx)
    instructions(inx) = to

    val result = new Program(instructions).runUntilRepeatingInstrOrFinishing()
    instructions(inx) = old

    if (result.isFinished()) Some(result)
    else None
  }

  def parseInstruction(line: String): Instruction = {
    val amount = Integer.parseInt(line.drop(4))
    line.take(3) match {
      case "nop" => Nop(amount)
      case "acc" => Acc(amount)
      case "jmp" => Jmp(amount)
    }
  }
}
