#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub filepath: String,
    pub position: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompileNote {
    pub location: Option<SourceLocation>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompileError {
    pub location: SourceLocation,
    pub message: String,
    pub notes: Vec<CompileNote>,
}
