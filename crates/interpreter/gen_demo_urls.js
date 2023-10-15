#! /bin/node

let fs = require("fs");

let items = [];

fs.readdirSync("./tests/tests_sources").forEach((file) => {
  items.push({
    name: file,
    href: `/?code=${encodeURIComponent(
      fs.readFileSync(`./tests/tests_sources/${file}`, "utf8"),
    )}`,
  });
});

console.log(JSON.stringify(items));
