#![expect(clippy::result_large_err)]
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
        let prog = format!("let p = @{} in readfile p", file_path.to_string_lossy());
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
        let prog = format!("let p = @{} in readlines p", file_path.to_string_lossy());
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
        let prog = format!("let p = @{} in exists p", existing_file.to_string_lossy());
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");

        match v {
            Value::Bool(b) => assert!(b, "expected true for existing file"),
            other => panic!("expected bool, got {:?}", other),
        }

        // Test: exists returns false for missing file
        let prog2 = format!("let p = @{} in exists p", missing_file.to_string_lossy());
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
        let prog = format!("let p = @{} in import p", module_path.to_string_lossy());
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
        while let Value::Function {
            ident: _, default, ..
        } = &v
        {
            if let Some(def) = default {
                v = apply_function(&v, (**def).clone(), &data, 0).expect("apply default");
                continue;
            } else {
                panic!("expected default to be present for test");
            }
        }

        let files = collect_file_templates(&v, &data).expect("collect");
        assert!(!files.is_empty(), "expected at least one filetemplate");
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

        // json_parse object (now returns dict)
        let json_obj_path = dir.join("avon_test_json_obj.json");
        let mut jo = fs::File::create(&json_obj_path).expect("create json obj");
        write!(jo, "{{\"k\": \"v\"}}").expect("write json obj");
        let progo = format!("json_parse \"{}\"", json_obj_path.to_string_lossy());
        let tokens = tokenize(progo.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &progo).expect("eval");
        match v {
            Value::Dict(_) => {
                // Should be a dict now
            }
            other => panic!("expected dict from json object, got {:?}", other),
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
        while let Value::Function {
            ident: _, default, ..
        } = &v
        {
            if let Some(def) = default {
                v = apply_function(&v, (**def).clone(), &prog, 0).expect("apply default");
                continue;
            } else {
                panic!("expected default to be present for test");
            }
        }

        let files = collect_file_templates(&v, &prog).expect("collect");
        assert!(!files.is_empty(), "expected at least one filetemplate");
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
        while let Value::Function { ident, default, .. } = &v {
            if let Some(named_val) = deploy_named.remove(ident) {
                v = apply_function(&v, Value::String(named_val), &prog, 0).expect("apply named");
                continue;
            } else if let Some(def) = default {
                // if default present (not in this test), it would be applied
                v = apply_function(&v, (**def).clone(), &prog, 0).expect("apply default");
                continue;
            } else {
                panic!("missing argument in test (no positional args provided)");
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

    #[test]
    fn test_template_concatenation() {
        // Test: template + template produces combined template
        let prog = r#"let t1 = {"Hello "} in let t2 = {"World!"} in t1 + t2"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "Hello World!");

        // Test: template with interpolation + template
        let prog2 = r#"let name = "Alice" in let greeting = {"Hello, {name}"} in let punct = {"!"} in greeting + punct"#
            .to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "Hello, Alice!");

        // Test: empty template + template
        let prog3 = r#"let t1 = {""} in let t2 = {"content"} in t1 + t2"#.to_string();
        let tokens3 = tokenize(prog3.clone()).expect("tokenize");
        let ast3 = parse(tokens3);
        let mut symbols3 = initial_builtins();
        let v3 = eval(ast3.program, &mut symbols3, &prog3).expect("eval");
        assert_eq!(v3.to_string(&prog3), "content");
    }

    #[test]
    fn test_path_concatenation() {
        // Test: path + path produces combined path
        let prog = r#"let p1 = @/home/user in let p2 = @/projects in p1 + p2"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        // Should concatenate with / separator
        assert!(
            v.to_string(&prog).contains("/home/user") && v.to_string(&prog).contains("/projects")
        );

        // Test: path with interpolation + path
        let prog2 = r#"let dir = "home" in let base = @/{dir} in let sub = @/config in base + sub"#
            .to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        let result = v2.to_string(&prog2);
        assert!(result.contains("home"));
        assert!(result.contains("config"));

        // Test: multiple path concatenations
        let prog3 = r#"let p1 = @/a in let p2 = @/b in let p3 = @/c in p1 + p2 + p3"#.to_string();
        let tokens3 = tokenize(prog3.clone()).expect("tokenize");
        let ast3 = parse(tokens3);
        let mut symbols3 = initial_builtins();
        let v3 = eval(ast3.program, &mut symbols3, &prog3).expect("eval");
        let result = v3.to_string(&prog3);
        assert!(result.contains("/a"));
        assert!(result.contains("/b"));
        assert!(result.contains("/c"));
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
    fn test_template_auto_conversion() {
        // Test that templates are automatically converted to strings in string functions
        let prog = r#"let name = "world" in let t = {"hello {name}"} in upper t"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "HELLO WORLD");

        // Test with split
        let prog2 = r#"let t = {"a,b,c"} in split t ",""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        match v2 {
            Value::List(items) => {
                assert_eq!(items.len(), 3);
                match &items[0] {
                    Value::String(s) => assert_eq!(s, "a"),
                    other => panic!("expected string 'a', got {:?}", other),
                }
            }
            other => panic!("expected list, got {:?}", other),
        }

        // Test with replace
        let prog3 = r#"let t = {"hello world"} in replace t "world" "avon""#.to_string();
        let tokens3 = tokenize(prog3.clone()).expect("tokenize");
        let ast3 = parse(tokens3);
        let mut symbols3 = initial_builtins();
        let v3 = eval(ast3.program, &mut symbols3, &prog3).expect("eval");
        assert_eq!(v3.to_string(&prog3), "hello avon");
    }

    #[test]
    fn test_markdown_to_html() {
        // Test basic markdown to HTML conversion
        let prog = "markdown_to_html \"# Hello\\nThis is **bold** text.\"".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        let result = v.to_string(&prog);
        assert!(
            result.contains("<h1>Hello</h1>"),
            "Expected <h1> tag, got: {}",
            result
        );
        assert!(
            result.contains("<strong>bold</strong>"),
            "Expected <strong> tag, got: {}",
            result
        );

        // Test with multiple headings
        let prog2 = "markdown_to_html \"## Subtitle\\n### Sub-subtitle\"".to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        let result2 = v2.to_string(&prog2);
        assert!(
            result2.contains("<h2>Subtitle</h2>"),
            "Expected <h2> tag, got: {}",
            result2
        );
        assert!(
            result2.contains("<h3>Sub-subtitle</h3>"),
            "Expected <h3> tag, got: {}",
            result2
        );
    }

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
        let prog = r#"let m = set [["name", "Alice"], ["age", "30"]] "name" "Bob" in get m "name""#
            .to_string();
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

        // Test that JSON objects are parsed as dicts and can be queried with get or dot notation
        let prog = format!(
            "let data = json_parse \"{}\" in get data \"name\"",
            json_path.to_string_lossy()
        );
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "Alice");

        // Test that keys can be extracted from dict
        let prog2 = format!(
            "let data = json_parse \"{}\" in keys data",
            json_path.to_string_lossy()
        );
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
            (
                "map (\\x concat x \"!\") [\"a\", \"b\", \"c\"]",
                "[a!, b!, c!]",
            ),
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
            (
                "markdown_to_html \"# Hello\\nWorld\"",
                "<h1>Hello</h1>\n<p>World</p>",
            ),
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
        // Test with list of lists
        let prog = r#"format_table [["A", "B"], ["1", "2"], ["3", "4"]] " | ""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        let result = v.to_string(&prog);
        assert!(result.contains("A | B"));
        assert!(result.contains("1 | 2"));
        assert!(result.contains("3 | 4"));

        // Test with dict (no parentheses needed)
        let prog2 = r#"format_table {a: 1, b: 2, c: 3} " | ""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        let result2 = v2.to_string(&prog2);
        // Dict should produce two rows: keys row and values row
        // The result should be a string with newlines separating rows
        // Check that we have the separator (note: separator is " | " with spaces)
        assert!(
            result2.contains("|"),
            "Result should contain pipe separator: {}",
            result2
        );
        // Check that keys and values are present (order may vary)
        assert!(
            result2.contains("a") || result2.contains("b") || result2.contains("c"),
            "Result should contain keys: {}",
            result2
        );
        assert!(
            result2.contains("1") || result2.contains("2") || result2.contains("3"),
            "Result should contain values: {}",
            result2
        );
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
    }

    #[test]
    fn test_list_operations_zip() {
        let prog = r#"zip [1, 2, 3] ["a", "b", "c"]"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 3);
                // Check first pair
                if let Value::List(pair) = &items[0] {
                    assert_eq!(pair.len(), 2);
                } else {
                    panic!("expected list of pairs");
                }
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn test_list_operations_unzip() {
        let prog = r#"unzip [[1, "a"], [2, "b"], [3, "c"]]"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::List(result) => {
                assert_eq!(result.len(), 2);
                // First list should be [1, 2, 3]
                if let Value::List(first) = &result[0] {
                    assert_eq!(first.len(), 3);
                }
                // Second list should be ["a", "b", "c"]
                if let Value::List(second) = &result[1] {
                    assert_eq!(second.len(), 3);
                }
            }
            other => panic!("expected list of two lists, got {:?}", other),
        }
    }

    #[test]
    fn test_list_operations_take() {
        let prog = r#"take 3 [1, 2, 3, 4, 5]"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 3);
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn test_list_operations_drop() {
        let prog = r#"drop 2 [1, 2, 3, 4, 5]"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 3);
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn test_list_operations_split_at() {
        let prog = r#"split_at 2 [1, 2, 3, 4, 5]"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::List(result) => {
                assert_eq!(result.len(), 2);
                if let Value::List(first) = &result[0] {
                    assert_eq!(first.len(), 2);
                }
                if let Value::List(second) = &result[1] {
                    assert_eq!(second.len(), 3);
                }
            }
            other => panic!("expected list of two lists, got {:?}", other),
        }
    }

    #[test]
    fn test_list_operations_partition() {
        let prog = r#"partition (\x x > 2) [1, 2, 3, 4, 5]"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::List(result) => {
                assert_eq!(result.len(), 2);
                if let Value::List(true_list) = &result[0] {
                    assert_eq!(true_list.len(), 3); // [3, 4, 5]
                }
                if let Value::List(false_list) = &result[1] {
                    assert_eq!(false_list.len(), 2); // [1, 2]
                }
            }
            other => panic!("expected list of two lists, got {:?}", other),
        }
    }

    #[test]
    fn test_list_operations_reverse() {
        let prog = r#"reverse [1, 2, 3]"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 3);
                // Check it's reversed
                if let Value::Number(Number::Int(n)) = &items[0] {
                    assert_eq!(*n, 3);
                }
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn test_list_operations_head() {
        let prog = r#"head [1, 2, 3]"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Number(Number::Int(n)) => assert_eq!(n, 1),
            other => panic!("expected number 1, got {:?}", other),
        }

        // Test empty list
        let prog2 = r#"head []"#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        match v2 {
            Value::None => {}
            other => panic!("expected None for empty list, got {:?}", other),
        }
    }

    #[test]
    fn test_list_operations_tail() {
        let prog = r#"tail [1, 2, 3, 4]"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 3);
                if let Value::Number(Number::Int(n)) = &items[0] {
                    assert_eq!(*n, 2);
                }
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn test_range_syntax() {
        let prog = r#"[1 .. 5]"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 5);
                if let Value::Number(Number::Int(n)) = &items[0] {
                    assert_eq!(*n, 1);
                }
                if let Value::Number(Number::Int(n)) = &items[4] {
                    assert_eq!(*n, 5);
                }
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn test_range_syntax_with_step() {
        let prog = r#"[1, 3 .. 10]"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 4); // [1, 4, 7, 10]
                if let Value::Number(Number::Int(n)) = &items[0] {
                    assert_eq!(*n, 1);
                }
                if let Value::Number(Number::Int(n)) = &items[1] {
                    assert_eq!(*n, 4);
                }
            }
            other => panic!("expected list, got {:?}", other),
        }
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
    fn test_assert_with_true_condition() {
        // assert returns value when condition is true
        let prog = r#"assert (5 > 3) 42"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "42");
    }

    #[test]
    fn test_assert_with_false_condition() {
        // assert errors when condition is false
        let prog = r#"assert (5 < 3) 42"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let result = eval(ast.program, &mut symbols, &prog);
        assert!(result.is_err());
        let err_msg = format!("{:?}", result.unwrap_err());
        assert!(err_msg.contains("assertion failed"));
    }

    #[test]
    fn test_assert_with_type_check() {
        // assert with is_string predicate
        let prog = r#"assert (is_string "hello") "hello""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "hello");

        // assert with is_number predicate (should fail with string)
        let prog2 = r#"assert (is_number "not a number") "not a number""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let result = eval(ast2.program, &mut symbols2, &prog2);
        assert!(result.is_err());
    }

    #[test]
    fn test_assert_with_comparison() {
        // assert with comparison
        let prog = r#"let x = 10 in assert (x > 0) x"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "10");
    }

    #[test]
    fn test_logical_or_operator() {
        // Test: false || true => true
        let prog = r#"false || true"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "true");

        // Test: true || false => true
        let prog2 = r#"true || false"#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "true");

        // Test: false || false => false
        let prog3 = r#"false || false"#.to_string();
        let tokens3 = tokenize(prog3.clone()).expect("tokenize");
        let ast3 = parse(tokens3);
        let mut symbols3 = initial_builtins();
        let v3 = eval(ast3.program, &mut symbols3, &prog3).expect("eval");
        assert_eq!(v3.to_string(&prog3), "false");
    }

    #[test]
    fn test_logical_and_operator() {
        // Test: true && true => true
        let prog = r#"true && true"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "true");

        // Test: true && false => false
        let prog2 = r#"true && false"#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "false");

        // Test: false && true => false
        let prog3 = r#"false && true"#.to_string();
        let tokens3 = tokenize(prog3.clone()).expect("tokenize");
        let ast3 = parse(tokens3);
        let mut symbols3 = initial_builtins();
        let v3 = eval(ast3.program, &mut symbols3, &prog3).expect("eval");
        assert_eq!(v3.to_string(&prog3), "false");
    }

    #[test]
    fn test_logical_operator_precedence() {
        // Test AND binds tighter than OR: a || b && c => a || (b && c)
        // true || false && false => true || (false && false) => true || false => true
        let prog = r#"true || false && false"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "true");

        // false && true || true => (false && true) || true => false || true => true
        let prog2 = r#"false && true || true"#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "true");

        // false && false || false => (false && false) || false => false || false => false
        let prog3 = r#"false && false || false"#.to_string();
        let tokens3 = tokenize(prog3.clone()).expect("tokenize");
        let ast3 = parse(tokens3);
        let mut symbols3 = initial_builtins();
        let v3 = eval(ast3.program, &mut symbols3, &prog3).expect("eval");
        assert_eq!(v3.to_string(&prog3), "false");
    }

    #[test]
    fn test_none_literal() {
        // Test: none evaluates to None
        let prog = r#"none"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "None");

        // Test: none == none => true
        let prog2 = r#"none == none"#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "true");

        // Test: none != none => false
        let prog3 = r#"none != none"#.to_string();
        let tokens3 = tokenize(prog3.clone()).expect("tokenize");
        let ast3 = parse(tokens3);
        let mut symbols3 = initial_builtins();
        let v3 = eval(ast3.program, &mut symbols3, &prog3).expect("eval");
        assert_eq!(v3.to_string(&prog3), "false");
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
    fn test_neg_builtin() {
        // Test negating a positive integer
        let prog = r#"neg 42"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "-42");

        // Test negating a larger number
        let prog2 = r#"neg 100"#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "-100");

        // Test negating a float
        let prog3 = r#"neg 3.14"#.to_string();
        let tokens3 = tokenize(prog3.clone()).expect("tokenize");
        let ast3 = parse(tokens3);
        let mut symbols3 = initial_builtins();
        let v3 = eval(ast3.program, &mut symbols3, &prog3).expect("eval");
        assert_eq!(v3.to_string(&prog3), "-3.14");

        // Test negating zero
        let prog4 = r#"neg 0"#.to_string();
        let tokens4 = tokenize(prog4.clone()).expect("tokenize");
        let ast4 = parse(tokens4);
        let mut symbols4 = initial_builtins();
        let v4 = eval(ast4.program, &mut symbols4, &prog4).expect("eval");
        assert_eq!(v4.to_string(&prog4), "0");
    }

    #[test]
    fn test_negative_number_literals() {
        // Test negative integer literal
        let prog = r#"-42"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Number(Number::Int(n)) => assert_eq!(n, -42),
            other => panic!("expected -42, got {:?}", other),
        }

        // Test negative float literal
        let prog2 = r#"-3.14"#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        #[expect(clippy::approx_constant)]
        match v2 {
            Value::Number(Number::Float(f)) => assert!((f + 3.14).abs() < 0.001),
            other => panic!("expected -3.14, got {:?}", other),
        }

        // Test negative numbers in lists
        let prog3 = r#"[-5, -4, -3]"#.to_string();
        let tokens3 = tokenize(prog3.clone()).expect("tokenize");
        let ast3 = parse(tokens3);
        let mut symbols3 = initial_builtins();
        let v3 = eval(ast3.program, &mut symbols3, &prog3).expect("eval");
        match v3 {
            Value::List(items) => {
                assert_eq!(items.len(), 3);
                if let Value::Number(Number::Int(n)) = &items[0] {
                    assert_eq!(*n, -5);
                }
            }
            other => panic!("expected list, got {:?}", other),
        }

        // Test negative numbers in ranges
        let prog4 = r#"[10, -1 .. 0]"#.to_string();
        let tokens4 = tokenize(prog4.clone()).expect("tokenize");
        let ast4 = parse(tokens4);
        let mut symbols4 = initial_builtins();
        let v4 = eval(ast4.program, &mut symbols4, &prog4).expect("eval");
        match v4 {
            Value::List(items) => {
                assert!(!items.is_empty());
                if let Value::Number(Number::Int(n)) = &items[0] {
                    assert_eq!(*n, 10);
                }
            }
            other => panic!("expected list, got {:?}", other),
        }

        // Test subtraction still works
        let prog5 = r#"10 - 5"#.to_string();
        let tokens5 = tokenize(prog5.clone()).expect("tokenize");
        let ast5 = parse(tokens5);
        let mut symbols5 = initial_builtins();
        let v5 = eval(ast5.program, &mut symbols5, &prog5).expect("eval");
        match v5 {
            Value::Number(Number::Int(n)) => assert_eq!(n, 5),
            other => panic!("expected 5, got {:?}", other),
        }

        // Test unary minus on variable (uses neg function)
        let prog6 = r#"let x = 5 in -x"#.to_string();
        let tokens6 = tokenize(prog6.clone()).expect("tokenize");
        let ast6 = parse(tokens6);
        let mut symbols6 = initial_builtins();
        let v6 = eval(ast6.program, &mut symbols6, &prog6).expect("eval");
        match v6 {
            Value::Number(Number::Int(n)) => assert_eq!(n, -5),
            other => panic!("expected -5, got {:?}", other),
        }

        // Test negative number in arithmetic
        let prog7 = r#"-5 * 3"#.to_string();
        let tokens7 = tokenize(prog7.clone()).expect("tokenize");
        let ast7 = parse(tokens7);
        let mut symbols7 = initial_builtins();
        let v7 = eval(ast7.program, &mut symbols7, &prog7).expect("eval");
        match v7 {
            Value::Number(Number::Int(n)) => assert_eq!(n, -15),
            other => panic!("expected -15, got {:?}", other),
        }
    }

    #[test]
    fn test_is_empty_builtin() {
        // Test is_empty on empty string
        let prog = r#"is_empty """#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "true");

        // Test is_empty on non-empty string
        let prog2 = r#"is_empty "hello""#.to_string();
        let tokens2 = tokenize(prog2.clone()).expect("tokenize");
        let ast2 = parse(tokens2);
        let mut symbols2 = initial_builtins();
        let v2 = eval(ast2.program, &mut symbols2, &prog2).expect("eval");
        assert_eq!(v2.to_string(&prog2), "false");

        // Test is_empty on empty list
        let prog3 = r#"is_empty []"#.to_string();
        let tokens3 = tokenize(prog3.clone()).expect("tokenize");
        let ast3 = parse(tokens3);
        let mut symbols3 = initial_builtins();
        let v3 = eval(ast3.program, &mut symbols3, &prog3).expect("eval");
        assert_eq!(v3.to_string(&prog3), "true");

        // Test is_empty on non-empty list
        let prog4 = r#"is_empty [1, 2, 3]"#.to_string();
        let tokens4 = tokenize(prog4.clone()).expect("tokenize");
        let ast4 = parse(tokens4);
        let mut symbols4 = initial_builtins();
        let v4 = eval(ast4.program, &mut symbols4, &prog4).expect("eval");
        assert_eq!(v4.to_string(&prog4), "false");
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
        // Test type checking in a computation pipeline using general assert
        let prog = r#"
            let x = assert (is_number 5) 5 in
            let doubled = x * 2 in
            let result = assert (is_number doubled) doubled in
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
        write!(f, "let double = \\x x * 2 in {{double: double}}").expect("write");

        let prog = format!(
            "let m = import \"{}\" in typeof m",
            module_path.to_string_lossy()
        );
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
        write!(f, "let double = \\x x * 2 in let triple = \\x x * 3 in {{double: double, triple: triple}}").expect("write");

        let prog = format!(
            "let math = import \"{}\" in math.double 5",
            module_path.to_string_lossy()
        );
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
        write!(
            f,
            "let add = \\x \\y x + y in let sub = \\x \\y x - y in {{add: add, sub: sub}}"
        )
        .expect("write");

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
        write!(f1, "let func = \\x x * 2 in {{func: func}}").expect("write");

        let mut f2 = fs::File::create(&module2_path).expect("create temp file 2");
        write!(f2, "let func = \\x x * 3 in {{func: func}}").expect("write");

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
        // Test is_dict predicate works correctly with new dict syntax
        let prog = "let d = {key: \"value\"} in is_dict d".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(true) => {}
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
                Value::Bool(false) => {}
                other => panic!(
                    "Expected false for {} ('{}'), got {:?}",
                    desc, test_prog, other
                ),
            }
        }
    }

    #[test]
    fn test_dict_creation_and_access() {
        // Test creating a dict and accessing members with new dict syntax
        let prog = "let d = {name: \"Alice\", age: 30} in d.name".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "Alice");
    }

    #[test]
    fn test_dict_with_functions() {
        // Test dict containing functions with new dict syntax
        let prog = "let d = {double: \\x x * 2, triple: \\x x * 3} in d.double 5".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "10");
    }

    // ============================================================================
    // NEW DICT SYNTAX TESTS ({key: value})
    // ============================================================================

    #[test]
    fn test_dict_literal_syntax_basic() {
        // Test basic dict literal syntax {key: value}
        let prog = "let d = {x: 10, y: 20} in d.x".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "10");
    }

    #[test]
    fn test_dict_literal_syntax_multiple_access() {
        // Test accessing multiple keys in dict literal
        let prog = "let d = {x: 10, y: 20} in let x = d.x in let y = d.y in [x, y]".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "[10, 20]");
    }

    #[test]
    fn test_dict_literal_syntax_nested() {
        // Test nested dict literals
        let prog = "let d = {user: {name: \"Alice\", age: 30}} in d.user.name".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "Alice");
    }

    #[test]
    fn test_dict_literal_syntax_with_strings() {
        // Test dict with string values
        let prog =
            "let d = {first: \"John\", last: \"Doe\"} in d.first + \" \" + d.last".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "John Doe");
    }

    #[test]
    fn test_dict_literal_syntax_with_lists() {
        // Test dict containing lists
        let prog = "let d = {items: [1, 2, 3], count: 3} in d.count".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "3");
    }

    #[test]
    fn test_dict_literal_syntax_with_functions() {
        // Test dict containing functions
        let prog = "let d = {double: \\x x * 2, triple: \\x x * 3} in d.double 5".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "10");
    }

    #[test]
    fn test_dict_literal_syntax_is_dict() {
        // Test is_dict on dict literal
        let prog = "let d = {x: 10} in is_dict d".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(true) => {}
            other => panic!("Expected true, got {:?}", other),
        }
    }

    #[test]
    fn test_dict_literal_syntax_with_get() {
        // Test using get function with dict literal
        let prog = "let d = {x: 10, y: 20} in get d \"x\"".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "10");
    }

    #[test]
    fn test_dict_literal_syntax_with_set() {
        // Test using set function with dict literal
        let prog = "let d = {x: 10, y: 20} in let d2 = set d \"x\" 100 in d2.x".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "100");
    }

    #[test]
    fn test_dict_literal_syntax_with_keys() {
        // Test keys function with dict literal
        let prog = "let d = {a: 1, b: 2, c: 3} in keys d".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        let result = v.to_string(&prog);
        assert!(result.contains("a") && result.contains("b") && result.contains("c"));
    }

    #[test]
    fn test_dict_literal_syntax_with_values() {
        // Test values function with dict literal
        let prog = "let d = {x: 100, y: 200} in values d".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        let result = v.to_string(&prog);
        assert!(result.contains("100") && result.contains("200"));
    }

    #[test]
    fn test_dict_literal_syntax_json_parse_returns_dict() {
        // Test that json_parse returns dict for objects
        use std::io::Write;
        let dir = std::env::temp_dir();
        let json_path = dir.join("avon_test_dict_syntax_parse.json");
        let mut jf = fs::File::create(&json_path).expect("create json");
        write!(jf, r#"{{"x": 10, "y": 20}}"#).expect("write json");
        drop(jf);

        let prog = format!(
            "let d = json_parse \"{}\" in is_dict d",
            json_path.to_string_lossy()
        );
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(true) => {}
            other => panic!("Expected json_parse to return dict, got {:?}", other),
        }
    }

    #[test]
    fn test_dict_literal_syntax_json_access() {
        // Test accessing dict returned from json_parse
        use std::io::Write;
        let dir = std::env::temp_dir();
        let json_path = dir.join("avon_test_dict_syntax_access.json");
        let mut jf = fs::File::create(&json_path).expect("create json");
        write!(jf, r#"{{"name": "Alice", "age": 30}}"#).expect("write json");
        drop(jf);

        let prog = format!(
            "let d = json_parse \"{}\" in get d \"age\"",
            json_path.to_string_lossy()
        );
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "30");
    }

    #[test]
    fn test_dict_literal_syntax_with_has_key() {
        // Test dict_has_key function with dict literal
        let prog = "let d = {x: 10, y: 20} in dict_has_key d \"x\"".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        match v {
            Value::Bool(true) => {}
            other => panic!("Expected true, got {:?}", other),
        }
    }

    #[test]
    fn test_dict_literal_syntax_with_expressions() {
        // Test dict values with complex expressions
        let prog =
            "let d = {sum: 5 + 10, product: 3 * 4} in let s = d.sum in let p = d.product in [s, p]"
                .to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "[15, 12]");
    }

    #[test]
    fn test_dict_literal_syntax_mixed_types() {
        // Test dict with mixed value types
        let prog = "let d = {num: 42, str: \"hello\", bool: true, list: [1, 2]} in let n = d.num in let s = d.str in let b = d.bool in [n, s, b]".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "[42, hello, true]");
    }

    #[test]
    fn test_dict_nested_three_levels() {
        // Test three-level nested dict access with chained dot notation
        let prog =
            "let d = {user: {profile: {name: \"Alice\"}}} in d.user.profile.name".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "Alice");
    }

    #[test]
    fn test_dict_nested_four_levels() {
        // Test four-level nested dict access
        let prog = "let d = {a: {b: {c: {d: 42}}}} in d.a.b.c.d".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "42");
    }

    #[test]
    fn test_dict_nested_five_levels() {
        // Test five-level nested dict access
        let prog = "let d = {level1: {level2: {level3: {level4: {level5: \"FOUND\"}}}}} in d.level1.level2.level3.level4.level5".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "FOUND");
    }

    #[test]
    fn test_dict_nested_with_mixed_types() {
        // Test nested dicts with mixed value types at different levels
        let prog = "let d = {user: {name: \"Bob\", age: 25, settings: {theme: \"dark\", notifications: true}}} in let name = d.user.name in let theme = d.user.settings.theme in [name, theme]".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "[Bob, dark]");
    }

    #[test]
    fn test_dict_nested_with_lists() {
        // Test nested dicts containing lists
        let prog =
            "let d = {org: {teams: {devs: [\"Alice\", \"Bob\", \"Charlie\"]}}} in d.org.teams.devs"
                .to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "[Alice, Bob, Charlie]");
    }

    #[test]
    fn test_dict_nested_chained_operations() {
        // Test chained dict access with operations
        let prog = "let d = {data: {values: {x: 10, y: 20}}} in d.data.values.x + d.data.values.y"
            .to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "30");
    }

    #[test]
    fn test_dict_nested_partial_chain_access() {
        // Test accessing intermediate levels of nested dicts
        let prog = "let d = {config: {db: {host: \"localhost\", port: 5432}}} in let db = d.config.db in [db.host, db.port]".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "[localhost, 5432]");
    }

    #[test]
    fn test_dict_nested_with_functions() {
        // Test nested dicts containing function values
        let prog = "let d = {math: {ops: {add: \\x \\y x + y}}} in d.math.ops.add 5 3".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "8");
    }

    #[test]
    fn test_dict_nested_with_get_function() {
        // Test using get function on nested dict access result
        let prog = "let d = {api: {endpoints: {users: \"GET /users\", posts: \"GET /posts\"}}} in get d.api.endpoints \"users\"".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "GET /users");
    }

    #[test]
    fn test_pipe_operator_basic() {
        // Test basic pipe operator: lhs -> rhs
        let prog = "[1, 2, 3] -> length".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "3");
    }

    #[test]
    fn test_pipe_operator_string() {
        // Test pipe with string function
        let prog = "\"hello\" -> upper".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "HELLO");
    }

    #[test]
    fn test_pipe_operator_chained() {
        // Test chained pipe operators
        let prog = "\"hello\" -> upper -> length".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "5");
    }

    #[test]
    fn test_pipe_operator_with_conversion() {
        // Test pipe with type conversion
        let prog = "42 -> to_string -> length".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "2");
    }

    #[test]
    fn test_pipe_operator_with_filter() {
        // Test pipe with filter operation
        let prog = "[1, 2, 3, 4, 5] -> filter (\\x x > 2) -> length".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "3");
    }

    #[test]
    fn test_env_var_existing() {
        // Test env_var reading a set variable
        std::env::set_var("AVON_TEST_KEY", "secret_val");
        let prog = "env_var \"AVON_TEST_KEY\"".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "secret_val");
        std::env::remove_var("AVON_TEST_KEY");
    }

    #[test]
    fn test_env_var_missing() {
        // Test env_var failing on missing variable
        let prog = "env_var \"AVON_MISSING_KEY\"".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let result = eval(ast.program, &mut symbols, &prog);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.message.contains("Missing environment variable"));
    }

    #[test]
    fn test_env_var_or_existing() {
        // Test env_var_or using existing variable
        std::env::set_var("AVON_TEST_KEY_OR", "real_val");
        let prog = "env_var_or \"AVON_TEST_KEY_OR\" \"default\"".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "real_val");
        std::env::remove_var("AVON_TEST_KEY_OR");
    }

    #[test]
    fn test_env_var_or_missing() {
        // Test env_var_or using default value
        let prog = "env_var_or \"AVON_MISSING_KEY_OR\" \"default_val\"".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "default_val");
    }

    // Atomic Deployment Safety Tests
    // These tests verify the claims made in the documentation about fail-safe deployment

    #[test]
    fn test_atomic_deployment_evaluation_failure_no_files() {
        // Claim: If evaluation fails, no files are written
        // Test: A program with a type error should not produce any file templates
        let prog = "let x = \"hello\" in x + 42".to_string(); // Type error: string + number
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let result = eval(ast.program, &mut symbols, &prog);

        // Evaluation should fail
        assert!(result.is_err());

        // Since evaluation failed, we can't even get to collect_file_templates
        // This verifies that evaluation errors prevent any file collection
    }

    #[test]
    fn test_atomic_deployment_non_deployable_result_no_files() {
        // Claim: If result isn't deployable (not FileTemplate or list), no files are written
        // Test: A program that evaluates to a string (not deployable) should fail collection
        let prog = "\"just a string\"".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");

        // collect_file_templates should fail for non-deployable values
        let result = collect_file_templates(&v, &prog);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.message.contains("expected filetemplate"));
    }

    #[test]
    fn test_atomic_deployment_valid_filetemplates_collect() {
        // Claim: Valid FileTemplates should be collectible
        // Test: A program that evaluates to a FileTemplate should be collectible
        let prog = "@/test.txt {\"content\"}".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");

        let result = collect_file_templates(&v, &prog);
        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].0, "/test.txt");
        assert_eq!(files[0].1, "content");
    }

    #[test]
    fn test_atomic_deployment_list_of_filetemplates_collect() {
        // Claim: Lists of FileTemplates should be collectible
        // Test: A program that evaluates to a list of FileTemplates should be collectible
        let prog = "[@/a.txt {\"a\"}, @/b.txt {\"b\"}]".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");

        let result = collect_file_templates(&v, &prog);
        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 2);
        assert_eq!(files[0].0, "/a.txt");
        assert_eq!(files[0].1, "a");
        assert_eq!(files[1].0, "/b.txt");
        assert_eq!(files[1].1, "b");
    }

    #[test]
    fn test_atomic_deployment_mixed_list_fails() {
        // Claim: Lists containing non-FileTemplate values should fail collection
        // Test: A list with a string and FileTemplate should fail
        let prog = "[\"not a filetemplate\", @/test.txt {\"content\"}]".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");

        let result = collect_file_templates(&v, &prog);
        assert!(result.is_err());
    }

    #[test]
    fn test_atomic_deployment_nested_list_collect() {
        // Claim: Nested lists of FileTemplates should be flattened and collectible
        // Test: A nested list structure should work
        let prog = "[[@/a.txt {\"a\"}], [@/b.txt {\"b\"}]]".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");

        let result = collect_file_templates(&v, &prog);
        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_atomic_deployment_env_var_failure_prevents_deployment() {
        // Claim: If env_var fails (missing secret), evaluation fails, preventing deployment
        // Test: A program using env_var with missing variable should fail evaluation
        let prog = "let secret = env_var \"MISSING_SECRET\" in @/config.yml {\"key: {secret}\"}"
            .to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let result = eval(ast.program, &mut symbols, &prog);

        // Evaluation should fail because env_var fails
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.message.contains("Missing environment variable"));

        // Since evaluation failed, no file templates can be collected
        // This verifies that missing secrets prevent deployment
    }

    // REPL Functionality Tests
    // These test the core functionality that the REPL uses

    #[test]
    fn test_repl_basic_evaluation() {
        // Test that basic expressions work (REPL uses this)
        let prog = "1 + 2".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "3");
    }

    #[test]
    fn test_repl_variable_persistence() {
        // Test that variables persist (REPL maintains symbol table)
        // Note: In the actual REPL, variables are stored when assigned, not from let expressions
        // This test simulates REPL behavior where you can assign to variables
        let mut symbols = initial_builtins();

        // First expression: define a variable (in REPL, this would be stored)
        // The REPL actually stores the result, not the let binding itself
        let prog1 = "42".to_string();
        let tokens1 = tokenize(prog1.clone()).expect("tokenize");
        let ast1 = parse(tokens1);
        let v1 = eval(ast1.program, &mut symbols, &prog1).expect("eval");
        assert_eq!(v1.to_string(&prog1), "42");

        // In actual REPL, you'd do: x = 42 (assignment, not let binding)
        // For this test, we simulate by manually inserting (REPL does this)
        symbols.insert("x".to_string(), v1);
        assert!(symbols.contains_key("x"));
    }

    #[test]
    fn test_repl_typeof_command() {
        // Test :type command functionality
        let prog = "[1, 2, 3]".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");

        // Verify it's a List (what :type would show)
        match v {
            Value::List(_) => {}
            _ => panic!("Expected List"),
        }
    }

    // REPL Expression Completion Tests
    use crate::cli::is_expression_complete;

    #[test]
    fn test_repl_complete_simple_expression() {
        // Simple expressions should be complete
        assert!(is_expression_complete("1 + 2"));
        assert!(is_expression_complete("\"hello\""));
        assert!(is_expression_complete("[1, 2, 3]"));
    }

    #[test]
    fn test_repl_incomplete_let_expression() {
        // let without in should be incomplete
        assert!(!is_expression_complete("let x = 42"));
        assert!(!is_expression_complete("let x = 42\n"));

        // let with in should be complete
        assert!(is_expression_complete("let x = 42 in x"));
        assert!(is_expression_complete("let x = 42 in x + 1"));
    }

    #[test]
    fn test_repl_nested_let_expressions() {
        // Nested lets should be complete when all have matching in
        assert!(is_expression_complete("let x = 1 in let y = 2 in x + y"));
        assert!(!is_expression_complete("let x = 1 in let y = 2"));
        assert!(!is_expression_complete("let x = 1"));
    }

    #[test]
    fn test_repl_incomplete_if_expression() {
        // if without then/else should be incomplete
        assert!(!is_expression_complete("if true"));
        assert!(!is_expression_complete("if true then 1"));

        // if with then and else should be complete
        assert!(is_expression_complete("if true then 1 else 2"));
    }

    #[test]
    fn test_repl_unbalanced_parens() {
        // Unbalanced parentheses should be incomplete
        assert!(!is_expression_complete("(1 + 2"));
        assert!(!is_expression_complete("1 + 2)"));
        assert!(is_expression_complete("(1 + 2)"));
    }

    #[test]
    fn test_repl_unbalanced_brackets() {
        // Unbalanced brackets should be incomplete
        assert!(!is_expression_complete("[1, 2"));
        assert!(!is_expression_complete("1, 2]"));
        assert!(is_expression_complete("[1, 2]"));
    }

    #[test]
    fn test_repl_string_literals() {
        // Strings should not affect keyword matching
        assert!(is_expression_complete("\"let x = 5\""));
        assert!(is_expression_complete("\"in\""));
        // But actual let without in should still be incomplete
        assert!(!is_expression_complete("let x = \"test\""));
        assert!(is_expression_complete("let x = \"test\" in x"));
    }

    #[test]
    fn test_pipe_operator_with_map() {
        // Test pipe with map operation
        let prog = "[1, 2, 3] -> map (\\x x * 2) -> length".to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), "3");
    }

    // Security tests
    #[test]
    fn test_path_traversal_blocked_in_readfile() {
        // Test that readfile blocks path traversal
        let prog = r#"readfile "../../etc/passwd""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let result = eval(ast.program, &mut symbols, &prog);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().message;
        assert!(
            err_msg.contains("..") || err_msg.contains("not allowed"),
            "Expected path traversal error, got: {}",
            err_msg
        );
    }

    #[test]
    fn test_path_traversal_blocked_in_import() {
        // Test that import blocks path traversal
        let prog = r#"import "../../etc/passwd""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let result = eval(ast.program, &mut symbols, &prog);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().message;
        assert!(
            err_msg.contains("..") || err_msg.contains("not allowed"),
            "Expected path traversal error, got: {}",
            err_msg
        );
    }

    #[test]
    fn test_path_traversal_blocked_in_fill_template() {
        // Test that fill_template blocks path traversal
        let prog = r#"fill_template "../../etc/passwd" {}"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let result = eval(ast.program, &mut symbols, &prog);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().message;
        assert!(
            err_msg.contains("..") || err_msg.contains("not allowed"),
            "Expected path traversal error, got: {}",
            err_msg
        );
    }

    #[test]
    fn test_safe_paths_allowed() {
        // Test that safe paths work
        let prog = r#"let p = @config/app.yml in typeof p"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let result = eval(ast.program, &mut symbols, &prog);
        // Should parse and evaluate (path is valid even if file doesn't exist)
        assert!(result.is_ok());
    }

    #[test]
    fn test_template_sandboxing() {
        // Templates should only evaluate expressions, not execute arbitrary code
        let prog = r#"{"value: {1 + 2}"}"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let result = eval(ast.program, &mut symbols, &prog);
        assert!(result.is_ok());
        // Template should evaluate safely
        match result.unwrap() {
            Value::Template(_, _) => (),
            _ => panic!("Expected template"),
        }
    }

    #[test]
    fn test_no_code_injection_in_strings() {
        // Strings should be treated as data, not code
        let prog = r#"let malicious = "readfile \"/etc/passwd\"" in malicious"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let result = eval(ast.program, &mut symbols, &prog);
        assert!(result.is_ok());
        match result.unwrap() {
            Value::String(s) => assert_eq!(s, "readfile \"/etc/passwd\""),
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_type_safety_enforced() {
        // Type mismatches should be caught
        let prog = r#"5 + "hello""#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let result = eval(ast.program, &mut symbols, &prog);
        assert!(result.is_err());
        let err = result.unwrap_err();
        // Check that the error has type information
        assert!(
            err.expected.is_some() && err.found.is_some(),
            "Expected type error with expected/found types, got: {}",
            err
        );
        // Verify it's actually a type mismatch
        let expected_str = err.expected.as_ref().unwrap();
        let found_str = err.found.as_ref().unwrap();
        assert!(
            (expected_str.contains("Number") && found_str.contains("String"))
                || (expected_str.contains("String") && found_str.contains("Number")),
            "Expected Number/String mismatch, got: {} vs {}",
            expected_str,
            found_str
        );
    }

    // ========================================================================
    // RANGE SYNTAX TESTS - Regression tests for the lexer fix
    // Tests range syntax without spaces [1..3] to prevent regressions
    // ========================================================================

    #[test]
    fn test_range_syntax_no_spaces_simple() {
        // Critical regression test: [1..3] without spaces should parse correctly
        let prog = "[1..3]";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 3);
                if let Value::Number(Number::Int(n)) = items[0] {
                    assert_eq!(n, 1);
                } else {
                    panic!("expected int");
                }
                if let Value::Number(Number::Int(n)) = items[1] {
                    assert_eq!(n, 2);
                } else {
                    panic!("expected int");
                }
                if let Value::Number(Number::Int(n)) = items[2] {
                    assert_eq!(n, 3);
                } else {
                    panic!("expected int");
                }
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn test_range_syntax_no_spaces_larger() {
        // Test larger range without spaces
        let prog = "[0..5]";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 6);
                for (i, item) in items.iter().enumerate() {
                    if let Value::Number(Number::Int(n)) = item {
                        assert_eq!(*n, i as i64);
                    } else {
                        panic!("expected int at index {}", i);
                    }
                }
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn test_range_syntax_no_spaces_with_step() {
        // Test range with step, no spaces: [0,2..10]
        let prog = "[0,2..10]";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 6);
                let expected = [0, 2, 4, 6, 8, 10];
                for (i, item) in items.iter().enumerate() {
                    if let Value::Number(Number::Int(n)) = item {
                        assert_eq!(*n, expected[i]);
                    } else {
                        panic!("expected int at index {}", i);
                    }
                }
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn test_range_syntax_negative_no_spaces() {
        // Test negative ranges without spaces
        let prog = "[-5..-1]";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 5);
                let expected = [-5, -4, -3, -2, -1];
                for (i, item) in items.iter().enumerate() {
                    if let Value::Number(Number::Int(n)) = item {
                        assert_eq!(*n, expected[i]);
                    } else {
                        panic!("expected int at index {}", i);
                    }
                }
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn test_range_syntax_with_spaces_backward_compat() {
        // Ensure spaced version still works (backward compatibility)
        let prog = "[1 .. 3]";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 3);
                if let Value::Number(Number::Int(n)) = items[0] {
                    assert_eq!(n, 1);
                } else {
                    panic!("expected int");
                }
                if let Value::Number(Number::Int(n)) = items[1] {
                    assert_eq!(n, 2);
                } else {
                    panic!("expected int");
                }
                if let Value::Number(Number::Int(n)) = items[2] {
                    assert_eq!(n, 3);
                } else {
                    panic!("expected int");
                }
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn test_range_syntax_step_with_spaces_backward_compat() {
        // Ensure spaced version with step still works
        let prog = "[0, 2 .. 10]";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 6);
                let expected = [0, 2, 4, 6, 8, 10];
                for (i, item) in items.iter().enumerate() {
                    if let Value::Number(Number::Int(n)) = item {
                        assert_eq!(*n, expected[i]);
                    } else {
                        panic!("expected int at index {}", i);
                    }
                }
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn test_float_literals_still_work() {
        // Ensure float parsing wasn't broken by the range syntax fix
        let prog = "1.5";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Number(Number::Float(n)) => assert_eq!(n, 1.5),
            other => panic!("expected float, got {:?}", other),
        }
    }

    #[test]
    fn test_float_literals_multiple() {
        // Test multiple float literals to ensure parsing is robust
        #[expect(clippy::approx_constant)]
        let test_cases = vec![
            ("0.5", 0.5),
            ("3.14", 3.14),
            ("100.001", 100.001),
            ("0.0", 0.0),
        ];

        for (prog, expected) in test_cases {
            let tokens = tokenize(prog.to_string()).expect("tokenize");
            let ast = parse(tokens);
            let mut symbols = initial_builtins();
            let v = eval(ast.program, &mut symbols, prog).expect("eval");
            match v {
                Value::Number(Number::Float(n)) => assert_eq!(n, expected, "failed for {}", prog),
                other => panic!("expected float for {}, got {:?}", prog, other),
            }
        }
    }

    #[test]
    fn test_float_arithmetic() {
        // Test that float arithmetic still works
        let prog = "1.5 + 2.5";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Number(Number::Float(n)) => assert_eq!(n, 4.0),
            other => panic!("expected float, got {:?}", other),
        }
    }

    #[test]
    fn test_range_in_map() {
        // Test range syntax in map operations
        let prog = "map (\\x x * 2) [1..3]";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 3);
                if let Value::Number(Number::Int(n)) = items[0] {
                    assert_eq!(n, 2);
                } else {
                    panic!("expected int");
                }
                if let Value::Number(Number::Int(n)) = items[1] {
                    assert_eq!(n, 4);
                } else {
                    panic!("expected int");
                }
                if let Value::Number(Number::Int(n)) = items[2] {
                    assert_eq!(n, 6);
                } else {
                    panic!("expected int");
                }
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn test_range_in_filter() {
        // Test range syntax in filter operations
        let prog = "filter (\\x (x > 2)) [1..5]";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 3);
                if let Value::Number(Number::Int(n)) = items[0] {
                    assert_eq!(n, 3);
                } else {
                    panic!("expected int");
                }
                if let Value::Number(Number::Int(n)) = items[1] {
                    assert_eq!(n, 4);
                } else {
                    panic!("expected int");
                }
                if let Value::Number(Number::Int(n)) = items[2] {
                    assert_eq!(n, 5);
                } else {
                    panic!("expected int");
                }
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn test_range_list_concatenation() {
        // Test concatenating ranges
        let prog = "[1..3] + [4..6]";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 6);
                let expected = [1, 2, 3, 4, 5, 6];
                for (i, item) in items.iter().enumerate() {
                    if let Value::Number(Number::Int(n)) = item {
                        assert_eq!(*n, expected[i] as i64);
                    } else {
                        panic!("expected int at index {}", i);
                    }
                }
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn test_nested_ranges() {
        // Test nested range expressions
        let prog = "[[1..3], [4..6]]";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 2);
                // Check first sublist
                match &items[0] {
                    Value::List(sub) => {
                        assert_eq!(sub.len(), 3);
                        if let Value::Number(Number::Int(n)) = sub[0] {
                            assert_eq!(n, 1);
                        } else {
                            panic!("expected int");
                        }
                        if let Value::Number(Number::Int(n)) = sub[1] {
                            assert_eq!(n, 2);
                        } else {
                            panic!("expected int");
                        }
                        if let Value::Number(Number::Int(n)) = sub[2] {
                            assert_eq!(n, 3);
                        } else {
                            panic!("expected int");
                        }
                    }
                    other => panic!("expected list, got {:?}", other),
                }
                // Check second sublist
                match &items[1] {
                    Value::List(sub) => {
                        assert_eq!(sub.len(), 3);
                        if let Value::Number(Number::Int(n)) = sub[0] {
                            assert_eq!(n, 4);
                        } else {
                            panic!("expected int");
                        }
                        if let Value::Number(Number::Int(n)) = sub[1] {
                            assert_eq!(n, 5);
                        } else {
                            panic!("expected int");
                        }
                        if let Value::Number(Number::Int(n)) = sub[2] {
                            assert_eq!(n, 6);
                        } else {
                            panic!("expected int");
                        }
                    }
                    other => panic!("expected list, got {:?}", other),
                }
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    // ========================================================================
    // ADDITIONAL EDGE CASE TESTS - 30+ comprehensive edge cases
    // ========================================================================

    #[test]
    fn test_edge_case_division_by_one() {
        let prog = "100 / 1";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Number(Number::Int(n)) => assert_eq!(n, 100),
            other => panic!("expected 100, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_zero_divided_by_number() {
        let prog = "0 / 5";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Number(Number::Int(n)) => assert_eq!(n, 0),
            other => panic!("expected 0, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_multiplication_by_zero() {
        let prog = "999 * 0";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Number(Number::Int(n)) => assert_eq!(n, 0),
            other => panic!("expected 0, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_multiplication_by_one() {
        let prog = "42 * 1";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Number(Number::Int(n)) => assert_eq!(n, 42),
            other => panic!("expected 42, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_negative_zero() {
        let prog = "-0";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Number(Number::Int(n)) => assert_eq!(n, 0),
            other => panic!("expected 0, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_nested_parentheses() {
        let prog = "(((42)))";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Number(Number::Int(n)) => assert_eq!(n, 42),
            other => panic!("expected 42, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_comparison_equal_numbers() {
        let prog = "42 == 42";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b),
            other => panic!("expected true, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_comparison_not_equal_numbers() {
        let prog = "42 != 43";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b),
            other => panic!("expected true, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_empty_string_is_empty() {
        let prog = "is_empty \"\"";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b),
            other => panic!("expected true, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_nonempty_string_not_empty() {
        let prog = "is_empty \"x\"";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(!b),
            other => panic!("expected false, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_empty_list_length() {
        let prog = "length []";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Number(Number::Int(n)) => assert_eq!(n, 0),
            other => panic!("expected 0, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_single_element_list_length() {
        let prog = "length [42]";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Number(Number::Int(n)) => assert_eq!(n, 1),
            other => panic!("expected 1, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_map_over_empty_list() {
        let prog = "map (\\x x * 2) []";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::List(items) => assert_eq!(items.len(), 0),
            other => panic!("expected empty list, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_filter_empty_result() {
        let prog = "filter (\\x (x > 100)) [1, 2, 3]";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::List(items) => assert_eq!(items.len(), 0),
            other => panic!("expected empty list, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_filter_all_match() {
        let prog = "filter (\\x (x > 0)) [1, 2, 3]";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::List(items) => assert_eq!(items.len(), 3),
            other => panic!("expected 3-element list, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_fold_on_empty_list() {
        let prog = "fold (\\acc \\x acc + x) 100 []";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Number(Number::Int(n)) => assert_eq!(n, 100),
            other => panic!("expected 100, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_fold_multiplication_starting_zero() {
        let prog = "fold (\\acc \\x acc * x) 0 [1, 2, 3]";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Number(Number::Int(n)) => assert_eq!(n, 0),
            other => panic!("expected 0, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_string_repeat_zero() {
        let prog = "repeat \"x\" 0";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::String(s) => assert_eq!(s, ""),
            other => panic!("expected empty string, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_string_repeat_one() {
        let prog = "repeat \"hello\" 1";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::String(s) => assert_eq!(s, "hello"),
            other => panic!("expected 'hello', got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_nested_list_access_empty() {
        let prog = "head []";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        // head of empty list returns None
        match v {
            Value::None => {}
            other => panic!("expected None, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_tail_single_element() {
        let prog = "tail [42]";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::List(items) => assert_eq!(items.len(), 0),
            other => panic!("expected empty list, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_take_zero_elements() {
        let prog = "take 0 [1, 2, 3]";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::List(items) => assert_eq!(items.len(), 0),
            other => panic!("expected empty list, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_take_more_than_list_length() {
        let prog = "take 10 [1, 2, 3]";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::List(items) => assert_eq!(items.len(), 3),
            other => panic!("expected 3-element list, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_drop_zero_elements() {
        let prog = "drop 0 [1, 2, 3]";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::List(items) => assert_eq!(items.len(), 3),
            other => panic!("expected 3-element list, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_drop_all_elements() {
        let prog = "drop 3 [1, 2, 3]";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::List(items) => assert_eq!(items.len(), 0),
            other => panic!("expected empty list, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_reverse_empty_list() {
        let prog = "reverse []";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::List(items) => assert_eq!(items.len(), 0),
            other => panic!("expected empty list, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_reverse_single_element() {
        let prog = "reverse [42]";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::List(items) => {
                assert_eq!(items.len(), 1);
                if let Value::Number(Number::Int(n)) = items[0] {
                    assert_eq!(n, 42);
                } else {
                    panic!("expected int");
                }
            }
            other => panic!("expected 1-element list, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_string_contains_empty() {
        let prog = "contains \"hello\" \"\"";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b), // empty string is in every string
            other => panic!("expected true, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_string_starts_with_empty() {
        let prog = "starts_with \"hello\" \"\"";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b), // every string starts with empty string
            other => panic!("expected true, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_string_ends_with_empty() {
        let prog = "ends_with \"world\" \"\"";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b), // every string ends with empty string
            other => panic!("expected true, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_multiple_let_bindings() {
        let prog = "let x = 1 in let y = 2 in let z = 3 in x + y + z";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Number(Number::Int(n)) => assert_eq!(n, 6),
            other => panic!("expected 6, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_let_binding_in_expression() {
        let prog = "(let x = 10 in x) + 5";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Number(Number::Int(n)) => assert_eq!(n, 15),
            other => panic!("expected 15, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_lambda_immediate_call() {
        let prog = "(\\x x + 1) 5";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Number(Number::Int(n)) => assert_eq!(n, 6),
            other => panic!("expected 6, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_lambda_curried_call() {
        let prog = "(\\x \\y x + y) 5 10";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Number(Number::Int(n)) => assert_eq!(n, 15),
            other => panic!("expected 15, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_list_zip_empty_lists() {
        let prog = "zip [] []";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::List(items) => assert_eq!(items.len(), 0),
            other => panic!("expected empty list, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_list_zip_unequal_lengths() {
        let prog = "zip [1, 2] [\"a\"]";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        // zip should stop at shortest list
        match v {
            Value::List(items) => assert_eq!(items.len(), 1),
            other => panic!("expected 1-element list, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_modulo_operator() {
        let prog = "10 % 3";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Number(Number::Int(n)) => assert_eq!(n, 1),
            other => panic!("expected 1, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_comparison_chain_inequality() {
        let prog = "5 > 3 && 3 > 1";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Bool(b) => assert!(b),
            other => panic!("expected true, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_large_number_arithmetic() {
        let prog = "1000000 + 2000000";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Number(Number::Int(n)) => assert_eq!(n, 3000000),
            other => panic!("expected 3000000, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_negative_number_arithmetic() {
        let prog = "-10 + 5";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::Number(Number::Int(n)) => assert_eq!(n, -5),
            other => panic!("expected -5, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_string_join_empty_list() {
        let prog = "join [] \",\"";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::String(s) => assert_eq!(s, ""),
            other => panic!("expected empty string, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_case_string_join_single_element() {
        let prog = "join [\"hello\"] \",\"";
        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, prog).expect("eval");
        match v {
            Value::String(s) => assert_eq!(s, "hello"),
            other => panic!("expected 'hello', got {:?}", other),
        }
    }
}
