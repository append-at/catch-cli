use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CatchSessionAnalyzingModuleStructureResult {
    pub status: String,
    pub structure: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CatchSessionExtractingCandidatesResult {
    pub status: String,
    pub candidates: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CatchPlatformInfo {
    pub platform: String,
    pub architecture_description: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CatchSessionAnalyzePlatformResult {
    pub status: String,
    pub platform_info: CatchPlatformInfo,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CatchDiffFile {
    pub file_path: String,
    pub patch_content: String,
    pub modified_content: String,
    pub original_content: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CatchSessionGeneratingDiffResult {
    pub files: Vec<CatchDiffFile>,
    pub status: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CatchSessionResult {
    pub step: String,
    pub status: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct CatchSessionOutput {
    pub docs: Vec<String>,
    pub fetching_code: CatchSessionResult,
    pub indexing_code: CatchSessionResult,
    pub generating_diff: CatchSessionGeneratingDiffResult,
    pub generating_docs: CatchSessionResult,
    pub analyzing_platform: CatchSessionAnalyzePlatformResult,
    pub extracting_candidates: CatchSessionExtractingCandidatesResult,
    pub analyzing_module_structure: CatchSessionAnalyzingModuleStructureResult,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CatchSessionProcessInfo {
    pub id: Option<String>,
    pub status: Option<String>,
    pub output: Option<CatchSessionOutput>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CatchSessionStatusResponse {
    pub process: CatchSessionProcessInfo,
}
