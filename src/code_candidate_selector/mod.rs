use crate::code_candidate_selector::ui::CodeCandidateSelector;
use crate::code_reader::CatchCLICodeFile;
use std::io;

mod ui;

pub fn select_codes(code_files: Vec<CatchCLICodeFile>) -> io::Result<Vec<CatchCLICodeFile>> {
    let code_files_clone = code_files.clone();
    let selector = CodeCandidateSelector::new(code_files_clone);

    let terminal = ratatui::init();

    match selector.run(terminal) {
        Ok(selected_paths) => {
            let filtered_code_files = filter_code_files(code_files, selected_paths);
            Ok(filtered_code_files)
        }
        Err(e) => Err(e),
    }
}

pub fn filter_code_files(
    all_files: Vec<CatchCLICodeFile>,
    paths: Vec<String>,
) -> Vec<CatchCLICodeFile> {
    let path_set: std::collections::HashSet<String> = paths.into_iter().collect();

    all_files
        .into_iter()
        .filter(|file| path_set.contains(&file.path))
        .collect()
}
