module.exports = function range(a, b, isInclusive) {
    function untilB(i, b) {
        return isInclusive ? i<=b : i<b; 
    }
    return { 
	map: function map(func) {
	    var arr = []
	    for (var i=a; untilB(i, b); i++) {
		arr.push(func(i))
	    }
	    return arr;
	},
	toArray: function toArray() {
	    var arr = [];
	    for (var i=a; untilB(i, b); i++) {
		arr.push(i);
	    }
	    return arr;
	},
	forEach: function forEach(func) {
	    for(var i=a; untilB(i, b); i++){
		func(i);
	    }
	    return undefined;
	},
	forEachAsync: function forEachAsync(func) {
	    for (var i=a; untilB(i, b); i++) {
		process.nextTick(func.bind(null, i))
	    }
	}
    }
}

