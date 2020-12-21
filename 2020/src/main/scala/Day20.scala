package colinlcrawford.adventofcode.day20

import colinlcrawford.adventofcode.{AOCDay}

case class Tile(id: Int, body: Array[String]) {
  val top: String = body(0)
  val right: String = body.map(_.last).mkString("")
  val bottom: String = body.last
  val left: String = body.map(_.head).mkString("")

  override def equals(o: Any): Boolean = {
    if (!o.isInstanceOf[Tile]) return false
    val ot = o.asInstanceOf[Tile]
    ot.id == id && ot.top == top && ot.right == right && ot.bottom == bottom && ot.left == left
  }
}

case class TileMap(left: Map[String, Set[Tile]], top: Map[String, Set[Tile]])

object Tiles {
  def parse(raw: Array[String]): Tile = {
    val id = raw(0).drop(5).takeWhile(_ != ':').toInt
    Tile(id, raw.drop(1))
  }

  def rotate(tile: Tile): Tile = tile.copy(body=rotStrs(tile.body))

  def rotateTimes(tile: Tile, n: Int): List[Tile] = {
    if (n == 0) return List[Tile]()
    val rotated = rotate(tile)
    rotated :: rotateTimes(rotated, n - 1)
  }

  def rotStrsTimes(s: Array[String], n: Int): List[Array[String]] = {
    if (n == 0) return List[Array[String]]()
    val rotated = rotStrs(s)
    rotated :: rotStrsTimes(rotated, n - 1)
  }

  def rotStrs(s: Array[String]): Array[String] = {
    (0 until s.size).map(j => (0 until s.size).map(i => (i, j))).map(r => r.map({ case(i, j) => s(i)(j) }).mkString("")).toArray
  }

  def flip(t: Tile): List[Tile] = List(
    t.copy(body=t.body.reverse),
    t.copy(body=t.body.map(r => r.reverse)),
    t.copy(body=t.body.reverse.map(r => r.reverse))
  )
}

object Day20 extends AOCDay {
  def dayNum(): Int = 20

  var part1Ans: Option[Array[Array[Tile]]] = None

  def part1(input: Array[String]): Option[String] = { 
    val tiles = groupInput(input).map(Tiles.parse)
    val combos = buildCombos(tiles)
    val sideMap = buildSideMap(combos)
    val size = scala.math.sqrt(tiles.size).toInt
    var board: Array[Array[Option[Tile]]] = Array.fill(size) { Array.fill(size){ None } }
    var ans = solve1((0, 0), board, tiles.map(_.id).toSet, sideMap, combos).get.map(r => r.map(_.get))
    part1Ans = Some(ans)
    Some(1.toLong * ans(0).head.id * ans(0).last.id * ans.last.head.id * ans.last.last.id).map(_.toString)
  }

  def part2(input: Array[String]): Option[String] = {
    val img = part1Ans.get
      .map(r => r.map(t => t.body.drop(1).dropRight(1).map(s => s.drop(1).dropRight(1))))
      .map(r => r.reduce((acc, next) => acc.zipWithIndex.map({ case (s, inx) => s + next(inx) })))
      .reduce((acc, next) => acc ++ next)
    val numHash = img.map(_.count(x => x == '#')).sum
    val candidates = imgFlips(img).flatMap(i => imgRots(i))

    var ans = 0
    var i = 0
    while (ans == 0 && i < candidates.size) {
      ans = search(candidates(i), (0, 0))
      i += 1
    }
    Some(getAns(numHash, ans)).map(_.toString)
  }

  def getAns(numHash: Int, numMonsters: Int): Int = numHash - (numMonsters * seaMonsterInxs.size)

  val seaMonsterLen = 20
  val seaMonsterHeight = 3
  def search(board: Array[String], inx: (Int, Int)): Int = {
    if (inx._1 > (board.size - seaMonsterHeight) || inx._2 > (board.size - seaMonsterLen)) return 0

    var next = (inx._1, inx._2 + 1)
    if (next._2 == (board.size - seaMonsterLen)) next = (inx._1 + 1, 0)

    check(board, inx) + search(board, next)
  }

  val seaMonsterInxs = Array[(Int, Int)](
    (0, 18), (1, 0), (1, 5), (1, 6), (1, 11), (1, 12), (1, 17), (1, 18),
    (1, 19), (2, 1), (2, 4), (2, 7), (2, 10), (2, 13), (2, 16)
  )
  def check(board: Array[String], inx: (Int, Int)): Int = {
    val box = board.slice(inx._1, inx._1 + seaMonsterHeight).map(_.slice(inx._2, inx._2 + seaMonsterLen))
    var valid = seaMonsterInxs.forall({ case (r, c) => box(r)(c) == '#' })
    if (valid) 1 else 0
  }

  def imgFlips(img: Array[String]): List[Array[String]] = {
    List(img, img.reverse, img.map(r => r.reverse), img.reverse.map(r => r.reverse))
  }

  def imgRots(img: Array[String]): List[Array[String]] = List(img) ++ Tiles.rotStrsTimes(img, 3)

  def solve1(pos: (Int, Int), soFar: Array[Array[Option[Tile]]], unused: Set[Int], sideMap: TileMap, tiles: Array[Tile]): Option[Array[Array[Option[Tile]]]] = {
    if (pos._1 == soFar.size) return Some(soFar)
    var nextPos = (pos._1, pos._2 + 1)
    if (nextPos._2 == soFar.size) nextPos = (pos._1 + 1, 0)

    val candidates: Array[Tile] = if (pos == (0, 0)) {
      tiles
    } else {
      // check top
      var matchTop = Set[Tile]()
      if (pos._1 != 0) matchTop = sideMap.top.getOrElse(soFar(pos._1 - 1)(pos._2).get.bottom, Set[Tile]())
      // check left
      var matchLeft = Set[Tile]()
      if (pos._2 != 0) matchLeft = sideMap.left.getOrElse(soFar(pos._1)(pos._2 - 1).get.right, Set[Tile]())

      (if (pos._1 == 0) matchLeft else if (pos._2 == 0) matchTop else matchLeft & matchTop).filter(t => unused.contains(t.id)).toArray
    }

    var i = 0
    while (i < candidates.size) {
      soFar(pos._1)(pos._2) = Some(candidates(i))
      val ans = solve1(nextPos, soFar, unused - candidates(i).id, sideMap, tiles)
      if (ans.isDefined) return ans
      i += 1
    }
    soFar(pos._1)(pos._2) = None
    None
  }

  def groupInput(input: Array[String]): Array[Array[String]] = {
    input.foldLeft(List[List[String]](List[String]()))((acc, line) => {
      if (line == "") List[String]() :: acc
      else (line :: acc.head) :: acc.tail
    }).map(_.reverse.toArray).toArray
  }

  def buildSideMap(tileCombos: Array[Tile]): TileMap = {
    var left = Map[String, Set[Tile]]()
    var top = Map[String, Set[Tile]]()
    tileCombos.foreach(tile => {
      left += (tile.left -> (left.getOrElse(tile.left, Set[Tile]()) + tile))
      top += (tile.top -> (top.getOrElse(tile.top, Set[Tile]()) + tile))
    })
    TileMap(left, top)
  }

  def buildCombos(tiles: Array[Tile]): Array[Tile] = {
    tiles.flatMap(tile => (Array(tile) ++ Tiles.flip(tile)).flatMap(t => t :: Tiles.rotateTimes(t, 3)))
  }
}
