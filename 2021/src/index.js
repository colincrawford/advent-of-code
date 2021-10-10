import { readFileSync } from 'fs'

import { day1 } from './day1.js'
import { day2 } from './day2.js'
import { day3 } from './day3.js'
import { day4 } from './day4.js'
import { day5 } from './day5.js'
import { day6 } from './day6.js'
import { day7 } from './day7.js'
import { day8 } from './day8.js'
import { day9 } from './day9.js'
import { day10 } from './day10.js'
import { day11 } from './day11.js'
import { day12 } from './day12.js'
import { day13 } from './day13.js'
import { day14 } from './day14.js'
import { day15 } from './day15.js'
import { day16 } from './day16.js'
import { day17 } from './day17.js'
import { day18 } from './day18.js'
import { day19 } from './day19.js'
import { day20 } from './day20.js'
import { day21 } from './day21.js'
import { day22 } from './day22.js'
import { day23 } from './day23.js'
import { day24 } from './day24.js'
import { day25 } from './day25.js'

const readInput = (day) => readFileSync(`./inputs/day${day}.txt`)

const days = [
  day1, day2, day3, day4, day5,
  day6, day7, day8, day9, day10,
  day11, day12, day13, day14, day15,
  day16, day17, day18, day19, day20,
  day21, day22, day23, day24, day25,
]

const main = (currentDay) => {
  days
    .filter((day, inx) => inx <= (currentDay - 1))
    .map((day, inx) => day(readInput(inx + 1)))
}

main(process.argv[2] || days.length)
