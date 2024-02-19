const fs = require("node:fs");
const path = require("node:path");

const historyDir = path.join(__dirname, "../history");
const files = fs.readdirSync(historyDir).map((v) => path.join(historyDir, v));

files.sort();

const systemPrompt = fs.readFileSync(path.join(__dirname, "sysprompt.txt"), "utf-8");
const rows = [];
const added = new Set([]);

for (let i = 0; i < files.length; i += 2) {
  const row = {
    messages: [
      {
        role: "system",
        content: systemPrompt,
      },
    ],
  };
  const userFile = files[i].includes("_user") ? files[i] : files[i + 1];
  const assistantFile = files[i].includes("_assistant") ? files[i] : files[i + 1];
  const userContent = `<document>${fs.readFileSync(userFile, "utf-8")}</document>`;

  if (added.has(userContent)) continue;
  added.add(userContent);

  row.messages.push({
    role: "user",
    content: `<document>${fs.readFileSync(userFile, "utf-8")}</document>`,
  });
  row.messages.push({
    role: "assistant",
    content: fs.readFileSync(assistantFile, "utf-8"),
  });

  rows.push(JSON.stringify(row));
}

const totalValidateItems = Math.ceil(rows.length / 10);
const validates = fs
  .readFileSync("fine_tuning_validate.jsonl", "utf-8")
  .split("\n")
  .map((v) => v.trim())
  .filter((v) => !!v)
  .map((v) => JSON.parse(v))
  .map((v) => {
    v.messages[0].content = systemPrompt;
    return JSON.stringify(v);
  });

function randomRange(min, max) {
  return Math.floor(Math.random() * (max - min + 1)) + min;
}

while (validates.length < totalValidateItems) {
  const getIndex = randomRange(0, rows.length);
  let removedItem = rows.splice(getIndex, 1)[0];
  if (!validates.includes(removedItem)) {
    validates.push(removedItem);
  }
}

fs.writeFileSync("fine_tuning.jsonl", rows.filter((v) => !validates.includes(v)).join("\n"));
fs.writeFileSync("fine_tuning_validate.jsonl", validates.join("\n"));

console.log("DONE.");
