import kotlin.text.Regex
import java.io.File

fun run(sueFilename: String, specFilename: String) {
  val sues = loadSues(sueFilename)
  val spec = loadSpec(specFilename)
  val matchingSues = sues.filter({spec.allows(it, ::evalAttr)})
  if (matchingSues.size > 0) {
    matchingSues.forEach {
      println("Matching Sue: $it")
    }
  } else {
    println("No matching Sues :C")
  }

  val matchingSues2 = sues.filter({spec.allows(it, ::evalAttr2)})
  if (matchingSues2.size > 0) {
    matchingSues2.forEach {
      println("Matching Sue (part 2): $it")
    }
  } else {
    println("No matching Sues (part 2) :C")
  }
}

data class Sue(val name: String, val attributes: Map<String, Int>)

val sueRegex = Regex("""(.*?): (.*)""")
val sueAttrRegex = Regex("""(\S+): (\d+)""")

fun loadSues(filename: String): List<Sue> {
  return File(filename).readLines().map {
    val res = sueRegex.matchEntire(it)
    if (res == null) {
      throw Throwable("Bad sue line: $it")
    }
    val name = res.groups.get(1)!!.value
    
    val attrsText = res.groups.get(2)!!.value
    val attrsRes = sueAttrRegex.findAll(attrsText)
    val attrs = attrsRes.map {
      it.groups.get(1)!!.value to it.groups.get(2)!!.value.toInt()
    }.toMap()

    Sue(name, attrs)
  }
}

data class Spec(val attributes: Map<String, Int>) {
  fun allows(sue: Sue, evalFun:(String, Int, Int) -> Boolean): Boolean {
    return attributes.toList().all({
      !sue.attributes.containsKey(it.first) || evalFun(it.first, it.second, sue.attributes[it.first]!!)
    })
  }
}

fun evalAttr(name: String, expected: Int, actual: Int): Boolean {
  return expected == actual
}

fun evalAttr2(name: String, expected: Int, actual: Int): Boolean {
  if (name == "cats" || name == "trees") {
    return actual > expected
  } else if (name == "pomeranians" || name == "goldfish") {
    return actual < expected
  } else {
    return expected == actual
  }
}

val specRegex = Regex("""(\S+): (\d+)""")

fun loadSpec(filename: String): Spec {
  val attrs = File(filename).readLines().map {
    val res = specRegex.matchEntire(it)
    if (res == null) {
      // is using Throwable bad Kotlin style? I'm not going to make
      // my own class just for this, anyway
      throw Throwable("Bad spec line: $it")
    }
    res.groups.get(1)!!.value to res.groups.get(2)!!.value.toInt()
  }.toMap()
   
  return Spec(attrs)
}

fun main(args: Array<String>) {
   run(args[0], args[1])
}
