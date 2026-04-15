// Generated from /Users/chandramohn/workspace/nexcore/grammar/nexflow/ServiceDSL.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::servicedsllistener::*;
use super::servicedslvisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const AUTHORIZE:isize=1; 
		pub const CACHE:isize=2; 
		pub const CALL:isize=3; 
		pub const CHECK:isize=4; 
		pub const COMPENSATE:isize=5; 
		pub const CONFIG:isize=6; 
		pub const CONSUMES:isize=7; 
		pub const CONTAINS:isize=8; 
		pub const DESCRIPTION:isize=9; 
		pub const EACH:isize=10; 
		pub const END:isize=11; 
		pub const FALLBACK:isize=12; 
		pub const FOR:isize=13; 
		pub const FROM:isize=14; 
		pub const HANDLER:isize=15; 
		pub const HEALTH:isize=16; 
		pub const IMPLEMENTS:isize=17; 
		pub const IMPORT:isize=18; 
		pub const IN:isize=19; 
		pub const INTO:isize=20; 
		pub const INVALIDATE_ON:isize=21; 
		pub const IS:isize=22; 
		pub const LOOKUP:isize=23; 
		pub const NULL:isize=24; 
		pub const ON_ERROR:isize=25; 
		pub const ON_ROLLBACK:isize=26; 
		pub const PARALLEL:isize=27; 
		pub const PERSIST:isize=28; 
		pub const PUBLISH:isize=29; 
		pub const READY:isize=30; 
		pub const RESPOND:isize=31; 
		pub const SAGA:isize=32; 
		pub const SCOPE:isize=33; 
		pub const SERVICE:isize=34; 
		pub const STEP:isize=35; 
		pub const TIMEOUT:isize=36; 
		pub const TO:isize=37; 
		pub const TRANSACTION:isize=38; 
		pub const TRANSFORM:isize=39; 
		pub const TTL:isize=40; 
		pub const USING:isize=41; 
		pub const VALIDATE:isize=42; 
		pub const WHEN:isize=43; 
		pub const WHERE:isize=44; 
		pub const WITH:isize=45; 
		pub const TRUE:isize=46; 
		pub const FALSE:isize=47; 
		pub const EQ:isize=48; 
		pub const NEQ:isize=49; 
		pub const LT:isize=50; 
		pub const GT:isize=51; 
		pub const LTE:isize=52; 
		pub const GTE:isize=53; 
		pub const INTEGER:isize=54; 
		pub const DECIMAL_LITERAL:isize=55; 
		pub const STRING_LITERAL:isize=56; 
		pub const IDENTIFIER:isize=57; 
		pub const DOT:isize=58; 
		pub const COMMA:isize=59; 
		pub const SLASH:isize=60; 
		pub const DASH:isize=61; 
		pub const DOT_SLASH:isize=62; 
		pub const DOT_DOT_SLASH:isize=63; 
		pub const LINE_COMMENT:isize=64; 
		pub const WS:isize=65;
	pub const RULE_compilationUnit:usize = 0; 
	pub const RULE_importStatement:usize = 1; 
	pub const RULE_importPath:usize = 2; 
	pub const RULE_pathSegment:usize = 3; 
	pub const RULE_word:usize = 4; 
	pub const RULE_serviceDefinition:usize = 5; 
	pub const RULE_serviceBody:usize = 6; 
	pub const RULE_serviceElement:usize = 7; 
	pub const RULE_descriptionDecl:usize = 8; 
	pub const RULE_implementsDecl:usize = 9; 
	pub const RULE_consumesDecl:usize = 10; 
	pub const RULE_configBlock:usize = 11; 
	pub const RULE_configDirective:usize = 12; 
	pub const RULE_configValue:usize = 13; 
	pub const RULE_handlerDecl:usize = 14; 
	pub const RULE_handlerBody:usize = 15; 
	pub const RULE_handlerStatement:usize = 16; 
	pub const RULE_authorizeStmt:usize = 17; 
	pub const RULE_validateStmt:usize = 18; 
	pub const RULE_lookupStmt:usize = 19; 
	pub const RULE_whereClause:usize = 20; 
	pub const RULE_transformStmt:usize = 21; 
	pub const RULE_persistStmt:usize = 22; 
	pub const RULE_callStmt:usize = 23; 
	pub const RULE_withClause:usize = 24; 
	pub const RULE_forEachClause:usize = 25; 
	pub const RULE_intoClause:usize = 26; 
	pub const RULE_publishStmt:usize = 27; 
	pub const RULE_respondStmt:usize = 28; 
	pub const RULE_onErrorBlock:usize = 29; 
	pub const RULE_errorCase:usize = 30; 
	pub const RULE_predicate:usize = 31; 
	pub const RULE_transactionBlock:usize = 32; 
	pub const RULE_onRollbackBlock:usize = 33; 
	pub const RULE_sagaBlock:usize = 34; 
	pub const RULE_sagaStep:usize = 35; 
	pub const RULE_compensateBlock:usize = 36; 
	pub const RULE_parallelBlock:usize = 37; 
	pub const RULE_cacheBlock:usize = 38; 
	pub const RULE_cacheEntry:usize = 39; 
	pub const RULE_healthBlock:usize = 40; 
	pub const RULE_healthCheck:usize = 41; 
	pub const RULE_readyDecl:usize = 42; 
	pub const RULE_qualifiedAnnotation:usize = 43; 
	pub const RULE_valueOrCfg:usize = 44; 
	pub const RULE_expression:usize = 45; 
	pub const RULE_expressionList:usize = 46; 
	pub const RULE_comparator:usize = 47; 
	pub const RULE_qualifiedRef:usize = 48; 
	pub const RULE_schemaRef:usize = 49; 
	pub const RULE_identifierList:usize = 50; 
	pub const RULE_literal:usize = 51;
	pub const ruleNames: [&'static str; 52] =  [
		"compilationUnit", "importStatement", "importPath", "pathSegment", "word", 
		"serviceDefinition", "serviceBody", "serviceElement", "descriptionDecl", 
		"implementsDecl", "consumesDecl", "configBlock", "configDirective", "configValue", 
		"handlerDecl", "handlerBody", "handlerStatement", "authorizeStmt", "validateStmt", 
		"lookupStmt", "whereClause", "transformStmt", "persistStmt", "callStmt", 
		"withClause", "forEachClause", "intoClause", "publishStmt", "respondStmt", 
		"onErrorBlock", "errorCase", "predicate", "transactionBlock", "onRollbackBlock", 
		"sagaBlock", "sagaStep", "compensateBlock", "parallelBlock", "cacheBlock", 
		"cacheEntry", "healthBlock", "healthCheck", "readyDecl", "qualifiedAnnotation", 
		"valueOrCfg", "expression", "expressionList", "comparator", "qualifiedRef", 
		"schemaRef", "identifierList", "literal"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;64] = [
		None, Some("'authorize'"), Some("'cache'"), Some("'call'"), Some("'check'"), 
		Some("'compensate'"), Some("'config'"), Some("'consumes'"), Some("'contains'"), 
		Some("'description'"), Some("'each'"), Some("'end'"), Some("'fallback'"), 
		Some("'for'"), Some("'from'"), Some("'handler'"), Some("'health'"), Some("'implements'"), 
		Some("'import'"), Some("'in'"), Some("'into'"), Some("'invalidate_on'"), 
		Some("'is'"), Some("'lookup'"), Some("'null'"), Some("'on_error'"), Some("'on_rollback'"), 
		Some("'parallel'"), Some("'persist'"), Some("'publish'"), Some("'ready'"), 
		Some("'respond'"), Some("'saga'"), Some("'scope'"), Some("'service'"), 
		Some("'step'"), Some("'timeout'"), Some("'to'"), Some("'transaction'"), 
		Some("'transform'"), Some("'ttl'"), Some("'using'"), Some("'validate'"), 
		Some("'when'"), Some("'where'"), Some("'with'"), Some("'true'"), Some("'false'"), 
		Some("'='"), Some("'!='"), Some("'<'"), Some("'>'"), Some("'<='"), Some("'>='"), 
		None, None, None, None, Some("'.'"), Some("','"), Some("'/'"), Some("'-'"), 
		Some("'./'"), Some("'../'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;66]  = [
		None, Some("AUTHORIZE"), Some("CACHE"), Some("CALL"), Some("CHECK"), Some("COMPENSATE"), 
		Some("CONFIG"), Some("CONSUMES"), Some("CONTAINS"), Some("DESCRIPTION"), 
		Some("EACH"), Some("END"), Some("FALLBACK"), Some("FOR"), Some("FROM"), 
		Some("HANDLER"), Some("HEALTH"), Some("IMPLEMENTS"), Some("IMPORT"), Some("IN"), 
		Some("INTO"), Some("INVALIDATE_ON"), Some("IS"), Some("LOOKUP"), Some("NULL"), 
		Some("ON_ERROR"), Some("ON_ROLLBACK"), Some("PARALLEL"), Some("PERSIST"), 
		Some("PUBLISH"), Some("READY"), Some("RESPOND"), Some("SAGA"), Some("SCOPE"), 
		Some("SERVICE"), Some("STEP"), Some("TIMEOUT"), Some("TO"), Some("TRANSACTION"), 
		Some("TRANSFORM"), Some("TTL"), Some("USING"), Some("VALIDATE"), Some("WHEN"), 
		Some("WHERE"), Some("WITH"), Some("TRUE"), Some("FALSE"), Some("EQ"), 
		Some("NEQ"), Some("LT"), Some("GT"), Some("LTE"), Some("GTE"), Some("INTEGER"), 
		Some("DECIMAL_LITERAL"), Some("STRING_LITERAL"), Some("IDENTIFIER"), Some("DOT"), 
		Some("COMMA"), Some("SLASH"), Some("DASH"), Some("DOT_SLASH"), Some("DOT_DOT_SLASH"), 
		Some("LINE_COMMENT"), Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,ServiceDSLParserExt<'input>, I, ServiceDSLParserContextType , dyn ServiceDSLListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type ServiceDSLTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, ServiceDSLParserContextType , dyn ServiceDSLListener<'input> + 'a>;

/// Parser for ServiceDSL grammar
pub struct ServiceDSLParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				ServiceDSLParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> ServiceDSLParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> ServiceDSLParser<'input, I, DefaultErrorStrategy<'input,ServiceDSLParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for ServiceDSLParser
pub trait ServiceDSLParserContext<'input>:
	for<'x> Listenable<dyn ServiceDSLListener<'input> + 'x > + 
	for<'x> Visitable<dyn ServiceDSLVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=ServiceDSLParserContextType>
{}

antlr_rust::coerce_from!{ 'input : ServiceDSLParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn ServiceDSLParserContext<'input> + 'input
where
    T: ServiceDSLVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn ServiceDSLVisitor<'input> + 'x))
    }
}

impl<'input> ServiceDSLParserContext<'input> for TerminalNode<'input,ServiceDSLParserContextType> {}
impl<'input> ServiceDSLParserContext<'input> for ErrorNode<'input,ServiceDSLParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn ServiceDSLParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn ServiceDSLListener<'input> + 'input }

pub struct ServiceDSLParserContextType;
antlr_rust::tid!{ServiceDSLParserContextType}

impl<'input> ParserNodeType<'input> for ServiceDSLParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn ServiceDSLParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct ServiceDSLParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> ServiceDSLParserExt<'input>{
}
antlr_rust::tid! { ServiceDSLParserExt<'a> }

impl<'input> TokenAware<'input> for ServiceDSLParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for ServiceDSLParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for ServiceDSLParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "ServiceDSL.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- compilationUnit ----------------
pub type CompilationUnitContextAll<'input> = CompilationUnitContext<'input>;


pub type CompilationUnitContext<'input> = BaseParserRuleContext<'input,CompilationUnitContextExt<'input>>;

#[derive(Clone)]
pub struct CompilationUnitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for CompilationUnitContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for CompilationUnitContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_compilationUnit(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_compilationUnit(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for CompilationUnitContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_compilationUnit(self);
	}
}

impl<'input> CustomRuleContext<'input> for CompilationUnitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compilationUnit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compilationUnit }
}
antlr_rust::tid!{CompilationUnitContextExt<'a>}

impl<'input> CompilationUnitContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CompilationUnitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompilationUnitContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CompilationUnitContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<CompilationUnitContextExt<'input>>{

fn serviceDefinition(&self) -> Option<Rc<ServiceDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn importStatement_all(&self) ->  Vec<Rc<ImportStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn importStatement(&self, i: usize) -> Option<Rc<ImportStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> CompilationUnitContextAttrs<'input> for CompilationUnitContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compilationUnit(&mut self,)
	-> Result<Rc<CompilationUnitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CompilationUnitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_compilationUnit);
        let mut _localctx: Rc<CompilationUnitContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(107);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==IMPORT {
				{
				{
				/*InvokeRule importStatement*/
				recog.base.set_state(104);
				recog.importStatement()?;

				}
				}
				recog.base.set_state(109);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule serviceDefinition*/
			recog.base.set_state(110);
			recog.serviceDefinition()?;

			recog.base.set_state(111);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- importStatement ----------------
pub type ImportStatementContextAll<'input> = ImportStatementContext<'input>;


pub type ImportStatementContext<'input> = BaseParserRuleContext<'input,ImportStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ImportStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for ImportStatementContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for ImportStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_importStatement(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_importStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for ImportStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_importStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importStatement }
}
antlr_rust::tid!{ImportStatementContextExt<'a>}

impl<'input> ImportStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportStatementContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<ImportStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IMPORT
/// Returns `None` if there is no child corresponding to token IMPORT
fn IMPORT(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IMPORT, 0)
}
fn importPath(&self) -> Option<Rc<ImportPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ImportStatementContextAttrs<'input> for ImportStatementContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn importStatement(&mut self,)
	-> Result<Rc<ImportStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ImportStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_importStatement);
        let mut _localctx: Rc<ImportStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(113);
			recog.base.match_token(IMPORT,&mut recog.err_handler)?;

			/*InvokeRule importPath*/
			recog.base.set_state(114);
			recog.importPath()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- importPath ----------------
pub type ImportPathContextAll<'input> = ImportPathContext<'input>;


pub type ImportPathContext<'input> = BaseParserRuleContext<'input,ImportPathContextExt<'input>>;

#[derive(Clone)]
pub struct ImportPathContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for ImportPathContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for ImportPathContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_importPath(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_importPath(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for ImportPathContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_importPath(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportPathContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importPath }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importPath }
}
antlr_rust::tid!{ImportPathContextExt<'a>}

impl<'input> ImportPathContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportPathContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportPathContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportPathContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<ImportPathContextExt<'input>>{

fn pathSegment_all(&self) ->  Vec<Rc<PathSegmentContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn pathSegment(&self, i: usize) -> Option<Rc<PathSegmentContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}
fn word(&self) -> Option<Rc<WordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token SLASH in current rule
fn SLASH_all(&self) -> Vec<Rc<TerminalNode<'input,ServiceDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SLASH, starting from 0.
/// Returns `None` if number of children corresponding to token SLASH is less or equal than `i`.
fn SLASH(&self, i: usize) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(SLASH, i)
}
/// Retrieves first TerminalNode corresponding to token DOT_SLASH
/// Returns `None` if there is no child corresponding to token DOT_SLASH
fn DOT_SLASH(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT_SLASH, 0)
}
/// Retrieves first TerminalNode corresponding to token DOT_DOT_SLASH
/// Returns `None` if there is no child corresponding to token DOT_DOT_SLASH
fn DOT_DOT_SLASH(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT_DOT_SLASH, 0)
}

}

impl<'input> ImportPathContextAttrs<'input> for ImportPathContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn importPath(&mut self,)
	-> Result<Rc<ImportPathContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ImportPathContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_importPath);
        let mut _localctx: Rc<ImportPathContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(117);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==DOT_SLASH || _la==DOT_DOT_SLASH {
				{
				recog.base.set_state(116);
				_la = recog.base.input.la(1);
				if { !(_la==DOT_SLASH || _la==DOT_DOT_SLASH) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				}
			}

			/*InvokeRule pathSegment*/
			recog.base.set_state(119);
			recog.pathSegment()?;

			recog.base.set_state(124);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==SLASH {
				{
				{
				recog.base.set_state(120);
				recog.base.match_token(SLASH,&mut recog.err_handler)?;

				/*InvokeRule pathSegment*/
				recog.base.set_state(121);
				recog.pathSegment()?;

				}
				}
				recog.base.set_state(126);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(127);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			/*InvokeRule word*/
			recog.base.set_state(128);
			recog.word()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- pathSegment ----------------
pub type PathSegmentContextAll<'input> = PathSegmentContext<'input>;


pub type PathSegmentContext<'input> = BaseParserRuleContext<'input,PathSegmentContextExt<'input>>;

#[derive(Clone)]
pub struct PathSegmentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for PathSegmentContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for PathSegmentContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_pathSegment(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_pathSegment(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for PathSegmentContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_pathSegment(self);
	}
}

impl<'input> CustomRuleContext<'input> for PathSegmentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pathSegment }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pathSegment }
}
antlr_rust::tid!{PathSegmentContextExt<'a>}

impl<'input> PathSegmentContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PathSegmentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PathSegmentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PathSegmentContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<PathSegmentContextExt<'input>>{

fn word_all(&self) ->  Vec<Rc<WordContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn word(&self, i: usize) -> Option<Rc<WordContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token DASH in current rule
fn DASH_all(&self) -> Vec<Rc<TerminalNode<'input,ServiceDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DASH, starting from 0.
/// Returns `None` if number of children corresponding to token DASH is less or equal than `i`.
fn DASH(&self, i: usize) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(DASH, i)
}

}

impl<'input> PathSegmentContextAttrs<'input> for PathSegmentContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pathSegment(&mut self,)
	-> Result<Rc<PathSegmentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PathSegmentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_pathSegment);
        let mut _localctx: Rc<PathSegmentContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule word*/
			recog.base.set_state(130);
			recog.word()?;

			recog.base.set_state(135);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==DASH {
				{
				{
				recog.base.set_state(131);
				recog.base.match_token(DASH,&mut recog.err_handler)?;

				/*InvokeRule word*/
				recog.base.set_state(132);
				recog.word()?;

				}
				}
				recog.base.set_state(137);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- word ----------------
pub type WordContextAll<'input> = WordContext<'input>;


pub type WordContext<'input> = BaseParserRuleContext<'input,WordContextExt<'input>>;

#[derive(Clone)]
pub struct WordContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for WordContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for WordContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_word(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_word(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for WordContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_word(self);
	}
}

impl<'input> CustomRuleContext<'input> for WordContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_word }
	//fn type_rule_index() -> usize where Self: Sized { RULE_word }
}
antlr_rust::tid!{WordContextExt<'a>}

impl<'input> WordContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<WordContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,WordContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait WordContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<WordContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token AUTHORIZE
/// Returns `None` if there is no child corresponding to token AUTHORIZE
fn AUTHORIZE(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(AUTHORIZE, 0)
}
/// Retrieves first TerminalNode corresponding to token CACHE
/// Returns `None` if there is no child corresponding to token CACHE
fn CACHE(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(CACHE, 0)
}
/// Retrieves first TerminalNode corresponding to token CALL
/// Returns `None` if there is no child corresponding to token CALL
fn CALL(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(CALL, 0)
}
/// Retrieves first TerminalNode corresponding to token CHECK
/// Returns `None` if there is no child corresponding to token CHECK
fn CHECK(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(CHECK, 0)
}
/// Retrieves first TerminalNode corresponding to token COMPENSATE
/// Returns `None` if there is no child corresponding to token COMPENSATE
fn COMPENSATE(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(COMPENSATE, 0)
}
/// Retrieves first TerminalNode corresponding to token CONFIG
/// Returns `None` if there is no child corresponding to token CONFIG
fn CONFIG(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(CONFIG, 0)
}
/// Retrieves first TerminalNode corresponding to token CONSUMES
/// Returns `None` if there is no child corresponding to token CONSUMES
fn CONSUMES(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(CONSUMES, 0)
}
/// Retrieves first TerminalNode corresponding to token CONTAINS
/// Returns `None` if there is no child corresponding to token CONTAINS
fn CONTAINS(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(CONTAINS, 0)
}
/// Retrieves first TerminalNode corresponding to token DESCRIPTION
/// Returns `None` if there is no child corresponding to token DESCRIPTION
fn DESCRIPTION(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(DESCRIPTION, 0)
}
/// Retrieves first TerminalNode corresponding to token EACH
/// Returns `None` if there is no child corresponding to token EACH
fn EACH(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(EACH, 0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
/// Retrieves first TerminalNode corresponding to token FALLBACK
/// Returns `None` if there is no child corresponding to token FALLBACK
fn FALLBACK(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(FALLBACK, 0)
}
/// Retrieves first TerminalNode corresponding to token FOR
/// Returns `None` if there is no child corresponding to token FOR
fn FOR(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(FOR, 0)
}
/// Retrieves first TerminalNode corresponding to token FROM
/// Returns `None` if there is no child corresponding to token FROM
fn FROM(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(FROM, 0)
}
/// Retrieves first TerminalNode corresponding to token HANDLER
/// Returns `None` if there is no child corresponding to token HANDLER
fn HANDLER(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(HANDLER, 0)
}
/// Retrieves first TerminalNode corresponding to token HEALTH
/// Returns `None` if there is no child corresponding to token HEALTH
fn HEALTH(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(HEALTH, 0)
}
/// Retrieves first TerminalNode corresponding to token IMPLEMENTS
/// Returns `None` if there is no child corresponding to token IMPLEMENTS
fn IMPLEMENTS(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IMPLEMENTS, 0)
}
/// Retrieves first TerminalNode corresponding to token IMPORT
/// Returns `None` if there is no child corresponding to token IMPORT
fn IMPORT(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IMPORT, 0)
}
/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IN, 0)
}
/// Retrieves first TerminalNode corresponding to token INTO
/// Returns `None` if there is no child corresponding to token INTO
fn INTO(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(INTO, 0)
}
/// Retrieves first TerminalNode corresponding to token INVALIDATE_ON
/// Returns `None` if there is no child corresponding to token INVALIDATE_ON
fn INVALIDATE_ON(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(INVALIDATE_ON, 0)
}
/// Retrieves first TerminalNode corresponding to token IS
/// Returns `None` if there is no child corresponding to token IS
fn IS(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IS, 0)
}
/// Retrieves first TerminalNode corresponding to token LOOKUP
/// Returns `None` if there is no child corresponding to token LOOKUP
fn LOOKUP(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(LOOKUP, 0)
}
/// Retrieves first TerminalNode corresponding to token NULL
/// Returns `None` if there is no child corresponding to token NULL
fn NULL(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(NULL, 0)
}
/// Retrieves first TerminalNode corresponding to token ON_ERROR
/// Returns `None` if there is no child corresponding to token ON_ERROR
fn ON_ERROR(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(ON_ERROR, 0)
}
/// Retrieves first TerminalNode corresponding to token ON_ROLLBACK
/// Returns `None` if there is no child corresponding to token ON_ROLLBACK
fn ON_ROLLBACK(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(ON_ROLLBACK, 0)
}
/// Retrieves first TerminalNode corresponding to token PARALLEL
/// Returns `None` if there is no child corresponding to token PARALLEL
fn PARALLEL(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(PARALLEL, 0)
}
/// Retrieves first TerminalNode corresponding to token PERSIST
/// Returns `None` if there is no child corresponding to token PERSIST
fn PERSIST(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(PERSIST, 0)
}
/// Retrieves first TerminalNode corresponding to token PUBLISH
/// Returns `None` if there is no child corresponding to token PUBLISH
fn PUBLISH(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(PUBLISH, 0)
}
/// Retrieves first TerminalNode corresponding to token READY
/// Returns `None` if there is no child corresponding to token READY
fn READY(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(READY, 0)
}
/// Retrieves first TerminalNode corresponding to token RESPOND
/// Returns `None` if there is no child corresponding to token RESPOND
fn RESPOND(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(RESPOND, 0)
}
/// Retrieves first TerminalNode corresponding to token SAGA
/// Returns `None` if there is no child corresponding to token SAGA
fn SAGA(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(SAGA, 0)
}
/// Retrieves first TerminalNode corresponding to token SCOPE
/// Returns `None` if there is no child corresponding to token SCOPE
fn SCOPE(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(SCOPE, 0)
}
/// Retrieves first TerminalNode corresponding to token SERVICE
/// Returns `None` if there is no child corresponding to token SERVICE
fn SERVICE(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(SERVICE, 0)
}
/// Retrieves first TerminalNode corresponding to token STEP
/// Returns `None` if there is no child corresponding to token STEP
fn STEP(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(STEP, 0)
}
/// Retrieves first TerminalNode corresponding to token TIMEOUT
/// Returns `None` if there is no child corresponding to token TIMEOUT
fn TIMEOUT(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(TIMEOUT, 0)
}
/// Retrieves first TerminalNode corresponding to token TO
/// Returns `None` if there is no child corresponding to token TO
fn TO(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(TO, 0)
}
/// Retrieves first TerminalNode corresponding to token TRANSACTION
/// Returns `None` if there is no child corresponding to token TRANSACTION
fn TRANSACTION(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(TRANSACTION, 0)
}
/// Retrieves first TerminalNode corresponding to token TRANSFORM
/// Returns `None` if there is no child corresponding to token TRANSFORM
fn TRANSFORM(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(TRANSFORM, 0)
}
/// Retrieves first TerminalNode corresponding to token TTL
/// Returns `None` if there is no child corresponding to token TTL
fn TTL(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(TTL, 0)
}
/// Retrieves first TerminalNode corresponding to token USING
/// Returns `None` if there is no child corresponding to token USING
fn USING(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(USING, 0)
}
/// Retrieves first TerminalNode corresponding to token VALIDATE
/// Returns `None` if there is no child corresponding to token VALIDATE
fn VALIDATE(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(VALIDATE, 0)
}
/// Retrieves first TerminalNode corresponding to token WHEN
/// Returns `None` if there is no child corresponding to token WHEN
fn WHEN(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(WHEN, 0)
}
/// Retrieves first TerminalNode corresponding to token WHERE
/// Returns `None` if there is no child corresponding to token WHERE
fn WHERE(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(WHERE, 0)
}
/// Retrieves first TerminalNode corresponding to token WITH
/// Returns `None` if there is no child corresponding to token WITH
fn WITH(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(WITH, 0)
}

}

impl<'input> WordContextAttrs<'input> for WordContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn word(&mut self,)
	-> Result<Rc<WordContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = WordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_word);
        let mut _localctx: Rc<WordContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(138);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << AUTHORIZE) | (1usize << CACHE) | (1usize << CALL) | (1usize << CHECK) | (1usize << COMPENSATE) | (1usize << CONFIG) | (1usize << CONSUMES) | (1usize << CONTAINS) | (1usize << DESCRIPTION) | (1usize << EACH) | (1usize << END) | (1usize << FALLBACK) | (1usize << FOR) | (1usize << FROM) | (1usize << HANDLER) | (1usize << HEALTH) | (1usize << IMPLEMENTS) | (1usize << IMPORT) | (1usize << IN) | (1usize << INTO) | (1usize << INVALIDATE_ON) | (1usize << IS) | (1usize << LOOKUP) | (1usize << NULL) | (1usize << ON_ERROR) | (1usize << ON_ROLLBACK) | (1usize << PARALLEL) | (1usize << PERSIST) | (1usize << PUBLISH) | (1usize << READY) | (1usize << RESPOND))) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (SAGA - 32)) | (1usize << (SCOPE - 32)) | (1usize << (SERVICE - 32)) | (1usize << (STEP - 32)) | (1usize << (TIMEOUT - 32)) | (1usize << (TO - 32)) | (1usize << (TRANSACTION - 32)) | (1usize << (TRANSFORM - 32)) | (1usize << (TTL - 32)) | (1usize << (USING - 32)) | (1usize << (VALIDATE - 32)) | (1usize << (WHEN - 32)) | (1usize << (WHERE - 32)) | (1usize << (WITH - 32)) | (1usize << (IDENTIFIER - 32)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- serviceDefinition ----------------
pub type ServiceDefinitionContextAll<'input> = ServiceDefinitionContext<'input>;


pub type ServiceDefinitionContext<'input> = BaseParserRuleContext<'input,ServiceDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct ServiceDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for ServiceDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for ServiceDefinitionContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_serviceDefinition(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_serviceDefinition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for ServiceDefinitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_serviceDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for ServiceDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_serviceDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_serviceDefinition }
}
antlr_rust::tid!{ServiceDefinitionContextExt<'a>}

impl<'input> ServiceDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ServiceDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ServiceDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ServiceDefinitionContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<ServiceDefinitionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SERVICE
/// Returns `None` if there is no child corresponding to token SERVICE
fn SERVICE(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(SERVICE, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn serviceBody(&self) -> Option<Rc<ServiceBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}

}

impl<'input> ServiceDefinitionContextAttrs<'input> for ServiceDefinitionContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn serviceDefinition(&mut self,)
	-> Result<Rc<ServiceDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ServiceDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_serviceDefinition);
        let mut _localctx: Rc<ServiceDefinitionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(140);
			recog.base.match_token(SERVICE,&mut recog.err_handler)?;

			recog.base.set_state(141);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			/*InvokeRule serviceBody*/
			recog.base.set_state(142);
			recog.serviceBody()?;

			recog.base.set_state(143);
			recog.base.match_token(END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- serviceBody ----------------
pub type ServiceBodyContextAll<'input> = ServiceBodyContext<'input>;


pub type ServiceBodyContext<'input> = BaseParserRuleContext<'input,ServiceBodyContextExt<'input>>;

#[derive(Clone)]
pub struct ServiceBodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for ServiceBodyContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for ServiceBodyContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_serviceBody(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_serviceBody(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for ServiceBodyContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_serviceBody(self);
	}
}

impl<'input> CustomRuleContext<'input> for ServiceBodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_serviceBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_serviceBody }
}
antlr_rust::tid!{ServiceBodyContextExt<'a>}

impl<'input> ServiceBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ServiceBodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ServiceBodyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ServiceBodyContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<ServiceBodyContextExt<'input>>{

fn serviceElement_all(&self) ->  Vec<Rc<ServiceElementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn serviceElement(&self, i: usize) -> Option<Rc<ServiceElementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ServiceBodyContextAttrs<'input> for ServiceBodyContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn serviceBody(&mut self,)
	-> Result<Rc<ServiceBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ServiceBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_serviceBody);
        let mut _localctx: Rc<ServiceBodyContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(148);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << CACHE) | (1usize << CONFIG) | (1usize << CONSUMES) | (1usize << DESCRIPTION) | (1usize << HANDLER) | (1usize << HEALTH) | (1usize << IMPLEMENTS) | (1usize << READY))) != 0) {
				{
				{
				/*InvokeRule serviceElement*/
				recog.base.set_state(145);
				recog.serviceElement()?;

				}
				}
				recog.base.set_state(150);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- serviceElement ----------------
pub type ServiceElementContextAll<'input> = ServiceElementContext<'input>;


pub type ServiceElementContext<'input> = BaseParserRuleContext<'input,ServiceElementContextExt<'input>>;

#[derive(Clone)]
pub struct ServiceElementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for ServiceElementContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for ServiceElementContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_serviceElement(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_serviceElement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for ServiceElementContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_serviceElement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ServiceElementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_serviceElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_serviceElement }
}
antlr_rust::tid!{ServiceElementContextExt<'a>}

impl<'input> ServiceElementContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ServiceElementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ServiceElementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ServiceElementContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<ServiceElementContextExt<'input>>{

fn descriptionDecl(&self) -> Option<Rc<DescriptionDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn implementsDecl(&self) -> Option<Rc<ImplementsDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn consumesDecl(&self) -> Option<Rc<ConsumesDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn configBlock(&self) -> Option<Rc<ConfigBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn handlerDecl(&self) -> Option<Rc<HandlerDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn cacheBlock(&self) -> Option<Rc<CacheBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn healthBlock(&self) -> Option<Rc<HealthBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn readyDecl(&self) -> Option<Rc<ReadyDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ServiceElementContextAttrs<'input> for ServiceElementContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn serviceElement(&mut self,)
	-> Result<Rc<ServiceElementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ServiceElementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_serviceElement);
        let mut _localctx: Rc<ServiceElementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(159);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 DESCRIPTION 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule descriptionDecl*/
					recog.base.set_state(151);
					recog.descriptionDecl()?;

					}
				}

			 IMPLEMENTS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule implementsDecl*/
					recog.base.set_state(152);
					recog.implementsDecl()?;

					}
				}

			 CONSUMES 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule consumesDecl*/
					recog.base.set_state(153);
					recog.consumesDecl()?;

					}
				}

			 CONFIG 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule configBlock*/
					recog.base.set_state(154);
					recog.configBlock()?;

					}
				}

			 HANDLER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule handlerDecl*/
					recog.base.set_state(155);
					recog.handlerDecl()?;

					}
				}

			 CACHE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule cacheBlock*/
					recog.base.set_state(156);
					recog.cacheBlock()?;

					}
				}

			 HEALTH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule healthBlock*/
					recog.base.set_state(157);
					recog.healthBlock()?;

					}
				}

			 READY 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule readyDecl*/
					recog.base.set_state(158);
					recog.readyDecl()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- descriptionDecl ----------------
pub type DescriptionDeclContextAll<'input> = DescriptionDeclContext<'input>;


pub type DescriptionDeclContext<'input> = BaseParserRuleContext<'input,DescriptionDeclContextExt<'input>>;

#[derive(Clone)]
pub struct DescriptionDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for DescriptionDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for DescriptionDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_descriptionDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_descriptionDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for DescriptionDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_descriptionDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for DescriptionDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_descriptionDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_descriptionDecl }
}
antlr_rust::tid!{DescriptionDeclContextExt<'a>}

impl<'input> DescriptionDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DescriptionDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DescriptionDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DescriptionDeclContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<DescriptionDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DESCRIPTION
/// Returns `None` if there is no child corresponding to token DESCRIPTION
fn DESCRIPTION(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(DESCRIPTION, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}

}

impl<'input> DescriptionDeclContextAttrs<'input> for DescriptionDeclContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn descriptionDecl(&mut self,)
	-> Result<Rc<DescriptionDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DescriptionDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_descriptionDecl);
        let mut _localctx: Rc<DescriptionDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(161);
			recog.base.match_token(DESCRIPTION,&mut recog.err_handler)?;

			recog.base.set_state(162);
			recog.base.match_token(STRING_LITERAL,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- implementsDecl ----------------
pub type ImplementsDeclContextAll<'input> = ImplementsDeclContext<'input>;


pub type ImplementsDeclContext<'input> = BaseParserRuleContext<'input,ImplementsDeclContextExt<'input>>;

#[derive(Clone)]
pub struct ImplementsDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for ImplementsDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for ImplementsDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_implementsDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_implementsDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for ImplementsDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_implementsDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImplementsDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_implementsDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_implementsDecl }
}
antlr_rust::tid!{ImplementsDeclContextExt<'a>}

impl<'input> ImplementsDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImplementsDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImplementsDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImplementsDeclContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<ImplementsDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IMPLEMENTS
/// Returns `None` if there is no child corresponding to token IMPLEMENTS
fn IMPLEMENTS(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IMPLEMENTS, 0)
}
fn identifierList(&self) -> Option<Rc<IdentifierListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ImplementsDeclContextAttrs<'input> for ImplementsDeclContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn implementsDecl(&mut self,)
	-> Result<Rc<ImplementsDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ImplementsDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_implementsDecl);
        let mut _localctx: Rc<ImplementsDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(164);
			recog.base.match_token(IMPLEMENTS,&mut recog.err_handler)?;

			/*InvokeRule identifierList*/
			recog.base.set_state(165);
			recog.identifierList()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- consumesDecl ----------------
pub type ConsumesDeclContextAll<'input> = ConsumesDeclContext<'input>;


pub type ConsumesDeclContext<'input> = BaseParserRuleContext<'input,ConsumesDeclContextExt<'input>>;

#[derive(Clone)]
pub struct ConsumesDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for ConsumesDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for ConsumesDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_consumesDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_consumesDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for ConsumesDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_consumesDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConsumesDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_consumesDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_consumesDecl }
}
antlr_rust::tid!{ConsumesDeclContextExt<'a>}

impl<'input> ConsumesDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConsumesDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConsumesDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConsumesDeclContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<ConsumesDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CONSUMES
/// Returns `None` if there is no child corresponding to token CONSUMES
fn CONSUMES(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(CONSUMES, 0)
}
fn identifierList(&self) -> Option<Rc<IdentifierListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ConsumesDeclContextAttrs<'input> for ConsumesDeclContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn consumesDecl(&mut self,)
	-> Result<Rc<ConsumesDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConsumesDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_consumesDecl);
        let mut _localctx: Rc<ConsumesDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(167);
			recog.base.match_token(CONSUMES,&mut recog.err_handler)?;

			/*InvokeRule identifierList*/
			recog.base.set_state(168);
			recog.identifierList()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- configBlock ----------------
pub type ConfigBlockContextAll<'input> = ConfigBlockContext<'input>;


pub type ConfigBlockContext<'input> = BaseParserRuleContext<'input,ConfigBlockContextExt<'input>>;

#[derive(Clone)]
pub struct ConfigBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for ConfigBlockContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for ConfigBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_configBlock(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_configBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for ConfigBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_configBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConfigBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_configBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_configBlock }
}
antlr_rust::tid!{ConfigBlockContextExt<'a>}

impl<'input> ConfigBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConfigBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConfigBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConfigBlockContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<ConfigBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CONFIG
/// Returns `None` if there is no child corresponding to token CONFIG
fn CONFIG(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(CONFIG, 0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
fn configDirective_all(&self) ->  Vec<Rc<ConfigDirectiveContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn configDirective(&self, i: usize) -> Option<Rc<ConfigDirectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ConfigBlockContextAttrs<'input> for ConfigBlockContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn configBlock(&mut self,)
	-> Result<Rc<ConfigBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConfigBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_configBlock);
        let mut _localctx: Rc<ConfigBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(170);
			recog.base.match_token(CONFIG,&mut recog.err_handler)?;

			recog.base.set_state(174);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==IDENTIFIER {
				{
				{
				/*InvokeRule configDirective*/
				recog.base.set_state(171);
				recog.configDirective()?;

				}
				}
				recog.base.set_state(176);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(177);
			recog.base.match_token(END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- configDirective ----------------
pub type ConfigDirectiveContextAll<'input> = ConfigDirectiveContext<'input>;


pub type ConfigDirectiveContext<'input> = BaseParserRuleContext<'input,ConfigDirectiveContextExt<'input>>;

#[derive(Clone)]
pub struct ConfigDirectiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for ConfigDirectiveContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for ConfigDirectiveContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_configDirective(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_configDirective(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for ConfigDirectiveContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_configDirective(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConfigDirectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_configDirective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_configDirective }
}
antlr_rust::tid!{ConfigDirectiveContextExt<'a>}

impl<'input> ConfigDirectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConfigDirectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConfigDirectiveContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConfigDirectiveContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<ConfigDirectiveContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn configValue_all(&self) ->  Vec<Rc<ConfigValueContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn configValue(&self, i: usize) -> Option<Rc<ConfigValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,ServiceDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ConfigDirectiveContextAttrs<'input> for ConfigDirectiveContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn configDirective(&mut self,)
	-> Result<Rc<ConfigDirectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConfigDirectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_configDirective);
        let mut _localctx: Rc<ConfigDirectiveContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(179);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			/*InvokeRule configValue*/
			recog.base.set_state(180);
			recog.configValue()?;

			recog.base.set_state(185);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(181);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule configValue*/
				recog.base.set_state(182);
				recog.configValue()?;

				}
				}
				recog.base.set_state(187);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- configValue ----------------
pub type ConfigValueContextAll<'input> = ConfigValueContext<'input>;


pub type ConfigValueContext<'input> = BaseParserRuleContext<'input,ConfigValueContextExt<'input>>;

#[derive(Clone)]
pub struct ConfigValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for ConfigValueContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for ConfigValueContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_configValue(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_configValue(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for ConfigValueContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_configValue(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConfigValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_configValue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_configValue }
}
antlr_rust::tid!{ConfigValueContextExt<'a>}

impl<'input> ConfigValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConfigValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConfigValueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConfigValueContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<ConfigValueContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
/// Retrieves first TerminalNode corresponding to token DECIMAL_LITERAL
/// Returns `None` if there is no child corresponding to token DECIMAL_LITERAL
fn DECIMAL_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(DECIMAL_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token TRUE
/// Returns `None` if there is no child corresponding to token TRUE
fn TRUE(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(TRUE, 0)
}
/// Retrieves first TerminalNode corresponding to token FALSE
/// Returns `None` if there is no child corresponding to token FALSE
fn FALSE(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(FALSE, 0)
}

}

impl<'input> ConfigValueContextAttrs<'input> for ConfigValueContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn configValue(&mut self,)
	-> Result<Rc<ConfigValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConfigValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_configValue);
        let mut _localctx: Rc<ConfigValueContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(188);
			_la = recog.base.input.la(1);
			if { !(((((_la - 46)) & !0x3f) == 0 && ((1usize << (_la - 46)) & ((1usize << (TRUE - 46)) | (1usize << (FALSE - 46)) | (1usize << (INTEGER - 46)) | (1usize << (DECIMAL_LITERAL - 46)) | (1usize << (STRING_LITERAL - 46)) | (1usize << (IDENTIFIER - 46)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- handlerDecl ----------------
pub type HandlerDeclContextAll<'input> = HandlerDeclContext<'input>;


pub type HandlerDeclContext<'input> = BaseParserRuleContext<'input,HandlerDeclContextExt<'input>>;

#[derive(Clone)]
pub struct HandlerDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for HandlerDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for HandlerDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_handlerDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_handlerDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for HandlerDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_handlerDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for HandlerDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_handlerDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_handlerDecl }
}
antlr_rust::tid!{HandlerDeclContextExt<'a>}

impl<'input> HandlerDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<HandlerDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HandlerDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait HandlerDeclContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<HandlerDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token HANDLER
/// Returns `None` if there is no child corresponding to token HANDLER
fn HANDLER(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(HANDLER, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn handlerBody(&self) -> Option<Rc<HandlerBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}

}

impl<'input> HandlerDeclContextAttrs<'input> for HandlerDeclContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn handlerDecl(&mut self,)
	-> Result<Rc<HandlerDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HandlerDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_handlerDecl);
        let mut _localctx: Rc<HandlerDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(190);
			recog.base.match_token(HANDLER,&mut recog.err_handler)?;

			recog.base.set_state(191);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			/*InvokeRule handlerBody*/
			recog.base.set_state(192);
			recog.handlerBody()?;

			recog.base.set_state(193);
			recog.base.match_token(END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- handlerBody ----------------
pub type HandlerBodyContextAll<'input> = HandlerBodyContext<'input>;


pub type HandlerBodyContext<'input> = BaseParserRuleContext<'input,HandlerBodyContextExt<'input>>;

#[derive(Clone)]
pub struct HandlerBodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for HandlerBodyContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for HandlerBodyContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_handlerBody(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_handlerBody(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for HandlerBodyContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_handlerBody(self);
	}
}

impl<'input> CustomRuleContext<'input> for HandlerBodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_handlerBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_handlerBody }
}
antlr_rust::tid!{HandlerBodyContextExt<'a>}

impl<'input> HandlerBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<HandlerBodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HandlerBodyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait HandlerBodyContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<HandlerBodyContextExt<'input>>{

fn handlerStatement_all(&self) ->  Vec<Rc<HandlerStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn handlerStatement(&self, i: usize) -> Option<Rc<HandlerStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> HandlerBodyContextAttrs<'input> for HandlerBodyContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn handlerBody(&mut self,)
	-> Result<Rc<HandlerBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HandlerBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_handlerBody);
        let mut _localctx: Rc<HandlerBodyContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(198);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << AUTHORIZE) | (1usize << CALL) | (1usize << LOOKUP) | (1usize << ON_ERROR) | (1usize << PARALLEL) | (1usize << PERSIST) | (1usize << PUBLISH) | (1usize << RESPOND))) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (SAGA - 32)) | (1usize << (TRANSACTION - 32)) | (1usize << (TRANSFORM - 32)) | (1usize << (VALIDATE - 32)))) != 0) {
				{
				{
				/*InvokeRule handlerStatement*/
				recog.base.set_state(195);
				recog.handlerStatement()?;

				}
				}
				recog.base.set_state(200);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- handlerStatement ----------------
pub type HandlerStatementContextAll<'input> = HandlerStatementContext<'input>;


pub type HandlerStatementContext<'input> = BaseParserRuleContext<'input,HandlerStatementContextExt<'input>>;

#[derive(Clone)]
pub struct HandlerStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for HandlerStatementContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for HandlerStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_handlerStatement(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_handlerStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for HandlerStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_handlerStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for HandlerStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_handlerStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_handlerStatement }
}
antlr_rust::tid!{HandlerStatementContextExt<'a>}

impl<'input> HandlerStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<HandlerStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HandlerStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait HandlerStatementContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<HandlerStatementContextExt<'input>>{

fn authorizeStmt(&self) -> Option<Rc<AuthorizeStmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn validateStmt(&self) -> Option<Rc<ValidateStmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lookupStmt(&self) -> Option<Rc<LookupStmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn transformStmt(&self) -> Option<Rc<TransformStmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn persistStmt(&self) -> Option<Rc<PersistStmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn callStmt(&self) -> Option<Rc<CallStmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn publishStmt(&self) -> Option<Rc<PublishStmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn respondStmt(&self) -> Option<Rc<RespondStmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn onErrorBlock(&self) -> Option<Rc<OnErrorBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn transactionBlock(&self) -> Option<Rc<TransactionBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn sagaBlock(&self) -> Option<Rc<SagaBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn parallelBlock(&self) -> Option<Rc<ParallelBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> HandlerStatementContextAttrs<'input> for HandlerStatementContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn handlerStatement(&mut self,)
	-> Result<Rc<HandlerStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HandlerStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_handlerStatement);
        let mut _localctx: Rc<HandlerStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(213);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 AUTHORIZE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule authorizeStmt*/
					recog.base.set_state(201);
					recog.authorizeStmt()?;

					}
				}

			 VALIDATE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule validateStmt*/
					recog.base.set_state(202);
					recog.validateStmt()?;

					}
				}

			 LOOKUP 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule lookupStmt*/
					recog.base.set_state(203);
					recog.lookupStmt()?;

					}
				}

			 TRANSFORM 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule transformStmt*/
					recog.base.set_state(204);
					recog.transformStmt()?;

					}
				}

			 PERSIST 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule persistStmt*/
					recog.base.set_state(205);
					recog.persistStmt()?;

					}
				}

			 CALL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule callStmt*/
					recog.base.set_state(206);
					recog.callStmt()?;

					}
				}

			 PUBLISH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule publishStmt*/
					recog.base.set_state(207);
					recog.publishStmt()?;

					}
				}

			 RESPOND 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule respondStmt*/
					recog.base.set_state(208);
					recog.respondStmt()?;

					}
				}

			 ON_ERROR 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					/*InvokeRule onErrorBlock*/
					recog.base.set_state(209);
					recog.onErrorBlock()?;

					}
				}

			 TRANSACTION 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 10);
					recog.base.enter_outer_alt(None, 10);
					{
					/*InvokeRule transactionBlock*/
					recog.base.set_state(210);
					recog.transactionBlock()?;

					}
				}

			 SAGA 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 11);
					recog.base.enter_outer_alt(None, 11);
					{
					/*InvokeRule sagaBlock*/
					recog.base.set_state(211);
					recog.sagaBlock()?;

					}
				}

			 PARALLEL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 12);
					recog.base.enter_outer_alt(None, 12);
					{
					/*InvokeRule parallelBlock*/
					recog.base.set_state(212);
					recog.parallelBlock()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- authorizeStmt ----------------
pub type AuthorizeStmtContextAll<'input> = AuthorizeStmtContext<'input>;


pub type AuthorizeStmtContext<'input> = BaseParserRuleContext<'input,AuthorizeStmtContextExt<'input>>;

#[derive(Clone)]
pub struct AuthorizeStmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for AuthorizeStmtContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for AuthorizeStmtContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_authorizeStmt(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_authorizeStmt(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for AuthorizeStmtContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_authorizeStmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for AuthorizeStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_authorizeStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_authorizeStmt }
}
antlr_rust::tid!{AuthorizeStmtContextExt<'a>}

impl<'input> AuthorizeStmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AuthorizeStmtContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AuthorizeStmtContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AuthorizeStmtContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<AuthorizeStmtContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token AUTHORIZE
/// Returns `None` if there is no child corresponding to token AUTHORIZE
fn AUTHORIZE(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(AUTHORIZE, 0)
}
/// Retrieves first TerminalNode corresponding to token SCOPE
/// Returns `None` if there is no child corresponding to token SCOPE
fn SCOPE(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(SCOPE, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}

}

impl<'input> AuthorizeStmtContextAttrs<'input> for AuthorizeStmtContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn authorizeStmt(&mut self,)
	-> Result<Rc<AuthorizeStmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AuthorizeStmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_authorizeStmt);
        let mut _localctx: Rc<AuthorizeStmtContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(215);
			recog.base.match_token(AUTHORIZE,&mut recog.err_handler)?;

			recog.base.set_state(216);
			recog.base.match_token(SCOPE,&mut recog.err_handler)?;

			recog.base.set_state(217);
			recog.base.match_token(STRING_LITERAL,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- validateStmt ----------------
pub type ValidateStmtContextAll<'input> = ValidateStmtContext<'input>;


pub type ValidateStmtContext<'input> = BaseParserRuleContext<'input,ValidateStmtContextExt<'input>>;

#[derive(Clone)]
pub struct ValidateStmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for ValidateStmtContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for ValidateStmtContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_validateStmt(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_validateStmt(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for ValidateStmtContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_validateStmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValidateStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_validateStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_validateStmt }
}
antlr_rust::tid!{ValidateStmtContextExt<'a>}

impl<'input> ValidateStmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValidateStmtContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValidateStmtContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ValidateStmtContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<ValidateStmtContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token VALIDATE
/// Returns `None` if there is no child corresponding to token VALIDATE
fn VALIDATE(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(VALIDATE, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token USING
/// Returns `None` if there is no child corresponding to token USING
fn USING(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(USING, 0)
}
fn qualifiedRef(&self) -> Option<Rc<QualifiedRefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ValidateStmtContextAttrs<'input> for ValidateStmtContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn validateStmt(&mut self,)
	-> Result<Rc<ValidateStmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValidateStmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_validateStmt);
        let mut _localctx: Rc<ValidateStmtContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(219);
			recog.base.match_token(VALIDATE,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(220);
			recog.expression()?;

			recog.base.set_state(221);
			recog.base.match_token(USING,&mut recog.err_handler)?;

			/*InvokeRule qualifiedRef*/
			recog.base.set_state(222);
			recog.qualifiedRef()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- lookupStmt ----------------
pub type LookupStmtContextAll<'input> = LookupStmtContext<'input>;


pub type LookupStmtContext<'input> = BaseParserRuleContext<'input,LookupStmtContextExt<'input>>;

#[derive(Clone)]
pub struct LookupStmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for LookupStmtContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for LookupStmtContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lookupStmt(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_lookupStmt(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for LookupStmtContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_lookupStmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for LookupStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lookupStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lookupStmt }
}
antlr_rust::tid!{LookupStmtContextExt<'a>}

impl<'input> LookupStmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LookupStmtContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LookupStmtContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LookupStmtContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<LookupStmtContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LOOKUP
/// Returns `None` if there is no child corresponding to token LOOKUP
fn LOOKUP(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(LOOKUP, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,ServiceDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves first TerminalNode corresponding to token FROM
/// Returns `None` if there is no child corresponding to token FROM
fn FROM(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(FROM, 0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
fn whereClause_all(&self) ->  Vec<Rc<WhereClauseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn whereClause(&self, i: usize) -> Option<Rc<WhereClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> LookupStmtContextAttrs<'input> for LookupStmtContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lookupStmt(&mut self,)
	-> Result<Rc<LookupStmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LookupStmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_lookupStmt);
        let mut _localctx: Rc<LookupStmtContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(224);
			recog.base.match_token(LOOKUP,&mut recog.err_handler)?;

			recog.base.set_state(225);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(226);
			recog.base.match_token(FROM,&mut recog.err_handler)?;

			recog.base.set_state(227);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(231);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==WHERE {
				{
				{
				/*InvokeRule whereClause*/
				recog.base.set_state(228);
				recog.whereClause()?;

				}
				}
				recog.base.set_state(233);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(234);
			recog.base.match_token(END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- whereClause ----------------
pub type WhereClauseContextAll<'input> = WhereClauseContext<'input>;


pub type WhereClauseContext<'input> = BaseParserRuleContext<'input,WhereClauseContextExt<'input>>;

#[derive(Clone)]
pub struct WhereClauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for WhereClauseContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for WhereClauseContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_whereClause(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_whereClause(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for WhereClauseContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_whereClause(self);
	}
}

impl<'input> CustomRuleContext<'input> for WhereClauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_whereClause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_whereClause }
}
antlr_rust::tid!{WhereClauseContextExt<'a>}

impl<'input> WhereClauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<WhereClauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,WhereClauseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait WhereClauseContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<WhereClauseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token WHERE
/// Returns `None` if there is no child corresponding to token WHERE
fn WHERE(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(WHERE, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn comparator(&self) -> Option<Rc<ComparatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> WhereClauseContextAttrs<'input> for WhereClauseContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn whereClause(&mut self,)
	-> Result<Rc<WhereClauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = WhereClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_whereClause);
        let mut _localctx: Rc<WhereClauseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(236);
			recog.base.match_token(WHERE,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(237);
			recog.expression()?;

			/*InvokeRule comparator*/
			recog.base.set_state(238);
			recog.comparator()?;

			/*InvokeRule expression*/
			recog.base.set_state(239);
			recog.expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- transformStmt ----------------
pub type TransformStmtContextAll<'input> = TransformStmtContext<'input>;


pub type TransformStmtContext<'input> = BaseParserRuleContext<'input,TransformStmtContextExt<'input>>;

#[derive(Clone)]
pub struct TransformStmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for TransformStmtContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for TransformStmtContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_transformStmt(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_transformStmt(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for TransformStmtContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_transformStmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for TransformStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_transformStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_transformStmt }
}
antlr_rust::tid!{TransformStmtContextExt<'a>}

impl<'input> TransformStmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TransformStmtContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TransformStmtContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TransformStmtContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<TransformStmtContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TRANSFORM
/// Returns `None` if there is no child corresponding to token TRANSFORM
fn TRANSFORM(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(TRANSFORM, 0)
}
fn expressionList(&self) -> Option<Rc<ExpressionListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token USING
/// Returns `None` if there is no child corresponding to token USING
fn USING(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(USING, 0)
}
fn qualifiedRef(&self) -> Option<Rc<QualifiedRefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token INTO
/// Returns `None` if there is no child corresponding to token INTO
fn INTO(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(INTO, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> TransformStmtContextAttrs<'input> for TransformStmtContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn transformStmt(&mut self,)
	-> Result<Rc<TransformStmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TransformStmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_transformStmt);
        let mut _localctx: Rc<TransformStmtContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(254);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(11,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(241);
					recog.base.match_token(TRANSFORM,&mut recog.err_handler)?;

					/*InvokeRule expressionList*/
					recog.base.set_state(242);
					recog.expressionList()?;

					recog.base.set_state(243);
					recog.base.match_token(USING,&mut recog.err_handler)?;

					/*InvokeRule qualifiedRef*/
					recog.base.set_state(244);
					recog.qualifiedRef()?;

					recog.base.set_state(245);
					recog.base.match_token(INTO,&mut recog.err_handler)?;

					recog.base.set_state(246);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(248);
					recog.base.match_token(TRANSFORM,&mut recog.err_handler)?;

					recog.base.set_state(249);
					recog.base.match_token(USING,&mut recog.err_handler)?;

					/*InvokeRule qualifiedRef*/
					recog.base.set_state(250);
					recog.qualifiedRef()?;

					recog.base.set_state(251);
					recog.base.match_token(INTO,&mut recog.err_handler)?;

					recog.base.set_state(252);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- persistStmt ----------------
pub type PersistStmtContextAll<'input> = PersistStmtContext<'input>;


pub type PersistStmtContext<'input> = BaseParserRuleContext<'input,PersistStmtContextExt<'input>>;

#[derive(Clone)]
pub struct PersistStmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for PersistStmtContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for PersistStmtContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_persistStmt(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_persistStmt(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for PersistStmtContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_persistStmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for PersistStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_persistStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_persistStmt }
}
antlr_rust::tid!{PersistStmtContextExt<'a>}

impl<'input> PersistStmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PersistStmtContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PersistStmtContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PersistStmtContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<PersistStmtContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PERSIST
/// Returns `None` if there is no child corresponding to token PERSIST
fn PERSIST(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(PERSIST, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token TO
/// Returns `None` if there is no child corresponding to token TO
fn TO(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(TO, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token EQ
/// Returns `None` if there is no child corresponding to token EQ
fn EQ(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(EQ, 0)
}

}

impl<'input> PersistStmtContextAttrs<'input> for PersistStmtContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn persistStmt(&mut self,)
	-> Result<Rc<PersistStmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PersistStmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_persistStmt);
        let mut _localctx: Rc<PersistStmtContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(268);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(12,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(256);
					recog.base.match_token(PERSIST,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(257);
					recog.expression()?;

					recog.base.set_state(258);
					recog.base.match_token(TO,&mut recog.err_handler)?;

					recog.base.set_state(259);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(261);
					recog.base.match_token(PERSIST,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(262);
					recog.expression()?;

					recog.base.set_state(263);
					recog.base.match_token(EQ,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(264);
					recog.expression()?;

					recog.base.set_state(265);
					recog.base.match_token(TO,&mut recog.err_handler)?;

					recog.base.set_state(266);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- callStmt ----------------
pub type CallStmtContextAll<'input> = CallStmtContext<'input>;


pub type CallStmtContext<'input> = BaseParserRuleContext<'input,CallStmtContextExt<'input>>;

#[derive(Clone)]
pub struct CallStmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for CallStmtContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for CallStmtContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_callStmt(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_callStmt(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for CallStmtContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_callStmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for CallStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_callStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_callStmt }
}
antlr_rust::tid!{CallStmtContextExt<'a>}

impl<'input> CallStmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CallStmtContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CallStmtContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CallStmtContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<CallStmtContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CALL
/// Returns `None` if there is no child corresponding to token CALL
fn CALL(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(CALL, 0)
}
fn qualifiedRef(&self) -> Option<Rc<QualifiedRefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn withClause_all(&self) ->  Vec<Rc<WithClauseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn withClause(&self, i: usize) -> Option<Rc<WithClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn forEachClause(&self) -> Option<Rc<ForEachClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn intoClause(&self) -> Option<Rc<IntoClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}

}

impl<'input> CallStmtContextAttrs<'input> for CallStmtContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn callStmt(&mut self,)
	-> Result<Rc<CallStmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CallStmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_callStmt);
        let mut _localctx: Rc<CallStmtContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(270);
			recog.base.match_token(CALL,&mut recog.err_handler)?;

			/*InvokeRule qualifiedRef*/
			recog.base.set_state(271);
			recog.qualifiedRef()?;

			recog.base.set_state(275);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==WITH {
				{
				{
				/*InvokeRule withClause*/
				recog.base.set_state(272);
				recog.withClause()?;

				}
				}
				recog.base.set_state(277);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(279);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FOR {
				{
				/*InvokeRule forEachClause*/
				recog.base.set_state(278);
				recog.forEachClause()?;

				}
			}

			recog.base.set_state(282);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==INTO {
				{
				/*InvokeRule intoClause*/
				recog.base.set_state(281);
				recog.intoClause()?;

				}
			}

			recog.base.set_state(285);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(16,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(284);
					recog.base.match_token(END,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- withClause ----------------
pub type WithClauseContextAll<'input> = WithClauseContext<'input>;


pub type WithClauseContext<'input> = BaseParserRuleContext<'input,WithClauseContextExt<'input>>;

#[derive(Clone)]
pub struct WithClauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for WithClauseContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for WithClauseContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_withClause(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_withClause(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for WithClauseContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_withClause(self);
	}
}

impl<'input> CustomRuleContext<'input> for WithClauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_withClause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_withClause }
}
antlr_rust::tid!{WithClauseContextExt<'a>}

impl<'input> WithClauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<WithClauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,WithClauseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait WithClauseContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<WithClauseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token WITH
/// Returns `None` if there is no child corresponding to token WITH
fn WITH(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(WITH, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token EQ
/// Returns `None` if there is no child corresponding to token EQ
fn EQ(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(EQ, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}

}

impl<'input> WithClauseContextAttrs<'input> for WithClauseContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn withClause(&mut self,)
	-> Result<Rc<WithClauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = WithClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_withClause);
        let mut _localctx: Rc<WithClauseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(294);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(17,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(287);
					recog.base.match_token(WITH,&mut recog.err_handler)?;

					recog.base.set_state(288);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(289);
					recog.base.match_token(EQ,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(290);
					recog.expression()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(291);
					recog.base.match_token(WITH,&mut recog.err_handler)?;

					recog.base.set_state(292);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(293);
					recog.base.match_token(STRING_LITERAL,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- forEachClause ----------------
pub type ForEachClauseContextAll<'input> = ForEachClauseContext<'input>;


pub type ForEachClauseContext<'input> = BaseParserRuleContext<'input,ForEachClauseContextExt<'input>>;

#[derive(Clone)]
pub struct ForEachClauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for ForEachClauseContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for ForEachClauseContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_forEachClause(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_forEachClause(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for ForEachClauseContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_forEachClause(self);
	}
}

impl<'input> CustomRuleContext<'input> for ForEachClauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_forEachClause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_forEachClause }
}
antlr_rust::tid!{ForEachClauseContextExt<'a>}

impl<'input> ForEachClauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ForEachClauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ForEachClauseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ForEachClauseContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<ForEachClauseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FOR
/// Returns `None` if there is no child corresponding to token FOR
fn FOR(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(FOR, 0)
}
/// Retrieves first TerminalNode corresponding to token EACH
/// Returns `None` if there is no child corresponding to token EACH
fn EACH(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(EACH, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ForEachClauseContextAttrs<'input> for ForEachClauseContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn forEachClause(&mut self,)
	-> Result<Rc<ForEachClauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ForEachClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_forEachClause);
        let mut _localctx: Rc<ForEachClauseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(296);
			recog.base.match_token(FOR,&mut recog.err_handler)?;

			recog.base.set_state(297);
			recog.base.match_token(EACH,&mut recog.err_handler)?;

			recog.base.set_state(298);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(299);
			recog.base.match_token(IN,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(300);
			recog.expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- intoClause ----------------
pub type IntoClauseContextAll<'input> = IntoClauseContext<'input>;


pub type IntoClauseContext<'input> = BaseParserRuleContext<'input,IntoClauseContextExt<'input>>;

#[derive(Clone)]
pub struct IntoClauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for IntoClauseContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for IntoClauseContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_intoClause(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_intoClause(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for IntoClauseContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_intoClause(self);
	}
}

impl<'input> CustomRuleContext<'input> for IntoClauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_intoClause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_intoClause }
}
antlr_rust::tid!{IntoClauseContextExt<'a>}

impl<'input> IntoClauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IntoClauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IntoClauseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IntoClauseContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<IntoClauseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTO
/// Returns `None` if there is no child corresponding to token INTO
fn INTO(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(INTO, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> IntoClauseContextAttrs<'input> for IntoClauseContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn intoClause(&mut self,)
	-> Result<Rc<IntoClauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IntoClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_intoClause);
        let mut _localctx: Rc<IntoClauseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(302);
			recog.base.match_token(INTO,&mut recog.err_handler)?;

			recog.base.set_state(303);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- publishStmt ----------------
pub type PublishStmtContextAll<'input> = PublishStmtContext<'input>;


pub type PublishStmtContext<'input> = BaseParserRuleContext<'input,PublishStmtContextExt<'input>>;

#[derive(Clone)]
pub struct PublishStmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for PublishStmtContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for PublishStmtContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_publishStmt(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_publishStmt(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for PublishStmtContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_publishStmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for PublishStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_publishStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_publishStmt }
}
antlr_rust::tid!{PublishStmtContextExt<'a>}

impl<'input> PublishStmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PublishStmtContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PublishStmtContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PublishStmtContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<PublishStmtContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PUBLISH
/// Returns `None` if there is no child corresponding to token PUBLISH
fn PUBLISH(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(PUBLISH, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> PublishStmtContextAttrs<'input> for PublishStmtContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn publishStmt(&mut self,)
	-> Result<Rc<PublishStmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PublishStmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_publishStmt);
        let mut _localctx: Rc<PublishStmtContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(305);
			recog.base.match_token(PUBLISH,&mut recog.err_handler)?;

			recog.base.set_state(306);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- respondStmt ----------------
pub type RespondStmtContextAll<'input> = RespondStmtContext<'input>;


pub type RespondStmtContext<'input> = BaseParserRuleContext<'input,RespondStmtContextExt<'input>>;

#[derive(Clone)]
pub struct RespondStmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for RespondStmtContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for RespondStmtContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_respondStmt(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_respondStmt(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for RespondStmtContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_respondStmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for RespondStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_respondStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_respondStmt }
}
antlr_rust::tid!{RespondStmtContextExt<'a>}

impl<'input> RespondStmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RespondStmtContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RespondStmtContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RespondStmtContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<RespondStmtContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token RESPOND
/// Returns `None` if there is no child corresponding to token RESPOND
fn RESPOND(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(RESPOND, 0)
}
/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
fn schemaRef(&self) -> Option<Rc<SchemaRefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RespondStmtContextAttrs<'input> for RespondStmtContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn respondStmt(&mut self,)
	-> Result<Rc<RespondStmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RespondStmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_respondStmt);
        let mut _localctx: Rc<RespondStmtContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(308);
			recog.base.match_token(RESPOND,&mut recog.err_handler)?;

			recog.base.set_state(309);
			recog.base.match_token(INTEGER,&mut recog.err_handler)?;

			recog.base.set_state(311);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==IDENTIFIER {
				{
				/*InvokeRule schemaRef*/
				recog.base.set_state(310);
				recog.schemaRef()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- onErrorBlock ----------------
pub type OnErrorBlockContextAll<'input> = OnErrorBlockContext<'input>;


pub type OnErrorBlockContext<'input> = BaseParserRuleContext<'input,OnErrorBlockContextExt<'input>>;

#[derive(Clone)]
pub struct OnErrorBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for OnErrorBlockContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for OnErrorBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_onErrorBlock(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_onErrorBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for OnErrorBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_onErrorBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for OnErrorBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_onErrorBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_onErrorBlock }
}
antlr_rust::tid!{OnErrorBlockContextExt<'a>}

impl<'input> OnErrorBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OnErrorBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OnErrorBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OnErrorBlockContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<OnErrorBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ON_ERROR
/// Returns `None` if there is no child corresponding to token ON_ERROR
fn ON_ERROR(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(ON_ERROR, 0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
fn errorCase_all(&self) ->  Vec<Rc<ErrorCaseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn errorCase(&self, i: usize) -> Option<Rc<ErrorCaseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OnErrorBlockContextAttrs<'input> for OnErrorBlockContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn onErrorBlock(&mut self,)
	-> Result<Rc<OnErrorBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OnErrorBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_onErrorBlock);
        let mut _localctx: Rc<OnErrorBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(313);
			recog.base.match_token(ON_ERROR,&mut recog.err_handler)?;

			recog.base.set_state(315); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule errorCase*/
				recog.base.set_state(314);
				recog.errorCase()?;

				}
				}
				recog.base.set_state(317); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==FALLBACK || _la==INTEGER) {break}
			}
			recog.base.set_state(319);
			recog.base.match_token(END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- errorCase ----------------
pub type ErrorCaseContextAll<'input> = ErrorCaseContext<'input>;


pub type ErrorCaseContext<'input> = BaseParserRuleContext<'input,ErrorCaseContextExt<'input>>;

#[derive(Clone)]
pub struct ErrorCaseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for ErrorCaseContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for ErrorCaseContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_errorCase(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_errorCase(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for ErrorCaseContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_errorCase(self);
	}
}

impl<'input> CustomRuleContext<'input> for ErrorCaseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_errorCase }
	//fn type_rule_index() -> usize where Self: Sized { RULE_errorCase }
}
antlr_rust::tid!{ErrorCaseContextExt<'a>}

impl<'input> ErrorCaseContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ErrorCaseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ErrorCaseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ErrorCaseContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<ErrorCaseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
/// Retrieves first TerminalNode corresponding to token WHEN
/// Returns `None` if there is no child corresponding to token WHEN
fn WHEN(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(WHEN, 0)
}
fn predicate(&self) -> Option<Rc<PredicateContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token FALLBACK
/// Returns `None` if there is no child corresponding to token FALLBACK
fn FALLBACK(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(FALLBACK, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token TO
/// Returns `None` if there is no child corresponding to token TO
fn TO(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(TO, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> ErrorCaseContextAttrs<'input> for ErrorCaseContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn errorCase(&mut self,)
	-> Result<Rc<ErrorCaseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ErrorCaseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_errorCase);
        let mut _localctx: Rc<ErrorCaseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(329);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 INTEGER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(321);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					recog.base.set_state(322);
					recog.base.match_token(WHEN,&mut recog.err_handler)?;

					/*InvokeRule predicate*/
					recog.base.set_state(323);
					recog.predicate()?;

					}
				}

			 FALLBACK 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(324);
					recog.base.match_token(FALLBACK,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(325);
					recog.expression()?;

					recog.base.set_state(326);
					recog.base.match_token(TO,&mut recog.err_handler)?;

					recog.base.set_state(327);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- predicate ----------------
pub type PredicateContextAll<'input> = PredicateContext<'input>;


pub type PredicateContext<'input> = BaseParserRuleContext<'input,PredicateContextExt<'input>>;

#[derive(Clone)]
pub struct PredicateContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for PredicateContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for PredicateContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_predicate(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_predicate(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for PredicateContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_predicate(self);
	}
}

impl<'input> CustomRuleContext<'input> for PredicateContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_predicate }
	//fn type_rule_index() -> usize where Self: Sized { RULE_predicate }
}
antlr_rust::tid!{PredicateContextExt<'a>}

impl<'input> PredicateContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PredicateContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PredicateContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PredicateContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<PredicateContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token IS
/// Returns `None` if there is no child corresponding to token IS
fn IS(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IS, 0)
}
/// Retrieves first TerminalNode corresponding to token NULL
/// Returns `None` if there is no child corresponding to token NULL
fn NULL(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(NULL, 0)
}
fn comparator(&self) -> Option<Rc<ComparatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CONTAINS
/// Returns `None` if there is no child corresponding to token CONTAINS
fn CONTAINS(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(CONTAINS, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> PredicateContextAttrs<'input> for PredicateContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn predicate(&mut self,)
	-> Result<Rc<PredicateContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PredicateContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_predicate);
        let mut _localctx: Rc<PredicateContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(343);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(21,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule expression*/
					recog.base.set_state(331);
					recog.expression()?;

					recog.base.set_state(332);
					recog.base.match_token(IS,&mut recog.err_handler)?;

					recog.base.set_state(333);
					recog.base.match_token(NULL,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule expression*/
					recog.base.set_state(335);
					recog.expression()?;

					/*InvokeRule comparator*/
					recog.base.set_state(336);
					recog.comparator()?;

					/*InvokeRule expression*/
					recog.base.set_state(337);
					recog.expression()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule expression*/
					recog.base.set_state(339);
					recog.expression()?;

					recog.base.set_state(340);
					recog.base.match_token(CONTAINS,&mut recog.err_handler)?;

					recog.base.set_state(341);
					_la = recog.base.input.la(1);
					if { !(_la==NULL || _la==IDENTIFIER) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- transactionBlock ----------------
pub type TransactionBlockContextAll<'input> = TransactionBlockContext<'input>;


pub type TransactionBlockContext<'input> = BaseParserRuleContext<'input,TransactionBlockContextExt<'input>>;

#[derive(Clone)]
pub struct TransactionBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for TransactionBlockContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for TransactionBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_transactionBlock(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_transactionBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for TransactionBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_transactionBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for TransactionBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_transactionBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_transactionBlock }
}
antlr_rust::tid!{TransactionBlockContextExt<'a>}

impl<'input> TransactionBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TransactionBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TransactionBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TransactionBlockContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<TransactionBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TRANSACTION
/// Returns `None` if there is no child corresponding to token TRANSACTION
fn TRANSACTION(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(TRANSACTION, 0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
fn handlerStatement_all(&self) ->  Vec<Rc<HandlerStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn handlerStatement(&self, i: usize) -> Option<Rc<HandlerStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn onRollbackBlock(&self) -> Option<Rc<OnRollbackBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TransactionBlockContextAttrs<'input> for TransactionBlockContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn transactionBlock(&mut self,)
	-> Result<Rc<TransactionBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TransactionBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_transactionBlock);
        let mut _localctx: Rc<TransactionBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(345);
			recog.base.match_token(TRANSACTION,&mut recog.err_handler)?;

			recog.base.set_state(349);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << AUTHORIZE) | (1usize << CALL) | (1usize << LOOKUP) | (1usize << ON_ERROR) | (1usize << PARALLEL) | (1usize << PERSIST) | (1usize << PUBLISH) | (1usize << RESPOND))) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (SAGA - 32)) | (1usize << (TRANSACTION - 32)) | (1usize << (TRANSFORM - 32)) | (1usize << (VALIDATE - 32)))) != 0) {
				{
				{
				/*InvokeRule handlerStatement*/
				recog.base.set_state(346);
				recog.handlerStatement()?;

				}
				}
				recog.base.set_state(351);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(353);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ON_ROLLBACK {
				{
				/*InvokeRule onRollbackBlock*/
				recog.base.set_state(352);
				recog.onRollbackBlock()?;

				}
			}

			recog.base.set_state(355);
			recog.base.match_token(END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- onRollbackBlock ----------------
pub type OnRollbackBlockContextAll<'input> = OnRollbackBlockContext<'input>;


pub type OnRollbackBlockContext<'input> = BaseParserRuleContext<'input,OnRollbackBlockContextExt<'input>>;

#[derive(Clone)]
pub struct OnRollbackBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for OnRollbackBlockContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for OnRollbackBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_onRollbackBlock(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_onRollbackBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for OnRollbackBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_onRollbackBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for OnRollbackBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_onRollbackBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_onRollbackBlock }
}
antlr_rust::tid!{OnRollbackBlockContextExt<'a>}

impl<'input> OnRollbackBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OnRollbackBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OnRollbackBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OnRollbackBlockContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<OnRollbackBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ON_ROLLBACK
/// Returns `None` if there is no child corresponding to token ON_ROLLBACK
fn ON_ROLLBACK(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(ON_ROLLBACK, 0)
}
fn handlerStatement_all(&self) ->  Vec<Rc<HandlerStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn handlerStatement(&self, i: usize) -> Option<Rc<HandlerStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OnRollbackBlockContextAttrs<'input> for OnRollbackBlockContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn onRollbackBlock(&mut self,)
	-> Result<Rc<OnRollbackBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OnRollbackBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_onRollbackBlock);
        let mut _localctx: Rc<OnRollbackBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(357);
			recog.base.match_token(ON_ROLLBACK,&mut recog.err_handler)?;

			recog.base.set_state(361);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << AUTHORIZE) | (1usize << CALL) | (1usize << LOOKUP) | (1usize << ON_ERROR) | (1usize << PARALLEL) | (1usize << PERSIST) | (1usize << PUBLISH) | (1usize << RESPOND))) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (SAGA - 32)) | (1usize << (TRANSACTION - 32)) | (1usize << (TRANSFORM - 32)) | (1usize << (VALIDATE - 32)))) != 0) {
				{
				{
				/*InvokeRule handlerStatement*/
				recog.base.set_state(358);
				recog.handlerStatement()?;

				}
				}
				recog.base.set_state(363);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- sagaBlock ----------------
pub type SagaBlockContextAll<'input> = SagaBlockContext<'input>;


pub type SagaBlockContext<'input> = BaseParserRuleContext<'input,SagaBlockContextExt<'input>>;

#[derive(Clone)]
pub struct SagaBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for SagaBlockContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for SagaBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_sagaBlock(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_sagaBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for SagaBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_sagaBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for SagaBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sagaBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sagaBlock }
}
antlr_rust::tid!{SagaBlockContextExt<'a>}

impl<'input> SagaBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SagaBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SagaBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SagaBlockContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<SagaBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SAGA
/// Returns `None` if there is no child corresponding to token SAGA
fn SAGA(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(SAGA, 0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
fn sagaStep_all(&self) ->  Vec<Rc<SagaStepContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn sagaStep(&self, i: usize) -> Option<Rc<SagaStepContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> SagaBlockContextAttrs<'input> for SagaBlockContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn sagaBlock(&mut self,)
	-> Result<Rc<SagaBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SagaBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_sagaBlock);
        let mut _localctx: Rc<SagaBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(364);
			recog.base.match_token(SAGA,&mut recog.err_handler)?;

			recog.base.set_state(366); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule sagaStep*/
				recog.base.set_state(365);
				recog.sagaStep()?;

				}
				}
				recog.base.set_state(368); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==STEP) {break}
			}
			recog.base.set_state(370);
			recog.base.match_token(END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- sagaStep ----------------
pub type SagaStepContextAll<'input> = SagaStepContext<'input>;


pub type SagaStepContext<'input> = BaseParserRuleContext<'input,SagaStepContextExt<'input>>;

#[derive(Clone)]
pub struct SagaStepContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for SagaStepContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for SagaStepContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_sagaStep(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_sagaStep(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for SagaStepContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_sagaStep(self);
	}
}

impl<'input> CustomRuleContext<'input> for SagaStepContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sagaStep }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sagaStep }
}
antlr_rust::tid!{SagaStepContextExt<'a>}

impl<'input> SagaStepContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SagaStepContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SagaStepContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SagaStepContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<SagaStepContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STEP
/// Returns `None` if there is no child corresponding to token STEP
fn STEP(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(STEP, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn compensateBlock(&self) -> Option<Rc<CompensateBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
fn handlerStatement_all(&self) ->  Vec<Rc<HandlerStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn handlerStatement(&self, i: usize) -> Option<Rc<HandlerStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> SagaStepContextAttrs<'input> for SagaStepContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn sagaStep(&mut self,)
	-> Result<Rc<SagaStepContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SagaStepContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_sagaStep);
        let mut _localctx: Rc<SagaStepContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(372);
			recog.base.match_token(STEP,&mut recog.err_handler)?;

			recog.base.set_state(373);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(377);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << AUTHORIZE) | (1usize << CALL) | (1usize << LOOKUP) | (1usize << ON_ERROR) | (1usize << PARALLEL) | (1usize << PERSIST) | (1usize << PUBLISH) | (1usize << RESPOND))) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (SAGA - 32)) | (1usize << (TRANSACTION - 32)) | (1usize << (TRANSFORM - 32)) | (1usize << (VALIDATE - 32)))) != 0) {
				{
				{
				/*InvokeRule handlerStatement*/
				recog.base.set_state(374);
				recog.handlerStatement()?;

				}
				}
				recog.base.set_state(379);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule compensateBlock*/
			recog.base.set_state(380);
			recog.compensateBlock()?;

			recog.base.set_state(381);
			recog.base.match_token(END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- compensateBlock ----------------
pub type CompensateBlockContextAll<'input> = CompensateBlockContext<'input>;


pub type CompensateBlockContext<'input> = BaseParserRuleContext<'input,CompensateBlockContextExt<'input>>;

#[derive(Clone)]
pub struct CompensateBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for CompensateBlockContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for CompensateBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_compensateBlock(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_compensateBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for CompensateBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_compensateBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for CompensateBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compensateBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compensateBlock }
}
antlr_rust::tid!{CompensateBlockContextExt<'a>}

impl<'input> CompensateBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CompensateBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompensateBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CompensateBlockContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<CompensateBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COMPENSATE
/// Returns `None` if there is no child corresponding to token COMPENSATE
fn COMPENSATE(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(COMPENSATE, 0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
fn handlerStatement_all(&self) ->  Vec<Rc<HandlerStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn handlerStatement(&self, i: usize) -> Option<Rc<HandlerStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> CompensateBlockContextAttrs<'input> for CompensateBlockContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compensateBlock(&mut self,)
	-> Result<Rc<CompensateBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CompensateBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_compensateBlock);
        let mut _localctx: Rc<CompensateBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(383);
			recog.base.match_token(COMPENSATE,&mut recog.err_handler)?;

			recog.base.set_state(387);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << AUTHORIZE) | (1usize << CALL) | (1usize << LOOKUP) | (1usize << ON_ERROR) | (1usize << PARALLEL) | (1usize << PERSIST) | (1usize << PUBLISH) | (1usize << RESPOND))) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (SAGA - 32)) | (1usize << (TRANSACTION - 32)) | (1usize << (TRANSFORM - 32)) | (1usize << (VALIDATE - 32)))) != 0) {
				{
				{
				/*InvokeRule handlerStatement*/
				recog.base.set_state(384);
				recog.handlerStatement()?;

				}
				}
				recog.base.set_state(389);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(390);
			recog.base.match_token(END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- parallelBlock ----------------
pub type ParallelBlockContextAll<'input> = ParallelBlockContext<'input>;


pub type ParallelBlockContext<'input> = BaseParserRuleContext<'input,ParallelBlockContextExt<'input>>;

#[derive(Clone)]
pub struct ParallelBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for ParallelBlockContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for ParallelBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_parallelBlock(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_parallelBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for ParallelBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_parallelBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParallelBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parallelBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parallelBlock }
}
antlr_rust::tid!{ParallelBlockContextExt<'a>}

impl<'input> ParallelBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParallelBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParallelBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParallelBlockContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<ParallelBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PARALLEL
/// Returns `None` if there is no child corresponding to token PARALLEL
fn PARALLEL(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(PARALLEL, 0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
fn handlerStatement_all(&self) ->  Vec<Rc<HandlerStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn handlerStatement(&self, i: usize) -> Option<Rc<HandlerStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ParallelBlockContextAttrs<'input> for ParallelBlockContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parallelBlock(&mut self,)
	-> Result<Rc<ParallelBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParallelBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_parallelBlock);
        let mut _localctx: Rc<ParallelBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(392);
			recog.base.match_token(PARALLEL,&mut recog.err_handler)?;

			recog.base.set_state(394); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule handlerStatement*/
				recog.base.set_state(393);
				recog.handlerStatement()?;

				}
				}
				recog.base.set_state(396); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << AUTHORIZE) | (1usize << CALL) | (1usize << LOOKUP) | (1usize << ON_ERROR) | (1usize << PARALLEL) | (1usize << PERSIST) | (1usize << PUBLISH) | (1usize << RESPOND))) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (SAGA - 32)) | (1usize << (TRANSACTION - 32)) | (1usize << (TRANSFORM - 32)) | (1usize << (VALIDATE - 32)))) != 0)) {break}
			}
			recog.base.set_state(398);
			recog.base.match_token(END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- cacheBlock ----------------
pub type CacheBlockContextAll<'input> = CacheBlockContext<'input>;


pub type CacheBlockContext<'input> = BaseParserRuleContext<'input,CacheBlockContextExt<'input>>;

#[derive(Clone)]
pub struct CacheBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for CacheBlockContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for CacheBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_cacheBlock(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_cacheBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for CacheBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_cacheBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for CacheBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cacheBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cacheBlock }
}
antlr_rust::tid!{CacheBlockContextExt<'a>}

impl<'input> CacheBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CacheBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CacheBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CacheBlockContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<CacheBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CACHE
/// Returns `None` if there is no child corresponding to token CACHE
fn CACHE(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(CACHE, 0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
fn cacheEntry_all(&self) ->  Vec<Rc<CacheEntryContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn cacheEntry(&self, i: usize) -> Option<Rc<CacheEntryContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> CacheBlockContextAttrs<'input> for CacheBlockContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn cacheBlock(&mut self,)
	-> Result<Rc<CacheBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CacheBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_cacheBlock);
        let mut _localctx: Rc<CacheBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(400);
			recog.base.match_token(CACHE,&mut recog.err_handler)?;

			recog.base.set_state(402); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule cacheEntry*/
				recog.base.set_state(401);
				recog.cacheEntry()?;

				}
				}
				recog.base.set_state(404); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==IDENTIFIER) {break}
			}
			recog.base.set_state(406);
			recog.base.match_token(END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- cacheEntry ----------------
pub type CacheEntryContextAll<'input> = CacheEntryContext<'input>;


pub type CacheEntryContext<'input> = BaseParserRuleContext<'input,CacheEntryContextExt<'input>>;

#[derive(Clone)]
pub struct CacheEntryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for CacheEntryContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for CacheEntryContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_cacheEntry(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_cacheEntry(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for CacheEntryContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_cacheEntry(self);
	}
}

impl<'input> CustomRuleContext<'input> for CacheEntryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cacheEntry }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cacheEntry }
}
antlr_rust::tid!{CacheEntryContextExt<'a>}

impl<'input> CacheEntryContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CacheEntryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CacheEntryContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CacheEntryContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<CacheEntryContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,ServiceDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves first TerminalNode corresponding to token TTL
/// Returns `None` if there is no child corresponding to token TTL
fn TTL(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(TTL, 0)
}
fn valueOrCfg(&self) -> Option<Rc<ValueOrCfgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token INVALIDATE_ON
/// Returns `None` if there is no child corresponding to token INVALIDATE_ON
fn INVALIDATE_ON(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(INVALIDATE_ON, 0)
}
fn identifierList(&self) -> Option<Rc<IdentifierListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CacheEntryContextAttrs<'input> for CacheEntryContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn cacheEntry(&mut self,)
	-> Result<Rc<CacheEntryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CacheEntryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_cacheEntry);
        let mut _localctx: Rc<CacheEntryContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(408);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(409);
			recog.base.match_token(TTL,&mut recog.err_handler)?;

			/*InvokeRule valueOrCfg*/
			recog.base.set_state(410);
			recog.valueOrCfg()?;

			recog.base.set_state(411);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(414);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==INVALIDATE_ON {
				{
				recog.base.set_state(412);
				recog.base.match_token(INVALIDATE_ON,&mut recog.err_handler)?;

				/*InvokeRule identifierList*/
				recog.base.set_state(413);
				recog.identifierList()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- healthBlock ----------------
pub type HealthBlockContextAll<'input> = HealthBlockContext<'input>;


pub type HealthBlockContext<'input> = BaseParserRuleContext<'input,HealthBlockContextExt<'input>>;

#[derive(Clone)]
pub struct HealthBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for HealthBlockContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for HealthBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_healthBlock(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_healthBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for HealthBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_healthBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for HealthBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_healthBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_healthBlock }
}
antlr_rust::tid!{HealthBlockContextExt<'a>}

impl<'input> HealthBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<HealthBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HealthBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait HealthBlockContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<HealthBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token HEALTH
/// Returns `None` if there is no child corresponding to token HEALTH
fn HEALTH(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(HEALTH, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
fn healthCheck_all(&self) ->  Vec<Rc<HealthCheckContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn healthCheck(&self, i: usize) -> Option<Rc<HealthCheckContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> HealthBlockContextAttrs<'input> for HealthBlockContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn healthBlock(&mut self,)
	-> Result<Rc<HealthBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HealthBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_healthBlock);
        let mut _localctx: Rc<HealthBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(416);
			recog.base.match_token(HEALTH,&mut recog.err_handler)?;

			recog.base.set_state(417);
			recog.base.match_token(STRING_LITERAL,&mut recog.err_handler)?;

			recog.base.set_state(421);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==CHECK {
				{
				{
				/*InvokeRule healthCheck*/
				recog.base.set_state(418);
				recog.healthCheck()?;

				}
				}
				recog.base.set_state(423);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(424);
			recog.base.match_token(END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- healthCheck ----------------
pub type HealthCheckContextAll<'input> = HealthCheckContext<'input>;


pub type HealthCheckContext<'input> = BaseParserRuleContext<'input,HealthCheckContextExt<'input>>;

#[derive(Clone)]
pub struct HealthCheckContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for HealthCheckContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for HealthCheckContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_healthCheck(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_healthCheck(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for HealthCheckContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_healthCheck(self);
	}
}

impl<'input> CustomRuleContext<'input> for HealthCheckContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_healthCheck }
	//fn type_rule_index() -> usize where Self: Sized { RULE_healthCheck }
}
antlr_rust::tid!{HealthCheckContextExt<'a>}

impl<'input> HealthCheckContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<HealthCheckContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HealthCheckContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait HealthCheckContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<HealthCheckContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CHECK
/// Returns `None` if there is no child corresponding to token CHECK
fn CHECK(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(CHECK, 0)
}
fn word_all(&self) ->  Vec<Rc<WordContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn word(&self, i: usize) -> Option<Rc<WordContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token TIMEOUT
/// Returns `None` if there is no child corresponding to token TIMEOUT
fn TIMEOUT(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(TIMEOUT, 0)
}
fn valueOrCfg(&self) -> Option<Rc<ValueOrCfgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> HealthCheckContextAttrs<'input> for HealthCheckContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn healthCheck(&mut self,)
	-> Result<Rc<HealthCheckContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HealthCheckContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_healthCheck);
        let mut _localctx: Rc<HealthCheckContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(426);
			recog.base.match_token(CHECK,&mut recog.err_handler)?;

			/*InvokeRule word*/
			recog.base.set_state(427);
			recog.word()?;

			/*InvokeRule word*/
			recog.base.set_state(428);
			recog.word()?;

			recog.base.set_state(429);
			recog.base.match_token(TIMEOUT,&mut recog.err_handler)?;

			/*InvokeRule valueOrCfg*/
			recog.base.set_state(430);
			recog.valueOrCfg()?;

			/*InvokeRule word*/
			recog.base.set_state(431);
			recog.word()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- readyDecl ----------------
pub type ReadyDeclContextAll<'input> = ReadyDeclContext<'input>;


pub type ReadyDeclContext<'input> = BaseParserRuleContext<'input,ReadyDeclContextExt<'input>>;

#[derive(Clone)]
pub struct ReadyDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for ReadyDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for ReadyDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_readyDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_readyDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for ReadyDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_readyDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReadyDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_readyDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_readyDecl }
}
antlr_rust::tid!{ReadyDeclContextExt<'a>}

impl<'input> ReadyDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReadyDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReadyDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReadyDeclContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<ReadyDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token READY
/// Returns `None` if there is no child corresponding to token READY
fn READY(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(READY, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}

}

impl<'input> ReadyDeclContextAttrs<'input> for ReadyDeclContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn readyDecl(&mut self,)
	-> Result<Rc<ReadyDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReadyDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_readyDecl);
        let mut _localctx: Rc<ReadyDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(433);
			recog.base.match_token(READY,&mut recog.err_handler)?;

			recog.base.set_state(434);
			recog.base.match_token(STRING_LITERAL,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- qualifiedAnnotation ----------------
pub type QualifiedAnnotationContextAll<'input> = QualifiedAnnotationContext<'input>;


pub type QualifiedAnnotationContext<'input> = BaseParserRuleContext<'input,QualifiedAnnotationContextExt<'input>>;

#[derive(Clone)]
pub struct QualifiedAnnotationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for QualifiedAnnotationContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for QualifiedAnnotationContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_qualifiedAnnotation(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_qualifiedAnnotation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for QualifiedAnnotationContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_qualifiedAnnotation(self);
	}
}

impl<'input> CustomRuleContext<'input> for QualifiedAnnotationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_qualifiedAnnotation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_qualifiedAnnotation }
}
antlr_rust::tid!{QualifiedAnnotationContextExt<'a>}

impl<'input> QualifiedAnnotationContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<QualifiedAnnotationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,QualifiedAnnotationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait QualifiedAnnotationContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<QualifiedAnnotationContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,ServiceDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}

}

impl<'input> QualifiedAnnotationContextAttrs<'input> for QualifiedAnnotationContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn qualifiedAnnotation(&mut self,)
	-> Result<Rc<QualifiedAnnotationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = QualifiedAnnotationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_qualifiedAnnotation);
        let mut _localctx: Rc<QualifiedAnnotationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(436);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(437);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			recog.base.set_state(438);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- valueOrCfg ----------------
pub type ValueOrCfgContextAll<'input> = ValueOrCfgContext<'input>;


pub type ValueOrCfgContext<'input> = BaseParserRuleContext<'input,ValueOrCfgContextExt<'input>>;

#[derive(Clone)]
pub struct ValueOrCfgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for ValueOrCfgContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for ValueOrCfgContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_valueOrCfg(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_valueOrCfg(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for ValueOrCfgContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_valueOrCfg(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueOrCfgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_valueOrCfg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_valueOrCfg }
}
antlr_rust::tid!{ValueOrCfgContextExt<'a>}

impl<'input> ValueOrCfgContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValueOrCfgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValueOrCfgContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ValueOrCfgContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<ValueOrCfgContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
/// Retrieves first TerminalNode corresponding to token DECIMAL_LITERAL
/// Returns `None` if there is no child corresponding to token DECIMAL_LITERAL
fn DECIMAL_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(DECIMAL_LITERAL, 0)
}
fn qualifiedAnnotation(&self) -> Option<Rc<QualifiedAnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ValueOrCfgContextAttrs<'input> for ValueOrCfgContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn valueOrCfg(&mut self,)
	-> Result<Rc<ValueOrCfgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValueOrCfgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_valueOrCfg);
        let mut _localctx: Rc<ValueOrCfgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(443);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 INTEGER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(440);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					}
				}

			 DECIMAL_LITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(441);
					recog.base.match_token(DECIMAL_LITERAL,&mut recog.err_handler)?;

					}
				}

			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule qualifiedAnnotation*/
					recog.base.set_state(442);
					recog.qualifiedAnnotation()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expression ----------------
pub type ExpressionContextAll<'input> = ExpressionContext<'input>;


pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for ExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expression(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for ExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::tid!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{

fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn qualifiedAnnotation(&self) -> Option<Rc<QualifiedAnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input,ServiceDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
/// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, i)
}
fn word_all(&self) ->  Vec<Rc<WordContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn word(&self, i: usize) -> Option<Rc<WordContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_expression);
        let mut _localctx: Rc<ExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(455);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(34,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule literal*/
					recog.base.set_state(445);
					recog.literal()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule qualifiedAnnotation*/
					recog.base.set_state(446);
					recog.qualifiedAnnotation()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(447);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(452);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==DOT {
						{
						{
						recog.base.set_state(448);
						recog.base.match_token(DOT,&mut recog.err_handler)?;

						/*InvokeRule word*/
						recog.base.set_state(449);
						recog.word()?;

						}
						}
						recog.base.set_state(454);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expressionList ----------------
pub type ExpressionListContextAll<'input> = ExpressionListContext<'input>;


pub type ExpressionListContext<'input> = BaseParserRuleContext<'input,ExpressionListContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for ExpressionListContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for ExpressionListContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expressionList(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_expressionList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for ExpressionListContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_expressionList(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expressionList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expressionList }
}
antlr_rust::tid!{ExpressionListContextExt<'a>}

impl<'input> ExpressionListContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionListContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<ExpressionListContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,ServiceDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ExpressionListContextAttrs<'input> for ExpressionListContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expressionList(&mut self,)
	-> Result<Rc<ExpressionListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_expressionList);
        let mut _localctx: Rc<ExpressionListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(457);
			recog.expression()?;

			recog.base.set_state(462);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(458);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(459);
				recog.expression()?;

				}
				}
				recog.base.set_state(464);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- comparator ----------------
pub type ComparatorContextAll<'input> = ComparatorContext<'input>;


pub type ComparatorContext<'input> = BaseParserRuleContext<'input,ComparatorContextExt<'input>>;

#[derive(Clone)]
pub struct ComparatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for ComparatorContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for ComparatorContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_comparator(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_comparator(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for ComparatorContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_comparator(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComparatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comparator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comparator }
}
antlr_rust::tid!{ComparatorContextExt<'a>}

impl<'input> ComparatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComparatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComparatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComparatorContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<ComparatorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EQ
/// Returns `None` if there is no child corresponding to token EQ
fn EQ(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(EQ, 0)
}
/// Retrieves first TerminalNode corresponding to token NEQ
/// Returns `None` if there is no child corresponding to token NEQ
fn NEQ(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(NEQ, 0)
}
/// Retrieves first TerminalNode corresponding to token LT
/// Returns `None` if there is no child corresponding to token LT
fn LT(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(LT, 0)
}
/// Retrieves first TerminalNode corresponding to token GT
/// Returns `None` if there is no child corresponding to token GT
fn GT(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(GT, 0)
}
/// Retrieves first TerminalNode corresponding to token LTE
/// Returns `None` if there is no child corresponding to token LTE
fn LTE(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(LTE, 0)
}
/// Retrieves first TerminalNode corresponding to token GTE
/// Returns `None` if there is no child corresponding to token GTE
fn GTE(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(GTE, 0)
}

}

impl<'input> ComparatorContextAttrs<'input> for ComparatorContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn comparator(&mut self,)
	-> Result<Rc<ComparatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComparatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_comparator);
        let mut _localctx: Rc<ComparatorContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(465);
			_la = recog.base.input.la(1);
			if { !(((((_la - 48)) & !0x3f) == 0 && ((1usize << (_la - 48)) & ((1usize << (EQ - 48)) | (1usize << (NEQ - 48)) | (1usize << (LT - 48)) | (1usize << (GT - 48)) | (1usize << (LTE - 48)) | (1usize << (GTE - 48)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- qualifiedRef ----------------
pub type QualifiedRefContextAll<'input> = QualifiedRefContext<'input>;


pub type QualifiedRefContext<'input> = BaseParserRuleContext<'input,QualifiedRefContextExt<'input>>;

#[derive(Clone)]
pub struct QualifiedRefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for QualifiedRefContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for QualifiedRefContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_qualifiedRef(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_qualifiedRef(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for QualifiedRefContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_qualifiedRef(self);
	}
}

impl<'input> CustomRuleContext<'input> for QualifiedRefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_qualifiedRef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_qualifiedRef }
}
antlr_rust::tid!{QualifiedRefContextExt<'a>}

impl<'input> QualifiedRefContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<QualifiedRefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,QualifiedRefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait QualifiedRefContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<QualifiedRefContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,ServiceDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}

}

impl<'input> QualifiedRefContextAttrs<'input> for QualifiedRefContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn qualifiedRef(&mut self,)
	-> Result<Rc<QualifiedRefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = QualifiedRefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 96, RULE_qualifiedRef);
        let mut _localctx: Rc<QualifiedRefContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(467);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(468);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			recog.base.set_state(469);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- schemaRef ----------------
pub type SchemaRefContextAll<'input> = SchemaRefContext<'input>;


pub type SchemaRefContext<'input> = BaseParserRuleContext<'input,SchemaRefContextExt<'input>>;

#[derive(Clone)]
pub struct SchemaRefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for SchemaRefContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for SchemaRefContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_schemaRef(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_schemaRef(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for SchemaRefContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_schemaRef(self);
	}
}

impl<'input> CustomRuleContext<'input> for SchemaRefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_schemaRef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_schemaRef }
}
antlr_rust::tid!{SchemaRefContextExt<'a>}

impl<'input> SchemaRefContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SchemaRefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SchemaRefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SchemaRefContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<SchemaRefContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,ServiceDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}

}

impl<'input> SchemaRefContextAttrs<'input> for SchemaRefContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn schemaRef(&mut self,)
	-> Result<Rc<SchemaRefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SchemaRefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 98, RULE_schemaRef);
        let mut _localctx: Rc<SchemaRefContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(471);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(474);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==DOT {
				{
				recog.base.set_state(472);
				recog.base.match_token(DOT,&mut recog.err_handler)?;

				recog.base.set_state(473);
				recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- identifierList ----------------
pub type IdentifierListContextAll<'input> = IdentifierListContext<'input>;


pub type IdentifierListContext<'input> = BaseParserRuleContext<'input,IdentifierListContextExt<'input>>;

#[derive(Clone)]
pub struct IdentifierListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for IdentifierListContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for IdentifierListContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_identifierList(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_identifierList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for IdentifierListContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_identifierList(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdentifierListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_identifierList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_identifierList }
}
antlr_rust::tid!{IdentifierListContextExt<'a>}

impl<'input> IdentifierListContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IdentifierListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdentifierListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IdentifierListContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<IdentifierListContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,ServiceDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,ServiceDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> IdentifierListContextAttrs<'input> for IdentifierListContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn identifierList(&mut self,)
	-> Result<Rc<IdentifierListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdentifierListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 100, RULE_identifierList);
        let mut _localctx: Rc<IdentifierListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(476);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(481);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(477);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				recog.base.set_state(478);
				recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(483);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- literal ----------------
pub type LiteralContextAll<'input> = LiteralContext<'input>;


pub type LiteralContext<'input> = BaseParserRuleContext<'input,LiteralContextExt<'input>>;

#[derive(Clone)]
pub struct LiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ServiceDSLParserContext<'input> for LiteralContext<'input>{}

impl<'input,'a> Listenable<dyn ServiceDSLListener<'input> + 'a> for LiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_literal(self);
		}
		fn exit(&self,listener: &mut (dyn ServiceDSLListener<'input> + 'a)) {
			listener.exit_literal(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ServiceDSLVisitor<'input> + 'a> for LiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn ServiceDSLVisitor<'input> + 'a)) {
		visitor.visit_literal(self);
	}
}

impl<'input> CustomRuleContext<'input> for LiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ServiceDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literal }
}
antlr_rust::tid!{LiteralContextExt<'a>}

impl<'input> LiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn ServiceDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LiteralContextAttrs<'input>: ServiceDSLParserContext<'input> + BorrowMut<LiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
/// Retrieves first TerminalNode corresponding to token DECIMAL_LITERAL
/// Returns `None` if there is no child corresponding to token DECIMAL_LITERAL
fn DECIMAL_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(DECIMAL_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token TRUE
/// Returns `None` if there is no child corresponding to token TRUE
fn TRUE(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(TRUE, 0)
}
/// Retrieves first TerminalNode corresponding to token FALSE
/// Returns `None` if there is no child corresponding to token FALSE
fn FALSE(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(FALSE, 0)
}
/// Retrieves first TerminalNode corresponding to token NULL
/// Returns `None` if there is no child corresponding to token NULL
fn NULL(&self) -> Option<Rc<TerminalNode<'input,ServiceDSLParserContextType>>> where Self:Sized{
	self.get_token(NULL, 0)
}

}

impl<'input> LiteralContextAttrs<'input> for LiteralContext<'input>{}

impl<'input, I, H> ServiceDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn literal(&mut self,)
	-> Result<Rc<LiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 102, RULE_literal);
        let mut _localctx: Rc<LiteralContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(484);
			_la = recog.base.input.la(1);
			if { !(_la==NULL || ((((_la - 46)) & !0x3f) == 0 && ((1usize << (_la - 46)) & ((1usize << (TRUE - 46)) | (1usize << (FALSE - 46)) | (1usize << (INTEGER - 46)) | (1usize << (DECIMAL_LITERAL - 46)) | (1usize << (STRING_LITERAL - 46)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x43\u{1e9}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x04\x2b\x09\x2b\x04\x2c\x09\x2c\x04\x2d\x09\x2d\x04\
	\x2e\x09\x2e\x04\x2f\x09\x2f\x04\x30\x09\x30\x04\x31\x09\x31\x04\x32\x09\
	\x32\x04\x33\x09\x33\x04\x34\x09\x34\x04\x35\x09\x35\x03\x02\x07\x02\x6c\
	\x0a\x02\x0c\x02\x0e\x02\x6f\x0b\x02\x03\x02\x03\x02\x03\x02\x03\x03\x03\
	\x03\x03\x03\x03\x04\x05\x04\x78\x0a\x04\x03\x04\x03\x04\x03\x04\x07\x04\
	\x7d\x0a\x04\x0c\x04\x0e\x04\u{80}\x0b\x04\x03\x04\x03\x04\x03\x04\x03\x05\
	\x03\x05\x03\x05\x07\x05\u{88}\x0a\x05\x0c\x05\x0e\x05\u{8b}\x0b\x05\x03\
	\x06\x03\x06\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x08\x07\x08\u{95}\
	\x0a\x08\x0c\x08\x0e\x08\u{98}\x0b\x08\x03\x09\x03\x09\x03\x09\x03\x09\x03\
	\x09\x03\x09\x03\x09\x03\x09\x05\x09\u{a2}\x0a\x09\x03\x0a\x03\x0a\x03\x0a\
	\x03\x0b\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x07\x0d\
	\u{af}\x0a\x0d\x0c\x0d\x0e\x0d\u{b2}\x0b\x0d\x03\x0d\x03\x0d\x03\x0e\x03\
	\x0e\x03\x0e\x03\x0e\x07\x0e\u{ba}\x0a\x0e\x0c\x0e\x0e\x0e\u{bd}\x0b\x0e\
	\x03\x0f\x03\x0f\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x11\x07\x11\
	\u{c7}\x0a\x11\x0c\x11\x0e\x11\u{ca}\x0b\x11\x03\x12\x03\x12\x03\x12\x03\
	\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x05\
	\x12\u{d8}\x0a\x12\x03\x13\x03\x13\x03\x13\x03\x13\x03\x14\x03\x14\x03\x14\
	\x03\x14\x03\x14\x03\x15\x03\x15\x03\x15\x03\x15\x03\x15\x07\x15\u{e8}\x0a\
	\x15\x0c\x15\x0e\x15\u{eb}\x0b\x15\x03\x15\x03\x15\x03\x16\x03\x16\x03\x16\
	\x03\x16\x03\x16\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\
	\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x05\x17\u{101}\x0a\x17\
	\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\
	\x03\x18\x03\x18\x03\x18\x05\x18\u{10f}\x0a\x18\x03\x19\x03\x19\x03\x19\
	\x07\x19\u{114}\x0a\x19\x0c\x19\x0e\x19\u{117}\x0b\x19\x03\x19\x05\x19\u{11a}\
	\x0a\x19\x03\x19\x05\x19\u{11d}\x0a\x19\x03\x19\x05\x19\u{120}\x0a\x19\x03\
	\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x05\x1a\u{129}\x0a\
	\x1a\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1c\x03\x1c\x03\
	\x1c\x03\x1d\x03\x1d\x03\x1d\x03\x1e\x03\x1e\x03\x1e\x05\x1e\u{13a}\x0a\
	\x1e\x03\x1f\x03\x1f\x06\x1f\u{13e}\x0a\x1f\x0d\x1f\x0e\x1f\u{13f}\x03\x1f\
	\x03\x1f\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\
	\x05\x20\u{14c}\x0a\x20\x03\x21\x03\x21\x03\x21\x03\x21\x03\x21\x03\x21\
	\x03\x21\x03\x21\x03\x21\x03\x21\x03\x21\x03\x21\x05\x21\u{15a}\x0a\x21\
	\x03\x22\x03\x22\x07\x22\u{15e}\x0a\x22\x0c\x22\x0e\x22\u{161}\x0b\x22\x03\
	\x22\x05\x22\u{164}\x0a\x22\x03\x22\x03\x22\x03\x23\x03\x23\x07\x23\u{16a}\
	\x0a\x23\x0c\x23\x0e\x23\u{16d}\x0b\x23\x03\x24\x03\x24\x06\x24\u{171}\x0a\
	\x24\x0d\x24\x0e\x24\u{172}\x03\x24\x03\x24\x03\x25\x03\x25\x03\x25\x07\
	\x25\u{17a}\x0a\x25\x0c\x25\x0e\x25\u{17d}\x0b\x25\x03\x25\x03\x25\x03\x25\
	\x03\x26\x03\x26\x07\x26\u{184}\x0a\x26\x0c\x26\x0e\x26\u{187}\x0b\x26\x03\
	\x26\x03\x26\x03\x27\x03\x27\x06\x27\u{18d}\x0a\x27\x0d\x27\x0e\x27\u{18e}\
	\x03\x27\x03\x27\x03\x28\x03\x28\x06\x28\u{195}\x0a\x28\x0d\x28\x0e\x28\
	\u{196}\x03\x28\x03\x28\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\
	\x05\x29\u{1a1}\x0a\x29\x03\x2a\x03\x2a\x03\x2a\x07\x2a\u{1a6}\x0a\x2a\x0c\
	\x2a\x0e\x2a\u{1a9}\x0b\x2a\x03\x2a\x03\x2a\x03\x2b\x03\x2b\x03\x2b\x03\
	\x2b\x03\x2b\x03\x2b\x03\x2b\x03\x2c\x03\x2c\x03\x2c\x03\x2d\x03\x2d\x03\
	\x2d\x03\x2d\x03\x2e\x03\x2e\x03\x2e\x05\x2e\u{1be}\x0a\x2e\x03\x2f\x03\
	\x2f\x03\x2f\x03\x2f\x03\x2f\x07\x2f\u{1c5}\x0a\x2f\x0c\x2f\x0e\x2f\u{1c8}\
	\x0b\x2f\x05\x2f\u{1ca}\x0a\x2f\x03\x30\x03\x30\x03\x30\x07\x30\u{1cf}\x0a\
	\x30\x0c\x30\x0e\x30\u{1d2}\x0b\x30\x03\x31\x03\x31\x03\x32\x03\x32\x03\
	\x32\x03\x32\x03\x33\x03\x33\x03\x33\x05\x33\u{1dd}\x0a\x33\x03\x34\x03\
	\x34\x03\x34\x07\x34\u{1e2}\x0a\x34\x0c\x34\x0e\x34\u{1e5}\x0b\x34\x03\x35\
	\x03\x35\x03\x35\x02\x02\x36\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\
	\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x30\x32\x34\x36\x38\x3a\
	\x3c\x3e\x40\x42\x44\x46\x48\x4a\x4c\x4e\x50\x52\x54\x56\x58\x5a\x5c\x5e\
	\x60\x62\x64\x66\x68\x02\x08\x03\x02\x40\x41\x04\x02\x03\x2f\x3b\x3b\x04\
	\x02\x30\x31\x38\x3b\x04\x02\x1a\x1a\x3b\x3b\x03\x02\x32\x37\x05\x02\x1a\
	\x1a\x30\x31\x38\x3a\x02\u{1ed}\x02\x6d\x03\x02\x02\x02\x04\x73\x03\x02\
	\x02\x02\x06\x77\x03\x02\x02\x02\x08\u{84}\x03\x02\x02\x02\x0a\u{8c}\x03\
	\x02\x02\x02\x0c\u{8e}\x03\x02\x02\x02\x0e\u{96}\x03\x02\x02\x02\x10\u{a1}\
	\x03\x02\x02\x02\x12\u{a3}\x03\x02\x02\x02\x14\u{a6}\x03\x02\x02\x02\x16\
	\u{a9}\x03\x02\x02\x02\x18\u{ac}\x03\x02\x02\x02\x1a\u{b5}\x03\x02\x02\x02\
	\x1c\u{be}\x03\x02\x02\x02\x1e\u{c0}\x03\x02\x02\x02\x20\u{c8}\x03\x02\x02\
	\x02\x22\u{d7}\x03\x02\x02\x02\x24\u{d9}\x03\x02\x02\x02\x26\u{dd}\x03\x02\
	\x02\x02\x28\u{e2}\x03\x02\x02\x02\x2a\u{ee}\x03\x02\x02\x02\x2c\u{100}\
	\x03\x02\x02\x02\x2e\u{10e}\x03\x02\x02\x02\x30\u{110}\x03\x02\x02\x02\x32\
	\u{128}\x03\x02\x02\x02\x34\u{12a}\x03\x02\x02\x02\x36\u{130}\x03\x02\x02\
	\x02\x38\u{133}\x03\x02\x02\x02\x3a\u{136}\x03\x02\x02\x02\x3c\u{13b}\x03\
	\x02\x02\x02\x3e\u{14b}\x03\x02\x02\x02\x40\u{159}\x03\x02\x02\x02\x42\u{15b}\
	\x03\x02\x02\x02\x44\u{167}\x03\x02\x02\x02\x46\u{16e}\x03\x02\x02\x02\x48\
	\u{176}\x03\x02\x02\x02\x4a\u{181}\x03\x02\x02\x02\x4c\u{18a}\x03\x02\x02\
	\x02\x4e\u{192}\x03\x02\x02\x02\x50\u{19a}\x03\x02\x02\x02\x52\u{1a2}\x03\
	\x02\x02\x02\x54\u{1ac}\x03\x02\x02\x02\x56\u{1b3}\x03\x02\x02\x02\x58\u{1b6}\
	\x03\x02\x02\x02\x5a\u{1bd}\x03\x02\x02\x02\x5c\u{1c9}\x03\x02\x02\x02\x5e\
	\u{1cb}\x03\x02\x02\x02\x60\u{1d3}\x03\x02\x02\x02\x62\u{1d5}\x03\x02\x02\
	\x02\x64\u{1d9}\x03\x02\x02\x02\x66\u{1de}\x03\x02\x02\x02\x68\u{1e6}\x03\
	\x02\x02\x02\x6a\x6c\x05\x04\x03\x02\x6b\x6a\x03\x02\x02\x02\x6c\x6f\x03\
	\x02\x02\x02\x6d\x6b\x03\x02\x02\x02\x6d\x6e\x03\x02\x02\x02\x6e\x70\x03\
	\x02\x02\x02\x6f\x6d\x03\x02\x02\x02\x70\x71\x05\x0c\x07\x02\x71\x72\x07\
	\x02\x02\x03\x72\x03\x03\x02\x02\x02\x73\x74\x07\x14\x02\x02\x74\x75\x05\
	\x06\x04\x02\x75\x05\x03\x02\x02\x02\x76\x78\x09\x02\x02\x02\x77\x76\x03\
	\x02\x02\x02\x77\x78\x03\x02\x02\x02\x78\x79\x03\x02\x02\x02\x79\x7e\x05\
	\x08\x05\x02\x7a\x7b\x07\x3e\x02\x02\x7b\x7d\x05\x08\x05\x02\x7c\x7a\x03\
	\x02\x02\x02\x7d\u{80}\x03\x02\x02\x02\x7e\x7c\x03\x02\x02\x02\x7e\x7f\x03\
	\x02\x02\x02\x7f\u{81}\x03\x02\x02\x02\u{80}\x7e\x03\x02\x02\x02\u{81}\u{82}\
	\x07\x3c\x02\x02\u{82}\u{83}\x05\x0a\x06\x02\u{83}\x07\x03\x02\x02\x02\u{84}\
	\u{89}\x05\x0a\x06\x02\u{85}\u{86}\x07\x3f\x02\x02\u{86}\u{88}\x05\x0a\x06\
	\x02\u{87}\u{85}\x03\x02\x02\x02\u{88}\u{8b}\x03\x02\x02\x02\u{89}\u{87}\
	\x03\x02\x02\x02\u{89}\u{8a}\x03\x02\x02\x02\u{8a}\x09\x03\x02\x02\x02\u{8b}\
	\u{89}\x03\x02\x02\x02\u{8c}\u{8d}\x09\x03\x02\x02\u{8d}\x0b\x03\x02\x02\
	\x02\u{8e}\u{8f}\x07\x24\x02\x02\u{8f}\u{90}\x07\x3b\x02\x02\u{90}\u{91}\
	\x05\x0e\x08\x02\u{91}\u{92}\x07\x0d\x02\x02\u{92}\x0d\x03\x02\x02\x02\u{93}\
	\u{95}\x05\x10\x09\x02\u{94}\u{93}\x03\x02\x02\x02\u{95}\u{98}\x03\x02\x02\
	\x02\u{96}\u{94}\x03\x02\x02\x02\u{96}\u{97}\x03\x02\x02\x02\u{97}\x0f\x03\
	\x02\x02\x02\u{98}\u{96}\x03\x02\x02\x02\u{99}\u{a2}\x05\x12\x0a\x02\u{9a}\
	\u{a2}\x05\x14\x0b\x02\u{9b}\u{a2}\x05\x16\x0c\x02\u{9c}\u{a2}\x05\x18\x0d\
	\x02\u{9d}\u{a2}\x05\x1e\x10\x02\u{9e}\u{a2}\x05\x4e\x28\x02\u{9f}\u{a2}\
	\x05\x52\x2a\x02\u{a0}\u{a2}\x05\x56\x2c\x02\u{a1}\u{99}\x03\x02\x02\x02\
	\u{a1}\u{9a}\x03\x02\x02\x02\u{a1}\u{9b}\x03\x02\x02\x02\u{a1}\u{9c}\x03\
	\x02\x02\x02\u{a1}\u{9d}\x03\x02\x02\x02\u{a1}\u{9e}\x03\x02\x02\x02\u{a1}\
	\u{9f}\x03\x02\x02\x02\u{a1}\u{a0}\x03\x02\x02\x02\u{a2}\x11\x03\x02\x02\
	\x02\u{a3}\u{a4}\x07\x0b\x02\x02\u{a4}\u{a5}\x07\x3a\x02\x02\u{a5}\x13\x03\
	\x02\x02\x02\u{a6}\u{a7}\x07\x13\x02\x02\u{a7}\u{a8}\x05\x66\x34\x02\u{a8}\
	\x15\x03\x02\x02\x02\u{a9}\u{aa}\x07\x09\x02\x02\u{aa}\u{ab}\x05\x66\x34\
	\x02\u{ab}\x17\x03\x02\x02\x02\u{ac}\u{b0}\x07\x08\x02\x02\u{ad}\u{af}\x05\
	\x1a\x0e\x02\u{ae}\u{ad}\x03\x02\x02\x02\u{af}\u{b2}\x03\x02\x02\x02\u{b0}\
	\u{ae}\x03\x02\x02\x02\u{b0}\u{b1}\x03\x02\x02\x02\u{b1}\u{b3}\x03\x02\x02\
	\x02\u{b2}\u{b0}\x03\x02\x02\x02\u{b3}\u{b4}\x07\x0d\x02\x02\u{b4}\x19\x03\
	\x02\x02\x02\u{b5}\u{b6}\x07\x3b\x02\x02\u{b6}\u{bb}\x05\x1c\x0f\x02\u{b7}\
	\u{b8}\x07\x3d\x02\x02\u{b8}\u{ba}\x05\x1c\x0f\x02\u{b9}\u{b7}\x03\x02\x02\
	\x02\u{ba}\u{bd}\x03\x02\x02\x02\u{bb}\u{b9}\x03\x02\x02\x02\u{bb}\u{bc}\
	\x03\x02\x02\x02\u{bc}\x1b\x03\x02\x02\x02\u{bd}\u{bb}\x03\x02\x02\x02\u{be}\
	\u{bf}\x09\x04\x02\x02\u{bf}\x1d\x03\x02\x02\x02\u{c0}\u{c1}\x07\x11\x02\
	\x02\u{c1}\u{c2}\x07\x3b\x02\x02\u{c2}\u{c3}\x05\x20\x11\x02\u{c3}\u{c4}\
	\x07\x0d\x02\x02\u{c4}\x1f\x03\x02\x02\x02\u{c5}\u{c7}\x05\x22\x12\x02\u{c6}\
	\u{c5}\x03\x02\x02\x02\u{c7}\u{ca}\x03\x02\x02\x02\u{c8}\u{c6}\x03\x02\x02\
	\x02\u{c8}\u{c9}\x03\x02\x02\x02\u{c9}\x21\x03\x02\x02\x02\u{ca}\u{c8}\x03\
	\x02\x02\x02\u{cb}\u{d8}\x05\x24\x13\x02\u{cc}\u{d8}\x05\x26\x14\x02\u{cd}\
	\u{d8}\x05\x28\x15\x02\u{ce}\u{d8}\x05\x2c\x17\x02\u{cf}\u{d8}\x05\x2e\x18\
	\x02\u{d0}\u{d8}\x05\x30\x19\x02\u{d1}\u{d8}\x05\x38\x1d\x02\u{d2}\u{d8}\
	\x05\x3a\x1e\x02\u{d3}\u{d8}\x05\x3c\x1f\x02\u{d4}\u{d8}\x05\x42\x22\x02\
	\u{d5}\u{d8}\x05\x46\x24\x02\u{d6}\u{d8}\x05\x4c\x27\x02\u{d7}\u{cb}\x03\
	\x02\x02\x02\u{d7}\u{cc}\x03\x02\x02\x02\u{d7}\u{cd}\x03\x02\x02\x02\u{d7}\
	\u{ce}\x03\x02\x02\x02\u{d7}\u{cf}\x03\x02\x02\x02\u{d7}\u{d0}\x03\x02\x02\
	\x02\u{d7}\u{d1}\x03\x02\x02\x02\u{d7}\u{d2}\x03\x02\x02\x02\u{d7}\u{d3}\
	\x03\x02\x02\x02\u{d7}\u{d4}\x03\x02\x02\x02\u{d7}\u{d5}\x03\x02\x02\x02\
	\u{d7}\u{d6}\x03\x02\x02\x02\u{d8}\x23\x03\x02\x02\x02\u{d9}\u{da}\x07\x03\
	\x02\x02\u{da}\u{db}\x07\x23\x02\x02\u{db}\u{dc}\x07\x3a\x02\x02\u{dc}\x25\
	\x03\x02\x02\x02\u{dd}\u{de}\x07\x2c\x02\x02\u{de}\u{df}\x05\x5c\x2f\x02\
	\u{df}\u{e0}\x07\x2b\x02\x02\u{e0}\u{e1}\x05\x62\x32\x02\u{e1}\x27\x03\x02\
	\x02\x02\u{e2}\u{e3}\x07\x19\x02\x02\u{e3}\u{e4}\x07\x3b\x02\x02\u{e4}\u{e5}\
	\x07\x10\x02\x02\u{e5}\u{e9}\x07\x3b\x02\x02\u{e6}\u{e8}\x05\x2a\x16\x02\
	\u{e7}\u{e6}\x03\x02\x02\x02\u{e8}\u{eb}\x03\x02\x02\x02\u{e9}\u{e7}\x03\
	\x02\x02\x02\u{e9}\u{ea}\x03\x02\x02\x02\u{ea}\u{ec}\x03\x02\x02\x02\u{eb}\
	\u{e9}\x03\x02\x02\x02\u{ec}\u{ed}\x07\x0d\x02\x02\u{ed}\x29\x03\x02\x02\
	\x02\u{ee}\u{ef}\x07\x2e\x02\x02\u{ef}\u{f0}\x05\x5c\x2f\x02\u{f0}\u{f1}\
	\x05\x60\x31\x02\u{f1}\u{f2}\x05\x5c\x2f\x02\u{f2}\x2b\x03\x02\x02\x02\u{f3}\
	\u{f4}\x07\x29\x02\x02\u{f4}\u{f5}\x05\x5e\x30\x02\u{f5}\u{f6}\x07\x2b\x02\
	\x02\u{f6}\u{f7}\x05\x62\x32\x02\u{f7}\u{f8}\x07\x16\x02\x02\u{f8}\u{f9}\
	\x07\x3b\x02\x02\u{f9}\u{101}\x03\x02\x02\x02\u{fa}\u{fb}\x07\x29\x02\x02\
	\u{fb}\u{fc}\x07\x2b\x02\x02\u{fc}\u{fd}\x05\x62\x32\x02\u{fd}\u{fe}\x07\
	\x16\x02\x02\u{fe}\u{ff}\x07\x3b\x02\x02\u{ff}\u{101}\x03\x02\x02\x02\u{100}\
	\u{f3}\x03\x02\x02\x02\u{100}\u{fa}\x03\x02\x02\x02\u{101}\x2d\x03\x02\x02\
	\x02\u{102}\u{103}\x07\x1e\x02\x02\u{103}\u{104}\x05\x5c\x2f\x02\u{104}\
	\u{105}\x07\x27\x02\x02\u{105}\u{106}\x07\x3b\x02\x02\u{106}\u{10f}\x03\
	\x02\x02\x02\u{107}\u{108}\x07\x1e\x02\x02\u{108}\u{109}\x05\x5c\x2f\x02\
	\u{109}\u{10a}\x07\x32\x02\x02\u{10a}\u{10b}\x05\x5c\x2f\x02\u{10b}\u{10c}\
	\x07\x27\x02\x02\u{10c}\u{10d}\x07\x3b\x02\x02\u{10d}\u{10f}\x03\x02\x02\
	\x02\u{10e}\u{102}\x03\x02\x02\x02\u{10e}\u{107}\x03\x02\x02\x02\u{10f}\
	\x2f\x03\x02\x02\x02\u{110}\u{111}\x07\x05\x02\x02\u{111}\u{115}\x05\x62\
	\x32\x02\u{112}\u{114}\x05\x32\x1a\x02\u{113}\u{112}\x03\x02\x02\x02\u{114}\
	\u{117}\x03\x02\x02\x02\u{115}\u{113}\x03\x02\x02\x02\u{115}\u{116}\x03\
	\x02\x02\x02\u{116}\u{119}\x03\x02\x02\x02\u{117}\u{115}\x03\x02\x02\x02\
	\u{118}\u{11a}\x05\x34\x1b\x02\u{119}\u{118}\x03\x02\x02\x02\u{119}\u{11a}\
	\x03\x02\x02\x02\u{11a}\u{11c}\x03\x02\x02\x02\u{11b}\u{11d}\x05\x36\x1c\
	\x02\u{11c}\u{11b}\x03\x02\x02\x02\u{11c}\u{11d}\x03\x02\x02\x02\u{11d}\
	\u{11f}\x03\x02\x02\x02\u{11e}\u{120}\x07\x0d\x02\x02\u{11f}\u{11e}\x03\
	\x02\x02\x02\u{11f}\u{120}\x03\x02\x02\x02\u{120}\x31\x03\x02\x02\x02\u{121}\
	\u{122}\x07\x2f\x02\x02\u{122}\u{123}\x07\x3b\x02\x02\u{123}\u{124}\x07\
	\x32\x02\x02\u{124}\u{129}\x05\x5c\x2f\x02\u{125}\u{126}\x07\x2f\x02\x02\
	\u{126}\u{127}\x07\x3b\x02\x02\u{127}\u{129}\x07\x3a\x02\x02\u{128}\u{121}\
	\x03\x02\x02\x02\u{128}\u{125}\x03\x02\x02\x02\u{129}\x33\x03\x02\x02\x02\
	\u{12a}\u{12b}\x07\x0f\x02\x02\u{12b}\u{12c}\x07\x0c\x02\x02\u{12c}\u{12d}\
	\x07\x3b\x02\x02\u{12d}\u{12e}\x07\x15\x02\x02\u{12e}\u{12f}\x05\x5c\x2f\
	\x02\u{12f}\x35\x03\x02\x02\x02\u{130}\u{131}\x07\x16\x02\x02\u{131}\u{132}\
	\x07\x3b\x02\x02\u{132}\x37\x03\x02\x02\x02\u{133}\u{134}\x07\x1f\x02\x02\
	\u{134}\u{135}\x07\x3b\x02\x02\u{135}\x39\x03\x02\x02\x02\u{136}\u{137}\
	\x07\x21\x02\x02\u{137}\u{139}\x07\x38\x02\x02\u{138}\u{13a}\x05\x64\x33\
	\x02\u{139}\u{138}\x03\x02\x02\x02\u{139}\u{13a}\x03\x02\x02\x02\u{13a}\
	\x3b\x03\x02\x02\x02\u{13b}\u{13d}\x07\x1b\x02\x02\u{13c}\u{13e}\x05\x3e\
	\x20\x02\u{13d}\u{13c}\x03\x02\x02\x02\u{13e}\u{13f}\x03\x02\x02\x02\u{13f}\
	\u{13d}\x03\x02\x02\x02\u{13f}\u{140}\x03\x02\x02\x02\u{140}\u{141}\x03\
	\x02\x02\x02\u{141}\u{142}\x07\x0d\x02\x02\u{142}\x3d\x03\x02\x02\x02\u{143}\
	\u{144}\x07\x38\x02\x02\u{144}\u{145}\x07\x2d\x02\x02\u{145}\u{14c}\x05\
	\x40\x21\x02\u{146}\u{147}\x07\x0e\x02\x02\u{147}\u{148}\x05\x5c\x2f\x02\
	\u{148}\u{149}\x07\x27\x02\x02\u{149}\u{14a}\x07\x3b\x02\x02\u{14a}\u{14c}\
	\x03\x02\x02\x02\u{14b}\u{143}\x03\x02\x02\x02\u{14b}\u{146}\x03\x02\x02\
	\x02\u{14c}\x3f\x03\x02\x02\x02\u{14d}\u{14e}\x05\x5c\x2f\x02\u{14e}\u{14f}\
	\x07\x18\x02\x02\u{14f}\u{150}\x07\x1a\x02\x02\u{150}\u{15a}\x03\x02\x02\
	\x02\u{151}\u{152}\x05\x5c\x2f\x02\u{152}\u{153}\x05\x60\x31\x02\u{153}\
	\u{154}\x05\x5c\x2f\x02\u{154}\u{15a}\x03\x02\x02\x02\u{155}\u{156}\x05\
	\x5c\x2f\x02\u{156}\u{157}\x07\x0a\x02\x02\u{157}\u{158}\x09\x05\x02\x02\
	\u{158}\u{15a}\x03\x02\x02\x02\u{159}\u{14d}\x03\x02\x02\x02\u{159}\u{151}\
	\x03\x02\x02\x02\u{159}\u{155}\x03\x02\x02\x02\u{15a}\x41\x03\x02\x02\x02\
	\u{15b}\u{15f}\x07\x28\x02\x02\u{15c}\u{15e}\x05\x22\x12\x02\u{15d}\u{15c}\
	\x03\x02\x02\x02\u{15e}\u{161}\x03\x02\x02\x02\u{15f}\u{15d}\x03\x02\x02\
	\x02\u{15f}\u{160}\x03\x02\x02\x02\u{160}\u{163}\x03\x02\x02\x02\u{161}\
	\u{15f}\x03\x02\x02\x02\u{162}\u{164}\x05\x44\x23\x02\u{163}\u{162}\x03\
	\x02\x02\x02\u{163}\u{164}\x03\x02\x02\x02\u{164}\u{165}\x03\x02\x02\x02\
	\u{165}\u{166}\x07\x0d\x02\x02\u{166}\x43\x03\x02\x02\x02\u{167}\u{16b}\
	\x07\x1c\x02\x02\u{168}\u{16a}\x05\x22\x12\x02\u{169}\u{168}\x03\x02\x02\
	\x02\u{16a}\u{16d}\x03\x02\x02\x02\u{16b}\u{169}\x03\x02\x02\x02\u{16b}\
	\u{16c}\x03\x02\x02\x02\u{16c}\x45\x03\x02\x02\x02\u{16d}\u{16b}\x03\x02\
	\x02\x02\u{16e}\u{170}\x07\x22\x02\x02\u{16f}\u{171}\x05\x48\x25\x02\u{170}\
	\u{16f}\x03\x02\x02\x02\u{171}\u{172}\x03\x02\x02\x02\u{172}\u{170}\x03\
	\x02\x02\x02\u{172}\u{173}\x03\x02\x02\x02\u{173}\u{174}\x03\x02\x02\x02\
	\u{174}\u{175}\x07\x0d\x02\x02\u{175}\x47\x03\x02\x02\x02\u{176}\u{177}\
	\x07\x25\x02\x02\u{177}\u{17b}\x07\x3b\x02\x02\u{178}\u{17a}\x05\x22\x12\
	\x02\u{179}\u{178}\x03\x02\x02\x02\u{17a}\u{17d}\x03\x02\x02\x02\u{17b}\
	\u{179}\x03\x02\x02\x02\u{17b}\u{17c}\x03\x02\x02\x02\u{17c}\u{17e}\x03\
	\x02\x02\x02\u{17d}\u{17b}\x03\x02\x02\x02\u{17e}\u{17f}\x05\x4a\x26\x02\
	\u{17f}\u{180}\x07\x0d\x02\x02\u{180}\x49\x03\x02\x02\x02\u{181}\u{185}\
	\x07\x07\x02\x02\u{182}\u{184}\x05\x22\x12\x02\u{183}\u{182}\x03\x02\x02\
	\x02\u{184}\u{187}\x03\x02\x02\x02\u{185}\u{183}\x03\x02\x02\x02\u{185}\
	\u{186}\x03\x02\x02\x02\u{186}\u{188}\x03\x02\x02\x02\u{187}\u{185}\x03\
	\x02\x02\x02\u{188}\u{189}\x07\x0d\x02\x02\u{189}\x4b\x03\x02\x02\x02\u{18a}\
	\u{18c}\x07\x1d\x02\x02\u{18b}\u{18d}\x05\x22\x12\x02\u{18c}\u{18b}\x03\
	\x02\x02\x02\u{18d}\u{18e}\x03\x02\x02\x02\u{18e}\u{18c}\x03\x02\x02\x02\
	\u{18e}\u{18f}\x03\x02\x02\x02\u{18f}\u{190}\x03\x02\x02\x02\u{190}\u{191}\
	\x07\x0d\x02\x02\u{191}\x4d\x03\x02\x02\x02\u{192}\u{194}\x07\x04\x02\x02\
	\u{193}\u{195}\x05\x50\x29\x02\u{194}\u{193}\x03\x02\x02\x02\u{195}\u{196}\
	\x03\x02\x02\x02\u{196}\u{194}\x03\x02\x02\x02\u{196}\u{197}\x03\x02\x02\
	\x02\u{197}\u{198}\x03\x02\x02\x02\u{198}\u{199}\x07\x0d\x02\x02\u{199}\
	\x4f\x03\x02\x02\x02\u{19a}\u{19b}\x07\x3b\x02\x02\u{19b}\u{19c}\x07\x2a\
	\x02\x02\u{19c}\u{19d}\x05\x5a\x2e\x02\u{19d}\u{1a0}\x07\x3b\x02\x02\u{19e}\
	\u{19f}\x07\x17\x02\x02\u{19f}\u{1a1}\x05\x66\x34\x02\u{1a0}\u{19e}\x03\
	\x02\x02\x02\u{1a0}\u{1a1}\x03\x02\x02\x02\u{1a1}\x51\x03\x02\x02\x02\u{1a2}\
	\u{1a3}\x07\x12\x02\x02\u{1a3}\u{1a7}\x07\x3a\x02\x02\u{1a4}\u{1a6}\x05\
	\x54\x2b\x02\u{1a5}\u{1a4}\x03\x02\x02\x02\u{1a6}\u{1a9}\x03\x02\x02\x02\
	\u{1a7}\u{1a5}\x03\x02\x02\x02\u{1a7}\u{1a8}\x03\x02\x02\x02\u{1a8}\u{1aa}\
	\x03\x02\x02\x02\u{1a9}\u{1a7}\x03\x02\x02\x02\u{1aa}\u{1ab}\x07\x0d\x02\
	\x02\u{1ab}\x53\x03\x02\x02\x02\u{1ac}\u{1ad}\x07\x06\x02\x02\u{1ad}\u{1ae}\
	\x05\x0a\x06\x02\u{1ae}\u{1af}\x05\x0a\x06\x02\u{1af}\u{1b0}\x07\x26\x02\
	\x02\u{1b0}\u{1b1}\x05\x5a\x2e\x02\u{1b1}\u{1b2}\x05\x0a\x06\x02\u{1b2}\
	\x55\x03\x02\x02\x02\u{1b3}\u{1b4}\x07\x20\x02\x02\u{1b4}\u{1b5}\x07\x3a\
	\x02\x02\u{1b5}\x57\x03\x02\x02\x02\u{1b6}\u{1b7}\x07\x3b\x02\x02\u{1b7}\
	\u{1b8}\x07\x3c\x02\x02\u{1b8}\u{1b9}\x07\x3b\x02\x02\u{1b9}\x59\x03\x02\
	\x02\x02\u{1ba}\u{1be}\x07\x38\x02\x02\u{1bb}\u{1be}\x07\x39\x02\x02\u{1bc}\
	\u{1be}\x05\x58\x2d\x02\u{1bd}\u{1ba}\x03\x02\x02\x02\u{1bd}\u{1bb}\x03\
	\x02\x02\x02\u{1bd}\u{1bc}\x03\x02\x02\x02\u{1be}\x5b\x03\x02\x02\x02\u{1bf}\
	\u{1ca}\x05\x68\x35\x02\u{1c0}\u{1ca}\x05\x58\x2d\x02\u{1c1}\u{1c6}\x07\
	\x3b\x02\x02\u{1c2}\u{1c3}\x07\x3c\x02\x02\u{1c3}\u{1c5}\x05\x0a\x06\x02\
	\u{1c4}\u{1c2}\x03\x02\x02\x02\u{1c5}\u{1c8}\x03\x02\x02\x02\u{1c6}\u{1c4}\
	\x03\x02\x02\x02\u{1c6}\u{1c7}\x03\x02\x02\x02\u{1c7}\u{1ca}\x03\x02\x02\
	\x02\u{1c8}\u{1c6}\x03\x02\x02\x02\u{1c9}\u{1bf}\x03\x02\x02\x02\u{1c9}\
	\u{1c0}\x03\x02\x02\x02\u{1c9}\u{1c1}\x03\x02\x02\x02\u{1ca}\x5d\x03\x02\
	\x02\x02\u{1cb}\u{1d0}\x05\x5c\x2f\x02\u{1cc}\u{1cd}\x07\x3d\x02\x02\u{1cd}\
	\u{1cf}\x05\x5c\x2f\x02\u{1ce}\u{1cc}\x03\x02\x02\x02\u{1cf}\u{1d2}\x03\
	\x02\x02\x02\u{1d0}\u{1ce}\x03\x02\x02\x02\u{1d0}\u{1d1}\x03\x02\x02\x02\
	\u{1d1}\x5f\x03\x02\x02\x02\u{1d2}\u{1d0}\x03\x02\x02\x02\u{1d3}\u{1d4}\
	\x09\x06\x02\x02\u{1d4}\x61\x03\x02\x02\x02\u{1d5}\u{1d6}\x07\x3b\x02\x02\
	\u{1d6}\u{1d7}\x07\x3c\x02\x02\u{1d7}\u{1d8}\x07\x3b\x02\x02\u{1d8}\x63\
	\x03\x02\x02\x02\u{1d9}\u{1dc}\x07\x3b\x02\x02\u{1da}\u{1db}\x07\x3c\x02\
	\x02\u{1db}\u{1dd}\x07\x3b\x02\x02\u{1dc}\u{1da}\x03\x02\x02\x02\u{1dc}\
	\u{1dd}\x03\x02\x02\x02\u{1dd}\x65\x03\x02\x02\x02\u{1de}\u{1e3}\x07\x3b\
	\x02\x02\u{1df}\u{1e0}\x07\x3d\x02\x02\u{1e0}\u{1e2}\x07\x3b\x02\x02\u{1e1}\
	\u{1df}\x03\x02\x02\x02\u{1e2}\u{1e5}\x03\x02\x02\x02\u{1e3}\u{1e1}\x03\
	\x02\x02\x02\u{1e3}\u{1e4}\x03\x02\x02\x02\u{1e4}\x67\x03\x02\x02\x02\u{1e5}\
	\u{1e3}\x03\x02\x02\x02\u{1e6}\u{1e7}\x09\x07\x02\x02\u{1e7}\x69\x03\x02\
	\x02\x02\x28\x6d\x77\x7e\u{89}\u{96}\u{a1}\u{b0}\u{bb}\u{c8}\u{d7}\u{e9}\
	\u{100}\u{10e}\u{115}\u{119}\u{11c}\u{11f}\u{128}\u{139}\u{13f}\u{14b}\u{159}\
	\u{15f}\u{163}\u{16b}\u{172}\u{17b}\u{185}\u{18e}\u{196}\u{1a0}\u{1a7}\u{1bd}\
	\u{1c6}\u{1c9}\u{1d0}\u{1dc}\u{1e3}";

