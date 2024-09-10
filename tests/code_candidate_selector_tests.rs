use catch_cli::code_candidate_selector::filter_code_files;
use catch_cli::code_reader::CatchCLICodeFile;

#[test]
fn test_filter_code_files() {
    let files = vec![
        CatchCLICodeFile {
            path: String::from("/path/to/file1.rs"),
            encrypted_file_content: String::from("content1"),
        },
        CatchCLICodeFile {
            path: String::from("/path/to/file2.rs"),
            encrypted_file_content: String::from("content2"),
        },
        CatchCLICodeFile {
            path: String::from("/path/to/file3.rs"),
            encrypted_file_content: String::from("content3"),
        },
    ];

    // case 1: all file select
    let paths1 = vec![
        String::from("/path/to/file1.rs"),
        String::from("/path/to/file2.rs"),
        String::from("/path/to/file3.rs"),
    ];
    let result1 = filter_code_files(files.clone(), paths1);
    assert_eq!(result1.len(), 3);

    // case 2: select only some files
    let paths2 = vec![
        String::from("/path/to/file1.rs"),
        String::from("/path/to/file3.rs"),
    ];
    let result2 = filter_code_files(files.clone(), paths2);
    assert_eq!(result2.len(), 2);
    assert!(result2.iter().any(|f| f.path == "/path/to/file1.rs"));
    assert!(result2.iter().any(|f| f.path == "/path/to/file3.rs"));

    // case 3: included path not found
    let paths3 = vec![
        String::from("/path/to/file2.rs"),
        String::from("/path/to/nonexistent.rs"),
    ];
    let result3 = filter_code_files(files.clone(), paths3);
    assert_eq!(result3.len(), 1);
    assert_eq!(result3[0].path, "/path/to/file2.rs");

    // test 4: empty path list
    let paths4: Vec<String> = vec![];
    let result4 = filter_code_files(files.clone(), paths4);
    assert_eq!(result4.len(), 0);

    // case 5: empty file list
    let empty_files: Vec<CatchCLICodeFile> = vec![];
    let paths5 = vec![String::from("/path/to/file1.rs")];
    let result5 = filter_code_files(empty_files, paths5);
    assert_eq!(result5.len(), 0);
}
