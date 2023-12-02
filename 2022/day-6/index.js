const fs = require("fs");

const file = fs.readFileSync("./input.txt", "utf8");

let buffer = [];
let position = 0;

const routine = (char) => {
  //   console.log(char);
  buffer.push(char);

  // if buffer is full (length is 4) then remove the first item
  if (buffer.length === 14) {
    // if there are no duplicate values in the buffer return true
    if (new Set(buffer).size === buffer.length) {
      return true;
    }

    buffer.shift();
  }
};

// loop through each char in file
for (let i = 0; i < file.length; i++) {
  if (routine(file[i])) {
    console.log("Found it!");
    position = i + 1;
    break;
  }
}

console.log("Position: ", position);
