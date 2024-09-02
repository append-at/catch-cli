use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct CatchConnectCLIResponse {
    pub public_key: String,
    pub integration_id: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct CatchSessionAnalyzingModuleStructureResult {
    pub status: String,
    pub structure: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct CatchSessionExtractingCandidatesResult {
    pub status: String,
    pub candidates: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct CatchPlatformInfo {
    pub platform: String,
    pub architecture_description: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct CatchSessionAnalyzePlatformResult {
    pub status: String,
    pub platform_info: CatchPlatformInfo,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct CatchDiffFile {
    pub file_path: String,
    pub patch_content: String,
    pub modified_content: String,
    pub original_content: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct CatchSessionGeneratingDiffResult {
    pub files: Vec<CatchDiffFile>,
    pub status: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct CatchSessionResult {
    pub step: String,
    pub status: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
struct CatchSessionOutput {
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
struct CatchSessionProcessInfo {
    pub id: String,
    pub status: String,
    pub output: CatchSessionOutput,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CatchSessionStatusResponse {
    pub process: CatchSessionProcessInfo,
}
