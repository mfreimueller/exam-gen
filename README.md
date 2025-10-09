# exam-gen

`exam-gen` is a tool that allows for creating unique tests for a given number
of students. The goal is to be able to create enough tests with unique
combinations to prevent students from directly copying each other, as I found
the regular approach of having two or more groups to be insufficient.

## Usage

To use `exam-gen`, first create a directory structure for your test.
See `example/` for an example project.

Your project must contain the following items:

- a config file (default name: "exam-gen.toml")
- a LaTeX template
- a directory containing questions

To run `exam-gen`, navigate into your project directory and execute the command:

`exam-gen [--config-file=]`

`exam-gen` create unique permutations of your questions and create a given
number of tests into an output directory, as specified in your config file.

## Config file

You can (and have to) specify multiple configuration parameters:

- `base_file` ... your template LaTeX-file
- `questions` ... your base directory containing the questions
- `student_count` ... how many tests you want to create
- `out_dir` ... the output directory

Note, that all directories and paths are relative to your current directory.

## Template file

When writing your template file, make sure to put placeholder at the appropriate
spots where you want your questions to be inserted.

Use the custom 'macro' `% @insert`, which has this syntax:

`% @insert <question count> <question type>`

For example:

`% @insert 2 philosophy`

This places two random questions from the 'philosophy' question pool.

## Questions

To write questions, first create a dedicated directory that acts as your
question root. Then, create your question pools by creating subdirectories.
Make sure, that the names are the same you use in your `template file`.

In your subdirectories containing your question pools, place your individual
questions. Here, the naming doesn't matter, as `exam-gen` iterates over all
files and picks them at random.

## Example

Compile `exam-gen` and then navigate to `example/` and run the following 
command:

`exam-gen --config-file=example.toml`

## Todos

- Improve code base. At the moment it's rather hacky.
- Refactor configuration names.
- Move stuff from the configuration to the CLI, for example the number of tests to compile.