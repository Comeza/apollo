use std::fmt::Display;

use reqwest::Url;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

pub use serde_json;
pub use serde;
pub use reqwest;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub remember_me: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Token {
    id_token: String,
}

pub struct Hermes {
    user_agent: &'static str,
    pub token: Token,
    pub base_url: Url
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoRes {
    pub build: BuildInfo,
    pub java: JavaInfo,
    pub programming_language_features: Vec<LanguageFeature>,
    pub features: Vec<String>,
    pub account_name: String,
    pub version_control_url: String,
    #[serde(rename = "commitHashURLTemplate")]
    pub commit_hash_url_template: String,
    #[serde(rename = "sshCloneURLTemplate")]
    pub ssh_clone_url_template: String,
    #[serde(rename = "buildPlanURLTemplate")]
    pub build_plan_url_template: String,
    pub active_profiles: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildInfo {
    pub artifact: String,
    pub name: String,
    pub time: DateTime<Utc>,
    pub version: String,
    pub group: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JavaInfo {
    pub vendor: String,
    pub version: String,
    pub runtime: RuntimeInfo,
    pub jvm: JVMInfo
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeInfo {
    pub name: String,
    pub version: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JVMInfo {
    pub name: String,
    pub vendor: String,
    pub version: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageFeature {
    pub programming_language: String,
    pub sequential_test_runs: bool,
    pub static_code_analysis: bool,
    pub plagiarism_check_supported: bool,
    pub package_name_required: bool,
    pub checkout_solution_repository_allowed: bool,
    pub project_types: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ForDashboard(Vec<Project>);

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: u32,
    pub title: String,
    pub short_name: String,
    pub student_group_name: String,
    pub teaching_assistant_group_name: Option<String>,
    pub editor_group_name: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub semester: String,
    pub test_course: bool,
    pub online_course: bool,
    pub max_complaints: u32,
    pub max_team_complaints: u32,
    pub max_complaint_time_days: u32,
    pub max_complaint_text_limit: u32,
    pub max_complaint_response_text_limit: u32,
    pub posts_enabled: bool,
    pub max_request_more_feedback_time_days: u32,
    pub color: String,
    pub registration_enabled: bool,
    pub registration_confirmation_message: String,
    pub presentation_score: f32,
    pub accuracy_of_scores: f32,
    pub exercises: Vec<Exercise>,
    pub complaints_enabled: bool,
    pub valid_start_and_end_date: bool,
    pub request_more_feedback_enabled: bool
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exercise {
    pub r#type: String,
    pub id: u32,
    pub title: String,
    pub short_name: String,
    pub max_points: f32,
    pub bonus_points: f32,
    pub assessment_type: String,
    pub release_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub difficulty: Option<String>, // maybe turn into an enum
    pub mode: String,
    pub allow_complaints_for_automatic_assessments: bool,
    pub included_in_overall_score: String,
    pub presentation_score_enabled: bool,
    pub second_correction_enabled: bool,
    #[serde(default)]
    pub student_participations: Vec<StudentParticipation>,
    pub publish_build_plan_url: bool,
    pub allow_online_editor: bool,
    pub allow_offline_ide: bool,
    pub static_code_analysis_enabled: bool,
    pub programming_language: String,
    pub package_name: String,
    pub sequential_test_runs: bool,
    pub show_test_names_to_students: bool,
    pub test_cases_changed: bool,
    pub project_key: String,
    pub project_type: String,
    pub testwise_coverage_enabled: bool,
    pub branch: String,
    pub checkout_solution_repository: bool,
    pub exercise_type: String,
    pub is_local_simulation: bool,
    pub released: bool,
    pub student_assigned_team_id_computed: bool,
    pub grading_instruction_feedback_used: bool,
    pub example_solution_published: bool,
    pub visible_to_students: bool,
    pub team_mode: bool
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudentParticipation {
    pub r#type: String,
    pub id: u32,
    pub initialization_state: String,
    pub initialization_date: DateTime<Utc>,
    pub test_run: bool,
    pub results: Vec<ParitcipationResult>,
    pub submissions: Vec<Submission>,
    pub student: Student,
    pub repository_url: String,
    pub branch: Option<String>,
    pub user_independent_repository_url: String,
    pub participant_identifier: String,
    pub participant_name: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParitcipationResult {
    pub id: u32,
    pub result_string: String,
    pub completion_date: DateTime<Utc>,
    pub successful: bool,
    pub score: f32,
    pub rated: bool,
    pub has_feedback: bool,
    pub submission: Submission,
    pub assessment_type: String,
    pub has_complaint: Option<bool> // A bit weird
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Submission {
    pub id: u32,
    pub submission_exercise_type: String,
    pub submitted: bool,
    pub r#type: String,
    pub submission_date: DateTime<Utc>,
    pub commit_hash: String,
    pub build_failed: bool,
    pub build_artifact: bool,
    pub empty: bool,
    pub duration_in_minutes: u32 // Maybe change to chrono::Duration or std::Duration?
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Student {
    pub id: u32,
    pub login: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub activated: bool,
    pub lang_key: String,
    pub last_notification_read: DateTime<Utc>,
    pub name: String,
    pub internal: bool,
    pub participant_identifier: String,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id_token)
    }
}

impl Hermes {
    const DEFAULT_UA: &'static str = concat!("hermes/", env!("CARGO_PKG_VERSION"));

    pub fn new<U: Into<Url>>(base_url: U) -> Self {
        Self { base_url: base_url.into(), token: String::default().into(), user_agent: Self::DEFAULT_UA }
    }

    pub fn token<T: Into<Token>>(self, token: T) -> Self {
        Self { token: token.into() , ..self }
    }

    pub fn set_token<'a, T: Into<&'a Token>>(&mut self, token: T) {
        self.token = token.into().clone();
    }

    pub fn api(&self, path: &str) -> Url {
        let mut url = self.base_url.clone();
        url.set_path(path);
        url
    }

    pub fn with_agent(self, user_agent: &'static str) -> Self {
        Self { user_agent, ..self }
    }

    pub async fn login(&mut self, login: &Credentials) -> reqwest::Result<()> {
        self.token = reqwest::Client::new()
            .post(self.api("api/authenticate"))
            .header(reqwest::header::USER_AGENT, self.user_agent)
            .json(login).send().await?.json().await?; // TODO: Proper error handling
        Ok(())
    }

    pub async fn get_info(&self) -> reqwest::Result<InfoRes> {
        match reqwest::Client::new()
            .get(self.api("management/info"))
            .bearer_auth(&self.token.id_token)
            .send().await {
                Ok(res) => Ok(res.json().await?),
                Err(err) => Err(err)
            }
    }

    pub async fn get_for_dashboard(&self) -> reqwest::Result<ForDashboard> {
        match reqwest::Client::new()
            .get(self.api("api/courses/for-dashboard"))
            .bearer_auth(&self.token.id_token)
            .send().await {
                Ok(res) => Ok(res.json().await?),
                Err(err) => Err(err)
            }
    }
}

impl<T: Into<String>> From<T> for Token {
    fn from(t: T) -> Self {
        Self { id_token: t.into() }
    }
}

impl std::ops::Deref for ForDashboard {
    type Target = Vec<Project>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Deref for Token {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.id_token
    }
}
