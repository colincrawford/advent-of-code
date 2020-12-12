package colinlcrawford.adventofcode.day2

import colinlcrawford.adventofcode.{AOCDay}

case class PasswordSpec(num1: Int, num2: Int, letter: Char, candidatePass: String)

object Day2 extends AOCDay {
  def dayNum(): Int = 2

  def part1(inputs: Array[String]): Option[String] = {
    val ans = inputs.map(parseLine).filter(isValidPassPart1).size
    Some(s"$ans")
  }

  def part2(inputs: Array[String]): Option[String] = {
    val ans = inputs.map(parseLine).filter(isValidPassPart2).size
    Some(s"$ans")
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
}
