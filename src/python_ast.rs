use rustpython_parser::{ast, Parse};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::string::String;

const EXCLUDED_DIRS: [&str; 4] = ["venv", ".pytest_cache", ".ruff_cache", ".venv"];

/// Recursively walks the path provided, parses all .py files and
/// returns a hashmap of all non-std lib imports
pub fn get_imports_from_src(directory_path: &Path) -> HashMap<String, bool> {
    let ext = "py";

    return find_files_with_extension(directory_path, ext);
}

fn find_files_with_extension(dir: &Path, extension: &str) -> HashMap<String, bool> {
    let mut result: HashMap<String, bool> = HashMap::new();

    if dir.is_dir() {
        for entry in std::fs::read_dir(dir).expect("Failed to read directory") {
            if let Ok(entry) = entry {
                let path = entry.path();
                let file_name = path.file_name().unwrap().to_string_lossy().to_string();

                // Check if the directory should be excluded
                if path.is_dir() && EXCLUDED_DIRS.contains(&file_name.as_str()) {
                    continue;
                }

                if path.is_file() {
                    if let Some(file_extension) = path.extension() {
                        if file_extension == extension {
                            let new_imports = get_imports_from_python_module(&path);
                            result.extend(new_imports);
                        }
                    }
                } else if path.is_dir() {
                    result.extend(find_files_with_extension(&path, extension));
                }
            }
        }
    }

    result
}

/// Reads the Python modules and the import statements, filters out standard lib modules and
/// returns the imports
fn get_imports_from_python_module(module_path: &PathBuf) -> HashMap<String, bool> {
    let mut imports: HashMap<String, bool> = HashMap::new();
    let module_str = module_path.to_str().unwrap();
    let python_source = fs::read_to_string(module_path).expect("Failed to read Python file");

    let python_statements = ast::Suite::parse(&python_source, module_str).unwrap();

    for statement in python_statements.iter() {
        match statement {
            ast::Stmt::Import(import_stmt) => {
                import_stmt.names.iter().for_each(|name| {
                    if !is_std_lib_module(name.name.as_str()) {
                        imports.insert(name.name.as_str().to_string(), true);
                    }
                });
            }
            ast::Stmt::ImportFrom(import_from_stmt) => {
                import_from_stmt.module.as_ref().map(|module| {
                    if !is_std_lib_module(&module.as_str()) {
                        let module_name = &module.as_str().split('.').next().unwrap().to_string();
                        imports.insert(module_name.to_string(), true);
                    }
                });
            }
            // no other use cases
            _ => {}
        }
    }
    return imports;
}

/// Checks if a module is part of the Python standard library.
fn is_std_lib_module(module_name: &str) -> bool {
    let std_lib_modules = python_std_lib();
    return std_lib_modules.contains_key(module_name);
}

/// Returns a hashmap of all the Python standard library modules.
fn python_std_lib() -> HashMap<&'static str, bool> {
    let mut std_lib_modules = HashMap::new();
    std_lib_modules.insert("abc", true);
    std_lib_modules.insert("aifc", true);
    std_lib_modules.insert("antigravity", true);
    std_lib_modules.insert("argparse", true);
    std_lib_modules.insert("array", true);
    std_lib_modules.insert("ast", true);
    std_lib_modules.insert("asynchat", true);
    std_lib_modules.insert("asyncio", true);
    std_lib_modules.insert("asyncore", true);
    std_lib_modules.insert("atexit", true);
    std_lib_modules.insert("audioop", true);
    std_lib_modules.insert("base64", true);
    std_lib_modules.insert("bdb", true);
    std_lib_modules.insert("binascii", true);
    std_lib_modules.insert("binhex", true);
    std_lib_modules.insert("bisect", true);
    std_lib_modules.insert("builtins", true);
    std_lib_modules.insert("bz2", true);
    std_lib_modules.insert("calendar", true);
    std_lib_modules.insert("cgi", true);
    std_lib_modules.insert("cgitb", true);
    std_lib_modules.insert("chunk", true);
    std_lib_modules.insert("cmath", true);
    std_lib_modules.insert("cmd", true);
    std_lib_modules.insert("code", true);
    std_lib_modules.insert("codecs", true);
    std_lib_modules.insert("codeop", true);
    std_lib_modules.insert("collections", true);
    std_lib_modules.insert("colorsys", true);
    std_lib_modules.insert("compileall", true);
    std_lib_modules.insert("concurrent", true);
    std_lib_modules.insert("configparser", true);
    std_lib_modules.insert("contextlib", true);
    std_lib_modules.insert("copy", true);
    std_lib_modules.insert("copyreg", true);
    std_lib_modules.insert("cProfile", true);
    std_lib_modules.insert("crypt", true);
    std_lib_modules.insert("csv", true);
    std_lib_modules.insert("ctypes", true);
    std_lib_modules.insert("curses", true);
    std_lib_modules.insert("dataclasses", true);
    std_lib_modules.insert("datetime", true);
    std_lib_modules.insert("dbm", true);
    std_lib_modules.insert("decimal", true);
    std_lib_modules.insert("difflib", true);
    std_lib_modules.insert("dis", true);
    std_lib_modules.insert("distutils", true);
    std_lib_modules.insert("doctest", true);
    std_lib_modules.insert("dummy_threading", true);
    std_lib_modules.insert("email", true);
    std_lib_modules.insert("encodings", true);
    std_lib_modules.insert("ensurepip", true);
    std_lib_modules.insert("enum", true);
    std_lib_modules.insert("errno", true);
    std_lib_modules.insert("faulthandler", true);
    std_lib_modules.insert("fcntl", true);
    std_lib_modules.insert("filecmp", true);
    std_lib_modules.insert("fileinput", true);
    std_lib_modules.insert("fnmatch", true);
    std_lib_modules.insert("formatter", true);
    std_lib_modules.insert("fractions", true);
    std_lib_modules.insert("ftplib", true);
    std_lib_modules.insert("functools", true);
    std_lib_modules.insert("gc", true);
    std_lib_modules.insert("getopt", true);
    std_lib_modules.insert("getpass", true);
    std_lib_modules.insert("gettext", true);
    std_lib_modules.insert("glob", true);
    std_lib_modules.insert("grp", true);
    std_lib_modules.insert("gzip", true);
    std_lib_modules.insert("hashlib", true);
    std_lib_modules.insert("heapq", true);
    std_lib_modules.insert("hmac", true);
    std_lib_modules.insert("html", true);
    std_lib_modules.insert("http", true);
    std_lib_modules.insert("imaplib", true);
    std_lib_modules.insert("imghdr", true);
    std_lib_modules.insert("imp", true);
    std_lib_modules.insert("importlib", true);
    std_lib_modules.insert("inspect", true);
    std_lib_modules.insert("io", true);
    std_lib_modules.insert("ipaddress", true);
    std_lib_modules.insert("itertools", true);
    std_lib_modules.insert("json", true);
    std_lib_modules.insert("keyword", true);
    std_lib_modules.insert("lib2to3", true);
    std_lib_modules.insert("linecache", true);
    std_lib_modules.insert("locale", true);
    std_lib_modules.insert("logging", true);
    std_lib_modules.insert("lzma", true);
    std_lib_modules.insert("macpath", true);
    std_lib_modules.insert("mailbox", true);
    std_lib_modules.insert("mailcap", true);
    std_lib_modules.insert("marshal", true);
    std_lib_modules.insert("math", true);
    std_lib_modules.insert("mimetypes", true);
    std_lib_modules.insert("mmap", true);
    std_lib_modules.insert("modulefinder", true);
    std_lib_modules.insert("msilib", true);
    std_lib_modules.insert("msvcrt", true);
    std_lib_modules.insert("multiprocessing", true);
    std_lib_modules.insert("netrc", true);
    std_lib_modules.insert("nis", true);
    std_lib_modules.insert("nntplib", true);
    std_lib_modules.insert("numbers", true);
    std_lib_modules.insert("operator", true);
    std_lib_modules.insert("optparse", true);
    std_lib_modules.insert("os", true);
    std_lib_modules.insert("ossaudiodev", true);
    std_lib_modules.insert("parser", true);
    std_lib_modules.insert("pathlib", true);
    std_lib_modules.insert("pdb", true);
    std_lib_modules.insert("pickle", true);
    std_lib_modules.insert("pickletools", true);
    std_lib_modules.insert("pipes", true);
    std_lib_modules.insert("pkgutil", true);
    std_lib_modules.insert("platform", true);
    std_lib_modules.insert("plistlib", true);
    std_lib_modules.insert("poplib", true);
    std_lib_modules.insert("posix", true);
    std_lib_modules.insert("pprint", true);
    std_lib_modules.insert("profile", true);
    std_lib_modules.insert("pstats", true);
    std_lib_modules.insert("pty", true);
    std_lib_modules.insert("pwd", true);
    std_lib_modules.insert("py_compile", true);
    std_lib_modules.insert("pyclbr", true);
    std_lib_modules.insert("pydoc", true);
    std_lib_modules.insert("queue", true);
    std_lib_modules.insert("quopri", true);
    std_lib_modules.insert("random", true);
    std_lib_modules.insert("re", true);
    std_lib_modules.insert("readline", true);
    std_lib_modules.insert("reprlib", true);
    std_lib_modules.insert("resource", true);
    std_lib_modules.insert("rlcompleter", true);
    std_lib_modules.insert("runpy", true);
    std_lib_modules.insert("sched", true);
    std_lib_modules.insert("secrets", true);
    std_lib_modules.insert("select", true);
    std_lib_modules.insert("selectors", true);
    std_lib_modules.insert("shelve", true);
    std_lib_modules.insert("shlex", true);
    std_lib_modules.insert("shutil", true);
    std_lib_modules.insert("signal", true);
    std_lib_modules.insert("site", true);
    std_lib_modules.insert("smtpd", true);
    std_lib_modules.insert("smtplib", true);
    std_lib_modules.insert("sndhdr", true);
    std_lib_modules.insert("socket", true);
    std_lib_modules.insert("socketserver", true);
    std_lib_modules.insert("spwd", true);
    std_lib_modules.insert("sqlite3", true);
    std_lib_modules.insert("ssl", true);
    std_lib_modules.insert("stat", true);
    std_lib_modules.insert("statistics", true);
    std_lib_modules.insert("string", true);
    std_lib_modules.insert("stringprep", true);
    std_lib_modules.insert("struct", true);
    std_lib_modules.insert("subprocess", true);
    std_lib_modules.insert("sunau", true);
    std_lib_modules.insert("symbol", true);
    std_lib_modules.insert("symtable", true);
    std_lib_modules.insert("sys", true);
    std_lib_modules.insert("sysconfig", true);
    std_lib_modules.insert("syslog", true);
    std_lib_modules.insert("tabnanny", true);
    std_lib_modules.insert("tarfile", true);
    std_lib_modules.insert("telnetlib", true);
    std_lib_modules.insert("tempfile", true);
    std_lib_modules.insert("termios", true);
    std_lib_modules.insert("test", true);
    std_lib_modules.insert("textwrap", true);
    std_lib_modules.insert("this", true);
    std_lib_modules.insert("threading", true);
    std_lib_modules.insert("time", true);
    std_lib_modules.insert("timeit", true);
    std_lib_modules.insert("tkinter", true);
    std_lib_modules.insert("token", true);
    std_lib_modules.insert("tokenize", true);
    std_lib_modules.insert("trace", true);
    std_lib_modules.insert("traceback", true);
    std_lib_modules.insert("tracemalloc", true);
    std_lib_modules.insert("tty", true);
    std_lib_modules.insert("turtle", true);
    std_lib_modules.insert("turtledemo", true);
    std_lib_modules.insert("types", true);
    std_lib_modules.insert("typing", true);
    std_lib_modules.insert("unicodedata", true);
    std_lib_modules.insert("unittest", true);
    std_lib_modules.insert("urllib", true);
    std_lib_modules.insert("uu", true);
    std_lib_modules.insert("uuid", true);
    std_lib_modules.insert("venv", true);
    std_lib_modules.insert("warnings", true);
    std_lib_modules.insert("wave", true);
    std_lib_modules.insert("weakref", true);
    std_lib_modules.insert("webbrowser", true);
    std_lib_modules.insert("winreg", true);
    std_lib_modules.insert("winsound", true);
    std_lib_modules.insert("wsgiref", true);
    std_lib_modules.insert("xdrlib", true);
    std_lib_modules.insert("xml", true);
    std_lib_modules.insert("xmlrpc", true);
    std_lib_modules.insert("zipapp", true);
    std_lib_modules.insert("zipfile", true);
    std_lib_modules.insert("zipimport", true);
    std_lib_modules.insert("zlib", true);
    std_lib_modules.insert("zoneinfo", true);
    // also adding these modules even though not in the std lib
    std_lib_modules.insert("setuptools", true);
    std_lib_modules.insert("wheel", true);

    return std_lib_modules;
}

#[test]
fn test_is_std_lib_module() {
    let std_lib_modules = python_std_lib();
    assert_eq!(std_lib_modules.contains_key("abc"), true);
    assert_eq!(std_lib_modules.contains_key("setuptools"), true);
    assert_eq!(std_lib_modules.contains_key("wheel"), true);
    assert_eq!(std_lib_modules.contains_key("requests"), false);
}
