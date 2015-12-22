node-range
==========

Simple Lazy Ranges for Node/Javascript

Install:

npm install node-range

/*
  Faster than Array.map since it does
  array composition and function application in same step
*/
var arr = range(1,11).map(function(i) {
    return i*5
})
console.log(arr)

/*
You can still do
*/
arr = range(1,11).toArray().map(function(i) {
    return i*2
})
console.log(arr)


/*
As well we can just execute a function against a range but not return an array
*/
range(1,11).forEach(function(i) {
    console.log(Math.exp(i,2))
})

/*
As well we can execute Async
*/
range(1,11).forEachAsync(function(i) {
    console.log('num:' + Math.exp(i,2));
})
console.log('This should come first')

