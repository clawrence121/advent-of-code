const fs = require("fs");

const file = fs.readFileSync("./input.txt", "utf8");

let tree = [];
let sizes = [];
let cursor = "";
let buffer = [];
let totals = [];

const setPath = (object, path, value) =>
  path
    .split(".")
    .reduce(
      (o, p, i) => (o[p] = path.split(".").length === ++i ? value : o[p] || {}),
      object
    );

const resolvePath = (object, path, defaultValue) =>
  path.split(".").reduce((o, p) => (o ? o[p] : defaultValue), object);

const command = (command) => {
  const [action, args] = command.split("$ ")[1].split(" ");

  switch (action.trim()) {
    case "cd":
      if (args !== "..") {
        if (args === "/") {
          cursor = "";
        }
        if (cursor.length > 0) {
          cursor += "." + args;
        } else {
          cursor += args;
        }
        setPath(tree, cursor, []);
        setPath(sizes, cursor, []);
      } else {
        const c = cursor.split(".");
        c.pop();
        cursor = c.join(".");
      }
      break;
    case "ls":
      break;
  }
};

file.split("\n").forEach((line) => {
  // handle new command
  if (line.includes("$")) {
    command(line);
    return;
  }

  // handle command std out response
  if (line.length >= 0 && !line.includes("dir")) {
    const temp = [...resolvePath(tree, cursor, []), line];

    const sum = temp
      .map((f) => parseInt(f.split(" ")[0]))
      .reduce((a, b) => a + b, 0);

    // console.log(temp, sum);

    setPath(tree, cursor, temp);
    setPath(sizes, cursor, [sum]);
  }
});

// console.log(resolvePath(tree, "/.a.e"), tree);
// console.log(resolvePath(sizes, "/.a.e"), sizes);

// traverse sizes, if less than 100000 add to total
const traverse = (object) => {
  Object.keys(object).forEach((key) => {
    if (key === "0") {
      if (object[key] <= 100000) {
        console.log(object, key);
        totals.push(object[key]);
      }
    } else {
      console.log(object, key);
      traverse(object[key]);
    }
  });
};
traverse(sizes);
const total = totals.reduce((a, b) => a + b, 0);
console.log(total);
