import { execSync } from "node:child_process";

import { initUtils } from "aoc-utils";
import dotenv from "dotenv";
import inquirer from "inquirer";

dotenv.config();

inquirer
  .prompt([
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
  ])
  .then((answers) => {
    const submission = execSync("cargo run", {
      cwd: `day_${answers.day}${answers.part}`,
    }).toString();

    const { submit } = initUtils({
      year: "2022",
      token: process.env.AOC_TOKEN,
    });

    submit({
      answer: submission,
      day: answers.day,
      level: answers.part,
    }).then((success) => {
      if (success) {
        console.log(`day${answers.day}${answers.part} correct! 🥳`);
      } else {
        console.log(`day${answers.day}${answers.part} incorrect... 🥹`);
      }
    });
  });
