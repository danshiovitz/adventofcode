var input = "1321131112";
var times = 50;

function seeAndSay(txt) {
  var i, j, ret;
  ret = "";
  for (i = 0; i < txt.length; i = j) {
    j = i;
    while (txt[j] == txt[i]) {
      j++;
    }
    ret += (j - i) + txt[i];
  }
  return ret;
}

var t;
print("Intial: " + input + ", length " + input.length);
for (t = 0; t < times; t++) {
  input = seeAndSay(input);
  print(input + ", length " + input.length);
}

print("Final: " + input + ", length " + input.length);

