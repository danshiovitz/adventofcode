import scala.io.Source

object prog {
  type Pairing = (String, String)
  val lineRex = """(\S+) would (gain|lose) (\d+) happiness units? by sitting next to (\S+).""".r

  def run(filename: String, includeMe: Boolean): Unit = {
    val (people, pairingScores) = loadData(filename, includeMe)
    val arrangements = people.toList.permutations
    val scoredArrangements = arrangements map (x => (x, score(x, pairingScores)))
    val bestArrangement = scoredArrangements.maxBy(x => x._2)
    println(s"Best score: ${bestArrangement._2} (${bestArrangement._1})")
  }

  def loadData(filename: String, includeMe: Boolean): (Iterable[String], Map[Pairing, Int]) = {
    val pairingScores = readPairingScores(filename)
    val people = pairingScores.keys.flatMap(x => List(x._1, x._2))

    if (!includeMe) {
      return (people, pairingScores)
    }

    val xtra1 = people.map(x => ((x, "Me") -> 0)).toMap
    val xtra2 = people.map(x => (("Me", x) -> 0)).toMap
    return (people ++ Seq("Me"), pairingScores ++ xtra1 ++ xtra2)
  }

  def readPairingScores(filename: String): Map[Pairing, Int] = {
    val s = Source.fromFile(filename)
    try {
      return s.getLines.map(parseSingle(_)).toMap
    } finally {
      s.close
    }
  }

  def parseSingle(line: String): (Pairing, Int) = {
    line match {
      case lineRex(subj, verb, amt, obj) => {
        val actualAmt = if (verb == "lose") -amt.toInt else amt.toInt
        return ((subj, obj), actualAmt)
      }
      case _ => throw new RuntimeException(s"Bad line: $line")
    }
  }

  def score(arrangement: List[String], pairingScores: Map[Pairing, Int]): Int = {
    def pairingOf(i: Int, j: Int): Pairing = {
      (arrangement((i + arrangement.length) % arrangement.length),
       arrangement((j + arrangement.length) % arrangement.length))
    }
    def singleScore(i: Int): Int = {
      pairingScores(pairingOf(i, i-1)) + pairingScores(pairingOf(i, i+1))
    }
    (0 until arrangement.length).map(singleScore(_)).sum
  }

  def main(args: Array[String]): Unit = {
    run(args(0), (args.length > 1 && args(1) == "part2"))
  }
}
