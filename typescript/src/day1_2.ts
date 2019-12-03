import fs = require("fs");

function fuelRequired(mass: number): number {
  const fuel = Math.floor(mass / 3) - 2;
  if (fuel <= 0) {
      return 0;
  }
  return fuel + fuelRequired(fuel);
}

const file = fs.readFileSync(__dirname + "/day1.txt", "utf8");
console.log(
  file
    .split("\n")
    .map(s => s.trim())
    .filter(Boolean)
    .map(mass => fuelRequired(parseInt(mass, 10)))
    .reduce((a, b) => a + b, 0)
);
