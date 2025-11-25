mod cli;
mod common;
mod eval;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let code = cli::run_cli(args);
    std::process::exit(code);
}

#[cfg(test)]
mod tests {
    use crate::common::*;
    use crate::eval::{apply_function, collect_file_templates, eval, initial_builtins};
    use crate::lexer::tokenize;
    use crate::parser::parse;
    use std::collections::HashMap;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_readfile_builtin() {
        let dir = std::env::temp_dir();
        let file_path = dir.join("avon_test_read.txt");
        let mut f = fs::File::create(&file_path).expect("create temp file");
        write!(f, "hello-from-file").expect("write");

        let prog = format!("readfile \"{}\"", file_path.to_string_lossy());
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::String(s) => assert!(s.contains("hello-from-file")),
            other => panic!("expected string, got {:?}", other),
        }
        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn test_exists_builtin() {
        let dir = std::env::temp_dir();
        let file_path = dir.join("avon_test_exists.txt");
        let _ = fs::remove_file(&file_path);
        let prog = format!("exists \"{}\"", file_path.to_string_lossy());
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(!b),
            other => panic!("expected bool, got {:?}", other),
        }

        // create the file and test true
        let mut f = fs::File::create(&file_path).expect("create");
        write!(f, "x").expect("write");
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b),
            other => panic!("expected bool, got {:?}", other),
        }
        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn test_fill_template_builtin() {
        use std::io::Write;
        let dir = std::env::temp_dir();
        let template_path = dir.join("avon_test_template.txt");
        
        // Create a template file with placeholders
        let mut f = fs::File::create(&template_path).expect("create temp template");
        write!(f, "Hello, {{name}}! Your email is {{email}}.").expect("write");
        drop(f);

        // Test fill_template with substitutions
        let prog = format!(
            "fill_template \"{}\" [[\"name\", \"Alice\"], [\"email\", \"alice@example.com\"]]",
            template_path.to_string_lossy()
        );
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        
        match v {
            Value::String(s) => {
                assert_eq!(s, "Hello, Alice! Your email is alice@example.com.");
            }
            other => panic!("expected string, got {:?}", other),
        }
        
        let _ = fs::remove_file(template_path);
    }

    #[test]
    fn test_path_as_value() {
        use std::io::Write;
        let dir = std::env::temp_dir();
        let file_path = dir.join("avon_test_path.txt");
        
        // Create a test file
        let mut f = fs::File::create(&file_path).expect("create temp file");
        write!(f, "content-from-path").expect("write");
        drop(f);

        // Test: store path in variable and use with readfile
        let prog = format!(
            "let p = @{} in readfile p",
            file_path.to_string_lossy()
        );
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        
        match v {
            Value::String(s) => {
                assert_eq!(s, "content-from-path");
            }
            other => panic!("expected string, got {:?}", other),
        }
        
        // Test: path with interpolation
        let dir_str = dir.to_string_lossy().to_string().replace("\\", "/");
        let prog2 = format!(
            "let name = \"avon_test_path\" in let p = @{}/{{name}}.txt in readfile p",
            dir_str
        );
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        
        match v2 {
            Value::String(s) => {
                assert_eq!(s, "content-from-path");
            }
            other => panic!("expected string, got {:?}", other),
        }
        
        // Test: path with exists, basename, dirname
        let prog3 = format!("let p = @{} in exists p", file_path.to_string_lossy());
        let tokens3 = tokenize(prog3.clone()).expect("tokenize");
        let ast3 = parse(tokens3);
        let mut symbols3 = initial_builtins();
        let v3 = eval(ast3.program, &mut symbols3, &prog3).expect("eval");
        
        match v3 {
            Value::Bool(b) => assert!(b),
            other => panic!("expected bool, got {:?}", other),
        }
        
        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn test_path_with_readlines() {
        use std::io::Write;
        let dir = std::env::temp_dir();
        let file_path = dir.join("avon_test_readlines.txt");
        
        // Create a multi-line test file
        let mut f = fs::File::create(&file_path).expect("create temp file");
        write!(f, "line1\nline2\nline3").expect("write");
        drop(f);

        // Test: path with readlines
        let prog = format!(
            "let p = @{} in readlines p",
            file_path.to_string_lossy()
        );
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        
        match v {
            Value::List(lines) => {
                assert_eq!(lines.len(), 3);
                assert_eq!(lines[0].to_string(&prog), "line1");
                assert_eq!(lines[1].to_string(&prog), "line2");
                assert_eq!(lines[2].to_string(&prog), "line3");
            }
            other => panic!("expected list, got {:?}", other),
        }
        
        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn test_path_with_basename_dirname() {
        // Test: basename with path value
        let prog = "let p = @/home/user/config.json in basename p";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        
        match v {
            Value::String(s) => {
                assert_eq!(s, "config.json");
            }
            other => panic!("expected string, got {:?}", other),
        }

        // Test: dirname with path value
        let prog2 = "let p = @/home/user/config.json in dirname p";
        let tokens2 = tokenize(prog2.to_string()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, prog2).expect("eval");
        
        match v2 {
            Value::String(s) => {
                assert!(s.contains("home") || s.contains("user"));
            }
            other => panic!("expected string, got {:?}", other),
        }
    }

    #[test]
    fn test_path_with_exists() {
        use std::io::Write;
        let dir = std::env::temp_dir();
        let existing_file = dir.join("avon_test_exists_true.txt");
        let missing_file = dir.join("avon_test_exists_false_missing.txt");
        
        // Create one file
        let mut f = fs::File::create(&existing_file).expect("create temp file");
        write!(f, "exists").expect("write");
        drop(f);
        
        // Ensure the other doesn't exist
        let _ = fs::remove_file(&missing_file);

        // Test: exists returns true for existing file
        let prog = format!(
            "let p = @{} in exists p",
            existing_file.to_string_lossy()
        );
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        
        match v {
            Value::Bool(b) => assert!(b, "expected true for existing file"),
            other => panic!("expected bool, got {:?}", other),
        }

        // Test: exists returns false for missing file
        let prog2 = format!(
            "let p = @{} in exists p",
            missing_file.to_string_lossy()
        );
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        
        match v2 {
            Value::Bool(b) => assert!(!b, "expected false for missing file"),
            other => panic!("expected bool, got {:?}", other),
        }
        
        let _ = fs::remove_file(existing_file);
    }

    #[test]
    fn test_path_with_fill_template() {
        use std::io::Write;
        let dir = std::env::temp_dir();
        let template_path = dir.join("avon_test_fill_path.txt");
        
        // Create a template with placeholders
        let mut f = fs::File::create(&template_path).expect("create temp file");
        write!(f, "Hello {{name}}, you are {{age}} years old.").expect("write");
        drop(f);

        // Test: fill_template with path value
        let prog = format!(
            "let p = @{} in fill_template p [[\"name\", \"Bob\"], [\"age\", \"25\"]]",
            template_path.to_string_lossy()
        );
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        
        match v {
            Value::String(s) => {
                assert_eq!(s, "Hello Bob, you are 25 years old.");
            }
            other => panic!("expected string, got {:?}", other),
        }
        
        let _ = fs::remove_file(template_path);
    }

    #[test]
    fn test_path_with_import() {
        use std::io::Write;
        let dir = std::env::temp_dir();
        let module_path = dir.join("avon_test_import_module.av");
        
        // Create a module file that returns a value
        let mut f = fs::File::create(&module_path).expect("create temp file");
        write!(f, "[\"imported\", \"data\"]").expect("write");
        drop(f);

        // Test: import with path value
        let prog = format!(
            "let p = @{} in import p",
            module_path.to_string_lossy()
        );
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 2);
                assert_eq!(items[0].to_string(&prog), "imported");
                assert_eq!(items[1].to_string(&prog), "data");
            }
            other => panic!("expected list, got {:?}", other),
        }
        
        let _ = fs::remove_file(module_path);
    }

    #[test]
    fn test_path_interpolation_multiple_vars() {
        use std::io::Write;
        let dir = std::env::temp_dir();
        let dir_str = dir.to_string_lossy().to_string().replace("\\", "/");
        
        // Create a test file with specific name pattern
        let file_path = dir.join("config_prod_app.json");
        let mut f = fs::File::create(&file_path).expect("create temp file");
        write!(f, "{{\"env\":\"production\"}}").expect("write");
        drop(f);

        // Test: path with multiple variable interpolations
        let prog = format!(
            "let env = \"prod\" in let name = \"app\" in let p = @{}/config_{{env}}_{{name}}.json in readfile p",
            dir_str
        );
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        
        match v {
            Value::String(s) => {
                assert!(s.contains("production"));
            }
            other => panic!("expected string, got {:?}", other),
        }
        
        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn test_examples_map_fold() {
        // map example
        let data = fs::read_to_string("examples/map_example.av").expect("read example");
        let tokens = tokenize(data.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &data).expect("eval");
        assert_eq!(v.to_string(&data), "[a-, b-, c-]");

        // fold example
        let data2 = fs::read_to_string("examples/fold_example.av").expect("read example");
        let tokens2 = tokenize(data2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &data2).expect("eval");
        // fold result should be a string concatenation
        assert!(v2.to_string(&data2).contains("a"));
    }

    #[test]
    fn test_let_cascade_and_filetemplate_collection() {
        let data = fs::read_to_string("examples/let_cascade.av").expect("read example");
        let tokens = tokenize(data.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &data).expect("eval");
        // collect_file_templates should accept a single FileTemplate from nested lets
        let files = collect_file_templates(&v, &data).expect("collect");
        assert!(files.len() == 1, "expected one filetemplate");
        let (path, content) = &files[0];
        assert!(path.contains("let_cascade.txt"));
        assert!(content.contains("Hello"));
        assert!(content.contains("World"));
    }

    #[test]
    fn test_comparisons_and_booleans() {
        let tests = vec![
            ("1 == 1", true),
            ("1 != 2", true),
            ("2 > 1", true),
            ("1 < 2", true),
            ("2 >= 2", true),
            ("1 <= 1", true),
        ];
        for (prog, expected) in tests {
            let tokens = tokenize(prog.to_string()).expect("tokenize");
            let ast = parse(tokens);
            let mut symbols = initial_builtins();
            let v = eval(ast.program, &mut symbols, prog).expect("eval");
            match v {
                Value::Bool(b) => assert_eq!(b, expected),
                other => panic!("expected bool, got {:?}", other),
            }
        }
    }

    #[test]
    fn test_deploy_uses_defaults() {
        // examples/deploy_list.av has a default for `name` => should produce two filetemplates when evaluated and defaults applied
        let data = fs::read_to_string("examples/deploy_list.av").expect("read example");
        let tokens = tokenize(data.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let mut v = eval(ast.program, &mut symbols, &data).expect("eval");

        // emulate the deploy application loop: apply defaults when present
        loop {
            match &v {
                Value::Function {
                    ident: _, default, ..
                } => {
                    if let Some(def) = default {
                        v = apply_function(&v, (**def).clone(), &data).expect("apply default");
                        continue;
                    } else {
                        panic!("expected default to be present for test");
                    }
                }
                _ => break,
            }
        }

        let files = collect_file_templates(&v, &data).expect("collect");
        assert!(files.len() >= 1, "expected at least one filetemplate");
    }

    #[test]
    fn test_readlines_and_json_and_import_and_walkdir() {
        // readlines
        let dir = std::env::temp_dir();
        let file_path = dir.join("avon_test_lines.txt");
        let mut f = fs::File::create(&file_path).expect("create temp file");
        write!(f, "line1\nline2\n").expect("write");
        let prog = format!("readlines \"{}\"", file_path.to_string_lossy());
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::List(items) => assert_eq!(items.len(), 2),
            other => panic!("expected list, got {:?}", other),
        }

        // json_parse array
        let json_path = dir.join("avon_test_json_array.json");
        let mut jf = fs::File::create(&json_path).expect("create json");
        write!(jf, "[1,2,3]").expect("write json");
        let progj = format!("json_parse \"{}\"", json_path.to_string_lossy());
        let tokens = tokenize(progj.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &progj).expect("eval");
        match v {
            Value::List(items) => assert_eq!(items.len(), 3),
            other => panic!("expected list from json array, got {:?}", other),
        }

        // json_parse object (now returns list of [key, value] pairs)
        let json_obj_path = dir.join("avon_test_json_obj.json");
        let mut jo = fs::File::create(&json_obj_path).expect("create json obj");
        write!(jo, "{}", r#"{"k": "v"}"#).expect("write json obj");
        let progo = format!("json_parse \"{}\"", json_obj_path.to_string_lossy());
        let tokens = tokenize(progo.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &progo).expect("eval");
        match v {
            Value::List(items) => {
                // Should be list of pairs like [["k", "v"]]
                assert_eq!(items.len(), 1);
            }
            other => panic!("expected list of pairs from json object, got {:?}", other),
        }

        // import: create a small file that evaluates to a number
        let imp_path = dir.join("avon_test_import.av");
        let mut impf = fs::File::create(&imp_path).expect("create import");
        write!(impf, "42").expect("write import");
        let progi = format!("import \"{}\"", imp_path.to_string_lossy());
        let tokens = tokenize(progi.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &progi).expect("eval");
        match v {
            Value::Number(Number::Int(i)) => assert_eq!(i, 42),
            other => panic!("expected number from import, got {:?}", other),
        }

        // walkdir: create a temp dir with files
        let base = dir.join("avon_walk_test_dir");
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).expect("mkdir");
        std::fs::write(base.join("a.txt"), "x").expect("write a");
        std::fs::create_dir_all(base.join("sub")).expect("mkdir sub");
        std::fs::write(base.join("sub/b.txt"), "y").expect("write b");
        let progd = format!("walkdir \"{}\"", base.to_string_lossy());
        let tokens = tokenize(progd.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &progd).expect("eval");
        match v {
            Value::List(items) => assert!(items.len() >= 2),
            other => panic!("expected list from walkdir, got {:?}", other),
        }
    }

    #[test]
    fn test_map_filter_fold_and_template_interpolation() {
        // map: add suffix
        let prog = "map (\\x concat x \"-\") [\"a\",\"b\",\"c\"]".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "[a-, b-, c-]");

        // filter: keep non-empty strings example
        let progf = "filter (\\x (x != \"\")) [\"\", \"x\"]".to_string();
        let tokens = tokenize(progf.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &progf).expect("eval");
        assert_eq!(v.to_string(&progf), "[x]");

        // fold: concatenate
        let progd = "fold (\\a \\b concat a b) \"\" [\"a\",\"b\"]".to_string();
        let tokens = tokenize(progd.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &progd).expect("eval");
        assert!(v.to_string(&progd).contains("ab"));

        // template list interpolation
        // Harder to craft template tokens directly; instead use an example file that contains template interpolation
        let data = fs::read_to_string("examples/list_insert.av").expect("read example");
        let tokens = tokenize(data.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &data).expect("eval");
        // result should be a FileTemplate (from examples) or at least a Template
        match v {
            Value::FileTemplate { .. }
            | Value::Template(_, _)
            | Value::List(_)
            | Value::Function { .. } => {}
            other => panic!("unexpected template eval result: {:?}", other),
        }
    }

    #[test]
    fn test_default_params_apply_on_deploy_emulation() {
        // Function with defaults for name and age; emulated deploy should apply defaults
        let prog =
            r#"\name ? "alice" \age ? "30" @/tmp/{name}_{age}.txt {"Name: {name}\nAge: {age}\n"}"#
                .to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let mut v = eval(ast.program, &mut symbols, &prog).expect("eval");

        // emulate deploy application loop: apply defaults when present
        loop {
            match &v {
                Value::Function {
                    ident: _, default, ..
                } => {
                    if let Some(def) = default {
                        v = apply_function(&v, (**def).clone(), &prog).expect("apply default");
                        continue;
                    } else {
                        panic!("expected default to be present for test");
                    }
                }
                _ => break,
            }
        }

        let files = collect_file_templates(&v, &prog).expect("collect");
        assert!(files.len() >= 1, "expected at least one filetemplate");
    }

    #[test]
    fn test_named_deploy_arg_binds_by_parameter_name() {
        // Function expecting named params; simulate supplying a named arg during deploy
        let prog = r#"\name \age ? "99" @/tmp/{name}_{age}.txt {"N:{name} A:{age}"}"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let mut v = eval(ast.program, &mut symbols, &prog).expect("eval");

        // simulate named args from CLI: -name bob
        let mut deploy_named: HashMap<String, String> = HashMap::new();
        deploy_named.insert("name".to_string(), "bob".to_string());

        // emulate the deploy application loop which prefers named args by param ident
        loop {
            match &v {
                Value::Function { ident, default, .. } => {
                    if let Some(named_val) = deploy_named.remove(ident) {
                        v = apply_function(&v, Value::String(named_val), &prog)
                            .expect("apply named");
                        continue;
                    } else if let Some(def) = default {
                        // if default present (not in this test), it would be applied
                        v = apply_function(&v, (**def).clone(), &prog).expect("apply default");
                        continue;
                    } else {
                        panic!("missing argument in test (no positional args provided)");
                    }
                }
                _ => break,
            }
        }

        let files = collect_file_templates(&v, &prog).expect("collect");
        // ensure the produced path contains the supplied named value
        assert!(
            files.iter().any(|(p, _)| p.contains("bob")),
            "expected output path to contain 'bob'"
        );
    }

    #[test]
    fn test_dedent_removes_common_indentation_for_templates() {
        // create a template with leading indentation in the source; dedent should remove it
        let prog =
            "@/tmp/dedent_test.txt {\"\n        line1\n            line2\n        \"}".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        let files = collect_file_templates(&v, &prog).expect("collect");
        assert_eq!(files.len(), 1);
        let (_path, content) = &files[0];
        // dedent should remove the common 8-space indent and preserve relative indent
        assert!(content.contains("line1"));
        assert!(content.contains("line2"));
        // ensure there's no leading 8-space prefix on lines
        for line in content.lines() {
            assert!(!line.starts_with("        "));
        }
    }

    #[test]
    fn test_examples_are_readable_and_tokenize() {
        let examples = vec![
            "examples/deploy_list.av",
            "examples/map_example.av",
            "examples/list_insert.av",
            "examples/test.av",
            "examples/import_example.av",
            "examples/string_functions.av",
            "examples/conditionals_template.av",
            "examples/function_defaults.av",
            "examples/join_replace.av",
            "examples/contains_starts_ends.av",
            "examples/nested_let.av",
            "examples/named_args.av",
            "examples/split_join.av",
            "examples/nginx_config.av",
            "examples/neovim_init.av",
            "examples/emacs_init.av",
            "examples/docker_compose.av",
        ];
        for ex in examples {
            let data = std::fs::read_to_string(ex).expect("read example");
            tokenize(data).expect("tokenize example");
        }
    }

    // Complex dedent tests
    #[test]
    fn test_dedent_preserves_relative_indentation() {
        // nested indentation: level1 (4 spaces), level2 (8 spaces)
        let prog =
            "@/tmp/dedent_rel.txt {\"\n    level1\n        level2\n    level1b\n\"}".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        let files = collect_file_templates(&v, &prog).expect("collect");
        assert_eq!(files.len(), 1);
        let (_p, content) = &files[0];
        let mut found_level1 = false;
        let mut found_level2 = false;
        for line in content.lines() {
            if line == "level1" {
                found_level1 = true;
            }
            if line == "    level2" {
                found_level2 = true;
            }
        }
        assert!(found_level1, "level1 missing");
        assert!(
            found_level2,
            "level2 should be indented 4 spaces relative to dedented base"
        );
    }

    #[test]
    fn test_dedent_handles_mixed_tabs_and_spaces() {
        // use tabs for indentation baseline; inner line adds spaces after tabs
        let prog = "@/tmp/dedent_tabs.txt {\"\n\t\tfoo\n\t\t\tbar\n\t\tbaz\n\"}".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        let files = collect_file_templates(&v, &prog).expect("collect");
        assert_eq!(files.len(), 1);
        let (_p, content) = &files[0];
        // After dedent, first content line should start with "foo" (no leading tabs)
        let lines: Vec<&str> = content.lines().collect();
        assert!(lines.len() >= 3);
        assert_eq!(lines[0], "foo");
        // the 'bar' line should start with one tab preserved
        assert!(
            lines[1].starts_with("\t"),
            "bar should preserve its extra tab"
        );
    }

    #[test]
    fn test_dedent_trims_leading_and_trailing_blank_lines() {
        let prog = "@/tmp/dedent_trim.txt {\"\n\n        hello world\n\n\"}".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        let files = collect_file_templates(&v, &prog).expect("collect");
        assert_eq!(files.len(), 1);
        let (_p, content) = &files[0];
        // trimmed content should equal the single line without surrounding blank lines
        assert_eq!(content.trim(), "hello world");
    }

    #[test]
    fn test_template_brace_count_interpolation() {
        // single-brace interpolation
        let prog = r#"let hello = "WORLD" in {"A {hello} B"}"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "A WORLD B");

        // double-brace template: literal single braces, interpolation uses {{}}
        let prog2 = r#"let hello = "WORLD" in {{"X {{hello}} Y"}}"#.to_string();
        let tokens = tokenize(prog2.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog2).expect("eval");
        assert_eq!(v.to_string(&prog2), "X WORLD Y");

        // double-brace template with single-brace literal preserved
        let prog3 = r#"{{"literal {hello} here"}}"#.to_string();
        let tokens = tokenize(prog3.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog3).expect("eval");
        assert_eq!(v.to_string(&prog3), "literal {hello} here");
    }

    #[test]
    fn test_template_brace_count_1() {
        let n = 1;
        let open = "{".repeat(n);
        let close = "}".repeat(n);
        let interp = format!("{}x{}", open, close);
        let prog = format!("let x = \"X\" in {}\"val {} end\"{}", open, interp, close);
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "val X end");
    }

    #[test]
    fn test_template_brace_count_2() {
        let n = 2;
        let open = "{".repeat(n);
        let close = "}".repeat(n);
        let interp = format!("{}x{}", open, close);
        let prog = format!("let x = \"X\" in {}\"val {} end\"{}", open, interp, close);
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "val X end");
    }

    #[test]
    fn test_template_brace_count_3() {
        let n = 3;
        let open = "{".repeat(n);
        let close = "}".repeat(n);
        let interp = format!("{}x{}", open, close);
        let prog = format!("let x = \"X\" in {}\"val {} end\"{}", open, interp, close);
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "val X end");
    }

    #[test]
    fn test_template_brace_count_4() {
        let n = 4;
        let open = "{".repeat(n);
        let close = "}".repeat(n);
        let interp = format!("{}x{}", open, close);
        let prog = format!("let x = \"X\" in {}\"val {} end\"{}", open, interp, close);
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "val X end");
    }

    #[test]
    fn test_template_brace_count_5() {
        let n = 5;
        let open = "{".repeat(n);
        let close = "}".repeat(n);
        let interp = format!("{}x{}", open, close);
        let prog = format!("let x = \"X\" in {}\"val {} end\"{}", open, interp, close);
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "val X end");
    }

    #[test]
    fn test_operators_across_types() {
        // numeric addition
        let prog1 = "1 + 2".to_string();
        let tokens = tokenize(prog1.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog1).expect("eval");
        assert_eq!(v.to_string(&prog1), "3");

        // string concatenation
        let prog2 = "\"a\" + \"b\"".to_string();
        let tokens = tokenize(prog2.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog2).expect("eval");
        assert_eq!(v.to_string(&prog2), "ab");

        // list concatenation
        let prog3 = "[\"1\"] + [\"2\"]".to_string();
        let tokens = tokenize(prog3.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog3).expect("eval");
        assert_eq!(v.to_string(&prog3), "[1, 2]");

        // numeric subtraction works, but string subtraction should error
        let prog4 = "5 - 2".to_string();
        let tokens = tokenize(prog4.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog4).expect("eval");
        assert_eq!(v.to_string(&prog4), "3");

        let prog_err = "\"a\" - \"b\"".to_string();
        let tokens = tokenize(prog_err.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let r = eval(ast.program, &mut symbols, &prog_err);
        assert!(r.is_err(), "expected error when subtracting strings");
    }

    // NEW COMPREHENSIVE TESTS FOR BRACES AND TEMPLATES

    #[test]
    fn test_single_brace_template_with_literal_braces() {
        // In single-brace template {" "}, {{ produces {, }} produces }
        let prog = r#"{"literal {{ and }} here"}"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "literal { and } here");
    }

    #[test]
    fn test_single_brace_template_with_three_braces() {
        // In single-brace template, {{{ produces {{
        let prog = r#"{"pattern {{{version}}}"}"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "pattern {{version}}");
    }

    #[test]
    fn test_double_brace_template_with_literal_single_braces() {
        // In double-brace template {{""}}, { and } are literals
        let prog = r#"{{"JSON { key: value }"}}"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "JSON { key: value }");
    }

    #[test]
    fn test_double_brace_template_with_interpolation() {
        // In double-brace template, {{var}} interpolates
        let prog = r#"let name = "Bob" in {{"Hello {{name}}!"}}"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "Hello Bob!");
    }

    #[test]
    fn test_github_actions_style_dollar_braces() {
        // Simulates ${{ github.ref }} in single-brace template
        let prog = r#"{"workflow: ${{{ github.ref }}}"}"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "workflow: ${{ github.ref }}");
    }

    #[test]
    fn test_javascript_object_literal_in_template() {
        // JavaScript object { key: value } should be preserved with escaping
        let prog = r#"{"script: func(){{ return {{ key: 'val' }}; }}"}"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(
            v.to_string(&prog),
            "script: func(){ return { key: 'val' }; }"
        );
    }

    #[test]
    fn test_nested_json_with_escaped_braces() {
        // Nested JSON objects with proper brace escaping
        let prog = r#"{"{{ "outer": {{ "inner": {{ "key": "value" }} }} }}"}"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(
            v.to_string(&prog),
            r#"{ "outer": { "inner": { "key": "value" } } }"#
        );
    }

    #[test]
    fn test_quotes_inside_templates_dont_close_template() {
        // Quotes inside template should be literal, not close the template
        let prog = r#"{"line with "quoted" text"}"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "line with \"quoted\" text");
    }

    #[test]
    fn test_template_with_mixed_interpolation_and_escapes() {
        // Mix of interpolation and literal braces
        let prog = r#"let x = "VAL" in {"start {{ literal {x} interpolated }} end"}"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "start { literal VAL interpolated } end");
    }

    #[test]
    fn test_complex_curly_brace_escaping_patterns() {
        // Test various brace patterns in single template
        let prog = r#"{"one: {{, two: {{{, three: {{{{, four: {{{{{"}"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        // {{->{ {{{->{{ {{{{->{{{ {{{{{->{{{{
        assert_eq!(
            v.to_string(&prog),
            "one: {, two: {{, three: {{{, four: {{{{"
        );
    }

    #[test]
    fn test_is_digit() {
        let prog = r#"is_digit "123""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b),
            other => panic!("expected true, got {:?}", other),
        }

        let prog2 = r#"is_digit "12a3""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        match v2 {
            Value::Bool(b) => assert!(!b),
            other => panic!("expected false, got {:?}", other),
        }
    }

    #[test]
    fn test_is_alpha() {
        let prog = r#"is_alpha "hello""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b),
            other => panic!("expected true, got {:?}", other),
        }

        let prog2 = r#"is_alpha "hello123""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        match v2 {
            Value::Bool(b) => assert!(!b),
            other => panic!("expected false, got {:?}", other),
        }
    }

    #[test]
    fn test_is_alphanumeric() {
        let prog = r#"is_alphanumeric "hello123""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b),
            other => panic!("expected true, got {:?}", other),
        }

        let prog2 = r#"is_alphanumeric "hello-123""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        match v2 {
            Value::Bool(b) => assert!(!b),
            other => panic!("expected false, got {:?}", other),
        }
    }

    #[test]
    fn test_is_whitespace() {
        let prog = "is_whitespace \"   \\t\\n\"".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b),
            other => panic!("expected true, got {:?}", other),
        }

        let prog2 = r#"is_whitespace "  a  ""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        match v2 {
            Value::Bool(b) => assert!(!b),
            other => panic!("expected false, got {:?}", other),
        }
    }

    #[test]
    fn test_is_uppercase() {
        let prog = r#"is_uppercase "HELLO""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b),
            other => panic!("expected true, got {:?}", other),
        }

        let prog2 = r#"is_uppercase "HeLLo""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        match v2 {
            Value::Bool(b) => assert!(!b),
            other => panic!("expected false, got {:?}", other),
        }
    }

    #[test]
    fn test_is_lowercase() {
        let prog = r#"is_lowercase "hello""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b),
            other => panic!("expected true, got {:?}", other),
        }

        let prog2 = r#"is_lowercase "Hello""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        match v2 {
            Value::Bool(b) => assert!(!b),
            other => panic!("expected false, got {:?}", other),
        }
    }

    #[test]
    fn test_is_empty() {
        let prog = r#"is_empty """#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b),
            other => panic!("expected true, got {:?}", other),
        }

        let prog2 = r#"is_empty "hello""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        match v2 {
            Value::Bool(b) => assert!(!b),
            other => panic!("expected false, got {:?}", other),
        }

        // Test with list
        let prog3 = "is_empty []".to_string();
        let tokens3 = tokenize(prog3.clone()).expect("tokenize");
        let ast3 = parse(tokens3);
        let mut symbols3 = initial_builtins();
        let v3 = eval(ast3.program, &mut symbols3, &prog3).expect("eval");
        match v3 {
            Value::Bool(b) => assert!(b),
            other => panic!("expected true, got {:?}", other),
        }
    }

    #[test]
    fn test_true_false_literals() {
        // Test that true and false work correctly as boolean literals
        let prog = r#"let x = true in if x then "yes" else "no""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "yes");

        let prog2 = r#"let x = false in if x then "yes" else "no""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "no");
    }

    // Comprehensive builtin function tests

    #[test]
    fn test_string_builtins_concat() {
        let prog = r#"concat "hello" " world""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "hello world");
    }

    #[test]
    fn test_string_builtins_upper_lower() {
        let prog = r#"upper "hello""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "HELLO");

        let prog2 = r#"lower "WORLD""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "world");
    }

    #[test]
    fn test_string_builtins_trim() {
        let prog = r#"trim "  hello  ""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "hello");
    }

    #[test]
    fn test_string_builtins_split_join() {
        let prog = r#"split "a,b,c" ",""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::List(items) => assert_eq!(items.len(), 3),
            other => panic!("expected list, got {:?}", other),
        }

        let prog2 = r#"join ["x", "y", "z"] "-""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "x-y-z");
    }

    #[test]
    fn test_string_builtins_replace() {
        let prog = r#"replace "hello world" "world" "avon""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "hello avon");
    }

    #[test]
    fn test_string_builtins_contains_starts_ends() {
        let prog = r#"contains "hello world" "wor""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b),
            other => panic!("expected bool, got {:?}", other),
        }

        let prog2 = r#"starts_with "hello" "hel""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        match v2 {
            Value::Bool(b) => assert!(b),
            other => panic!("expected bool, got {:?}", other),
        }

        let prog3 = r#"ends_with "hello" "lo""#.to_string();
        let tokens3 = tokenize(prog3.clone()).expect("tokenize");
        let ast3 = parse(tokens3);
        let mut symbols3 = initial_builtins();
        let v3 = eval(ast3.program, &mut symbols3, &prog3).expect("eval");
        match v3 {
            Value::Bool(b) => assert!(b),
            other => panic!("expected bool, got {:?}", other),
        }
    }

    #[test]
    fn test_string_builtins_length() {
        let prog = r#"length "hello""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Number(Number::Int(n)) => assert_eq!(n, 5),
            other => panic!("expected int, got {:?}", other),
        }
    }

    #[test]
    fn test_string_builtins_repeat() {
        let prog = r#"repeat "ab" 3"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "ababab");
    }

    #[test]
    fn test_string_builtins_pad() {
        let prog = r#"pad_left "hi" 5 " ""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "   hi");

        let prog2 = r#"pad_right "hi" 5 " ""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "hi   ");
    }

    #[test]
    fn test_string_builtins_indent() {
        let prog = "indent \"line1\\nline2\" 2".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "  line1\n  line2");
    }

    #[test]
    fn test_list_builtins_map() {
        let prog = r#"map (\x concat x "!") ["a", "b", "c"]"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 3);
                assert_eq!(items[0].to_string(&prog), "a!");
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn test_list_builtins_filter() {
        let prog = r#"filter (\x x > 5) [3, 7, 2, 9, 4]"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 2); // 7 and 9
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn test_list_builtins_fold() {
        let prog = r#"fold (\acc \x acc + x) 0 [1, 2, 3, 4]"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Number(Number::Int(n)) => assert_eq!(n, 10),
            other => panic!("expected int 10, got {:?}", other),
        }
    }

    #[test]
    fn test_list_builtins_flatmap() {
        let prog = r#"flatmap (\x [x, x]) [1, 2, 3]"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::List(items) => assert_eq!(items.len(), 6),
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn test_list_builtins_flatten() {
        let prog = r#"flatten [[1, 2], [3, 4], [5]]"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::List(items) => assert_eq!(items.len(), 5),
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn test_list_length() {
        let prog = r#"length [1, 2, 3, 4, 5]"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Number(Number::Int(n)) => assert_eq!(n, 5),
            other => panic!("expected int, got {:?}", other),
        }
    }

    #[test]
    fn test_map_get() {
        // Test get with existing key
        let prog = r#"get [["name", "Alice"], ["age", "30"]] "name""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "Alice");

        // Test get with missing key returns None
        let prog2 = r#"get [["name", "Alice"]] "missing""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "None");
    }

    #[test]
    fn test_map_set() {
        // Test set updates existing key
        let prog = r#"let m = set [["name", "Alice"], ["age", "30"]] "name" "Bob" in get m "name""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "Bob");

        // Test set adds new key
        let prog2 = r#"let m = set [["name", "Alice"]] "age" "30" in length m"#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        match v2 {
            Value::Number(Number::Int(n)) => assert_eq!(n, 2),
            other => panic!("expected int 2, got {:?}", other),
        }
    }

    #[test]
    fn test_map_keys() {
        let prog = r#"keys [["name", "Alice"], ["age", "30"], ["city", "NYC"]]"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 3);
                assert_eq!(items[0].to_string(&prog), "name");
                assert_eq!(items[1].to_string(&prog), "age");
                assert_eq!(items[2].to_string(&prog), "city");
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn test_map_values() {
        let prog = r#"values [["name", "Alice"], ["age", "30"], ["city", "NYC"]]"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 3);
                assert_eq!(items[0].to_string(&prog), "Alice");
                assert_eq!(items[1].to_string(&prog), "30");
                assert_eq!(items[2].to_string(&prog), "NYC");
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn test_map_has_key() {
        // Test has_key with existing key
        let prog = r#"has_key [["name", "Alice"], ["age", "30"]] "name""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b),
            other => panic!("expected bool true, got {:?}", other),
        }

        // Test has_key with missing key
        let prog2 = r#"has_key [["name", "Alice"]] "missing""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        match v2 {
            Value::Bool(b) => assert!(!b),
            other => panic!("expected bool false, got {:?}", other),
        }
    }

    #[test]
    fn test_json_parse_object_returns_map() {
        use std::io::Write;
        let dir = std::env::temp_dir();
        let json_path = dir.join("avon_test_json_map.json");
        
        // Create a JSON object
        let mut jf = fs::File::create(&json_path).expect("create json");
        write!(jf, r#"{{"name": "Alice", "age": 30}}"#).expect("write json");
        drop(jf);

        // Test that JSON objects are parsed as list of pairs and can be queried with get
        let prog = format!("let data = json_parse \"{}\" in get data \"name\"", json_path.to_string_lossy());
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "Alice");

        // Test that keys can be extracted
        let prog2 = format!("let data = json_parse \"{}\" in keys data", json_path.to_string_lossy());
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        match v2 {
            Value::List(items) => {
                assert_eq!(items.len(), 2);
            }
            other => panic!("expected list of keys, got {:?}", other),
        }
        
        let _ = fs::remove_file(json_path);
    }

    #[test]
    fn test_map_operations_chaining() {
        // Test chaining map operations
        let prog = r#"let m = [["a", "1"], ["b", "2"]] in let m2 = set m "c" "3" in let m3 = set m2 "a" "10" in get m3 "a""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "10");
    }

    #[test]
    fn test_path_builtins() {
        let prog = r#"basename "/path/to/file.txt""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "file.txt");

        let prog2 = r#"dirname "/path/to/file.txt""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "/path/to");
    }

    #[test]
    fn test_html_builtins() {
        let prog = r#"html_escape "<div>""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "&lt;div&gt;");

        let prog2 = r#"html_tag "div" "Hello""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "<div>Hello</div>");

        let prog3 = r#"html_attr "class" "btn""#.to_string();
        let tokens3 = tokenize(prog3.clone()).expect("tokenize");
        let ast3 = parse(tokens3);
        let mut symbols3 = initial_builtins();
        let v3 = eval(ast3.program, &mut symbols3, &prog3).expect("eval");
        assert_eq!(v3.to_string(&prog3), "class=\"btn\"");
    }

    #[test]
    fn test_os_builtin() {
        let prog = "os".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        // os should return "linux", "macos", or "windows"
        match v {
            Value::String(s) => assert!(s == "linux" || s == "macos" || s == "windows"),
            other => panic!("expected string, got {:?}", other),
        }
    }

    #[test]
    fn test_if_expr_with_predicates() {
        // Test if expression with all the new predicate functions
        let tests = vec![
            (r#"if is_digit "123" then "yes" else "no""#, "yes"),
            (r#"if is_digit "12a" then "yes" else "no""#, "no"),
            (r#"if is_alpha "abc" then "yes" else "no""#, "yes"),
            (r#"if is_alphanumeric "abc123" then "yes" else "no""#, "yes"),
            (r#"if is_uppercase "ABC" then "yes" else "no""#, "yes"),
            (r#"if is_lowercase "abc" then "yes" else "no""#, "yes"),
            (r#"if is_empty "" then "yes" else "no""#, "yes"),
            (r#"if is_empty [] then "yes" else "no""#, "yes"),
            (r#"if contains "hello" "ell" then "yes" else "no""#, "yes"),
            (
                r#"if starts_with "hello" "hel" then "yes" else "no""#,
                "yes",
            ),
            (r#"if ends_with "hello" "lo" then "yes" else "no""#, "yes"),
        ];

        for (prog_str, expected) in tests {
            let prog = prog_str.to_string();
            let tokens = tokenize(prog.clone()).expect("tokenize");
            let ast = parse(tokens);
            let mut symbols = initial_builtins();
            let v = eval(ast.program, &mut symbols, &prog).expect("eval");
            assert_eq!(v.to_string(&prog), expected, "Failed for: {}", prog_str);
        }
    }

    #[test]
    fn test_currying_builtins() {
        // Test that builtins support partial application (currying)
        let prog = r#"let add_suffix = concat " world" in add_suffix "hello""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), " worldhello");

        // Test currying with replace
        let prog2 = r#"let replace_world = replace "hello world" "world" in replace_world "avon""#
            .to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "hello avon");

        // Test currying with pad_left
        let prog3 = r#"let pad5 = pad_left "x" 5 in pad5 "-""#.to_string();
        let tokens3 = tokenize(prog3.clone()).expect("tokenize");
        let ast3 = parse(tokens3);
        let mut symbols3 = initial_builtins();
        let v3 = eval(ast3.program, &mut symbols3, &prog3).expect("eval");
        assert_eq!(v3.to_string(&prog3), "----x");

        // Test currying with map
        let prog4 = r#"let mapper = map (\x x + 1) in mapper [1, 2, 3]"#.to_string();
        let tokens4 = tokenize(prog4.clone()).expect("tokenize");
        let ast4 = parse(tokens4);
        let mut symbols4 = initial_builtins();
        let v4 = eval(ast4.program, &mut symbols4, &prog4).expect("eval");
        match v4 {
            Value::List(items) => {
                assert_eq!(items.len(), 3);
                match &items[0] {
                    Value::Number(Number::Int(n)) => assert_eq!(*n, 2),
                    other => panic!("expected 2, got {:?}", other),
                }
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    // test that test all builtin functions work without error
    #[test]
    fn test_all_builtins_no_error_and_correct_output() {
        let builtins = vec![
            ("is_digit \"123\"", "true"),
            ("is_alpha \"abc\"", "true"),
            ("is_alphanumeric \"abc123\"", "true"),
            ("is_whitespace \" \\t\\n\"", "true"),
            ("is_uppercase \"HELLO\"", "true"),
            ("is_lowercase \"hello\"", "true"),
            ("is_empty \"\"", "true"),
            ("concat \"hello\" \" world\"", "hello world"),
            ("upper \"hello\"", "HELLO"),
            ("lower \"WORLD\"", "world"),
            ("trim \"  hello  \"", "hello"),
            ("split \"a,b,c\" \",\"", "[a, b, c]"),
            ("join [\"x\", \"y\", \"z\"] \"-\"", "x-y-z"),
            ("replace \"hello world\" \"world\" \"avon\"", "hello avon"),
            ("contains \"hello world\" \"wor\"", "true"),
            ("starts_with \"hello\" \"hel\"", "true"),
            ("ends_with \"hello\" \"lo\"", "true"),
            ("length \"hello\"", "5"),
            ("repeat \"ab\" 3", "ababab"),
            ("pad_left \"hi\" 5 \" \"", "   hi"),
            ("pad_right \"hi\" 5 \" \"", "hi   "),
            ("indent \"line1\\nline2\" 2", "  line1\n  line2"),
            ("map (\\x concat x \"!\") [\"a\", \"b\", \"c\"]", "[a!, b!, c!]"),
            ("filter (\\x x > 5) [3, 7, 2, 9, 4]", "[7, 9]"),
            ("fold (\\acc \\x acc + x) 0 [1, 2, 3, 4]", "10"),
            ("flatmap (\\x [x, x]) [1, 2, 3]", "[1, 1, 2, 2, 3, 3]"),
            ("flatten [[1, 2], [3, 4], [5]]", "[1, 2, 3, 4, 5]"),
            ("length [1, 2, 3, 4, 5]", "5"),
            ("basename \"/path/to/file.txt\"", "file.txt"),
            ("dirname \"/path/to/file.txt\"", "/path/to"),
            ("html_escape \"<div>\"", "&lt;div&gt;"),
            ("html_tag \"div\" \"Hello\"", "<div>Hello</div>"),
            ("html_attr \"class\" \"btn\"", "class=\"btn\""),
            ("os", "linux|macos|windows"), // OS-specific output
        ];
        for (prog_str, expected_output) in builtins {
            let prog = prog_str.to_string();
            let tokens = tokenize(prog.clone()).expect("tokenize");
            let ast = parse(tokens);
            let mut symbols = initial_builtins();
            let r = eval(ast.program, &mut symbols, &prog);
            assert!(r.is_ok(), "builtin '{}' failed to eval", prog_str);
            let output = r.unwrap().to_string(&prog);
            if expected_output.contains('|') {
                // Handle OS-specific output
                let options: Vec<&str> = expected_output.split('|').collect();
                assert!(
                    options.contains(&output.as_str()),
                    "builtin '{}' produced unexpected output '{}'",
                    prog_str,
                    output
                );
            } else {
                assert_eq!(
                    output, expected_output,
                    "builtin '{}' produced unexpected output '{}'",
                    prog_str, output
                );
            }
        }
    }

    // Tests for new formatting functions

    #[test]
    fn test_format_hex() {
        let prog = "format_hex 255".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "ff");

        let prog2 = "format_hex 16".to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "10");
    }

    #[test]
    fn test_format_octal() {
        let prog = "format_octal 64".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "100");
    }

    #[test]
    fn test_format_binary() {
        let prog = "format_binary 15".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "1111");

        let prog2 = "format_binary 255".to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "11111111");
    }

    #[test]
    fn test_format_scientific() {
        let prog = "format_scientific 12345.6789 2".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert!(v.to_string(&prog).contains("e"));
        assert!(v.to_string(&prog).contains("1.23"));
    }

    #[test]
    fn test_format_bytes() {
        // Test bytes
        let prog = "format_bytes 512".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "512 B");

        // Test KB
        let prog2 = "format_bytes 2048".to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert!(v2.to_string(&prog2).contains("KB"));

        // Test MB
        let prog3 = "format_bytes 1536000".to_string();
        let tokens3 = tokenize(prog3.clone()).expect("tokenize");
        let ast3 = parse(tokens3);
        let mut symbols3 = initial_builtins();
        let v3 = eval(ast3.program, &mut symbols3, &prog3).expect("eval");
        assert!(v3.to_string(&prog3).contains("MB"));
    }

    #[test]
    fn test_format_list() {
        let prog = r#"format_list ["a", "b", "c"] ", ""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "a, b, c");

        let prog2 = r#"format_list [1, 2, 3] " | ""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "1 | 2 | 3");
    }

    #[test]
    fn test_format_table() {
        let prog = r#"format_table [["A", "B"], ["1", "2"], ["3", "4"]] " | ""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        let result = v.to_string(&prog);
        assert!(result.contains("A | B"));
        assert!(result.contains("1 | 2"));
        assert!(result.contains("3 | 4"));
    }

    #[test]
    fn test_format_json() {
        // Test string
        let prog = r#"format_json "hello""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "\"hello\"");

        // Test number
        let prog2 = "format_json 42".to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "42");

        // Test list
        let prog3 = r#"format_json [1, 2, 3]"#.to_string();
        let tokens3 = tokenize(prog3.clone()).expect("tokenize");
        let ast3 = parse(tokens3);
        let mut symbols3 = initial_builtins();
        let v3 = eval(ast3.program, &mut symbols3, &prog3).expect("eval");
        assert_eq!(v3.to_string(&prog3), "[1, 2, 3]");
    }

    #[test]
    fn test_format_currency() {
        let prog = r#"format_currency 19.99 "$""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "$19.99");

        let prog2 = r#"format_currency 100 "€""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "€100.00");
    }

    #[test]
    fn test_format_percent() {
        let prog = "format_percent 0.75 2".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "75.00%");

        let prog2 = "format_percent 0.856 1".to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "85.6%");
    }

    #[test]
    fn test_format_bool() {
        // Test yes/no format
        let prog = r#"format_bool (1 == 1) "yes/no""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "Yes");

        let prog2 = r#"format_bool (1 == 2) "yes/no""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "No");

        // Test on/off format
        let prog3 = r#"format_bool (5 > 3) "on/off""#.to_string();
        let tokens3 = tokenize(prog3.clone()).expect("tokenize");
        let ast3 = parse(tokens3);
        let mut symbols3 = initial_builtins();
        let v3 = eval(ast3.program, &mut symbols3, &prog3).expect("eval");
        assert_eq!(v3.to_string(&prog3), "On");

        // Test enabled format
        let prog4 = r#"format_bool (1 == 1) "enabled""#.to_string();
        let tokens4 = tokenize(prog4.clone()).expect("tokenize");
        let ast4 = parse(tokens4);
        let mut symbols4 = initial_builtins();
        let v4 = eval(ast4.program, &mut symbols4, &prog4).expect("eval");
        assert_eq!(v4.to_string(&prog4), "Enabled");
    }

    #[test]
    fn test_truncate() {
        let prog = r#"truncate "This is a long string" 10"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "This is...");

        // Test string shorter than max length
        let prog2 = r#"truncate "Short" 10"#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "Short");
    }

    #[test]
    fn test_center() {
        let prog = r#"center "Hi" 10"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        let result = v.to_string(&prog);
        assert_eq!(result.len(), 10);
        assert_eq!(result.trim(), "Hi");
        assert!(result.starts_with("    "));

        // Test string longer than width
        let prog2 = r#"center "VeryLongText" 5"#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "VeryLongText");
    }

    #[test]
    fn test_all_formatting_functions() {
        // Quick smoke test for all new formatting functions
        let tests = vec![
            ("format_hex 255", "ff"),
            ("format_octal 8", "10"),
            ("format_binary 7", "111"),
            ("format_bytes 1024", "1.00 KB"),
            ("format_list [\"a\", \"b\"] \",\"", "a,b"),
            ("format_json 123", "123"),
            ("format_currency 10 \"$\"", "$10.00"),
            ("format_percent 0.5 0", "50%"),
            ("truncate \"hello\" 3", "hel"),
            ("center \"x\" 3", " x "),
        ];

        for (prog_str, expected) in tests {
            let prog = prog_str.to_string();
            let tokens = tokenize(prog.clone()).expect("tokenize");
            let ast = parse(tokens);
            let mut symbols = initial_builtins();
            let v = eval(ast.program, &mut symbols, &prog).expect("eval");
            assert_eq!(v.to_string(&prog), expected, "Failed for: {}", prog_str);
        }
    }

    // ========== Type Checking and Debugging Functions Tests ==========

    #[test]
    fn test_typeof() {
        let tests = vec![
            (r#"typeof "hello""#, "String"),
            (r#"typeof 42"#, "Number"),
            (r#"typeof 3.14"#, "Number"),
            (r#"typeof [1, 2, 3]"#, "List"),
            (r#"typeof (1 == 1)"#, "Bool"),
            (r#"typeof (\x x + 1)"#, "Function"),
        ];

        for (prog_str, expected) in tests {
            let prog = prog_str.to_string();
            let tokens = tokenize(prog.clone()).expect("tokenize");
            let ast = parse(tokens);
            let mut symbols = initial_builtins();
            let v = eval(ast.program, &mut symbols, &prog).expect("eval");
            assert_eq!(v.to_string(&prog), expected, "Failed for: {}", prog_str);
        }
    }

    #[test]
    fn test_is_string() {
        let prog = r#"is_string "hello""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b),
            other => panic!("expected bool true, got {:?}", other),
        }

        let prog2 = r#"is_string 42"#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        match v2 {
            Value::Bool(b) => assert!(!b),
            other => panic!("expected bool false, got {:?}", other),
        }
    }

    #[test]
    fn test_is_number() {
        let prog = r#"is_number 42"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b),
            other => panic!("expected bool true, got {:?}", other),
        }

        let prog2 = r#"is_number 3.14"#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        match v2 {
            Value::Bool(b) => assert!(b),
            other => panic!("expected bool true, got {:?}", other),
        }

        let prog3 = r#"is_number "42""#.to_string();
        let tokens3 = tokenize(prog3.clone()).expect("tokenize");
        let ast3 = parse(tokens3);
        let mut symbols3 = initial_builtins();
        let v3 = eval(ast3.program, &mut symbols3, &prog3).expect("eval");
        match v3 {
            Value::Bool(b) => assert!(!b),
            other => panic!("expected bool false, got {:?}", other),
        }
    }

    #[test]
    fn test_is_int() {
        let prog = r#"is_int 42"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b),
            other => panic!("expected bool true, got {:?}", other),
        }

        let prog2 = r#"is_int 3.14"#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        match v2 {
            Value::Bool(b) => assert!(!b),
            other => panic!("expected bool false, got {:?}", other),
        }
    }

    #[test]
    fn test_is_float() {
        let prog = r#"is_float 3.14"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b),
            other => panic!("expected bool true, got {:?}", other),
        }

        let prog2 = r#"is_float 42"#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        match v2 {
            Value::Bool(b) => assert!(!b),
            other => panic!("expected bool false, got {:?}", other),
        }
    }

    #[test]
    fn test_is_list() {
        let prog = r#"is_list [1, 2, 3]"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b),
            other => panic!("expected bool true, got {:?}", other),
        }

        let prog2 = r#"is_list "not a list""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        match v2 {
            Value::Bool(b) => assert!(!b),
            other => panic!("expected bool false, got {:?}", other),
        }
    }

    #[test]
    fn test_is_bool() {
        let prog = r#"is_bool (1 == 1)"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b),
            other => panic!("expected bool true, got {:?}", other),
        }

        let prog2 = r#"is_bool 1"#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        match v2 {
            Value::Bool(b) => assert!(!b),
            other => panic!("expected bool false, got {:?}", other),
        }
    }

    #[test]
    fn test_is_function() {
        let prog = r#"is_function (\x x + 1)"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b),
            other => panic!("expected bool true, got {:?}", other),
        }

        let prog2 = r#"is_function 42"#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        match v2 {
            Value::Bool(b) => assert!(!b),
            other => panic!("expected bool false, got {:?}", other),
        }
    }

    #[test]
    fn test_assert_string() {
        let prog = r#"assert_string "hello""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "hello");

        // Test failure case
        let prog2 = r#"assert_string 42"#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let result = eval(ast2.program, &mut symbols2, &prog2);
        assert!(result.is_err());
    }

    #[test]
    fn test_assert_number() {
        let prog = r#"assert_number 42"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "42");

        // Test failure case
        let prog2 = r#"assert_number "not a number""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let result = eval(ast2.program, &mut symbols2, &prog2);
        assert!(result.is_err());
    }

    #[test]
    fn test_assert_int() {
        let prog = r#"assert_int 42"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "42");

        // Test failure case with float
        let prog2 = r#"assert_int 3.14"#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let result = eval(ast2.program, &mut symbols2, &prog2);
        assert!(result.is_err());
    }

    #[test]
    fn test_assert_list() {
        let prog = r#"assert_list [1, 2, 3]"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::List(items) => assert_eq!(items.len(), 3),
            _ => panic!("expected list"),
        }

        // Test failure case
        let prog2 = r#"assert_list "not a list""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let result = eval(ast2.program, &mut symbols2, &prog2);
        assert!(result.is_err());
    }

    #[test]
    fn test_assert_bool() {
        let prog = r#"assert_bool (1 == 1)"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b),
            other => panic!("expected bool true, got {:?}", other),
        }

        // Test failure case
        let prog2 = r#"assert_bool 1"#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let result = eval(ast2.program, &mut symbols2, &prog2);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_function() {
        let prog = r#"error "Custom error message""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let result = eval(ast.program, &mut symbols, &prog);
        assert!(result.is_err());
        let err_msg = format!("{:?}", result.unwrap_err());
        assert!(err_msg.contains("Custom error message"));
    }

    #[test]
    fn test_trace_function() {
        // trace should return the value unchanged
        let prog = r#"trace "label" 42"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "42");

        // Test with string
        let prog2 = r#"trace "test" "hello""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "hello");
    }

    #[test]
    fn test_debug_function() {
        // debug should return the value unchanged
        let prog = r#"debug 42"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "42");

        // Test with list
        let prog2 = r#"debug [1, 2, 3]"#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        match v2 {
            Value::List(items) => assert_eq!(items.len(), 3),
            _ => panic!("expected list"),
        }
    }

    #[test]
    fn test_type_checking_in_pipeline() {
        // Test type checking in a computation pipeline
        let prog = r#"
            let x = assert_number 5 in
            let doubled = x * 2 in
            let result = assert_number doubled in
            result
        "#
        .to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "10");
    }

    #[test]
    fn test_typeof_with_map_operations() {
        // Test typeof works with map data structures
        let prog = r#"typeof [["key", "value"]]"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "List");
    }

    #[test]
    fn test_dict_import_returns_dict_type() {
        // Create a test module file that returns a dict
        let dir = std::env::temp_dir();
        let module_path = dir.join("test_module.av");
        let mut f = fs::File::create(&module_path).expect("create temp file");
        write!(f, "let double = \\x x * 2 in dict [[\"double\", double]]").expect("write");

        let prog = format!("let m = import \"{}\" in typeof m", module_path.to_string_lossy());
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "Dict");
        
        let _ = fs::remove_file(module_path);
    }

    #[test]
    fn test_import_returns_actual_value() {
        // Test that import returns whatever the file evaluates to
        let dir = std::env::temp_dir();
        let module_path = dir.join("test_import_value.av");
        let mut f = fs::File::create(&module_path).expect("create temp file");
        write!(f, "42").expect("write");

        let prog = format!("import \"{}\"", module_path.to_string_lossy());
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "42");
        
        let _ = fs::remove_file(module_path);
    }

    #[test]
    fn test_dict_member_access() {
        // Create a test module file that returns a dict
        let dir = std::env::temp_dir();
        let module_path = dir.join("test_math_module.av");
        let mut f = fs::File::create(&module_path).expect("create temp file");
        write!(f, "let double = \\x x * 2 in let triple = \\x x * 3 in dict [[\"double\", double], [\"triple\", triple]]").expect("write");

        let prog = format!("let math = import \"{}\" in math.double 5", module_path.to_string_lossy());
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "10");
        
        let _ = fs::remove_file(module_path);
    }

    #[test]
    fn test_dict_multiple_members() {
        // Test accessing multiple members from same dict
        let dir = std::env::temp_dir();
        let module_path = dir.join("test_multi_module.av");
        let mut f = fs::File::create(&module_path).expect("create temp file");
        write!(f, "let add = \\x \\y x + y in let sub = \\x \\y x - y in dict [[\"add\", add], [\"sub\", sub]]").expect("write");

        let prog = format!(
            "let m = import \"{}\" in let a = m.add 10 5 in let b = m.sub 10 5 in concat (concat (typeof a) \",\") (typeof b)",
            module_path.to_string_lossy()
        );
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "Number,Number");
        
        let _ = fs::remove_file(module_path);
    }

    #[test]
    fn test_dict_namespace_isolation() {
        // Test that two dicts can have same-named keys without conflict
        let dir = std::env::temp_dir();
        let module1_path = dir.join("test_ns1.av");
        let module2_path = dir.join("test_ns2.av");
        
        let mut f1 = fs::File::create(&module1_path).expect("create temp file 1");
        write!(f1, "let func = \\x x * 2 in dict [[\"func\", func]]").expect("write");
        
        let mut f2 = fs::File::create(&module2_path).expect("create temp file 2");
        write!(f2, "let func = \\x x * 3 in dict [[\"func\", func]]").expect("write");

        let prog = format!(
            "let m1 = import \"{}\" in let m2 = import \"{}\" in let r1 = m1.func 5 in let r2 = m2.func 5 in r1 + r2",
            module1_path.to_string_lossy(),
            module2_path.to_string_lossy()
        );
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        // m1.func 5 = 10, m2.func 5 = 15, total = 25
        assert_eq!(v.to_string(&prog), "25");
        
        let _ = fs::remove_file(module1_path);
        let _ = fs::remove_file(module2_path);
    }

    #[test]
    fn test_is_dict_predicate() {
        // Test is_dict predicate works correctly
        let prog = "is_dict (dict [[\"key\", \"value\"]])".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(true) => {},
            other => panic!("Expected true, got {:?}", other),
        }
    }

    #[test]
    fn test_is_dict_returns_false_for_non_dicts() {
        // Test is_dict returns false for other types
        let tests = vec![
            ("is_dict 42", "number"),
            ("is_dict \"string\"", "string"),
            ("is_dict [1, 2, 3]", "list"),
            ("is_dict (\\x x)", "function"),
        ];
        
        for (test_prog, desc) in tests {
            let prog = test_prog.to_string();
            let tokens = tokenize(prog.clone()).expect("tokenize");
            let ast = parse(tokens);
            let mut symbols = initial_builtins();
            let v = eval(ast.program, &mut symbols, &prog).expect("eval");
            match v {
                Value::Bool(false) => {},
                other => panic!("Expected false for {} ('{}'), got {:?}", desc, test_prog, other),
            }
        }
    }

    #[test]
    fn test_dict_creation_and_access() {
        // Test creating a dict and accessing members
        let prog = "let d = dict [[\"name\", \"Alice\"], [\"age\", 30]] in d.name".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "Alice");
    }

    #[test]
    fn test_dict_with_functions() {
        // Test dict containing functions
        let prog = "let d = dict [[\"double\", \\x x * 2], [\"triple\", \\x x * 3]] in d.double 5".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "10");
    }
}
