const fs = require("node:fs");
const path = require("node:path");

const historyDir = path.join(__dirname, "../history");
const files = fs.readdirSync(historyDir).map((v) => path.join(historyDir, v));

files.sort();

const systemPrompt = fs.readFileSync(
  path.join(__dirname, "sysprompt.txt"),
  "utf-8",
);
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
  const assistantFile = files[i].includes("_assistant")
    ? files[i]
    : files[i + 1];
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

fs.writeFileSync("fine_tuning.jsonl", rows.join("\n"));

console.log("DONE.");
