//! JCL AST types -- typed representation of JOB/EXEC/DD statements.
//!
//! Covers the subset of JCL relevant to .proc batch emission:
//! - Job structure (name, class, params)
//! - Steps (EXEC PGM= / EXEC proc)
//! - DD statements (DSN, DISP, DCB, SYSOUT, concatenation)
//! - Conditional execution (IF/THEN/ELSE/ENDIF, COND=)
//! - Procedure definitions (PROC/PEND)

use std::collections::HashMap;

use serde::Serialize;

// ---------------------------------------------------------------------------
// Top-level: JCL source file
// ---------------------------------------------------------------------------

/// A parsed JCL source file -- either an execution JCL (with JOB card)
/// or a cataloged procedure (PROC/PEND).
#[derive(Debug, Clone, Serialize)]
#[allow(clippy::large_enum_variant)]
pub enum JclSource {
    /// Execution JCL: JOB card + steps.
    Job(JclJob),
    /// Cataloged procedure definition.
    Proc(JclProc),
}

// ---------------------------------------------------------------------------
// JOB
// ---------------------------------------------------------------------------

/// A JCL job: the JOB card plus all steps and control statements.
#[derive(Debug, Clone, Serialize)]
pub struct JclJob {
    /// Job name (1-8 chars, from the name field).
    pub name: String,
    /// Accounting information (positional, from JOB card).
    pub accounting: Option<String>,
    /// Programmer name (positional, from JOB card).
    pub programmer: Option<String>,
    /// Job-level keyword parameters.
    pub params: JobParams,
    /// JCLLIB ORDER= (procedure search path).
    pub jcllib: Vec<String>,
    /// JOBLIB DD (step-0 library concatenation).
    pub joblib: Vec<DdStatement>,
    /// Ordered list of steps and control flow statements.
    pub body: Vec<JobBodyItem>,
    /// Raw comments collected from the JCL.
    pub comments: Vec<String>,
}

/// Job-level keyword parameters (from the JOB card).
#[derive(Debug, Clone, Default, Serialize)]
pub struct JobParams {
    /// Job class (single char A-Z, 0-9).
    pub class: Option<String>,
    /// Message class.
    pub msgclass: Option<String>,
    /// Message level (statements, messages).
    pub msglevel: Option<(u8, u8)>,
    /// NOTIFY= user to notify on completion.
    pub notify: Option<String>,
    /// COND= job-level condition (max RC, operator).
    pub cond: Option<CondParam>,
    /// REGION= memory limit.
    pub region: Option<String>,
    /// TIME= CPU time limit.
    pub time: Option<String>,
    /// PRTY= priority (0-15).
    pub priority: Option<u8>,
    /// TYPRUN= (SCAN, HOLD, JCLHOLD, COPY).
    pub typrun: Option<String>,
    /// RESTART= step name for restart.
    pub restart: Option<String>,
    /// SCHENV= scheduling environment.
    pub schenv: Option<String>,
    /// All raw parameters (for anything not explicitly modeled).
    pub raw: HashMap<String, String>,
}

/// An item in the job body -- either a step or a control-flow statement.
#[derive(Debug, Clone, Serialize)]
#[allow(clippy::large_enum_variant)]
pub enum JobBodyItem {
    /// An EXEC step with its DD statements.
    Step(JclStep),
    /// IF condition THEN
    If(IfStatement),
    /// ELSE
    Else,
    /// ENDIF
    EndIf,
    /// INCLUDE MEMBER=name
    Include(String),
    /// SET symbolic=value
    Set { symbol: String, value: String },
    /// Comment block.
    Comment(String),
}

// ---------------------------------------------------------------------------
// STEP (EXEC)
// ---------------------------------------------------------------------------

/// A JCL step: EXEC statement + associated DD statements.
#[derive(Debug, Clone, Serialize)]
pub struct JclStep {
    /// Step name (optional, 1-8 chars).
    pub name: Option<String>,
    /// What this step executes.
    pub exec: ExecType,
    /// Step-level keyword parameters.
    pub params: StepParams,
    /// DD statements associated with this step.
    pub dd_statements: Vec<DdStatement>,
    /// Comments within this step.
    pub comments: Vec<String>,
}

/// What a step executes: a program or a procedure.
#[derive(Debug, Clone, Serialize)]
pub enum ExecType {
    /// EXEC PGM=program_name
    Pgm(String),
    /// EXEC proc_name (or EXEC PROC=proc_name)
    Proc {
        /// Procedure name.
        name: String,
        /// Symbolic parameter overrides passed to the proc.
        overrides: HashMap<String, String>,
    },
}

/// Step-level keyword parameters (from the EXEC statement).
#[derive(Debug, Clone, Default, Serialize)]
pub struct StepParams {
    /// PARM= passed to the program.
    pub parm: Option<String>,
    /// COND= step-level condition code test.
    pub cond: Option<CondParam>,
    /// REGION= memory limit.
    pub region: Option<String>,
    /// TIME= CPU time limit.
    pub time: Option<String>,
    /// ACCT= accounting info.
    pub acct: Option<String>,
    /// ADDRSPC= (VIRT or REAL).
    pub addrspc: Option<String>,
    /// DYNAMNBR= dynamic allocation limit.
    pub dynamnbr: Option<String>,
    /// MEMLIMIT= memory limit above the bar.
    pub memlimit: Option<String>,
    /// PARMDD= DD name containing PARM data.
    pub parmdd: Option<String>,
    /// All raw parameters (for anything not explicitly modeled).
    pub raw: HashMap<String, String>,
}

// ---------------------------------------------------------------------------
// DD Statement
// ---------------------------------------------------------------------------

/// A DD statement: dataset definition for a step.
#[derive(Debug, Clone, Serialize)]
pub struct DdStatement {
    /// DD name (e.g., SYSIN, SYSOUT, SORTIN, user-defined).
    pub name: String,
    /// Qualified name if step override (e.g., STEP1.SYSIN).
    pub step_qualifier: Option<String>,
    /// DD type and parameters.
    pub kind: DdKind,
    /// Concatenated DD statements (unnamed continuations).
    pub concatenations: Vec<DdKind>,
}

/// The kind of DD statement.
#[derive(Debug, Clone, Serialize)]
#[allow(clippy::large_enum_variant)]
pub enum DdKind {
    /// DSN=dataset.name with DISP, DCB, SPACE, etc.
    Dataset(DatasetDd),
    /// SYSOUT=class (print/spool output).
    Sysout(SysoutDd),
    /// DD * or DD DATA (inline data).
    InlineData {
        /// The inline data lines.
        data: Vec<String>,
        /// DLM= custom delimiter (default is /*).
        delimiter: Option<String>,
    },
    /// DD DUMMY (null dataset).
    Dummy,
    /// DD DYNAM (dynamic allocation).
    Dynam,
    /// DDNAME=other_dd (reference to another DD).
    DdRef(String),
    /// PATH=/unix/path (HFS file).
    Path(PathDd),
}

/// A dataset DD: DSN + disposition + DCB + space.
#[derive(Debug, Clone, Default, Serialize)]
pub struct DatasetDd {
    /// DSN= dataset name (may include member in parens).
    pub dsn: Option<String>,
    /// Member name if DSN=dataset(member).
    pub member: Option<String>,
    /// DISP= disposition: (status, normal-term, abnormal-term).
    pub disp: Option<Disposition>,
    /// DCB parameters.
    pub dcb: DcbParams,
    /// SPACE= allocation.
    pub space: Option<String>,
    /// UNIT= device type.
    pub unit: Option<String>,
    /// VOL=SER= volume serial.
    pub volume: Option<String>,
    /// LABEL= tape label info.
    pub label: Option<String>,
    /// DATACLAS= SMS data class.
    pub dataclas: Option<String>,
    /// STORCLAS= SMS storage class.
    pub storclas: Option<String>,
    /// MGMTCLAS= SMS management class.
    pub mgmtclas: Option<String>,
    /// RECORG= VSAM record organization (KS, ES, RR, LS).
    pub recorg: Option<String>,
    /// KEYOFF= VSAM key offset.
    pub keyoff: Option<String>,
    /// KEYLEN= key length.
    pub keylen: Option<String>,
    /// LIKE= model dataset.
    pub like: Option<String>,
    /// REFDD= reference DD for attributes.
    pub refdd: Option<String>,
    /// EXPDT= expiration date.
    pub expdt: Option<String>,
    /// RETPD= retention period.
    pub retpd: Option<String>,
    /// DSNTYPE= dataset type (LIBRARY, PDS, BASIC, LARGE, etc.).
    pub dsntype: Option<String>,
    /// All raw parameters.
    pub raw: HashMap<String, String>,
}

/// DISP= (status, normal-termination, abnormal-termination).
#[derive(Debug, Clone, Serialize)]
pub struct Disposition {
    /// Status: NEW, OLD, SHR, MOD.
    pub status: DispStatus,
    /// Normal termination: DELETE, KEEP, PASS, CATLG, UNCATLG.
    pub normal: Option<DispAction>,
    /// Abnormal termination: DELETE, KEEP, CATLG, UNCATLG.
    pub abnormal: Option<DispAction>,
}

/// DISP status (first positional).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DispStatus {
    New,
    Old,
    Shr,
    Mod,
}

/// DISP normal/abnormal action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DispAction {
    Delete,
    Keep,
    Pass,
    Catlg,
    Uncatlg,
}

/// DCB parameters (Data Control Block).
#[derive(Debug, Clone, Default, Serialize)]
pub struct DcbParams {
    /// RECFM= record format (F, FB, V, VB, FBA, VBA, U).
    pub recfm: Option<String>,
    /// LRECL= logical record length.
    pub lrecl: Option<u32>,
    /// BLKSIZE= block size.
    pub blksize: Option<u32>,
    /// DSORG= dataset organization (PS, PO, DA, IS).
    pub dsorg: Option<String>,
}

/// SYSOUT DD: print/spool output.
#[derive(Debug, Clone, Default, Serialize)]
pub struct SysoutDd {
    /// Output class (A-Z, 0-9, or *).
    pub class: String,
    /// DEST= destination.
    pub dest: Option<String>,
    /// COPIES= copy count.
    pub copies: Option<u16>,
    /// All raw parameters.
    pub raw: HashMap<String, String>,
}

/// PATH= DD for HFS/zFS files.
#[derive(Debug, Clone, Serialize)]
pub struct PathDd {
    /// Unix file path.
    pub path: String,
    /// PATHDISP= (status, abnormal).
    pub pathdisp: Option<String>,
    /// PATHMODE= file permission bits.
    pub pathmode: Option<String>,
    /// PATHOPTS= open options.
    pub pathopts: Option<String>,
}

// ---------------------------------------------------------------------------
// Conditional Execution
// ---------------------------------------------------------------------------

/// COND= parameter (job-level or step-level).
///
/// Format: COND=((code,operator,stepname),...) or COND=EVEN/ONLY
#[derive(Debug, Clone, Serialize)]
pub enum CondParam {
    /// COND=EVEN -- execute even if prior step abends.
    Even,
    /// COND=ONLY -- execute only if prior step abends.
    Only,
    /// One or more condition tests.
    Tests(Vec<CondTest>),
}

/// A single COND test: (return_code, operator[, stepname]).
#[derive(Debug, Clone, Serialize)]
pub struct CondTest {
    /// Return code to compare against (0-4095).
    pub code: u16,
    /// Comparison operator.
    pub operator: CondOperator,
    /// Step name to test (None = any prior step).
    pub step: Option<String>,
}

/// COND= comparison operators.
///
/// Semantics: the step is BYPASSED if the test is TRUE.
/// E.g., COND=(4,LT) means "bypass if 4 < prior_RC" (i.e., skip if RC > 4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CondOperator {
    /// GT: bypass if code > stepRC
    Gt,
    /// GE: bypass if code >= stepRC
    Ge,
    /// EQ: bypass if code == stepRC
    Eq,
    /// LT: bypass if code < stepRC
    Lt,
    /// LE: bypass if code <= stepRC
    Le,
    /// NE: bypass if code != stepRC
    Ne,
}

/// IF statement (JCL conditional processing).
///
/// Format: // IF (condition) THEN
#[derive(Debug, Clone, Serialize)]
pub struct IfStatement {
    /// The condition expression.
    pub condition: IfCondition,
}

/// IF condition: one or more tests joined by AND/OR.
#[derive(Debug, Clone, Serialize)]
pub enum IfCondition {
    /// A single test: stepname.RC op value, stepname.ABEND, etc.
    Test(IfTest),
    /// NOT condition
    Not(Box<IfCondition>),
    /// condition AND condition
    And(Box<IfCondition>, Box<IfCondition>),
    /// condition OR condition
    Or(Box<IfCondition>, Box<IfCondition>),
}

/// A single IF test.
#[derive(Debug, Clone, Serialize)]
pub enum IfTest {
    /// RC comparison: [stepname.]RC op value
    Rc {
        step: Option<String>,
        operator: CondOperator,
        value: u16,
    },
    /// ABEND test: [stepname.]ABEND [= TRUE/FALSE]
    Abend {
        step: Option<String>,
        value: bool,
    },
    /// ABENDCC comparison: [stepname.]ABENDCC op value
    AbendCc {
        step: Option<String>,
        operator: CondOperator,
        value: String,
    },
    /// RUN test: [stepname.]RUN [= TRUE/FALSE]
    Run {
        step: Option<String>,
        value: bool,
    },
}

// ---------------------------------------------------------------------------
// Procedure
// ---------------------------------------------------------------------------

/// A cataloged procedure definition (PROC ... PEND).
#[derive(Debug, Clone, Serialize)]
pub struct JclProc {
    /// Procedure name.
    pub name: Option<String>,
    /// Symbolic parameters with defaults.
    pub symbols: HashMap<String, Option<String>>,
    /// Steps within the procedure.
    pub body: Vec<JobBodyItem>,
}

// ---------------------------------------------------------------------------
// Convenience impls
// ---------------------------------------------------------------------------

impl JclJob {
    /// Returns all steps in the job body (flattened, ignoring control flow).
    pub fn steps(&self) -> Vec<&JclStep> {
        self.body.iter().filter_map(|item| {
            if let JobBodyItem::Step(step) = item {
                Some(step)
            } else {
                None
            }
        }).collect()
    }

    /// Returns the step with the given name, if any.
    pub fn step_by_name(&self, name: &str) -> Option<&JclStep> {
        self.steps().into_iter().find(|s| {
            s.name.as_deref() == Some(name)
        })
    }
}

impl JclStep {
    /// Returns the program name if this is an EXEC PGM= step.
    pub fn program(&self) -> Option<&str> {
        match &self.exec {
            ExecType::Pgm(name) => Some(name.as_str()),
            ExecType::Proc { .. } => None,
        }
    }

    /// Returns the procedure name if this is an EXEC proc step.
    pub fn proc_name(&self) -> Option<&str> {
        match &self.exec {
            ExecType::Pgm(_) => None,
            ExecType::Proc { name, .. } => Some(name.as_str()),
        }
    }

    /// Returns the DD statement with the given name, if any.
    pub fn dd(&self, name: &str) -> Option<&DdStatement> {
        self.dd_statements.iter().find(|dd| dd.name.eq_ignore_ascii_case(name))
    }

    /// Returns true if this step executes a known sort utility.
    pub fn is_sort(&self) -> bool {
        matches!(self.program(), Some(p) if
            p.eq_ignore_ascii_case("SORT")
            || p.eq_ignore_ascii_case("DFSORT")
            || p.eq_ignore_ascii_case("SYNCSORT")
            || p.eq_ignore_ascii_case("ICETOOL")
        )
    }

    /// Returns true if this step executes IDCAMS (VSAM utility).
    pub fn is_idcams(&self) -> bool {
        matches!(self.program(), Some(p) if p.eq_ignore_ascii_case("IDCAMS"))
    }

    /// Returns true if this step is an IEFBR14 no-op (allocation only).
    pub fn is_noop(&self) -> bool {
        matches!(self.program(), Some(p) if p.eq_ignore_ascii_case("IEFBR14"))
    }

    /// Returns true if this step executes a DB2 utility (DSNTEP2, IKJEFT01 with DSN).
    pub fn is_db2(&self) -> bool {
        matches!(self.program(), Some(p) if
            p.eq_ignore_ascii_case("DSNTEP2")
            || p.eq_ignore_ascii_case("DSNTEP4")
            || p.eq_ignore_ascii_case("DSNTIAD")
            || p.eq_ignore_ascii_case("IKJEFT01")
            || p.eq_ignore_ascii_case("IKJEFT1B")
        )
    }
}

impl DdStatement {
    /// Returns the dataset name, if this is a dataset DD.
    pub fn dsn(&self) -> Option<&str> {
        match &self.kind {
            DdKind::Dataset(ds) => ds.dsn.as_deref(),
            _ => None,
        }
    }

    /// Returns the DISP status, if this is a dataset DD.
    pub fn disp_status(&self) -> Option<DispStatus> {
        match &self.kind {
            DdKind::Dataset(ds) => ds.disp.as_ref().map(|d| d.status),
            _ => None,
        }
    }

    /// Returns true if this is a SYSOUT DD.
    pub fn is_sysout(&self) -> bool {
        matches!(&self.kind, DdKind::Sysout(_))
    }

    /// Returns true if this is inline data (DD * or DD DATA).
    pub fn is_inline(&self) -> bool {
        matches!(&self.kind, DdKind::InlineData { .. })
    }

    /// Returns true if this is a DUMMY DD.
    pub fn is_dummy(&self) -> bool {
        matches!(&self.kind, DdKind::Dummy)
    }

    /// Returns true if this DD name is a standard system DD.
    pub fn is_system_dd(&self) -> bool {
        let upper = self.name.to_uppercase();
        matches!(upper.as_str(),
            "SYSIN" | "SYSOUT" | "SYSPRINT" | "SYSUDUMP" | "SYSABEND"
            | "SYSDBOUT" | "SYSUT1" | "SYSUT2" | "SYSUT3" | "SYSUT4"
            | "SYSLMOD" | "SYSLIN" | "SYSLIB" | "STEPLIB" | "JOBLIB"
            | "SORTLIB" | "SORTIN" | "SORTOUT" | "SORTWK01" | "SORTWK02"
            | "SORTWK03" | "CEEDUMP" | "SYSTSIN" | "SYSTSPRT"
        )
    }
}

impl std::fmt::Display for DispStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::New => write!(f, "NEW"),
            Self::Old => write!(f, "OLD"),
            Self::Shr => write!(f, "SHR"),
            Self::Mod => write!(f, "MOD"),
        }
    }
}

impl std::fmt::Display for DispAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Delete => write!(f, "DELETE"),
            Self::Keep => write!(f, "KEEP"),
            Self::Pass => write!(f, "PASS"),
            Self::Catlg => write!(f, "CATLG"),
            Self::Uncatlg => write!(f, "UNCATLG"),
        }
    }
}

impl std::fmt::Display for CondOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gt => write!(f, "GT"),
            Self::Ge => write!(f, "GE"),
            Self::Eq => write!(f, "EQ"),
            Self::Lt => write!(f, "LT"),
            Self::Le => write!(f, "LE"),
            Self::Ne => write!(f, "NE"),
        }
    }
}
