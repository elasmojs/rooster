"use strict";

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

var rnd = require('rnd');

var Polygon = function Polygon() {
  _classCallCheck(this, Polygon);

  this.name = 'Polygon' + rnd.get();
};

var poly1 = new Polygon();
$g.response.body = poly1.name; // expected output: "Polygon"
