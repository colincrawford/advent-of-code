package colinlcrawford.adventofcode

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

object Day8 {
  def run(): Unit = {
    val input = readInput().map(parseInstruction)
    part1(input)
    part2(input)
  }

  def readInput(): Array[String] = Utils.readInputFile("day8-input.txt")

  def part1(input: Array[Instruction]): Unit = {
    val acc = new Program(input).runUntilRepeatingInstrOrFinishing().acc
    Utils.printAnswer(8, 1, Some(acc))
  }

  def part2(input: Array[Instruction]): Unit = {
    var ans: Option[Int] = None
    input.zipWithIndex.find({
      case (instr, inx) => {
        val prog = instr match {
          case Acc(amt) => None
          case Nop(amt) => checkForFinishingProgram(input, inx, Jmp(amt))
          case Jmp(amt) => checkForFinishingProgram(input, inx, Nop(amt))
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
    Utils.printAnswer(8, 2, ans)
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
