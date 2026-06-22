//! Runtime types for the ink runtime JSON format.
//! These types mirror the C# ink engine runtime objects.
//!
//! JSON format overview:
//! - Containers are JSON arrays; the last element is either null or a dict
//!   with named sub-containers and optional "#f" (flags), "#n" (name)
//! - String values use "^" prefix: "Hello" → "^Hello"
//! - Numbers, bools use standard JSON (true=1, false=0)
//! - Special objects: {"->": "path"}, {"VAR=": "name", "re": true}, etc.

use std::collections::HashMap;

/// Ink runtime object that can be serialized to JSON.
#[derive(Debug, Clone)]
pub enum InkObject {
    /// A container holds an ordered list of objects and optional named sub-containers.
    Container(Container),
    /// Control commands: "ev", "/ev", "out", "pop", etc.
    ControlCommand(ControlCommand),
    /// Native function calls: "+", "-", "==", "&&", etc.
    NativeFuncCall(NativeFuncCall),
    /// A divert to another point in the story.
    Divert(Divert),
    /// Variable assignment (VAR= or temp=).
    VariableAssignment(VariableAssignment),
    /// Variable reference (VAR?).
    VariableReference(VariableReference),
    /// A choice point in the story.
    ChoicePoint(ChoicePoint),
    /// A tag attached to content.
    Tag(Tag),
    /// Void value.
    Void,
    /// String value with ^ prefix.
    String(String),
    /// Divert target reference.
    DivertTarget(String),
    /// Variable pointer (for function parameters).
    VariablePointer { varname: String, context_index: i32 },
    /// Integer value.
    Int(i64),
    /// Float value.
    Float(f64),
}

impl InkObject {
    /// Returns true if this is a "truthy" value in ink (non-zero numbers, etc.)
    pub fn as_bool(&self) -> bool {
        match self {
            InkObject::Int(n) => *n != 0,
            InkObject::Float(f) => *f != 0.0,
            _ => false,
        }
    }
}

// -------------------------------------------------------------------------------------------------
// Container
// -------------------------------------------------------------------------------------------------

/// A container holds content (as an ordered array) and optionally named sub-elements.
#[derive(Debug, Clone)]
pub struct Container {
    /// Ordered list of content elements.
    pub content: Vec<InkObject>,
    /// Named sub-containers (e.g., stitches, choice containers).
    pub named: HashMap<String, Container>,
    /// Flags for visit/turn counting.
    pub flags: ContainerFlags,
    /// Optional name for this container.
    pub name: Option<String>,
    /// Optional count of visits (when visited count tracking is needed).
    pub visits: Option<i32>,
    /// Optional turn count (when turn-since tracking is needed).
    pub turn_index: Option<i32>,
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl Container {
    pub fn new() -> Self {
        Container {
            content: Vec::new(),
            named: HashMap::new(),
            flags: ContainerFlags::empty(),
            name: None,
            visits: None,
            turn_index: None,
        }
    }

    /// Add content to this container.
    pub fn push(&mut self, obj: InkObject) {
        self.content.push(obj);
    }

    /// Add a named sub-container.
    pub fn add_named(&mut self, name: &str, container: Container) {
        self.named.insert(name.to_string(), container);
    }

    /// Set the container's name.
    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }
}

/// Container flags (bitfield).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ContainerFlags(u32);

impl ContainerFlags {
    pub const VISITS: u32 = 0x1;
    pub const TURNS: u32 = 0x2;
    pub const COUNT_START_ONLY: u32 = 0x4;

    pub fn empty() -> Self {
        ContainerFlags(0)
    }

    pub fn from_bits(bits: u32) -> Self {
        ContainerFlags(bits)
    }

    pub fn visits(self) -> Self {
        ContainerFlags(self.0 | Self::VISITS)
    }

    pub fn turns(self) -> Self {
        ContainerFlags(self.0 | Self::TURNS)
    }

    pub fn count_start_only(self) -> Self {
        ContainerFlags(self.0 | Self::COUNT_START_ONLY)
    }

    pub fn value(self) -> u32 {
        self.0
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
}

// -------------------------------------------------------------------------------------------------
// Control commands
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlCommand {
    EvalStart,         // "ev"
    EvalEnd,           // "/ev"
    Out,               // "out"
    Pop,               // "pop"
    TunnelRet,         // "->->"
    FuncRet,           // "~ret"
    Duplicate,         // "du"
    StringEvalStart,   // "str"
    StringEvalEnd,     // "/str"
    NoOp,              // "nop"
    ChoiceCount,       // "choiceCnt"
    Turn,              // "turn"
    Turns,             // "turns"
    Visit,             // "visit"
    Sequence,          // "seq"
    Thread,            // "thread"
    Done,              // "done"
    End,               // "end"
    StopThread,        // "stopThread"
}

impl ControlCommand {
    pub fn as_str(&self) -> &'static str {
        match self {
            ControlCommand::EvalStart => "ev",
            ControlCommand::EvalEnd => "/ev",
            ControlCommand::Out => "out",
            ControlCommand::Pop => "pop",
            ControlCommand::TunnelRet => "->->",
            ControlCommand::FuncRet => "~ret",
            ControlCommand::Duplicate => "du",
            ControlCommand::StringEvalStart => "str",
            ControlCommand::StringEvalEnd => "/str",
            ControlCommand::NoOp => "nop",
            ControlCommand::ChoiceCount => "choiceCnt",
            ControlCommand::Turn => "turn",
            ControlCommand::Turns => "turns",
            ControlCommand::Visit => "visit",
            ControlCommand::Sequence => "seq",
            ControlCommand::Thread => "thread",
            ControlCommand::Done => "done",
            ControlCommand::End => "end",
            ControlCommand::StopThread => "stopThread",
        }
    }
}

// -------------------------------------------------------------------------------------------------
// Native function calls
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeFuncCall {
    Add,       // "+"
    Subtract,  // "-"
    Divide,    // "/"
    Multiply,  // "*"
    Modulo,    // "%"
    Negate,    // "_" (unary minus)
    Equal,     // "=="
    Greater,   // ">"
    Less,      // "<"
    GreaterEq, // ">="
    LessEq,    // "<="
    NotEqual,  // "!="
    Not,       // "!" (unary not)
    And,       // "&&"
    Or,        // "||"
    Min,       // "MIN"
    Max,       // "MAX"
}

impl NativeFuncCall {
    pub fn as_str(&self) -> &'static str {
        match self {
            NativeFuncCall::Add => "+",
            NativeFuncCall::Subtract => "-",
            NativeFuncCall::Divide => "/",
            NativeFuncCall::Multiply => "*",
            NativeFuncCall::Modulo => "%",
            NativeFuncCall::Negate => "_",
            NativeFuncCall::Equal => "==",
            NativeFuncCall::Greater => ">",
            NativeFuncCall::Less => "<",
            NativeFuncCall::GreaterEq => ">=",
            NativeFuncCall::LessEq => "<=",
            NativeFuncCall::NotEqual => "!=",
            NativeFuncCall::Not => "!",
            NativeFuncCall::And => "&&",
            NativeFuncCall::Or => "||",
            NativeFuncCall::Min => "MIN",
            NativeFuncCall::Max => "MAX",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "+" => Some(NativeFuncCall::Add),
            "-" => Some(NativeFuncCall::Subtract),
            "/" => Some(NativeFuncCall::Divide),
            "*" => Some(NativeFuncCall::Multiply),
            "%" => Some(NativeFuncCall::Modulo),
            "_" => Some(NativeFuncCall::Negate),
            "==" => Some(NativeFuncCall::Equal),
            ">" => Some(NativeFuncCall::Greater),
            "<" => Some(NativeFuncCall::Less),
            ">=" => Some(NativeFuncCall::GreaterEq),
            "<=" => Some(NativeFuncCall::LessEq),
            "!=" => Some(NativeFuncCall::NotEqual),
            "!" => Some(NativeFuncCall::Not),
            "&&" => Some(NativeFuncCall::And),
            "||" => Some(NativeFuncCall::Or),
            "MIN" => Some(NativeFuncCall::Min),
            "MAX" => Some(NativeFuncCall::Max),
            _ => None,
        }
    }
}

// -------------------------------------------------------------------------------------------------
// Divert
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Divert {
    /// Target path (e.g., "knot.stitch").
    pub target: Option<String>,
    /// If true, target is a variable containing a divert target.
    pub is_variable: bool,
    /// If true, this is a function call (pushes to callstack).
    pub is_function_call: bool,
    /// If true, this is a tunnel call.
    pub is_tunnel: bool,
    /// If true, this is an external function call.
    pub is_external: bool,
    /// For external calls, the number of arguments.
    pub external_args: Option<i32>,
    /// If true, divert is conditional (pops from stack).
    pub is_conditional: bool,
    /// If true, divert target can be a stitch within the current knot.
    pub stitches_also: bool,
}

impl Default for Divert {
    fn default() -> Self {
        Divert {
            target: None,
            is_variable: false,
            is_function_call: false,
            is_tunnel: false,
            is_external: false,
            external_args: None,
            is_conditional: false,
            stitches_also: false,
        }
    }
}

impl Divert {
    /// Create a standard divert to a target path.
    pub fn new(target: &str) -> Self {
        Divert {
            target: Some(target.to_string()),
            ..Default::default()
        }
    }

    /// Create a divert to a variable.
    pub fn to_variable(varname: &str) -> Self {
        Divert {
            target: Some(varname.to_string()),
            is_variable: true,
            ..Default::default()
        }
    }

    /// Create a function call divert.
    pub fn function(target: &str) -> Self {
        Divert {
            target: Some(target.to_string()),
            is_function_call: true,
            ..Default::default()
        }
    }

    /// Create a tunnel divert.
    pub fn tunnel(target: &str) -> Self {
        Divert {
            target: Some(target.to_string()),
            is_tunnel: true,
            ..Default::default()
        }
    }

    /// Create an external function call.
    pub fn external(name: &str, args: i32) -> Self {
        Divert {
            target: Some(name.to_string()),
            is_external: true,
            external_args: Some(args),
            is_function_call: true,
            ..Default::default()
        }
    }

    /// Mark this as a tunnel onwards (used for ->->).
    pub fn tunnel_onwards() -> Self {
        Divert {
            is_tunnel: true,
            ..Default::default()
        }
    }
}

// -------------------------------------------------------------------------------------------------
// Variable assignment
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct VariableAssignment {
    pub varname: String,
    /// true for VAR= (reassignment), false for temp= (new temp).
    pub is_global: bool,
    /// For temp variables in functions.
    pub context_index: i32,
}

impl VariableAssignment {
    pub fn global(name: &str, _reassign: bool) -> Self {
        VariableAssignment {
            varname: name.to_string(),
            is_global: true,
            context_index: 0,
        }
    }

    pub fn temp(name: &str) -> Self {
        VariableAssignment {
            varname: name.to_string(),
            is_global: false,
            context_index: -1,
        }
    }
}

// -------------------------------------------------------------------------------------------------
// Variable reference
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct VariableReference {
    pub varname: String,
    /// Context index: -1 = unknown, 0 = global, >0 = local.
    pub context_index: i32,
    /// If true, this is a read-count reference (CNT?).
    pub is_read_count: bool,
}

impl VariableReference {
    pub fn new(varname: &str) -> Self {
        VariableReference {
            varname: varname.to_string(),
            context_index: -1,
            is_read_count: false,
        }
    }

    pub fn read_count(path: &str) -> Self {
        VariableReference {
            varname: path.to_string(),
            context_index: -1,
            is_read_count: true,
        }
    }
}

// -------------------------------------------------------------------------------------------------
// Choice point
// -------------------------------------------------------------------------------------------------

/// Choice point flags (bitfield, matching C# ChoicePointFlags).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChoiceFlags(u32);

impl ChoiceFlags {
    pub const EMPTY: u32 = 0;
    pub const HAS_CONDITION: u32 = 0x1;
    pub const HAS_START_CONTENT: u32 = 0x2;
    pub const HAS_CHOICE_ONLY_CONTENT: u32 = 0x4;
    pub const IS_INVISIBLE_DEFAULT: u32 = 0x8;
    pub const ONCE_ONLY: u32 = 0x10;
    pub const HAS_DYNAMIC_CONTENT: u32 = 0x20;
    pub const TURNS_SINCE_CHOICE: u32 = 0x40;
    pub const HAS_TEXT: u32 = 0x80;
    pub const HAS_INCREMENTAL_COUNT: u32 = 0x100;

    pub fn from_bits(bits: u32) -> Self {
        ChoiceFlags(bits)
    }

    pub fn value(self) -> u32 {
        self.0
    }

    pub fn has_condition(mut self) -> Self {
        self.0 |= Self::HAS_CONDITION;
        self
    }

    pub fn has_start_content(mut self) -> Self {
        self.0 |= Self::HAS_START_CONTENT;
        self
    }

    pub fn has_choice_only_content(mut self) -> Self {
        self.0 |= Self::HAS_CHOICE_ONLY_CONTENT;
        self
    }

    pub fn is_invisible_default(mut self) -> Self {
        self.0 |= Self::IS_INVISIBLE_DEFAULT;
        self
    }

    pub fn once_only(mut self) -> Self {
        self.0 |= Self::ONCE_ONLY;
        self
    }

    pub fn has_dynamic_content(mut self) -> Self {
        self.0 |= Self::HAS_DYNAMIC_CONTENT;
        self
    }
}

impl Default for ChoiceFlags {
    fn default() -> Self {
        ChoiceFlags(Self::ONCE_ONLY) // Default is once-only
    }
}

#[derive(Debug, Clone)]
pub struct ChoicePoint {
    /// Path to the container that runs when this choice is chosen.
    pub target_path: String,
    /// Flags for conditional/start content/choice-only content.
    pub flags: ChoiceFlags,
    /// Path for start content (choice text before []).
    pub start_content_path: Option<String>,
    /// Path for choice-only content (content inside []).
    pub choice_only_content_path: Option<String>,
    /// For once-only choices, the original choice text for comparison.
    pub original_text_hash: Option<i32>,
}

impl ChoicePoint {
    pub fn new(target: &str) -> Self {
        ChoicePoint {
            target_path: target.to_string(),
            flags: ChoiceFlags::default(),
            start_content_path: None,
            choice_only_content_path: None,
            original_text_hash: None,
        }
    }

    pub fn has_condition(mut self) -> Self {
        self.flags = self.flags.has_condition();
        self
    }

    pub fn has_start_content(mut self) -> Self {
        self.flags = self.flags.has_start_content();
        self
    }

    pub fn has_choice_only_content(mut self) -> Self {
        self.flags = self.flags.has_choice_only_content();
        self
    }

    pub fn is_invisible_default(mut self) -> Self {
        self.flags = self.flags.is_invisible_default();
        self
    }
}

// -------------------------------------------------------------------------------------------------
// Tag
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Tag {
    pub text: String,
}

impl Tag {
    pub fn new(text: &str) -> Self {
        Tag { text: text.to_string() }
    }
}

// -------------------------------------------------------------------------------------------------
// InkListDeclaration (for LIST declarations in story)
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ListDefinition {
    pub name: String,
    pub items: Vec<ListItem>,
}

#[derive(Debug, Clone)]
pub struct ListItem {
    pub name: String,
    pub value: i32,
}

// -------------------------------------------------------------------------------------------------
// Story (top-level runtime object)
// -------------------------------------------------------------------------------------------------

/// The compiled ink story, ready for serialization to JSON.
#[derive(Debug, Clone)]
pub struct Story {
    /// Runtime version (currently 21).
    pub ink_version: i32,
    /// Root container containing the entire story.
    pub root: Container,
    /// Global variable declarations (initial values).
    pub variables: Vec<VariableAssignment>,
    /// List declarations.
    pub lists: Vec<ListDefinition>,
    /// Named knots (for quick lookup during compilation).
    /// (This is built during codegen, not serialized to JSON)
    #[doc(hidden)]
    pub knot_map: HashMap<String, Container>,
}

impl Default for Story {
    fn default() -> Self {
        Story {
            ink_version: 21,
            root: Container::new(),
            variables: Vec::new(),
            lists: Vec::new(),
            knot_map: HashMap::new(),
        }
    }
}

impl Story {
    pub fn new() -> Self {
        Story::default()
    }

    /// Add a knot as a named sub-container of the root.
    pub fn add_knot(&mut self, name: &str, container: Container) {
        let name_for_map = name.replace(' ', "_");
        self.knot_map.insert(name_for_map.clone(), container.clone());
        self.root.add_named(&name_for_map, container);
    }

    /// Add a global variable.
    pub fn add_variable(&mut self, varname: &str) {
        self.variables.push(VariableAssignment::global(varname, false));
    }

    /// Add a list declaration.
    pub fn add_list(&mut self, list: ListDefinition) {
        self.lists.push(list);
    }
}