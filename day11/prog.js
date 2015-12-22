var range = require('node-range');

function toValues(input) {
  return range(0, input.length).map(function(i) { return input.charCodeAt(i); });
}

function fromValues(values) {
  return values.map(function(v) { return String.fromCharCode(v); }).join("");
}

function isLegal(values) {
  var i, found, prevpair;

  found = false;
  for (i = 0; i < values.length - 2; i++) {
    if (values[i] + 1 == values[i+1] && values[i] + 2 == values[i+2]) {
      found = true;
      break;
    }
  }

  if (!found) {
    return false;
  }

  for (i = 0; i < values.length; i++) {
    if (values[i] == "i".charCodeAt(0) || values[i] == "o".charCodeAt(0) || values[i] == "l".charCodeAt(0)) {
      return false;
    }
  }

  found = false;
  prevpair = -1;
  for (i = 0; i < values.length - 1; i++) {
    if (values[i] == values[i+1] && values[i] != prevpair) {
      if (prevpair != -1) {
        found = true;
        break;
      } else {
        prevpair = values[i];
      }
    }
  }

  if (!found) {
    return false;
  }

  return true;
}

function increment(values) {
  var i;
  for (i = values.length - 1; i >= 0; i--) {
    if (values[i] < "z".charCodeAt(0)) {
      values[i]++;
      return;
    } else {
      values[i] = "a".charCodeAt(0); // and continue
    }
  }
  console.log("oh no, we reached the end!");
}

function run(input, inctimes) {
  var i;
  values = toValues(input);
  for (i = 0; i < inctimes; i++) {
    increment(values);
  }
  while (!isLegal(values)) {
    //console.log("Illegal: " + fromValues(values));
    increment(values);
  }
  console.log("Final input: " + fromValues(values));
}

run(process.argv[2], 1);
