//! JCL parser for the NexMig toolchain.
//!
//! Parses IBM JCL (Job Control Language) source into a typed AST.
//! JCL defines batch job structure: job steps, file allocations,
//! program execution, and conditional processing.
//!
//! The AST maps directly to `.proc` batch orchestration in the
//! Nexflow DSL suite.
//!
//! # Usage
//!
//! ```ignore
//! use cobol_jcl::parser::parse_jcl;
//! use cobol_jcl::ast::JclSource;
//!
//! let jcl = r#"
//! //MYJOB  JOB ,'JOHN DOE',CLASS=A
//! //STEP1  EXEC PGM=MYPROG
//! //INPUT  DD DSN=MY.DATASET,DISP=SHR
//! //OUTPUT DD SYSOUT=*
//! //"#;
//!
//! let source = parse_jcl(jcl).unwrap();
//! match source {
//!     JclSource::Job(job) => {
//!         println!("Job: {}", job.name);
//!         for step in job.steps() {
//!             println!("  Step: {:?} -> {:?}", step.name, step.program());
//!         }
//!     }
//!     JclSource::Proc(proc) => {
//!         println!("Proc: {:?}", proc.name);
//!     }
//! }
//! ```

pub mod ast;
pub mod error;
pub mod graph;
pub mod parser;
pub mod resolve;
