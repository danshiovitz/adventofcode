import groovy.transform.InheritConstructors

def run(fileName, quantity) {
  def containers = loadFile(fileName)
  def combos = calcCombos(containers, quantity)
  printf("Number of combos: %d\n", combos.size())
  
  def distinctComboSizes = combos.groupBy({it.size()})
  def minComboSize = distinctComboSizes.keySet().min()
  printf("Combos of min size (%d): %d\n", minComboSize, distinctComboSizes[minComboSize].size())
}

def loadFile(fileName) {
  return new File(fileName).readLines().collect{ it.toInteger() }
}

def calcCombos(containers, quantity) {
  if (quantity == 0) {
    return [[]]
  }

  if (containers.size() == 0) {
    return []
  }

  def head = containers[0]
  def rest = containers.takeRight(containers.size() - 1)
  
  def combos = []

  if (head <= quantity) {
    combos += calcCombos(rest, quantity - head).collect{[head] + it}
  }
  combos += calcCombos(rest, quantity)

  return combos
}

run(args[0], args[1].toInteger())
