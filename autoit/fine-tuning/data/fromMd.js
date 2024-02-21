const fs = require("fs");
const { join } = require("path");

const rows = fs
  .readFileSync("sub-1.md", "utf-8")
  .split("\n")
  .filter((v) => !!v)
  .map((v) => v.split("|"));

let t = Date.now();
const historyDir = join(__dirname, "../../history");

rows.forEach((v) => {
  const [assistant, user] = v;
  fs.writeFileSync(join(historyDir, `${t}_assistant.txt`), assistant);
  fs.writeFileSync(join(historyDir, `${t}_user.txt`), user);
  t += 1;
});

console.log("DONE.");
