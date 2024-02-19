const cheerio = require("cheerio");
const fs = require("fs");
const path = require("path");
const $ = cheerio.load(fs.readFileSync("sub.html", "utf-8"));

const rows = [];

$("tr").each((i, tr) => {
  const cols = [];
  $(tr)
    .find("td")
    .each((_, e) => {
      cols.push($(e).text());
    });
  const result = cols.map((v) => v.trim()).filter((v) => !!v);
  if (result.length === 2) {
    rows.push(result);
  }
});

let t = Date.now();
const historyDir = path.join(__dirname, "../../history");

rows.forEach((v) => {
  const [assistant, user] = v;
  fs.writeFileSync(path.join(historyDir, `${t}_assistant.txt`), assistant);
  fs.writeFileSync(path.join(historyDir, `${t}_user.txt`), user);
  t += 1;
});

console.log("DONE.");
