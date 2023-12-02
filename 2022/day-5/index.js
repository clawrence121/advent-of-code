const { create } = require("domain");
const fs = require("fs");

const data = fs.readFileSync("input.txt", "utf8");

const stacks = data.split("\n\nmove")[0];

// build a map of stacks
const temp = stacks.split("\n").map((line) => line.split(""));
const rotate = (arr) =>
  arr[0].map((val, index) => arr.map((row) => row[index]));
const stackMap = rotate(temp)
  .map((stack) => stack.join(""))
  .filter((crate) => crate.trim().match(/^[a-z0-9]+$/i))
  .map((c) => c.trim());
let stackObject = stackMap.reduce((acc, stack, index) => {
  acc[index + 1] = stack.slice(0, -1).split("").reverse().join("");
  return acc;
}, {});

console.log(stackObject);

const moves = data.split("\n\n")[1].split("\n");

// Play the game
moves.forEach((element) => {
  const [from, to] = element.split(" from ")[1].split(" to ");
  const moveCount = parseInt(element.split(" ")[1]);

  let fromStack = stackObject[from];
  const toStack = stackObject[to];

  //   for (let i = 0; i < moveCount; i++) {
  //     const charToMove = fromStack[fromStack.length - 1];
  //     stackObject[from] = fromStack.slice(0, -1);
  //     stackObject[to] = stackObject[to] + charToMove;
  //     fromStack = stackObject[from];
  //   }

  const removed = fromStack.slice(-moveCount);
  stackObject[from] = fromStack.slice(0, -moveCount);
  stackObject[to] = stackObject[to] + removed;
});

const chars = [];
for (const [key, value] of Object.entries(stackObject)) {
  console.log(`${key}: ${value.slice(-1)}`);
  chars.push(value.slice(-1));
}

console.log(chars.join(""));
