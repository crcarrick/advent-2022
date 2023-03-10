import path from "node:path";

import { initUtils } from "aoc-utils";
import fs from "fs-extra";
import dotenv from "dotenv";

dotenv.config();

function getInputFile(data) {
  if (data.part === "a") return;

  const inputPath = path.join(`day_${data.day}a`, "input.txt");

  return fs.existsSync(inputPath) ? path.join("..", inputPath) : [];
}

function getTestInput(data) {
  if (data.part === "a") return;

  const mainPath = path.join(`day_${data.day}a`, "src", "main.rs");

  if (fs.existsSync(mainPath)) {
    const result = /const INPUT: &str = "(?<input>(.|\n)*)";/.exec(
      fs.readFileSync(mainPath, "utf8")
    );

    return result?.groups?.input;
  }
}

export default function (plop) {
  plop.setActionType("fetchInput", (data) => {
    return new Promise((resolve, reject) => {
      const { input } = initUtils({
        year: "2022",
        token: process.env.AOC_TOKEN,
      });

      input(data.day)
        .then((input) => {
          fs.outputFileSync(
            path.join(`day_${data.day}${data.part}`, "input.txt"),
            input
          );

          resolve();
        })
        .catch(reject);
    });
  });

  plop.setGenerator("solution", {
    description: "Scaffold a solution to Advent of Code 2022 problem",
    prompts: [
      {
        type: "input",
        name: "day",
        message: "day",
        validate: (input) => !isNaN(input),
      },
      {
        type: "choices",
        name: "part",
        message: "part",
        default: "a",
        choices: ["a", "b", "c"],
        validate: (input) => ["a", "b", "c"].includes(input),
      },
    ],
    actions: (data) => {
      const dir = path.join("..", `day_${data.day}${data.part}`);

      // try to copy the input.txt & test input from a previous part
      const inputFile = getInputFile(data);
      const testInput = getTestInput(data);

      return [
        {
          type: "add",
          path: path.join(dir, "Cargo.toml"),
          templateFile: path.join("templates", "Cargo.toml.hbs"),
        },
        data.part === "a"
          ? {
              type: "fetchInput",
            }
          : {
              type: "add",
              path: path.join(dir, "input.txt"),
              templateFile: inputFile,
            },
        {
          type: "add",
          path: path.join(dir, "src", "main.rs"),
          data: { testInput },
          templateFile: path.join("templates", "main.rs.hbs"),
        },
      ];
    },
  });
}
