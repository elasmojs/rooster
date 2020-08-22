var mathlib = require('./lib/calc.js');
var msg = require('./lib/msg.js');

var a = 5;
var b = 5;

var results = {
    add: mathlib.add(a, b),
    subtract: mathlib.subtract(a, b),
    multiply: mathlib.multiply(a, b),
    divide: mathlib.divide(a, b),
    msg: msg
}

JSON.stringify(results);
