// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

#[derive(Debug)]
enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function as described above.
    // pub fn transformer(input: ???) -> ??? { ??? }
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();

        for (text, cmd) in input {
            let new_string = match cmd {
                Command::Uppercase => text.to_uppercase(),
                Command::Trim => text.trim().to_string(),
                Command::Append(size) => text + &"bar".repeat(size),
            };
            results.push(new_string);
        }

        //while let Some(value) = input.pop() {
        //    let (mut text, cmd) = value;
        //    match cmd {
        //        Command::Uppercase => results.insert(0, text.to_uppercase()),
        //        Command::Trim => results.insert(0, text.trim().to_string()),
        //        Command::Append(mut size) => {
        //            while size > 0 {
        //                text.push_str("bar");
        //                size -= 1;
        //            }
        //            results.insert(0, text);
        //        }
        //    }
        //}

        results
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    // use ???;
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
