import fs = require("fs");

function fuelRequired(mass: string) {
  const fuel = Math.floor(parseInt(mass, 10) / 3) - 2;
  return Math.max(0, fuel);
}

const file = fs.readFileSync(__dirname + "/day1.txt", "utf8");
console.log(
  file
    .split("\n")
    .map(s => s.trim())
    .filter(Boolean)
    .map(fuelRequired)
    .reduce((a, b) => a + b, 0)
);
