import path from "path";

export default function (
  /** @type {import('plop').NodePlopAPI} */
  plop
) {
  plop.setGenerator("solution", {
    description: "Scaffold a solution to Advent of Code 2022 problem",
    prompts: [
      {
        type: "input",
        name: "day",
        message: "Day?",
        validate: (input) => !isNaN(input),
      },
      {
        type: "choices",
        name: "part",
        message: "Part?",
        default: "a",
        choices: ["a", "b"],
      },
    ],
    actions: (data) => {
      const dir = path.join("..", `day_${data.day}${data.part}`);

      return [
        {
          type: "add",
          path: path.join(dir, "Cargo.toml"),
          templateFile: "templates/Cargo.toml.hbs",
        },
        {
          type: "add",
          path: path.join(dir, "input.txt"),
        },
        {
          type: "add",
          path: path.join(dir, "src", "main.rs"),
          templateFile: "templates/main.rs.hbs",
        },
      ];
    },
  });
}
