import ceylon.file {
  parsePath,
  forEachLine,
  File
}

import ceylon.regex {
  regex
}

import ceylon.collection {
  HashMap
}

Boolean validateName(String name, String checksum) {
  value counts = HashMap<Character,Integer> {};
  for (c in name) {
    if (c == '-') {
      continue;
    }
    counts.put(c, counts.getOrDefault(c, 0) + 1);
  }
  Integer getCount(Character c) => counts.getOrDefault(c, 0);
  
  value topValues = counts.keys.
    sort(comparing(byDecreasing(getCount),
                   byIncreasing(Character.lowercased))).
    take(5);
  value computedChecksum = String(topValues);
  //  print("name: " + name + " checksum: " + checksum + " computed: " + computedChecksum);
  return computedChecksum.equals(checksum);
}

void decryptName(String name, Integer sector) {
  value builder = StringBuilder();
  value alphabet = 'a'..'z';
  
  for (c in name) {
    if (c == '-') {
      builder.append(' '.string);
    } else {
      builder.append((alphabet[(c.integer - 'a'.integer + sector) % 26] else '?').string);
    }
  }

  print("Input: " + name + " Output: " + builder.string);
}

void processFile(String filename) {
  value resource = parsePath(filename).resource;
  if (!is File resource) {
    print("Bad input file: " + filename);
    return;
  }
  
  value roomRegex = regex("^([a-z]+(?:-[a-z]+)+)-([0-9]+)\\[([a-z]+)\\]$");
  variable Integer validSectorTotal = 0;
  
  forEachLine(resource, (String line) {
    value match = roomRegex.find(line);
    if (!exists match) {
      print("Bad format for line: " + line);
      return;
    }

    value name = match.groups[0];
    value sector = Integer.parse(match.groups[1] else "bogus");
    value checksum = match.groups[2];

    if (!exists name) {
      print("Bad inner format for line (no name): " + line);
      return;
    }

    if (!exists checksum) {
      print("Bad inner format for line (no checksum): " + line);
      return;
    }

    if (!is Integer sector) {
      print("Bad inner format for line (bad sector): " + line);
      return;
    }

    if (validateName(name, checksum)) {
      validSectorTotal += sector;
    }

    decryptName(name, sector);
  });

  print("Valid sector total: " + validSectorTotal.string);
}

shared void run() {
  String? filename = process.arguments[0];
  if (exists filename) {
    processFile(filename);
  } else {
    print("Oops, no filename given");
  }
}
    
