const fs = require("fs");

const processText = (text) => {
  if (text.startsWith("-")) text = text.replace("-", "");
  if (text.endsWith("-")) {
    text = text.split("-");
    text.pop();
    text = text.join("-");
  }
  while (
    text.includes("- ") ||
    text.includes(" -") ||
    text.includes(" - ") ||
    text.includes("…") ||
    text.includes("...")
  ) {
    text = text
      .split(" - ")
      .join(" ")
      .split("- ")
      .join("")
      .split(" -")
      .join("")
      .split("...")
      .join("")
      .split("…")
      .join("");
  }
  return text.trim();
};

const isFirstLetter = (str) => {
  return str.charAt(0) === str.charAt(0).toUpperCase();
};

const toMsg = (set) => {
  const arr = [...set];
  const msg = [];

  for (let i = arr.length - 1; i >= 0; i--) {
    let isIncluded = false;
    for (let j = 0; j < msg.length; j++) {
      if (msg[j].includes(arr[i]) || arr[i].includes(msg[j])) {
        isIncluded = true;
        if (arr[i].length > msg[j].length) {
          msg[j] = arr[i];
        }
        break;
      }
    }
    if (!isIncluded) {
      msg.unshift(arr[i]);
    }
  }

  return msg.map(processText).join(" ");
};

const lines = fs
  .readFileSync("sub.md", "utf-8")
  .split("\n")
  .map((v) =>
    v.split("|").map((v) => {
      return v;
    }),
  );
const newLines = [];

for (let i = 0; i < lines.length; i++) {
  const [a, u] = lines[i];
  let indexOffset = 0;
  const newLine = [new Set([a]), new Set([u])];

  for (let j = i + 1; j < lines.length; j++) {
    const [nA, nU] = lines[j];
    if (nA.includes(a) || nU.includes(u) || a.includes(nA) || u.includes(nU)) {
      newLine[0].add(nA);
      newLine[1].add(nU);
      indexOffset += 1;
    } else {
      break;
    }
  }

  const lineData = newLine.map(toMsg);
  newLines.push(lineData);

  i += indexOffset;
}

let results = [];

newLines.forEach((lineData) => {
  if (isFirstLetter(lineData[0])) {
    results.push([lineData]);
  } else {
    results[results.length - 1].push(lineData);
  }
});

results = results.map((v) => {
  const pair = [[], []];

  v.forEach((v1) => {
    pair[0].push(v1[0]);
    pair[1].push(v1[1]);
  });

  return [pair[0].join(" "), pair[1].join(" ")];
});

fs.writeFileSync("sub-1.md", results.map((v) => v.join("|")).join("\n"));

console.log("DONE.");
