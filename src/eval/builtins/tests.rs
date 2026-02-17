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
fn test_default_function() {
    // Test 1: None triggers default
    match eval_prog("default \"fallback\" none") {
        Value::String(s) => assert_eq!(s, "fallback"),
        v => panic!("expected 'fallback', got {:?}", v),
    }

    // Test 2: Non-None value is returned as-is
    match eval_prog("default \"fallback\" \"actual\"") {
        Value::String(s) => assert_eq!(s, "actual"),
        v => panic!("expected 'actual', got {:?}", v),
    }

    // Test 3: Works with numbers
    match eval_prog("default 100 none") {
        Value::Number(Number::Int(100)) => {}
        v => panic!("expected 100, got {:?}", v),
    }

    match eval_prog("default 100 42") {
        Value::Number(Number::Int(42)) => {}
        v => panic!("expected 42, got {:?}", v),
    }

    // Test 4: false is NOT treated as None
    match eval_prog("default true false") {
        Value::Bool(false) => {}
        v => panic!("expected false, got {:?}", v),
    }

    // Test 5: 0 is NOT treated as None
    match eval_prog("default 999 0") {
        Value::Number(Number::Int(0)) => {}
        v => panic!("expected 0, got {:?}", v),
    }

    // Test 6: Empty string is NOT treated as None
    match eval_prog("default \"fallback\" \"\"") {
        Value::String(s) => assert_eq!(s, ""),
        v => panic!("expected empty string, got {:?}", v),
    }

    // Test 7: Works with head (returns None on empty list)
    match eval_prog("head [] -> default \"empty\"") {
        Value::String(s) => assert_eq!(s, "empty"),
        v => panic!("expected 'empty', got {:?}", v),
    }

    // Test 8: Works with get (returns None on missing key)
    match eval_prog("get {a: 1} \"b\" -> default 0") {
        Value::Number(Number::Int(0)) => {}
        v => panic!("expected 0, got {:?}", v),
    }

    // Test 9: Works with find (returns None when no match)
    match eval_prog("find (\\x x > 100) [1, 2, 3] -> default 999") {
        Value::Number(Number::Int(999)) => {}
        v => panic!("expected 999, got {:?}", v),
    }

    // Test 10: Multi-level chaining
    match eval_prog("head [] -> default (last [] -> default \"final\")") {
        Value::String(s) => assert_eq!(s, "final"),
        v => panic!("expected 'final', got {:?}", v),
    }

    // Test 11: Works with lists
    match eval_prog("default [1, 2] none") {
        Value::List(l) => assert_eq!(l.len(), 2),
        v => panic!("expected list, got {:?}", v),
    }

    // Test 12: Works with dicts
    match eval_prog("default {x: 0} none") {
        Value::Dict(d) => assert_eq!(d.len(), 1),
        v => panic!("expected dict, got {:?}", v),
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

#[test]
fn test_nested_templates_single_brace() {
    // Test basic nested template with single braces
    let result = eval_prog(r#"{"outer { map (\x {"inner"}) [1] } end"}"#);
    match result {
        Value::Template(_, _) => {
            // Template evaluated successfully - the actual string conversion
            // is tested through the interpreter's template evaluation
        }
        v => panic!("expected template, got {:?}", v),
    }
}

#[test]
fn test_nested_templates_double_brace() {
    // Test nested template with double braces
    let result = eval_prog(r#"{{"outer {{map (\x {"inner"}) [1]}}"}} "#);
    match result {
        Value::Template(_, _) => {
            // Template evaluated successfully
        }
        v => panic!("expected template, got {:?}", v),
    }
}

#[test]
fn test_nested_templates_mixed_braces() {
    // Test nested template with mixed brace levels
    let result = eval_prog(r#"{"a { map (\x {{"double-brace"}}) [1] } b"}"#);
    match result {
        Value::Template(_, _) => {
            // Template evaluated successfully
        }
        v => panic!("expected template, got {:?}", v),
    }
}

#[test]
fn test_nested_templates_with_interpolation() {
    // Test nested template with variable interpolation
    let result = eval_prog(
        r#"let items = [{ val: "a" }, { val: "b" }] in
           {"<div>{ map (\i {"<p>{i.val}</p>"}) items }</div>"}"#,
    );
    match result {
        Value::Template(_, _) => {
            // Template evaluated successfully with nested templates in map
        }
        v => panic!("expected template, got {:?}", v),
    }
}

#[test]
fn test_nested_templates_complex() {
    // Test complex nested templates with multiple levels
    let result = eval_prog(
        r#"let buttons = [{ name: "Home", url: "index.html" }, { name: "About", url: "about.html" }] in
           {"<nav>{ map (\btn {"<a href=\"{btn.url}\">{btn.name}</a>"}) buttons }</nav>"}"#,
    );
    match result {
        Value::Template(_, _) => {
            // Template evaluated successfully with nested templates in map
        }
        v => panic!("expected template, got {:?}", v),
    }
}

#[test]
fn test_xml_parsing() {
    use std::io::Write;

    let path = "test_data.xml";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(
        b"<root><person name=\"Alice\" age=\"30\"><email>alice@example.com</email></person></root>",
    )
    .expect("write file");

    let prog = format!("xml_parse \"{}\"", path);
    let result = eval_prog(&prog);

    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::Dict(root) => {
            match root.get("tag").unwrap() {
                Value::String(s) => assert_eq!(s, "root"),
                v => panic!("expected string tag, got {:?}", v),
            }
            match root.get("children").unwrap() {
                Value::List(children) => {
                    assert_eq!(children.len(), 1);
                    if let Value::Dict(person) = &children[0] {
                        match person.get("tag").unwrap() {
                            Value::String(s) => assert_eq!(s, "person"),
                            v => panic!("expected 'person', got {:?}", v),
                        }
                        if let Value::Dict(attrs) = person.get("attrs").unwrap() {
                            match attrs.get("name").unwrap() {
                                Value::String(s) => assert_eq!(s, "Alice"),
                                v => panic!("expected 'Alice', got {:?}", v),
                            }
                            match attrs.get("age").unwrap() {
                                Value::String(s) => assert_eq!(s, "30"),
                                v => panic!("expected '30', got {:?}", v),
                            }
                        } else {
                            panic!("expected dict for attrs");
                        }
                        if let Value::List(kids) = person.get("children").unwrap() {
                            assert_eq!(kids.len(), 1);
                            if let Value::Dict(email) = &kids[0] {
                                match email.get("tag").unwrap() {
                                    Value::String(s) => assert_eq!(s, "email"),
                                    v => panic!("expected 'email', got {:?}", v),
                                }
                                match email.get("text").unwrap() {
                                    Value::String(s) => assert_eq!(s, "alice@example.com"),
                                    v => panic!("expected email text, got {:?}", v),
                                }
                            } else {
                                panic!("expected dict for email element");
                            }
                        } else {
                            panic!("expected list for children");
                        }
                    } else {
                        panic!("expected dict for person");
                    }
                }
                v => panic!("expected list of children, got {:?}", v),
            }
        }
        v => panic!("expected dict, got {:?}", v),
    }
}

#[test]
fn test_opml_parsing() {
    use std::io::Write;

    let path = "test_data.opml";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(
        b"<?xml version=\"1.0\"?>\n\
          <opml version=\"2.0\">\n\
          <head><title>Test</title></head>\n\
          <body>\n\
            <outline text=\"News\" title=\"News\">\n\
              <outline text=\"HN\" type=\"rss\" xmlUrl=\"https://hn.com/rss\"/>\n\
            </outline>\n\
          </body>\n\
          </opml>",
    )
    .expect("write file");

    let prog = format!("opml_parse \"{}\"", path);
    let result = eval_prog(&prog);

    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::Dict(opml) => {
            // Check version
            match opml.get("version").unwrap() {
                Value::String(s) => assert_eq!(s, "2.0"),
                v => panic!("expected version string, got {:?}", v),
            }
            // Check head
            if let Value::Dict(head) = opml.get("head").unwrap() {
                match head.get("title").unwrap() {
                    Value::String(s) => assert_eq!(s, "Test"),
                    v => panic!("expected title string, got {:?}", v),
                }
            } else {
                panic!("expected dict for head");
            }
            // Check outlines
            if let Value::List(outlines) = opml.get("outlines").unwrap() {
                assert_eq!(outlines.len(), 1);
                if let Value::Dict(news) = &outlines[0] {
                    match news.get("text").unwrap() {
                        Value::String(s) => assert_eq!(s, "News"),
                        v => panic!("expected 'News', got {:?}", v),
                    }
                    // Check nested outline
                    if let Value::List(kids) = news.get("children").unwrap() {
                        assert_eq!(kids.len(), 1);
                        if let Value::Dict(hn) = &kids[0] {
                            match hn.get("text").unwrap() {
                                Value::String(s) => assert_eq!(s, "HN"),
                                v => panic!("expected 'HN', got {:?}", v),
                            }
                            match hn.get("type").unwrap() {
                                Value::String(s) => assert_eq!(s, "rss"),
                                v => panic!("expected 'rss', got {:?}", v),
                            }
                            match hn.get("xmlUrl").unwrap() {
                                Value::String(s) => assert_eq!(s, "https://hn.com/rss"),
                                v => panic!("expected xmlUrl, got {:?}", v),
                            }
                        } else {
                            panic!("expected dict for HN outline");
                        }
                    } else {
                        panic!("expected list for children");
                    }
                } else {
                    panic!("expected dict for News outline");
                }
            } else {
                panic!("expected list for outlines");
            }
        }
        v => panic!("expected dict, got {:?}", v),
    }
}

#[test]
fn test_format_xml() {
    // Build a dict structure and format it to XML
    let prog = r#"
        let data = {
            tag: "root",
            children: [
                {tag: "item", attrs: {id: "1"}, text: "Hello"},
                {tag: "item", attrs: {id: "2"}, text: "World"}
            ]
        } in
        format_xml data
    "#;
    match eval_prog(prog) {
        Value::String(s) => {
            assert!(s.contains("<root>"), "should contain <root>");
            assert!(s.contains("</root>"), "should contain </root>");
            assert!(s.contains("<item id=\"1\">Hello</item>"), "should contain first item");
            assert!(s.contains("<item id=\"2\">World</item>"), "should contain second item");
        }
        v => panic!("expected string, got {:?}", v),
    }

    // Test self-closing element
    let prog2 = r#"
        let data = {tag: "br"} in
        format_xml data
    "#;
    match eval_prog(prog2) {
        Value::String(s) => {
            assert!(s.contains("<br />"), "empty element should be self-closing");
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_format_opml() {
    let prog = r#"
        let data = {
            version: "2.0",
            head: {title: "My Feeds"},
            outlines: [
                {text: "Tech", children: [
                    {text: "HN", type: "rss", xmlUrl: "https://hn.com/rss"}
                ]}
            ]
        } in
        format_opml data
    "#;
    match eval_prog(prog) {
        Value::String(s) => {
            assert!(s.contains("<?xml version=\"1.0\""), "should have xml declaration");
            assert!(s.contains("<opml version=\"2.0\">"), "should have opml element");
            assert!(s.contains("<title>My Feeds</title>"), "should have title");
            assert!(s.contains("text=\"Tech\""), "should have Tech outline");
            assert!(s.contains("text=\"HN\""), "should have HN outline");
            assert!(s.contains("xmlUrl=\"https://hn.com/rss\""), "should have xmlUrl");
            assert!(s.contains("</opml>"), "should close opml");
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_xml_roundtrip() {
    use std::io::Write;

    let path = "test_roundtrip.xml";
    let xml_content =
        b"<config><db host=\"localhost\" port=\"5432\"/><cache enabled=\"true\"/></config>";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(xml_content).expect("write file");

    // Parse then format
    let prog = format!("let data = xml_parse \"{}\" in format_xml data", path);
    let result = eval_prog(&prog);

    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::String(s) => {
            assert!(
                s.contains("<config>"),
                "roundtrip should preserve root tag"
            );
            assert!(
                s.contains("host=\"localhost\""),
                "roundtrip should preserve attrs"
            );
            assert!(
                s.contains("port=\"5432\""),
                "roundtrip should preserve attrs"
            );
            assert!(
                s.contains("<cache"),
                "roundtrip should preserve elements"
            );
            assert!(
                s.contains("enabled=\"true\""),
                "roundtrip should preserve attrs"
            );
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_opml_roundtrip() {
    use std::io::Write;

    let path = "test_roundtrip.opml";
    let opml_content = b"<?xml version=\"1.0\"?>\n\
        <opml version=\"2.0\">\n\
        <head><title>Feeds</title></head>\n\
        <body><outline text=\"Blog\" type=\"rss\" xmlUrl=\"https://example.com/rss\"/></body>\n\
        </opml>";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(opml_content).expect("write file");

    let prog = format!("let data = opml_parse \"{}\" in format_opml data", path);
    let result = eval_prog(&prog);

    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::String(s) => {
            assert!(
                s.contains("<opml version=\"2.0\">"),
                "roundtrip should preserve version"
            );
            assert!(
                s.contains("<title>Feeds</title>"),
                "roundtrip should preserve head"
            );
            assert!(
                s.contains("text=\"Blog\""),
                "roundtrip should preserve outline text"
            );
            assert!(
                s.contains("type=\"rss\""),
                "roundtrip should preserve outline type"
            );
            assert!(
                s.contains("xmlUrl=\"https://example.com/rss\""),
                "roundtrip should preserve xmlUrl"
            );
        }
        v => panic!("expected string, got {:?}", v),
    }
}

// ── INI tests ────────────────────────────────────────────────

#[test]
fn test_ini_parsing() {
    use std::io::Write;

    let path = "test_data.ini";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(
        b"version=1.0\n\n[database]\nhost=localhost\nport=3306\n\n[server]\nport=8080\ndebug=true\n",
    )
    .expect("write file");

    let prog = format!("ini_parse \"{}\"", path);
    let result = eval_prog(&prog);

    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::Dict(ini) => {
            // Check global section
            if let Value::Dict(global) = ini.get("global").unwrap() {
                match global.get("version").unwrap() {
                    Value::String(s) => assert_eq!(s, "1.0"),
                    v => panic!("expected version string, got {:?}", v),
                }
            } else {
                panic!("expected dict for global section");
            }
            // Check database section
            if let Value::Dict(db) = ini.get("database").unwrap() {
                match db.get("host").unwrap() {
                    Value::String(s) => assert_eq!(s, "localhost"),
                    v => panic!("expected host string, got {:?}", v),
                }
                match db.get("port").unwrap() {
                    Value::String(s) => assert_eq!(s, "3306"),
                    v => panic!("expected port string, got {:?}", v),
                }
            } else {
                panic!("expected dict for database section");
            }
            // Check server section
            if let Value::Dict(srv) = ini.get("server").unwrap() {
                match srv.get("port").unwrap() {
                    Value::String(s) => assert_eq!(s, "8080"),
                    v => panic!("expected port string, got {:?}", v),
                }
                match srv.get("debug").unwrap() {
                    Value::String(s) => assert_eq!(s, "true"),
                    v => panic!("expected debug string, got {:?}", v),
                }
            } else {
                panic!("expected dict for server section");
            }
        }
        v => panic!("expected dict, got {:?}", v),
    }
}

#[test]
fn test_format_ini_dict() {
    let prog = r#"
        let data = {
            database: {host: "localhost", port: "3306"},
            server: {port: "8080", debug: "true"}
        } in
        format_ini data
    "#;
    match eval_prog(prog) {
        Value::String(s) => {
            assert!(
                s.contains("[database]"),
                "should have database section header"
            );
            assert!(
                s.contains("host=localhost"),
                "should have host key"
            );
            assert!(
                s.contains("port=3306"),
                "should have port key in database"
            );
            assert!(
                s.contains("[server]"),
                "should have server section header"
            );
            assert!(
                s.contains("debug=true"),
                "should have debug key"
            );
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_format_ini_with_global() {
    let prog = r#"
        let data = {
            global: {version: "1.0"},
            app: {name: "myapp"}
        } in
        format_ini data
    "#;
    match eval_prog(prog) {
        Value::String(s) => {
            // Global keys should appear without a [section] header
            assert!(
                s.contains("version=1.0"),
                "should have global version key"
            );
            assert!(
                !s.contains("[global]"),
                "global section should NOT have a header"
            );
            assert!(
                s.contains("[app]"),
                "should have app section header"
            );
            assert!(
                s.contains("name=myapp"),
                "should have name key"
            );
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_ini_roundtrip() {
    use std::io::Write;

    let path = "test_roundtrip.ini";
    let ini_content = b"[database]\nhost=localhost\nport=3306\n\n[server]\ndebug=true\nport=8080\n";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(ini_content).expect("write file");

    let prog = format!("let data = ini_parse \"{}\" in format_ini data", path);
    let result = eval_prog(&prog);

    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::String(s) => {
            assert!(
                s.contains("[database]"),
                "roundtrip should preserve database section"
            );
            assert!(
                s.contains("host=localhost"),
                "roundtrip should preserve host"
            );
            assert!(
                s.contains("port=3306"),
                "roundtrip should preserve port"
            );
            assert!(
                s.contains("[server]"),
                "roundtrip should preserve server section"
            );
            assert!(
                s.contains("debug=true"),
                "roundtrip should preserve debug"
            );
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_cross_ini_to_json() {
    use std::io::Write;

    let path = "test_i2j.ini";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"[database]\nhost=localhost\nport=3306\n").expect("write file");

    let prog = format!(
        "let data = ini_parse \"{}\" in format_json data",
        path
    );
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::String(s) => {
            assert!(s.contains("\"database\""), "JSON should contain database key");
            assert!(s.contains("\"host\""), "JSON should contain host key");
            assert!(s.contains("\"localhost\""), "JSON should contain localhost");
            assert!(s.contains("\"port\""), "JSON should contain port key");
            assert!(s.contains("\"3306\""), "JSON should contain 3306");
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_cross_json_to_ini() {
    use std::io::Write;

    let path = "test_j2i.json";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(
        b"{\"database\": {\"host\": \"localhost\", \"port\": \"5432\"}}",
    )
    .expect("write file");

    let prog = format!(
        "let data = json_parse \"{}\" in format_ini data",
        path
    );
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::String(s) => {
            assert!(s.contains("[database]"), "INI should have database section");
            assert!(s.contains("host=localhost"), "INI should have host");
            assert!(s.contains("port=5432"), "INI should have port");
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_parsed_ini_works_with_get_has_key() {
    use std::io::Write;

    let path = "test_ini_ops.ini";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"[database]\nhost=localhost\nport=3306\n").expect("write file");

    // Test get on parsed INI
    let prog = format!(
        "let cfg = ini_parse \"{}\" in get cfg \"database\"",
        path
    );
    match eval_prog(&prog) {
        Value::Dict(db) => {
            match db.get("host").unwrap() {
                Value::String(s) => assert_eq!(s, "localhost"),
                v => panic!("expected host string, got {:?}", v),
            }
        }
        v => panic!("expected dict, got {:?}", v),
    }

    // Test has_key
    let prog = format!(
        "let cfg = ini_parse \"{}\" in has_key cfg \"database\"",
        path
    );
    match eval_prog(&prog) {
        Value::Bool(b) => assert!(b, "should have 'database' key"),
        v => panic!("expected bool, got {:?}", v),
    }

    // Test keys
    let prog = format!(
        "let cfg = ini_parse \"{}\" in keys cfg",
        path
    );
    match eval_prog(&prog) {
        Value::List(ks) => {
            let key_strs: Vec<String> = ks.iter().map(|v| match v {
                Value::String(s) => s.clone(),
                _ => String::new(),
            }).collect();
            assert!(key_strs.contains(&"database".to_string()), "keys should include database");
        }
        v => panic!("expected list, got {:?}", v),
    }

    // Test dot notation
    let prog = format!(
        "let cfg = ini_parse \"{}\" in cfg.database.host",
        path
    );
    match eval_prog(&prog) {
        Value::String(s) => assert_eq!(s, "localhost", "dot notation should access nested value"),
        v => panic!("expected string, got {:?}", v),
    }

    std::fs::remove_file(path).expect("remove file");
}

// ── HTML tests ───────────────────────────────────────────────

#[test]
fn test_html_parsing() {
    use std::io::Write;

    let path = "test_data.html";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(
        b"<html><head><title>Hello</title></head><body><p class=\"intro\">World</p></body></html>",
    )
    .expect("write file");

    let prog = format!("html_parse \"{}\"", path);
    let result = eval_prog(&prog);

    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::Dict(root) => {
            // Root element is <html>
            match root.get("tag").unwrap() {
                Value::String(s) => assert_eq!(s, "html"),
                v => panic!("expected tag string, got {:?}", v),
            }
            // Should have children
            match root.get("children").unwrap() {
                Value::List(children) => {
                    assert!(
                        children.len() >= 2,
                        "html should have head and body children"
                    );
                }
                v => panic!("expected children list, got {:?}", v),
            }
        }
        v => panic!("expected dict, got {:?}", v),
    }
}

#[test]
fn test_html_parsing_with_attrs() {
    use std::io::Write;

    let path = "test_html_attrs.html";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"<html><body><a href=\"https://example.com\" id=\"link1\">Click</a></body></html>")
        .expect("write file");

    let prog = format!(
        "let doc = html_parse \"{}\" in doc.children",
        path
    );
    let result = eval_prog(&prog);

    std::fs::remove_file(path).expect("remove file");

    // Navigate into the structure to find the <a> element
    match result {
        Value::List(_) => {
            // If we got a list, the structure is correct
        }
        v => panic!("expected list for children, got {:?}", v),
    }
}

#[test]
fn test_format_html_simple() {
    let prog = r#"
        let data = {
            tag: "div",
            attrs: {class: "container"},
            children: [
                {tag: "p", attrs: {}, children: [], text: "Hello"}
            ],
            text: ""
        } in
        format_html data
    "#;
    match eval_prog(prog) {
        Value::String(s) => {
            assert!(
                s.contains("<div"),
                "should contain opening div tag"
            );
            assert!(
                s.contains("class=\"container\""),
                "should contain class attribute"
            );
            assert!(
                s.contains("<p>Hello</p>"),
                "should contain p tag with text"
            );
            assert!(
                s.contains("</div>"),
                "should contain closing div tag"
            );
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_format_html_void_elements() {
    let prog = r#"
        let data = {
            tag: "div",
            attrs: {},
            children: [
                {tag: "br", attrs: {}, children: [], text: ""},
                {tag: "img", attrs: {src: "photo.jpg", alt: "Photo"}, children: [], text: ""},
                {tag: "hr", attrs: {}, children: [], text: ""}
            ],
            text: ""
        } in
        format_html data
    "#;
    match eval_prog(prog) {
        Value::String(s) => {
            assert!(
                s.contains("<br>"),
                "should render br as void element"
            );
            assert!(
                !s.contains("</br>"),
                "br should NOT have closing tag"
            );
            assert!(
                s.contains("<img"),
                "should contain img tag"
            );
            assert!(
                s.contains("src=\"photo.jpg\""),
                "should contain src attribute"
            );
            assert!(
                !s.contains("</img>"),
                "img should NOT have closing tag"
            );
            assert!(
                s.contains("<hr>"),
                "should render hr as void element"
            );
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_html_roundtrip() {
    use std::io::Write;

    let path = "test_roundtrip.html";
    let html_content = b"<html><head><title>Test</title></head><body><p class=\"main\">Hello</p></body></html>";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(html_content).expect("write file");

    let prog = format!(
        "let data = html_parse \"{}\" in format_html data",
        path
    );
    let result = eval_prog(&prog);

    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::String(s) => {
            assert!(
                s.contains("<html"),
                "roundtrip should preserve html tag"
            );
            assert!(
                s.contains("<title>Test</title>"),
                "roundtrip should preserve title"
            );
            assert!(
                s.contains("class=\"main\""),
                "roundtrip should preserve class attr"
            );
            assert!(
                s.contains("Hello"),
                "roundtrip should preserve text content"
            );
            assert!(
                s.contains("</body>"),
                "roundtrip should have closing body"
            );
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_cross_html_to_json() {
    use std::io::Write;

    let path = "test_h2j.html";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"<html><body><p class=\"intro\">Hello</p></body></html>")
        .expect("write file");

    let prog = format!(
        "let data = html_parse \"{}\" in format_json data",
        path
    );
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::String(s) => {
            assert!(s.contains("\"tag\""), "JSON should contain tag key");
            assert!(s.contains("\"html\""), "JSON should contain html tag value");
            assert!(s.contains("\"children\""), "JSON should contain children key");
            assert!(s.contains("\"intro\""), "JSON should contain class value");
        }
        v => panic!("expected string, got {:?}", v),
    }
}

// ═══════════════════════════════════════════════════════════════
// Tutorial claim verification tests
// Every claim in BUILTIN_FUNCTIONS.md "Data Format Conversion"
// section is tested here.
// ═══════════════════════════════════════════════════════════════

// ── Parser tests (JSON, YAML, TOML read from files) ──────────

#[test]
fn test_json_parsing() {
    use std::io::Write;

    let path = "test_json_parse.json";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"{\"name\": \"Alice\", \"age\": 30, \"active\": true}")
        .expect("write file");

    let prog = format!("json_parse \"{}\"", path);
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::Dict(d) => {
            match d.get("name").unwrap() {
                Value::String(s) => assert_eq!(s, "Alice"),
                v => panic!("expected string, got {:?}", v),
            }
            match d.get("age").unwrap() {
                Value::Number(Number::Int(30)) => {}
                v => panic!("expected 30, got {:?}", v),
            }
            match d.get("active").unwrap() {
                Value::Bool(true) => {}
                v => panic!("expected true, got {:?}", v),
            }
        }
        v => panic!("expected dict, got {:?}", v),
    }
}

#[test]
fn test_json_parsing_array() {
    use std::io::Write;

    let path = "test_json_array.json";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"[1, 2, 3]").expect("write file");

    let prog = format!("json_parse \"{}\"", path);
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::List(list) => {
            assert_eq!(list.len(), 3);
            match &list[0] {
                Value::Number(Number::Int(1)) => {}
                v => panic!("expected 1, got {:?}", v),
            }
        }
        v => panic!("expected list, got {:?}", v),
    }
}

#[test]
fn test_yaml_parsing() {
    use std::io::Write;

    let path = "test_yaml_parse.yml";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"name: Bob\nport: 8080\ndebug: false\n")
        .expect("write file");

    let prog = format!("yaml_parse \"{}\"", path);
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::Dict(d) => {
            match d.get("name").unwrap() {
                Value::String(s) => assert_eq!(s, "Bob"),
                v => panic!("expected 'Bob', got {:?}", v),
            }
            match d.get("port").unwrap() {
                Value::Number(Number::Int(8080)) => {}
                v => panic!("expected 8080, got {:?}", v),
            }
            match d.get("debug").unwrap() {
                Value::Bool(false) => {}
                v => panic!("expected false, got {:?}", v),
            }
        }
        v => panic!("expected dict, got {:?}", v),
    }
}

#[test]
fn test_toml_parsing() {
    use std::io::Write;

    let path = "test_toml_parse.toml";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"name = \"myapp\"\nversion = \"1.0\"\n\n[database]\nhost = \"localhost\"\nport = 5432\n")
        .expect("write file");

    let prog = format!("toml_parse \"{}\"", path);
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::Dict(d) => {
            match d.get("name").unwrap() {
                Value::String(s) => assert_eq!(s, "myapp"),
                v => panic!("expected 'myapp', got {:?}", v),
            }
            match d.get("version").unwrap() {
                Value::String(s) => assert_eq!(s, "1.0"),
                v => panic!("expected '1.0', got {:?}", v),
            }
            // Check nested table
            if let Value::Dict(db) = d.get("database").unwrap() {
                match db.get("host").unwrap() {
                    Value::String(s) => assert_eq!(s, "localhost"),
                    v => panic!("expected 'localhost', got {:?}", v),
                }
                match db.get("port").unwrap() {
                    Value::Number(Number::Int(5432)) => {}
                    v => panic!("expected 5432, got {:?}", v),
                }
            } else {
                panic!("expected dict for database section");
            }
        }
        v => panic!("expected dict, got {:?}", v),
    }
}

// ── Formatter tests (JSON, YAML, TOML produce correct output) ──

#[test]
fn test_format_json_dict() {
    let prog = r#"format_json {name: "Alice", age: 30}"#;
    match eval_prog(prog) {
        Value::String(s) => {
            assert!(s.contains("\"name\""), "JSON should have quoted keys");
            assert!(s.contains("\"Alice\""), "JSON should have quoted string values");
            assert!(s.contains("30"), "JSON should have number values");
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_format_json_list() {
    let prog = r#"format_json [1, 2, 3]"#;
    match eval_prog(prog) {
        Value::String(s) => {
            assert!(s.starts_with('['), "JSON array should start with [");
            assert!(s.ends_with(']'), "JSON array should end with ]");
            assert!(s.contains('1') && s.contains('2') && s.contains('3'));
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_format_yaml_dict() {
    let prog = r#"format_yaml {name: "Alice", age: 30}"#;
    match eval_prog(prog) {
        Value::String(s) => {
            // YAML uses key: value (no quotes on keys)
            assert!(s.contains("name:"), "YAML should have unquoted keys");
            assert!(s.contains("Alice"), "YAML should have values");
            assert!(s.contains("age:"), "YAML should have age key");
            assert!(s.contains("30"), "YAML should have number value");
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_format_yaml_list() {
    let prog = r#"format_yaml [1, 2, 3]"#;
    match eval_prog(prog) {
        Value::String(s) => {
            // YAML lists use "- " prefix
            assert!(s.contains("- 1"), "YAML list should use dash prefix");
            assert!(s.contains("- 2"));
            assert!(s.contains("- 3"));
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_format_toml_dict() {
    let prog = r#"format_toml {name: "myapp", version: "1.0"}"#;
    match eval_prog(prog) {
        Value::String(s) => {
            // TOML uses key = "value"
            assert!(
                s.contains("name = \"myapp\""),
                "TOML should use key = \"value\" format, got: {}",
                s
            );
            assert!(s.contains("version = \"1.0\""));
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_format_toml_nested() {
    let prog = r#"format_toml {package: {name: "app", version: "1.0"}}"#;
    match eval_prog(prog) {
        Value::String(s) => {
            // Nested dicts become TOML sections [section]
            assert!(
                s.contains("[package]"),
                "TOML should have section headers, got: {}",
                s
            );
            assert!(s.contains("name = \"app\""));
        }
        v => panic!("expected string, got {:?}", v),
    }
}

// ── Roundtrip tests for ALL 6 formats ──────────────────────

#[test]
fn test_json_roundtrip() {
    use std::io::Write;

    let path = "test_json_rt.json";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"{\"host\": \"localhost\", \"port\": 3000, \"debug\": true}")
        .expect("write file");

    let prog = format!("let data = json_parse \"{}\" in format_json data", path);
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::String(s) => {
            assert!(s.contains("\"host\""), "roundtrip should preserve host key");
            assert!(
                s.contains("\"localhost\""),
                "roundtrip should preserve host value"
            );
            assert!(s.contains("3000"), "roundtrip should preserve port");
            assert!(s.contains("true"), "roundtrip should preserve debug bool");
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_yaml_roundtrip() {
    use std::io::Write;

    let path = "test_yaml_rt.yml";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"server:\n  host: localhost\n  port: 9090\n")
        .expect("write file");

    let prog = format!("let data = yaml_parse \"{}\" in format_yaml data", path);
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::String(s) => {
            assert!(s.contains("server:"), "roundtrip should preserve top key");
            assert!(
                s.contains("host:") || s.contains("host :"),
                "roundtrip should preserve nested key"
            );
            assert!(
                s.contains("localhost"),
                "roundtrip should preserve host value"
            );
            assert!(s.contains("9090"), "roundtrip should preserve port");
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_toml_roundtrip() {
    use std::io::Write;

    let path = "test_toml_rt.toml";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"name = \"demo\"\n\n[server]\nhost = \"0.0.0.0\"\nport = 8080\n")
        .expect("write file");

    let prog = format!("let data = toml_parse \"{}\" in format_toml data", path);
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::String(s) => {
            assert!(
                s.contains("name = \"demo\""),
                "roundtrip should preserve top-level key"
            );
            assert!(
                s.contains("[server]"),
                "roundtrip should preserve section header"
            );
            assert!(
                s.contains("host = \"0.0.0.0\""),
                "roundtrip should preserve server host"
            );
            assert!(
                s.contains("port = 8080"),
                "roundtrip should preserve port"
            );
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_csv_roundtrip() {
    use std::io::Write;

    let path = "test_csv_rt.csv";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"name,role\nAlice,admin\nBob,user\n")
        .expect("write file");

    let prog = format!("let data = csv_parse \"{}\" in format_csv data", path);
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::String(s) => {
            assert!(
                s.contains("name") && s.contains("role"),
                "roundtrip should preserve headers"
            );
            assert!(
                s.contains("Alice") && s.contains("admin"),
                "roundtrip should preserve row 1"
            );
            assert!(
                s.contains("Bob") && s.contains("user"),
                "roundtrip should preserve row 2"
            );
        }
        v => panic!("expected string, got {:?}", v),
    }
}

// ── Cross-format conversion tests ────────────────────────────
// These verify the tutorial claim:
//   "Parse one format and output another —
//    Avon values are the universal intermediate representation"

#[test]
fn test_cross_json_to_yaml() {
    use std::io::Write;

    let path = "test_j2y.json";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"{\"host\": \"localhost\", \"port\": 5432}")
        .expect("write file");

    let prog = format!(
        "let data = json_parse \"{}\" in format_yaml data",
        path
    );
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::String(s) => {
            // YAML output should have the data from the JSON input
            assert!(s.contains("host:"), "YAML should have host key");
            assert!(s.contains("localhost"), "YAML should have host value");
            assert!(s.contains("port:"), "YAML should have port key");
            assert!(s.contains("5432"), "YAML should have port value");
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_cross_yaml_to_toml() {
    use std::io::Write;

    let path = "test_y2t.yml";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"name: myapp\nversion: '2.0'\n")
        .expect("write file");

    let prog = format!(
        "let data = yaml_parse \"{}\" in format_toml data",
        path
    );
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::String(s) => {
            assert!(
                s.contains("name = \"myapp\""),
                "TOML should have name, got: {}",
                s
            );
            assert!(
                s.contains("version = \"2.0\""),
                "TOML should have version, got: {}",
                s
            );
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_cross_toml_to_json() {
    use std::io::Write;

    let path = "test_t2j.toml";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"title = \"Demo\"\ncount = 42\n")
        .expect("write file");

    let prog = format!(
        "let data = toml_parse \"{}\" in format_json data",
        path
    );
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::String(s) => {
            assert!(
                s.contains("\"title\""),
                "JSON should have quoted title key"
            );
            assert!(s.contains("\"Demo\""), "JSON should have Demo value");
            assert!(s.contains("42"), "JSON should have count value");
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_cross_csv_to_json() {
    use std::io::Write;

    let path = "test_c2j.csv";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"name,score\nAlice,95\nBob,87\n")
        .expect("write file");

    let prog = format!(
        "let data = csv_parse \"{}\" in format_json data",
        path
    );
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::String(s) => {
            assert!(s.starts_with('['), "JSON array should start with [");
            assert!(s.contains("\"name\""), "JSON should have name key");
            assert!(s.contains("\"Alice\""), "JSON should have Alice");
            assert!(s.contains("\"95\""), "JSON should have score (as string)");
            assert!(s.contains("\"Bob\""), "JSON should have Bob");
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_cross_xml_to_json() {
    use std::io::Write;

    let path = "test_x2j.xml";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"<config><db host=\"localhost\" port=\"5432\"/></config>")
        .expect("write file");

    let prog = format!(
        "let data = xml_parse \"{}\" in format_json data",
        path
    );
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::String(s) => {
            assert!(
                s.contains("\"tag\""),
                "JSON should represent XML tag key"
            );
            assert!(
                s.contains("\"config\""),
                "JSON should contain config tag name"
            );
            assert!(
                s.contains("\"localhost\""),
                "JSON should contain host attr value"
            );
            assert!(
                s.contains("\"5432\""),
                "JSON should contain port attr value"
            );
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_cross_xml_to_yaml() {
    use std::io::Write;

    let path = "test_x2y.xml";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"<root><item>Hello</item></root>")
        .expect("write file");

    let prog = format!(
        "let data = xml_parse \"{}\" in format_yaml data",
        path
    );
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::String(s) => {
            assert!(s.contains("tag:"), "YAML should have tag key");
            assert!(s.contains("root"), "YAML should have root tag value");
            assert!(s.contains("Hello"), "YAML should contain item text");
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_cross_json_to_csv() {
    // JSON array of objects → CSV (tabular conversion)
    use std::io::Write;

    let path = "test_j2c.json";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(
        b"[{\"name\": \"Alice\", \"role\": \"admin\"}, {\"name\": \"Bob\", \"role\": \"user\"}]",
    )
    .expect("write file");

    let prog = format!(
        "let data = json_parse \"{}\" in format_csv data",
        path
    );
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::String(s) => {
            assert!(
                s.contains("name") && s.contains("role"),
                "CSV should have headers from JSON keys"
            );
            assert!(s.contains("Alice"), "CSV should contain Alice");
            assert!(s.contains("Bob"), "CSV should contain Bob");
            assert!(s.contains("admin"), "CSV should contain admin");
        }
        v => panic!("expected string, got {:?}", v),
    }
}

// ── Universal Avon Value type tests ──────────────────────────
// These verify that parsed data becomes standard Avon Dict/List
// values that work with ALL collection operations.

#[test]
fn test_parsed_json_works_with_map() {
    use std::io::Write;

    let path = "test_j_map.json";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"[{\"name\": \"Alice\"}, {\"name\": \"Bob\"}]")
        .expect("write file");

    let prog = format!(
        "let data = json_parse \"{}\" in map (\\x x.name) data",
        path
    );
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::List(list) => {
            assert_eq!(list.len(), 2);
            match &list[0] {
                Value::String(s) => assert_eq!(s, "Alice"),
                v => panic!("expected Alice, got {:?}", v),
            }
            match &list[1] {
                Value::String(s) => assert_eq!(s, "Bob"),
                v => panic!("expected Bob, got {:?}", v),
            }
        }
        v => panic!("expected list, got {:?}", v),
    }
}

#[test]
fn test_parsed_json_works_with_filter() {
    use std::io::Write;

    let path = "test_j_filter.json";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(
        b"[{\"name\": \"Alice\", \"active\": true}, {\"name\": \"Bob\", \"active\": false}]",
    )
    .expect("write file");

    let prog = format!(
        "let data = json_parse \"{}\" in filter (\\x x.active) data",
        path
    );
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::List(list) => {
            assert_eq!(list.len(), 1, "filter should keep only active users");
            if let Value::Dict(d) = &list[0] {
                match d.get("name").unwrap() {
                    Value::String(s) => assert_eq!(s, "Alice"),
                    v => panic!("expected Alice, got {:?}", v),
                }
            } else {
                panic!("expected dict");
            }
        }
        v => panic!("expected list, got {:?}", v),
    }
}

#[test]
fn test_parsed_json_works_with_fold() {
    use std::io::Write;

    let path = "test_j_fold.json";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"[10, 20, 30]").expect("write file");

    let prog = format!(
        "let data = json_parse \"{}\" in fold (\\acc \\x acc + x) 0 data",
        path
    );
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::Number(Number::Int(60)) => {}
        v => panic!("expected 60, got {:?}", v),
    }
}

#[test]
fn test_parsed_json_works_with_keys_values() {
    use std::io::Write;

    let path = "test_j_kv.json";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"{\"host\": \"localhost\", \"port\": 3000}")
        .expect("write file");

    // Test keys
    let prog_keys = format!("let data = json_parse \"{}\" in sort (keys data)", path);
    let result_keys = eval_prog(&prog_keys);

    match result_keys {
        Value::List(list) => {
            assert_eq!(list.len(), 2);
            // Sorted: host, port
            match &list[0] {
                Value::String(s) => assert_eq!(s, "host"),
                v => panic!("expected 'host', got {:?}", v),
            }
            match &list[1] {
                Value::String(s) => assert_eq!(s, "port"),
                v => panic!("expected 'port', got {:?}", v),
            }
        }
        v => panic!("expected list, got {:?}", v),
    }

    // Test values
    let prog_vals = format!("let data = json_parse \"{}\" in length (values data)", path);
    let result_vals = eval_prog(&prog_vals);
    std::fs::remove_file(path).expect("remove file");

    match result_vals {
        Value::Number(Number::Int(2)) => {}
        v => panic!("expected 2 values, got {:?}", v),
    }
}

#[test]
fn test_parsed_yaml_works_with_get_has_key() {
    use std::io::Write;

    let path = "test_y_get.yml";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"server:\n  host: localhost\n  port: 9090\n")
        .expect("write file");

    // Test get
    let prog_get = format!(
        "let data = yaml_parse \"{}\" in get data \"server\"",
        path
    );
    let result_get = eval_prog(&prog_get);
    match result_get {
        Value::Dict(d) => {
            match d.get("host").unwrap() {
                Value::String(s) => assert_eq!(s, "localhost"),
                v => panic!("expected 'localhost', got {:?}", v),
            }
        }
        v => panic!("expected dict from get, got {:?}", v),
    }

    // Test has_key
    let prog_has = format!(
        "let data = yaml_parse \"{}\" in has_key data \"server\"",
        path
    );
    let result_has = eval_prog(&prog_has);
    std::fs::remove_file(path).expect("remove file");

    match result_has {
        Value::Bool(true) => {}
        v => panic!("expected true, got {:?}", v),
    }
}

#[test]
fn test_parsed_toml_works_with_dot_access() {
    use std::io::Write;

    let path = "test_t_dot.toml";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"[package]\nname = \"avon\"\nversion = \"1.0\"\n")
        .expect("write file");

    // Dot notation access on parsed TOML
    let prog = format!(
        "let data = toml_parse \"{}\" in data.package.name",
        path
    );
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::String(s) => assert_eq!(s, "avon"),
        v => panic!("expected 'avon', got {:?}", v),
    }
}

#[test]
fn test_parsed_csv_works_with_filter_map() {
    use std::io::Write;

    let path = "test_c_fm.csv";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"name,department\nAlice,Engineering\nBob,Marketing\nCharlie,Engineering\n")
        .expect("write file");

    // Filter then map — tutorial claim: "Read CSV, filter rows, output as JSON"
    let prog = format!(
        "let data = csv_parse \"{}\" in \
         let eng = filter (\\x x.department == \"Engineering\") data in \
         map (\\x x.name) eng",
        path
    );
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::List(list) => {
            assert_eq!(list.len(), 2, "should have 2 engineering employees");
            match &list[0] {
                Value::String(s) => assert_eq!(s, "Alice"),
                v => panic!("expected 'Alice', got {:?}", v),
            }
            match &list[1] {
                Value::String(s) => assert_eq!(s, "Charlie"),
                v => panic!("expected 'Charlie', got {:?}", v),
            }
        }
        v => panic!("expected list, got {:?}", v),
    }
}

#[test]
fn test_parsed_xml_children_work_with_map() {
    use std::io::Write;

    let path = "test_x_map.xml";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(
        b"<people><person name=\"Alice\" age=\"30\"/><person name=\"Bob\" age=\"25\"/></people>",
    )
    .expect("write file");

    // Tutorial claim: map (\p p.attrs.name) doc.children
    let prog = format!(
        "let doc = xml_parse \"{}\" in map (\\p p.attrs.name) doc.children",
        path
    );
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::List(list) => {
            assert_eq!(list.len(), 2);
            match &list[0] {
                Value::String(s) => assert_eq!(s, "Alice"),
                v => panic!("expected 'Alice', got {:?}", v),
            }
            match &list[1] {
                Value::String(s) => assert_eq!(s, "Bob"),
                v => panic!("expected 'Bob', got {:?}", v),
            }
        }
        v => panic!("expected list, got {:?}", v),
    }
}

#[test]
fn test_parsed_opml_outlines_work_with_flatmap() {
    use std::io::Write;

    let path = "test_o_fm.opml";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(
        b"<?xml version=\"1.0\"?>\n\
          <opml version=\"2.0\">\n\
          <head><title>Test</title></head>\n\
          <body>\n\
            <outline text=\"Cat1\">\n\
              <outline text=\"Feed1\" xmlUrl=\"url1\"/>\n\
              <outline text=\"Feed2\" xmlUrl=\"url2\"/>\n\
            </outline>\n\
            <outline text=\"Cat2\">\n\
              <outline text=\"Feed3\" xmlUrl=\"url3\"/>\n\
            </outline>\n\
          </body>\n\
          </opml>",
    )
    .expect("write file");

    // Flatten all feeds from nested OPML structure
    let prog = format!(
        "let feeds = opml_parse \"{}\" in \
         flatmap (\\cat map (\\f f.text) cat.children) feeds.outlines",
        path
    );
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::List(list) => {
            assert_eq!(list.len(), 3, "should have 3 total feeds across categories");
            match &list[0] {
                Value::String(s) => assert_eq!(s, "Feed1"),
                v => panic!("expected 'Feed1', got {:?}", v),
            }
            match &list[2] {
                Value::String(s) => assert_eq!(s, "Feed3"),
                v => panic!("expected 'Feed3', got {:?}", v),
            }
        }
        v => panic!("expected list, got {:?}", v),
    }
}

// ── Transform + Convert (end-to-end pipeline tests) ──────────
// These verify the tutorial claim:
//   "Parse data, transform it, then output in any format"

#[test]
fn test_csv_filter_to_json() {
    use std::io::Write;

    let path = "test_c_f_j.csv";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"name,status\nAlice,active\nBob,inactive\nCharlie,active\n")
        .expect("write file");

    // Tutorial claim: "Read CSV, filter rows, output as JSON"
    let prog = format!(
        "let data = csv_parse \"{}\" in \
         let active = filter (\\u u.status == \"active\") data in \
         format_json active",
        path
    );
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::String(s) => {
            assert!(s.contains("\"Alice\""), "JSON should contain Alice");
            assert!(s.contains("\"Charlie\""), "JSON should contain Charlie");
            assert!(!s.contains("\"Bob\""), "JSON should NOT contain Bob (filtered out)");
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_xml_extract_to_csv() {
    use std::io::Write;

    let path = "test_x_e_c.xml";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(
        b"<people><person name=\"Alice\" age=\"30\"/><person name=\"Bob\" age=\"25\"/></people>",
    )
    .expect("write file");

    // Tutorial claim: "Read XML, extract data, output as CSV"
    let prog = format!(
        "let doc = xml_parse \"{}\" in \
         let rows = map (\\p {{name: p.attrs.name, age: p.attrs.age}}) doc.children in \
         format_csv rows",
        path
    );
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::String(s) => {
            assert!(
                s.contains("name") && s.contains("age"),
                "CSV should have headers from extracted data"
            );
            assert!(s.contains("Alice"), "CSV should contain Alice");
            assert!(s.contains("30"), "CSV should contain age 30");
            assert!(s.contains("Bob"), "CSV should contain Bob");
            assert!(s.contains("25"), "CSV should contain age 25");
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_opml_to_json_flat_feeds() {
    use std::io::Write;

    let path = "test_o2j.opml";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(
        b"<?xml version=\"1.0\"?>\n\
          <opml version=\"2.0\">\n\
          <head><title>Feeds</title></head>\n\
          <body>\n\
            <outline text=\"Tech\">\n\
              <outline text=\"HN\" type=\"rss\" xmlUrl=\"https://hn.com/rss\"/>\n\
            </outline>\n\
            <outline text=\"Science\">\n\
              <outline text=\"Nature\" type=\"rss\" xmlUrl=\"https://nature.com/rss\"/>\n\
            </outline>\n\
          </body>\n\
          </opml>",
    )
    .expect("write file");

    // Flatten OPML structure and convert to JSON
    let prog = format!(
        "let feeds = opml_parse \"{}\" in \
         let flat = flatmap (\\cat \
           map (\\f {{category: cat.text, name: f.text, url: f.xmlUrl}}) cat.children \
         ) feeds.outlines in \
         format_json flat",
        path
    );
    let result = eval_prog(&prog);
    std::fs::remove_file(path).expect("remove file");

    match result {
        Value::String(s) => {
            assert!(s.starts_with('['), "should be JSON array");
            assert!(s.contains("\"Tech\""), "should have Tech category");
            assert!(s.contains("\"HN\""), "should have HN name");
            assert!(s.contains("\"Science\""), "should have Science category");
            assert!(s.contains("\"Nature\""), "should have Nature name");
            assert!(
                s.contains("https://hn.com/rss"),
                "should have HN feed URL"
            );
        }
        v => panic!("expected string, got {:?}", v),
    }
}

// ── Verify all parsers work with file paths ──────────────────
// Parsers accept both file paths and raw strings (dual-mode)

#[test]
fn test_all_parsers_require_file_paths() {
    use std::io::Write;

    // Create temp files for all 8 formats
    let files: &[(&str, &[u8], &str)] = &[
        (
            "test_all_p.json",
            b"{\"ok\": true}",
            "json_parse",
        ),
        ("test_all_p.yml", b"ok: true", "yaml_parse"),
        ("test_all_p.toml", b"ok = true", "toml_parse"),
        ("test_all_p.csv", b"key,val\nok,true", "csv_parse"),
        (
            "test_all_p.xml",
            b"<root ok=\"true\"/>",
            "xml_parse",
        ),
        (
            "test_all_p.html",
            b"<html><body><p>ok</p></body></html>",
            "html_parse",
        ),
        (
            "test_all_p.opml",
            b"<?xml version=\"1.0\"?><opml version=\"2.0\"><head><title>T</title></head><body><outline text=\"ok\"/></body></opml>",
            "opml_parse",
        ),
        (
            "test_all_p.ini",
            b"[section]\nok=true\n",
            "ini_parse",
        ),
    ];

    for (path, content, parser) in files {
        let mut file = std::fs::File::create(path).expect("create file");
        file.write_all(content).expect("write file");

        let prog = format!("{} \"{}\"", parser, path);
        let result = eval_prog(&prog);

        // All parsers should return some non-error value
        match result {
            Value::Dict(_) | Value::List(_) => {} // expected
            v => panic!("{} returned unexpected type: {:?}", parser, v),
        }
    }

    // Clean up
    for (path, _, _) in files {
        std::fs::remove_file(path).expect("remove file");
    }
}

// ── Verify all *_parse_string functions parse raw strings ────

#[test]
fn test_json_parse_string() {
    let prog = r#"json_parse_string "{\"name\": \"Alice\", \"age\": 30}""#;
    match eval_prog(prog) {
        Value::Dict(d) => {
            match d.get("name").unwrap() {
                Value::String(s) => assert_eq!(s, "Alice"),
                v => panic!("expected name string, got {:?}", v),
            }
            match d.get("age").unwrap() {
                Value::Number(Number::Int(n)) => assert_eq!(*n, 30),
                v => panic!("expected age int, got {:?}", v),
            }
        }
        v => panic!("expected dict, got {:?}", v),
    }
}

#[test]
fn test_yaml_parse_string() {
    let prog = r#"yaml_parse_string "name: Alice\nage: 30""#;
    match eval_prog(prog) {
        Value::Dict(d) => {
            match d.get("name").unwrap() {
                Value::String(s) => assert_eq!(s, "Alice"),
                v => panic!("expected name string, got {:?}", v),
            }
            match d.get("age").unwrap() {
                Value::Number(Number::Int(n)) => assert_eq!(*n, 30),
                v => panic!("expected age int, got {:?}", v),
            }
        }
        v => panic!("expected dict, got {:?}", v),
    }
}

#[test]
fn test_toml_parse_string() {
    let prog = r#"toml_parse_string "[server]\nhost = \"localhost\"\nport = 8080""#;
    match eval_prog(prog) {
        Value::Dict(d) => {
            if let Value::Dict(srv) = d.get("server").unwrap() {
                match srv.get("host").unwrap() {
                    Value::String(s) => assert_eq!(s, "localhost"),
                    v => panic!("expected host string, got {:?}", v),
                }
                match srv.get("port").unwrap() {
                    Value::Number(Number::Int(n)) => assert_eq!(*n, 8080),
                    v => panic!("expected port int, got {:?}", v),
                }
            } else {
                panic!("expected dict for server section");
            }
        }
        v => panic!("expected dict, got {:?}", v),
    }
}

#[test]
fn test_csv_parse_string() {
    let prog = r#"csv_parse_string "name,score\nAlice,95\nBob,87""#;
    match eval_prog(prog) {
        Value::List(rows) => {
            assert_eq!(rows.len(), 2, "should have 2 data rows");
            if let Value::Dict(row) = &rows[0] {
                match row.get("name").unwrap() {
                    Value::String(s) => assert_eq!(s, "Alice"),
                    v => panic!("expected name string, got {:?}", v),
                }
                match row.get("score").unwrap() {
                    Value::String(s) => assert_eq!(s, "95"),
                    v => panic!("expected score string, got {:?}", v),
                }
            } else {
                panic!("expected dict row");
            }
        }
        v => panic!("expected list, got {:?}", v),
    }
}

#[test]
fn test_xml_parse_string() {
    let prog = r#"xml_parse_string "<root><item name=\"test\">Hello</item></root>""#;
    match eval_prog(prog) {
        Value::Dict(d) => {
            match d.get("tag").unwrap() {
                Value::String(s) => assert_eq!(s, "root"),
                v => panic!("expected tag string, got {:?}", v),
            }
            match d.get("children").unwrap() {
                Value::List(children) => {
                    assert!(!children.is_empty(), "should have children");
                }
                v => panic!("expected children list, got {:?}", v),
            }
        }
        v => panic!("expected dict, got {:?}", v),
    }
}

#[test]
fn test_html_parse_string() {
    let prog = r#"html_parse_string "<div class=\"box\"><p>Hello</p></div>""#;
    match eval_prog(prog) {
        Value::Dict(d) => {
            match d.get("tag").unwrap() {
                Value::String(s) => assert_eq!(s, "html"),
                v => panic!("expected tag string, got {:?}", v),
            }
        }
        v => panic!("expected dict, got {:?}", v),
    }
}

#[test]
fn test_ini_parse_string() {
    let prog = r#"ini_parse_string "[database]\nhost=localhost\nport=3306""#;
    match eval_prog(prog) {
        Value::Dict(d) => {
            if let Value::Dict(db) = d.get("database").unwrap() {
                match db.get("host").unwrap() {
                    Value::String(s) => assert_eq!(s, "localhost"),
                    v => panic!("expected host string, got {:?}", v),
                }
                match db.get("port").unwrap() {
                    Value::String(s) => assert_eq!(s, "3306"),
                    v => panic!("expected port string, got {:?}", v),
                }
            } else {
                panic!("expected dict for database section");
            }
        }
        v => panic!("expected dict, got {:?}", v),
    }
}

#[test]
fn test_opml_parse_string() {
    let prog = r#"opml_parse_string "<?xml version=\"1.0\"?><opml version=\"2.0\"><head><title>Test</title></head><body><outline text=\"Feed\"/></body></opml>""#;
    match eval_prog(prog) {
        Value::Dict(d) => {
            match d.get("version").unwrap() {
                Value::String(s) => assert_eq!(s, "2.0"),
                v => panic!("expected version string, got {:?}", v),
            }
            match d.get("outlines").unwrap() {
                Value::List(outlines) => {
                    assert_eq!(outlines.len(), 1, "should have 1 outline");
                }
                v => panic!("expected outlines list, got {:?}", v),
            }
        }
        v => panic!("expected dict, got {:?}", v),
    }
}

#[test]
fn test_string_parse_roundtrip() {
    // Parse JSON string → format_json → should work end-to-end
    let prog = r#"
        let data = json_parse_string "{\"x\": 1, \"y\": 2}" in
        format_json data
    "#;
    match eval_prog(prog) {
        Value::String(s) => {
            assert!(s.contains("\"x\""), "roundtrip should contain x key");
            assert!(s.contains("\"y\""), "roundtrip should contain y key");
        }
        v => panic!("expected string, got {:?}", v),
    }
}

#[test]
fn test_string_parse_cross_format() {
    // Parse JSON string → format as YAML
    let prog = r#"
        let data = json_parse_string "{\"name\": \"Alice\"}" in
        format_yaml data
    "#;
    match eval_prog(prog) {
        Value::String(s) => {
            assert!(s.contains("name"), "YAML should contain name");
            assert!(s.contains("Alice"), "YAML should contain Alice");
        }
        v => panic!("expected string, got {:?}", v),
    }
}

// ── Verify typeof on parsed data returns expected types ──────

#[test]
fn test_parsed_values_have_correct_types() {
    use std::io::Write;

    let path = "test_typeof.json";
    let mut file = std::fs::File::create(path).expect("create file");
    file.write_all(b"{\"str\": \"hello\", \"num\": 42, \"flag\": true, \"list\": [1,2]}")
        .expect("write file");

    // Verify typeof works on parsed data
    let prog = format!(
        "let d = json_parse \"{}\" in typeof d",
        path
    );
    match eval_prog(&prog) {
        Value::String(s) => assert_eq!(s, "Dict", "parsed JSON object should be Dict"),
        v => panic!("expected 'Dict', got {:?}", v),
    }

    let prog = format!(
        "let d = json_parse \"{}\" in typeof d.str",
        path
    );
    match eval_prog(&prog) {
        Value::String(s) => assert_eq!(s, "String", "parsed JSON string should be String"),
        v => panic!("expected 'String', got {:?}", v),
    }

    let prog = format!(
        "let d = json_parse \"{}\" in typeof d.num",
        path
    );
    match eval_prog(&prog) {
        Value::String(s) => assert_eq!(s, "Number", "parsed JSON number should be Number"),
        v => panic!("expected 'Number', got {:?}", v),
    }

    let prog = format!(
        "let d = json_parse \"{}\" in typeof d.flag",
        path
    );
    match eval_prog(&prog) {
        Value::String(s) => assert_eq!(s, "Bool", "parsed JSON bool should be Bool"),
        v => panic!("expected 'Bool', got {:?}", v),
    }

    let prog = format!(
        "let d = json_parse \"{}\" in typeof d.list",
        path
    );
    match eval_prog(&prog) {
        Value::String(s) => assert_eq!(s, "List", "parsed JSON array should be List"),
        v => panic!("expected 'List', got {:?}", v),
    }

    std::fs::remove_file(path).expect("remove file");
}