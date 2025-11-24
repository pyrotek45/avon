mod common;
mod lexer;
mod parser;
mod eval;
mod cli;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    cli::run_cli(args);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::common::*;
    use crate::lexer::tokenize;
    use crate::parser::parse;
    use crate::eval::{eval, initial_builtins, apply_function, collect_file_templates};
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
        assert!(v2.to_string(&data2).contains("a") );
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
                Value::Function { ident: _, default, .. } => {
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

        // json_parse object (currently stringified)
        let json_obj_path = dir.join("avon_test_json_obj.json");
        let mut jo = fs::File::create(&json_obj_path).expect("create json obj");
    write!(jo, "{}", r#"{"k": "v"}"#).expect("write json obj");
        let progo = format!("json_parse \"{}\"", json_obj_path.to_string_lossy());
        let tokens = tokenize(progo.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &progo).expect("eval");
        match v {
            Value::String(s) => assert!(s.contains("k")),
            other => panic!("expected string from json object, got {:?}", other),
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
            Value::FileTemplate { .. } | Value::Template(_, _) | Value::List(_) | Value::Function { .. } => {}
            other => panic!("unexpected template eval result: {:?}", other),
        }
    }

    #[test]
    fn test_default_params_apply_on_deploy_emulation() {
        // Function with defaults for name and age; emulated deploy should apply defaults
        let prog = r#"\name ? "alice" \age ? "30" @/tmp/{name}_{age}.txt {"Name: {name}\nAge: {age}\n"}"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let mut v = eval(ast.program, &mut symbols, &prog).expect("eval");

        // emulate deploy application loop: apply defaults when present
        loop {
            match &v {
                Value::Function { ident: _, default, .. } => {
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
                        v = apply_function(&v, Value::String(named_val), &prog).expect("apply named");
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
        assert!(files.iter().any(|(p, _)| p.contains("bob")), "expected output path to contain 'bob'");
    }

    #[test]
    fn test_dedent_removes_common_indentation_for_templates() {
        // create a template with leading indentation in the source; dedent should remove it
        let prog = "@/tmp/dedent_test.txt {\"\n        line1\n            line2\n        \"}".to_string();
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
        let prog = "@/tmp/dedent_rel.txt {\"\n    level1\n        level2\n    level1b\n\"}".to_string();
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
            if line == "level1" { found_level1 = true; }
            if line == "    level2" { found_level2 = true; }
        }
        assert!(found_level1, "level1 missing");
        assert!(found_level2, "level2 should be indented 4 spaces relative to dedented base");
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
        assert!(lines[1].starts_with("\t"), "bar should preserve its extra tab");
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
        assert_eq!(v.to_string(&prog), "script: func(){ return { key: 'val' }; }");
    }

    #[test]
    fn test_nested_json_with_escaped_braces() {
        // Nested JSON objects with proper brace escaping
        let prog = r#"{"{{ "outer": {{ "inner": {{ "key": "value" }} }} }}"}"#.to_string();
        let tokens = tokenize(prog.clone()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        let v = eval(ast.program, &mut symbols, &prog).expect("eval");
        assert_eq!(v.to_string(&prog), r#"{ "outer": { "inner": { "key": "value" } } }"#);
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
        assert_eq!(v.to_string(&prog), "one: {, two: {{, three: {{{, four: {{{{");
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
            (r#"if starts_with "hello" "hel" then "yes" else "no""#, "yes"),
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
        let prog2 = r#"let replace_world = replace "hello world" "world" in replace_world "avon""#.to_string();
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
    
}
