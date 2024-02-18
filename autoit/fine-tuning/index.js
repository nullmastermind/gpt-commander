const fs = require("node:fs");
const path = require("node:path");

const historyDir = path.join(__dirname, "../history");
const files = fs.readdirSync(historyDir).map((v) => path.join(historyDir, v));

files.sort();

const rows = [];

for (let i = 0; i < files.length; i += 2) {
  const row = {
    messages: [],
  };
  const userFile = files[i].includes("_user") ? files[i] : files[i + 1];
  const assistantFile = files[i].includes("_assistant")
    ? files[i]
    : files[i + 1];
  row.messages.push({
    role: "user",
    content: fs.readFileSync(userFile, "utf-8"),
  });
  row.messages.push({
    role: "assistant",
    content: fs.readFileSync(assistantFile, "utf-8"),
  });
  rows.push(JSON.stringify(row));
}

fs.writeFileSync("fine_tuning.jsonl", rows.join("\n"));

console.log("DONE.");
