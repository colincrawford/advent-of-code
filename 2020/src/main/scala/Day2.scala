package colinlcrawford.adventofcode

case class PasswordSpec(num1: Int, num2: Int, letter: Char, candidatePass: String)

object Day2 {
  def run(): Unit = {
    val inputs = readInput()
    part1(inputs)
    part2(inputs)
  }

  def readInput(): Array[PasswordSpec] = {
    Utils.readInputFile("day2-input.txt").map(parseLine)
  }

  def parseLine(line: String): PasswordSpec = {
    val minMaxSepInx = line.indexOf('-')
    val letterPassSepInx = line.indexOf(':')
    val num1 = line.substring(0, minMaxSepInx).toInt
    val num2 = line.substring(minMaxSepInx + 1, letterPassSepInx - 2).toInt
    val letter = line(letterPassSepInx - 1)
    val passwordCandidate = line.substring(letterPassSepInx + 2, line.size)
    PasswordSpec(num1, num2, letter,passwordCandidate)
  }

  def isValidPassPart1(passSpec: PasswordSpec): Boolean = {
    val letterCount = passSpec.candidatePass.count(_ == passSpec.letter)
    letterCount >= passSpec.num1 && letterCount <= passSpec.num2
  }

  def isValidPassPart2(passSpec: PasswordSpec): Boolean = {
    val letter1Matches = passSpec.candidatePass(passSpec.num1 - 1) == passSpec.letter
    val letter2Matches = passSpec.candidatePass(passSpec.num2 - 1) == passSpec.letter
    if (letter1Matches && letter2Matches) false
    else letter1Matches || letter2Matches
  }

  def part1(inputs: Array[PasswordSpec]): Unit = {
    val ans = inputs.filter(isValidPassPart1).size
    Utils.printAnswer(2, 1, Some(ans))
  }

  def part2(inputs: Array[PasswordSpec]): Unit = {
    val ans = inputs.filter(isValidPassPart2).size
    Utils.printAnswer(2, 2, Some(ans))
  }
}
