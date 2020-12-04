package colinlcrawford.adventofcode

case class Field(name: String, validator: String => Boolean)

object Day4 {
  def run(): Unit = {
    val input = readInput()
    part1(input)
    part2(input)
  }

  def readInput(): List[String] = groupPassportInfo(Utils.readInputFile("day4-input.txt"))

  def part1(input: List[String]): Unit = Utils.printAnswer(4, 1, Some(input.filter(hasRequiredFields).length))

  def part2(input: List[String]): Unit = Utils.printAnswer(4, 2, Some(input.filter(hasRequiredFields).filter(hasValidFields).length))

  val requiredFields = List[Field](
    Field("byr", validByr), Field("iyr", validIyr), Field("eyr", validEyr),
    Field("hgt", validHgt), Field("hcl", validHcl), Field("ecl", validEcl),
    Field("pid", validPid)
  )

  def hasRequiredFields(passport: String): Boolean = requiredFields.forall(field => passport.contains(field.name))

  def hasValidFields(passport: String): Boolean = requiredFields.forall(field => field.validator(getField(field.name, passport)))

  def validByr(byr: String): Boolean = isFourDigitsBetween(byr, 1920, 2002)

  def validIyr(iyr: String): Boolean = isFourDigitsBetween(iyr, 2010, 2020)

  def validEyr(eyr: String): Boolean = isFourDigitsBetween(eyr, 2020, 2030)

  def isFourDigitsBetween(value: String, min: Int, max: Int): Boolean = {
    if (value.length != 4) return false
    val num = Integer.parseInt(value)
    num <= max && num >= min
  }

  def validHgt(hgt: String): Boolean = hgt.takeRight(2) match {
    case "cm" => isValidHeightSpec(hgt, 150, 193)
    case "in" => isValidHeightSpec(hgt, 59, 76)
    case _ => false
  }


  def isValidHeightSpec(spec: String, min: Int, max: Int): Boolean = {
    val num = Integer.parseInt(spec.take(spec.length - 2))
    num >= min && num <= max
  }

  val validHclDigits = (('0' to '9') ++ ('a' to 'f')).toSet
  def validHcl(hcl: String): Boolean = {
    if (hcl.length != 7 || !hcl.startsWith("#")) return false
    hcl.takeRight(hcl.length - 1).forall(validHclDigits.contains(_))
  }

  val validEcls = Set("amb", "blu", "brn", "gry", "grn", "hzl", "oth")
  def validEcl(ecl: String): Boolean = validEcls.contains(ecl)

  def validPid(pid: String): Boolean = {
    try {
      Integer.parseInt(pid)
      pid.length == 9
    } catch {
      case _: Throwable => false
    }
  }

  def getField(field: String, passport: String): String = {
    val startInx = passport.indexOf(field)
    var endInx = passport.substring(startInx).indexOf(" ")
    if (endInx == -1) {
      endInx = passport.length
    } else {
      endInx = startInx + endInx
    }
    passport.substring(startInx + field.length + 1, endInx)
  }

  def groupPassportInfo(input: Array[String]): List[String] = {
    var passports = List[String]()
    var currentPassport = ""
    for (line <- input) {
      if (line == "") {
        passports = currentPassport :: passports
        currentPassport = ""
      } else if (currentPassport != "") {
        currentPassport += (" " + line)
      } else {
        currentPassport += line
      }
    }

    if (currentPassport != "") currentPassport :: passports else passports
  }
}
