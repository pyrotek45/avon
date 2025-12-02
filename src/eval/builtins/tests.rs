#[cfg(test)]
mod tests {
    use crate::common::{Number, Value};
    use crate::eval::{eval, initial_builtins};
    use crate::lexer::tokenize;
    use crate::parser::parse;

    fn eval_prog(prog: &str) -> Value {
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        eval(ast.program, &mut symbols, prog).expect("eval")
    }

    #[test]
    fn test_math_extended() {
        match eval_prog("abs (0 - 5)") {
            Value::Number(Number::Int(5)) => {}
            v => panic!("expected 5, got {:?}", v),
        }
        match eval_prog("gcd 12 15") {
            Value::Number(Number::Int(3)) => {}
            v => panic!("expected 3, got {:?}", v),
        }
        match eval_prog("lcm 4 6") {
            Value::Number(Number::Int(12)) => {}
            v => panic!("expected 12, got {:?}", v),
        }
    }

    #[test]
    fn test_aggregate_extended() {
        match eval_prog("product [1, 2, 3, 4]") {
            Value::Number(Number::Int(24)) => {}
            v => panic!("expected 24, got {:?}", v),
        }
    }

    #[test]
    fn test_list_extended() {
        // windows
        match eval_prog("windows 2 [1, 2, 3]") {
            Value::List(list) => {
                assert_eq!(list.len(), 2);
                // [[1, 2], [2, 3]]
            }
            v => panic!("expected list, got {:?}", v),
        }

        // chunks
        match eval_prog("chunks 2 [1, 2, 3, 4, 5]") {
            Value::List(list) => {
                assert_eq!(list.len(), 3);
                // [[1, 2], [3, 4], [5]]
            }
            v => panic!("expected list, got {:?}", v),
        }

        // transpose
        match eval_prog("transpose [[1, 2], [3, 4]]") {
            Value::List(list) => {
                // [[1, 3], [2, 4]]
                assert_eq!(list.len(), 2);
            }
            v => panic!("expected list, got {:?}", v),
        }

        // permutations
        match eval_prog("permutations 2 [1, 2, 3]") {
            Value::List(list) => {
                assert_eq!(list.len(), 6);
            }
            v => panic!("expected list, got {:?}", v),
        }

        // combinations
        match eval_prog("combinations 2 [1, 2, 3]") {
            Value::List(list) => {
                assert_eq!(list.len(), 3);
            }
            v => panic!("expected list, got {:?}", v),
        }
    }

    #[test]
    fn test_regex() {
        match eval_prog("regex_match \"^a.*z$\" \"abcz\"") {
            Value::Bool(true) => {}
            v => panic!("expected true, got {:?}", v),
        }
        match eval_prog("regex_replace \"foo\" \"bar\" \"foo baz\"") {
            Value::String(s) => assert_eq!(s, "bar baz"),
            v => panic!("expected string, got {:?}", v),
        }
        match eval_prog("regex_split \",\" \"a,b,c\"") {
            Value::List(list) => assert_eq!(list.len(), 3),
            v => panic!("expected list, got {:?}", v),
        }
        match eval_prog("scan \"\\d+\" \"123 456\"") {
            Value::List(list) => assert_eq!(list.len(), 2),
            v => panic!("expected list, got {:?}", v),
        }
    }

    #[test]
    fn test_csv_formatting() {
        // Test list of dicts (with headers)
        let prog = r#"
            let data = [
                {name: "Alice", age: 30},
                {name: "Bob", age: 25}
            ] in
            format_csv data
        "#;
        match eval_prog(prog) {
            Value::String(s) => {
                assert!(s.contains("name,age") || s.contains("age,name"));
                assert!(s.contains("Alice,30") || s.contains("30,Alice"));
                assert!(s.contains("Bob,25") || s.contains("25,Bob"));
            }
            v => panic!("expected string, got {:?}", v),
        }

        // Test list of lists (no headers)
        let prog = r#"
            let data = [
                ["Alice", 30],
                ["Bob", 25]
            ] in
            format_csv data
        "#;
        match eval_prog(prog) {
            Value::String(s) => {
                assert!(s.contains("Alice,30"));
                assert!(s.contains("Bob,25"));
                assert!(!s.contains("name")); // No headers
            }
            v => panic!("expected string, got {:?}", v),
        }
    }

    #[test]
    fn test_csv_parsing() {
        use std::io::Write;

        // Create a temp csv file
        let path = "test_data.csv";
        let mut file = std::fs::File::create(path).expect("create file");
        file.write_all(b"name,age\nAlice,30\nBob,25")
            .expect("write file");

        let prog = format!("csv_parse \"{}\"", path);
        let result = eval_prog(&prog);

        // Clean up
        std::fs::remove_file(path).expect("remove file");

        match result {
            Value::List(rows) => {
                assert_eq!(rows.len(), 2);
                if let Value::Dict(d) = &rows[0] {
                    match d.get("name").unwrap() {
                        Value::String(s) => assert_eq!(s, "Alice"),
                        _ => panic!("expected string"),
                    }
                    match d.get("age").unwrap() {
                        Value::String(s) => assert_eq!(s, "30"),
                        _ => panic!("expected string"),
                    }
                } else {
                    panic!("expected dict row");
                }
            }
            v => panic!("expected list, got {:?}", v),
        }
    }
}
