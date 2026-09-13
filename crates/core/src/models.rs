use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub size: u64,
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default)]
    pub lines: u32,
    #[serde(default)]
    pub preview: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub is_config: bool,
    #[serde(default)]
    pub is_entrypoint: bool,
}

fn default_language() -> String {
    "unknown".into()
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScanReport {
    pub candidates: u32,
    pub kept: u32,
    #[serde(default)]
    pub oversized: Vec<String>,
    #[serde(default)]
    pub oversized_count: u32,
    #[serde(default)]
    pub binary_count: u32,
    #[serde(default)]
    pub minified_count: u32,
    #[serde(default)]
    pub priority_dropped: u32,
    #[serde(default)]
    pub skipped_dirs: Vec<String>,
}

impl ScanReport {
    pub fn partial(&self) -> bool {
        self.kept < self.candidates
    }

    pub fn summary_line(&self) -> String {
        let mut parts = vec![format!("{} of {} files", self.kept, self.candidates)];
        let mut drops = Vec::new();
        if self.oversized_count > 0 {
            drops.push(format!("{} oversized", self.oversized_count));
        }
        if self.binary_count > 0 {
            drops.push(format!("{} binary", self.binary_count));
        }
        if self.minified_count > 0 {
            drops.push(format!("{} minified/generated", self.minified_count));
        }
        if self.priority_dropped > 0 {
            drops.push(format!(
                "{} lower-priority over the file cap",
                self.priority_dropped
            ));
        }
        if !drops.is_empty() {
            parts.push(format!("skipped: {}", drops.join(", ")));
        }
        parts.join("; ")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectContext {
    pub name: String,
    pub root: String,
    #[serde(default)]
    pub files: Vec<FileInfo>,
    #[serde(default)]
    pub file_tree: String,
    #[serde(default)]
    pub coverage: Option<ScanReport>,
}

impl ProjectContext {
    pub fn total_lines(&self) -> u64 {
        self.files.iter().map(|f| f.lines as u64).sum()
    }
}

// --- LLM analysis output models ---

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TechItem {
    pub name: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub version: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectOverview {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub one_liner: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub tech_stack: Vec<TechItem>,
    #[serde(default)]
    pub setup_instructions: Vec<String>,
    #[serde(default)]
    pub key_features: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub line: u32,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileDoc {
    pub path: String,
    #[serde(default)]
    pub purpose: String,
    #[serde(default)]
    pub key_symbols: Vec<Symbol>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Relationship {
    pub source: String,
    pub target: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Concept {
    pub name: String,
    #[serde(default)]
    pub explanation: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModuleDoc {
    pub name: String,
    #[serde(default)]
    pub purpose: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub files: Vec<FileDoc>,
    #[serde(default)]
    pub relationships: Vec<Relationship>,
    #[serde(default)]
    pub key_concepts: Vec<Concept>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    #[serde(default)]
    pub purpose: String,
    #[serde(default)]
    pub files: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArchitectureDiagram {
    #[serde(default)]
    pub architecture_type: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub components: Vec<Component>,
    #[serde(default)]
    pub mermaid_component: String,
    #[serde(default)]
    pub mermaid_sequence: String,
    #[serde(default)]
    pub data_flow: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReadingStep {
    pub order: u32,
    pub title: String,
    #[serde(default)]
    pub files: Vec<String>,
    #[serde(default)]
    pub explanation: String,
    #[serde(default)]
    pub time_estimate: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReadingGuide {
    #[serde(default)]
    pub introduction: String,
    #[serde(default)]
    pub steps: Vec<ReadingStep>,
    #[serde(default)]
    pub tips: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WikiData {
    #[serde(default)]
    pub overview: ProjectOverview,
    #[serde(default)]
    pub modules: Vec<ModuleDoc>,
    #[serde(default)]
    pub architecture: ArchitectureDiagram,
    #[serde(default)]
    pub reading_guide: ReadingGuide,
    #[serde(default)]
    pub file_index: std::collections::HashMap<String, FileDoc>,
}

// --- Indexer data structures ---

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SymbolKind {
    #[default]
    Function,
    Method,
    Class,
    Struct,
    Enum,
    Trait,
    Interface,
    Module,
    Constant,
    TypeAlias,
    Macro,
    Field,
}

impl std::fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolKind::Function => write!(f, "fn"),
            SymbolKind::Method => write!(f, "method"),
            SymbolKind::Class => write!(f, "class"),
            SymbolKind::Struct => write!(f, "struct"),
            SymbolKind::Enum => write!(f, "enum"),
            SymbolKind::Trait => write!(f, "trait"),
            SymbolKind::Interface => write!(f, "interface"),
            SymbolKind::Module => write!(f, "mod"),
            SymbolKind::Constant => write!(f, "const"),
            SymbolKind::TypeAlias => write!(f, "type"),
            SymbolKind::Macro => write!(f, "macro"),
            SymbolKind::Field => write!(f, "field"),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Public,
    #[default]
    Private,
    Protected,
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::Public => write!(f, "pub"),
            Visibility::Private => write!(f, ""),
            Visibility::Protected => write!(f, "protected"),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(default)]
    pub type_hint: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndexedSymbol {
    pub name: String,
    pub kind: SymbolKind,
    pub line: usize,
    #[serde(default)]
    pub end_line: usize,
    #[serde(default)]
    pub visibility: Visibility,
    #[serde(default)]
    pub params: Vec<Parameter>,
    #[serde(default)]
    pub return_type: Option<String>,
    #[serde(default)]
    pub doc_comment: Option<String>,
    #[serde(default)]
    pub decorators: Vec<String>,
    #[serde(default)]
    pub calls: Vec<String>,
    #[serde(default)]
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndexedImport {
    pub raw: String,
    #[serde(default)]
    pub resolved_path: Option<String>,
    #[serde(default)]
    pub symbols: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileMetrics {
    pub code_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
    pub complexity: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndexedFile {
    pub path: String,
    pub language: String,
    pub size: usize,
    pub lines: usize,
    pub metrics: FileMetrics,
    #[serde(default)]
    pub symbols: Vec<IndexedSymbol>,
    #[serde(default)]
    pub imports: Vec<IndexedImport>,
    #[serde(default)]
    pub exports: Vec<String>,
    pub content_hash: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModuleIndex {
    pub name: String,
    pub files: Vec<IndexedFile>,
    #[serde(default)]
    pub internal_edges: Vec<(String, String)>,
    #[serde(default)]
    pub external_deps: Vec<(String, String)>,
    pub total_symbols: usize,
    pub total_complexity: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallEdge {
    pub from_file: String,
    pub from_symbol: String,
    pub to_file: String,
    pub to_symbol: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectIndex {
    pub modules: Vec<ModuleIndex>,
    #[serde(default)]
    pub call_graph: Vec<CallEdge>,
    #[serde(default)]
    pub symbol_index: std::collections::HashMap<String, Vec<String>>,
}
