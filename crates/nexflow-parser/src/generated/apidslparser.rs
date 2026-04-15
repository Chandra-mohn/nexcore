// Generated from /Users/chandramohn/workspace/nexcore/grammar/nexflow/ApiDSL.g4 by ANTLR 4.8
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
use super::apidsllistener::*;
use super::apidslvisitor::*;

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

		pub const API:isize=1; 
		pub const ASYNC:isize=2; 
		pub const AUTH:isize=3; 
		pub const BASE:isize=4; 
		pub const BURST:isize=5; 
		pub const CACHE:isize=6; 
		pub const CONFIG:isize=7; 
		pub const CONTENT_TYPE:isize=8; 
		pub const CORS:isize=9; 
		pub const DEFAULT:isize=10; 
		pub const DEFAULT_SIZE:isize=11; 
		pub const DEPENDENCIES:isize=12; 
		pub const DEPRECATED:isize=13; 
		pub const DESCRIPTION:isize=14; 
		pub const END:isize=15; 
		pub const ENDPOINT:isize=16; 
		pub const ERRORS:isize=17; 
		pub const EVENT:isize=18; 
		pub const HEADER:isize=19; 
		pub const HEADERS:isize=20; 
		pub const HEALTH:isize=21; 
		pub const IDEMPOTENT:isize=22; 
		pub const IMPORT:isize=23; 
		pub const KEY:isize=24; 
		pub const LIST:isize=25; 
		pub const MAX_AGE:isize=26; 
		pub const MAX_SIZE:isize=27; 
		pub const METHOD:isize=28; 
		pub const METHODS:isize=29; 
		pub const OPTIONAL:isize=30; 
		pub const ORIGINS:isize=31; 
		pub const PAGINATED:isize=32; 
		pub const PAGINATION:isize=33; 
		pub const PARAMS:isize=34; 
		pub const PARTITIONED_BY:isize=35; 
		pub const PATH:isize=36; 
		pub const PAYLOAD:isize=37; 
		pub const PER:isize=38; 
		pub const QUERY:isize=39; 
		pub const RATE_LIMIT:isize=40; 
		pub const READY:isize=41; 
		pub const REPLACEMENT:isize=42; 
		pub const REQUEST:isize=43; 
		pub const REQUIRED:isize=44; 
		pub const RESPONSE:isize=45; 
		pub const SCOPE:isize=46; 
		pub const SUNSET:isize=47; 
		pub const TARGETS:isize=48; 
		pub const TIMEOUT:isize=49; 
		pub const TOPIC:isize=50; 
		pub const USING:isize=51; 
		pub const VALIDATE:isize=52; 
		pub const VERSION:isize=53; 
		pub const GET:isize=54; 
		pub const POST:isize=55; 
		pub const PUT:isize=56; 
		pub const PATCH:isize=57; 
		pub const DELETE:isize=58; 
		pub const TRUE:isize=59; 
		pub const FALSE:isize=60; 
		pub const INTEGER:isize=61; 
		pub const DECIMAL_LITERAL:isize=62; 
		pub const STRING_LITERAL:isize=63; 
		pub const IDENTIFIER:isize=64; 
		pub const DOT:isize=65; 
		pub const COMMA:isize=66; 
		pub const SLASH:isize=67; 
		pub const DASH:isize=68; 
		pub const DOT_SLASH:isize=69; 
		pub const DOT_DOT_SLASH:isize=70; 
		pub const LINE_COMMENT:isize=71; 
		pub const WS:isize=72;
	pub const RULE_compilationUnit:usize = 0; 
	pub const RULE_importStatement:usize = 1; 
	pub const RULE_importPath:usize = 2; 
	pub const RULE_pathSegment:usize = 3; 
	pub const RULE_word:usize = 4; 
	pub const RULE_apiDefinition:usize = 5; 
	pub const RULE_apiBody:usize = 6; 
	pub const RULE_apiElement:usize = 7; 
	pub const RULE_versionDecl:usize = 8; 
	pub const RULE_baseDecl:usize = 9; 
	pub const RULE_descriptionDecl:usize = 10; 
	pub const RULE_targetsDecl:usize = 11; 
	pub const RULE_authDefaultDecl:usize = 12; 
	pub const RULE_contentTypeDecl:usize = 13; 
	pub const RULE_rateLimitDecl:usize = 14; 
	pub const RULE_paginationDecl:usize = 15; 
	pub const RULE_configBlock:usize = 16; 
	pub const RULE_configDirective:usize = 17; 
	pub const RULE_configValue:usize = 18; 
	pub const RULE_corsBlock:usize = 19; 
	pub const RULE_corsDirective:usize = 20; 
	pub const RULE_endpointDecl:usize = 21; 
	pub const RULE_deprecatedEndpointDecl:usize = 22; 
	pub const RULE_endpointBody:usize = 23; 
	pub const RULE_endpointClause:usize = 24; 
	pub const RULE_methodDecl:usize = 25; 
	pub const RULE_httpMethod:usize = 26; 
	pub const RULE_pathDecl:usize = 27; 
	pub const RULE_paramsBlock:usize = 28; 
	pub const RULE_queryBlock:usize = 29; 
	pub const RULE_headersBlock:usize = 30; 
	pub const RULE_paramDecl:usize = 31; 
	pub const RULE_paramModifier:usize = 32; 
	pub const RULE_requestDecl:usize = 33; 
	pub const RULE_responseDecl:usize = 34; 
	pub const RULE_responseModifier:usize = 35; 
	pub const RULE_errorsBlock:usize = 36; 
	pub const RULE_errorMapping:usize = 37; 
	pub const RULE_authDecl:usize = 38; 
	pub const RULE_authScheme:usize = 39; 
	pub const RULE_authSchemeArg:usize = 40; 
	pub const RULE_validateDecl:usize = 41; 
	pub const RULE_timeoutDecl:usize = 42; 
	pub const RULE_cacheDecl:usize = 43; 
	pub const RULE_idempotentDecl:usize = 44; 
	pub const RULE_asyncDecl:usize = 45; 
	pub const RULE_sunsetDecl:usize = 46; 
	pub const RULE_replacementDecl:usize = 47; 
	pub const RULE_eventDecl:usize = 48; 
	pub const RULE_eventBody:usize = 49; 
	pub const RULE_eventClause:usize = 50; 
	pub const RULE_topicDecl:usize = 51; 
	pub const RULE_payloadDecl:usize = 52; 
	pub const RULE_partitionedByDecl:usize = 53; 
	pub const RULE_dependenciesBlock:usize = 54; 
	pub const RULE_dependencyDecl:usize = 55; 
	pub const RULE_healthDecl:usize = 56; 
	pub const RULE_readyDecl:usize = 57; 
	pub const RULE_qualifiedAnnotation:usize = 58; 
	pub const RULE_valueOrCfg:usize = 59; 
	pub const RULE_valueOrCfgList:usize = 60; 
	pub const RULE_schemaRef:usize = 61; 
	pub const RULE_qualifiedRef:usize = 62; 
	pub const RULE_typeRef:usize = 63; 
	pub const RULE_identifierList:usize = 64; 
	pub const RULE_stringList:usize = 65; 
	pub const RULE_httpMethodList:usize = 66; 
	pub const RULE_literal:usize = 67;
	pub const ruleNames: [&'static str; 68] =  [
		"compilationUnit", "importStatement", "importPath", "pathSegment", "word", 
		"apiDefinition", "apiBody", "apiElement", "versionDecl", "baseDecl", "descriptionDecl", 
		"targetsDecl", "authDefaultDecl", "contentTypeDecl", "rateLimitDecl", 
		"paginationDecl", "configBlock", "configDirective", "configValue", "corsBlock", 
		"corsDirective", "endpointDecl", "deprecatedEndpointDecl", "endpointBody", 
		"endpointClause", "methodDecl", "httpMethod", "pathDecl", "paramsBlock", 
		"queryBlock", "headersBlock", "paramDecl", "paramModifier", "requestDecl", 
		"responseDecl", "responseModifier", "errorsBlock", "errorMapping", "authDecl", 
		"authScheme", "authSchemeArg", "validateDecl", "timeoutDecl", "cacheDecl", 
		"idempotentDecl", "asyncDecl", "sunsetDecl", "replacementDecl", "eventDecl", 
		"eventBody", "eventClause", "topicDecl", "payloadDecl", "partitionedByDecl", 
		"dependenciesBlock", "dependencyDecl", "healthDecl", "readyDecl", "qualifiedAnnotation", 
		"valueOrCfg", "valueOrCfgList", "schemaRef", "qualifiedRef", "typeRef", 
		"identifierList", "stringList", "httpMethodList", "literal"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;71] = [
		None, Some("'api'"), Some("'async'"), Some("'auth'"), Some("'base'"), 
		Some("'burst'"), Some("'cache'"), Some("'config'"), Some("'content_type'"), 
		Some("'cors'"), Some("'default'"), Some("'default_size'"), Some("'dependencies'"), 
		Some("'deprecated'"), Some("'description'"), Some("'end'"), Some("'endpoint'"), 
		Some("'errors'"), Some("'event'"), Some("'header'"), Some("'headers'"), 
		Some("'health'"), Some("'idempotent'"), Some("'import'"), Some("'key'"), 
		Some("'list'"), Some("'max_age'"), Some("'max_size'"), Some("'method'"), 
		Some("'methods'"), Some("'optional'"), Some("'origins'"), Some("'paginated'"), 
		Some("'pagination'"), Some("'params'"), Some("'partitioned_by'"), Some("'path'"), 
		Some("'payload'"), Some("'per'"), Some("'query'"), Some("'rate_limit'"), 
		Some("'ready'"), Some("'replacement'"), Some("'request'"), Some("'required'"), 
		Some("'response'"), Some("'scope'"), Some("'sunset'"), Some("'targets'"), 
		Some("'timeout'"), Some("'topic'"), Some("'using'"), Some("'validate'"), 
		Some("'version'"), Some("'GET'"), Some("'POST'"), Some("'PUT'"), Some("'PATCH'"), 
		Some("'DELETE'"), Some("'true'"), Some("'false'"), None, None, None, None, 
		Some("'.'"), Some("','"), Some("'/'"), Some("'-'"), Some("'./'"), Some("'../'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;73]  = [
		None, Some("API"), Some("ASYNC"), Some("AUTH"), Some("BASE"), Some("BURST"), 
		Some("CACHE"), Some("CONFIG"), Some("CONTENT_TYPE"), Some("CORS"), Some("DEFAULT"), 
		Some("DEFAULT_SIZE"), Some("DEPENDENCIES"), Some("DEPRECATED"), Some("DESCRIPTION"), 
		Some("END"), Some("ENDPOINT"), Some("ERRORS"), Some("EVENT"), Some("HEADER"), 
		Some("HEADERS"), Some("HEALTH"), Some("IDEMPOTENT"), Some("IMPORT"), Some("KEY"), 
		Some("LIST"), Some("MAX_AGE"), Some("MAX_SIZE"), Some("METHOD"), Some("METHODS"), 
		Some("OPTIONAL"), Some("ORIGINS"), Some("PAGINATED"), Some("PAGINATION"), 
		Some("PARAMS"), Some("PARTITIONED_BY"), Some("PATH"), Some("PAYLOAD"), 
		Some("PER"), Some("QUERY"), Some("RATE_LIMIT"), Some("READY"), Some("REPLACEMENT"), 
		Some("REQUEST"), Some("REQUIRED"), Some("RESPONSE"), Some("SCOPE"), Some("SUNSET"), 
		Some("TARGETS"), Some("TIMEOUT"), Some("TOPIC"), Some("USING"), Some("VALIDATE"), 
		Some("VERSION"), Some("GET"), Some("POST"), Some("PUT"), Some("PATCH"), 
		Some("DELETE"), Some("TRUE"), Some("FALSE"), Some("INTEGER"), Some("DECIMAL_LITERAL"), 
		Some("STRING_LITERAL"), Some("IDENTIFIER"), Some("DOT"), Some("COMMA"), 
		Some("SLASH"), Some("DASH"), Some("DOT_SLASH"), Some("DOT_DOT_SLASH"), 
		Some("LINE_COMMENT"), Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,ApiDSLParserExt<'input>, I, ApiDSLParserContextType , dyn ApiDSLListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type ApiDSLTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, ApiDSLParserContextType , dyn ApiDSLListener<'input> + 'a>;

/// Parser for ApiDSL grammar
pub struct ApiDSLParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> ApiDSLParser<'input, I, H>
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
				ApiDSLParserExt{
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

impl<'input, I> ApiDSLParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> ApiDSLParser<'input, I, DefaultErrorStrategy<'input,ApiDSLParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for ApiDSLParser
pub trait ApiDSLParserContext<'input>:
	for<'x> Listenable<dyn ApiDSLListener<'input> + 'x > + 
	for<'x> Visitable<dyn ApiDSLVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=ApiDSLParserContextType>
{}

antlr_rust::coerce_from!{ 'input : ApiDSLParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn ApiDSLParserContext<'input> + 'input
where
    T: ApiDSLVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn ApiDSLVisitor<'input> + 'x))
    }
}

impl<'input> ApiDSLParserContext<'input> for TerminalNode<'input,ApiDSLParserContextType> {}
impl<'input> ApiDSLParserContext<'input> for ErrorNode<'input,ApiDSLParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn ApiDSLParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn ApiDSLListener<'input> + 'input }

pub struct ApiDSLParserContextType;
antlr_rust::tid!{ApiDSLParserContextType}

impl<'input> ParserNodeType<'input> for ApiDSLParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn ApiDSLParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct ApiDSLParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> ApiDSLParserExt<'input>{
}
antlr_rust::tid! { ApiDSLParserExt<'a> }

impl<'input> TokenAware<'input> for ApiDSLParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for ApiDSLParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for ApiDSLParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "ApiDSL.g4"}

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

impl<'input> ApiDSLParserContext<'input> for CompilationUnitContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for CompilationUnitContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_compilationUnit(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_compilationUnit(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for CompilationUnitContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_compilationUnit(self);
	}
}

impl<'input> CustomRuleContext<'input> for CompilationUnitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compilationUnit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compilationUnit }
}
antlr_rust::tid!{CompilationUnitContextExt<'a>}

impl<'input> CompilationUnitContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CompilationUnitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompilationUnitContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CompilationUnitContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<CompilationUnitContextExt<'input>>{

fn apiDefinition(&self) -> Option<Rc<ApiDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
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

impl<'input, I, H> ApiDSLParser<'input, I, H>
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
			recog.base.set_state(139);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==IMPORT {
				{
				{
				/*InvokeRule importStatement*/
				recog.base.set_state(136);
				recog.importStatement()?;

				}
				}
				recog.base.set_state(141);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule apiDefinition*/
			recog.base.set_state(142);
			recog.apiDefinition()?;

			recog.base.set_state(143);
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

impl<'input> ApiDSLParserContext<'input> for ImportStatementContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for ImportStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_importStatement(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_importStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for ImportStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_importStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importStatement }
}
antlr_rust::tid!{ImportStatementContextExt<'a>}

impl<'input> ImportStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportStatementContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<ImportStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IMPORT
/// Returns `None` if there is no child corresponding to token IMPORT
fn IMPORT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IMPORT, 0)
}
fn importPath(&self) -> Option<Rc<ImportPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ImportStatementContextAttrs<'input> for ImportStatementContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
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
			recog.base.set_state(145);
			recog.base.match_token(IMPORT,&mut recog.err_handler)?;

			/*InvokeRule importPath*/
			recog.base.set_state(146);
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

impl<'input> ApiDSLParserContext<'input> for ImportPathContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for ImportPathContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_importPath(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_importPath(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for ImportPathContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_importPath(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportPathContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importPath }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importPath }
}
antlr_rust::tid!{ImportPathContextExt<'a>}

impl<'input> ImportPathContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportPathContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportPathContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportPathContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<ImportPathContextExt<'input>>{

fn pathSegment_all(&self) ->  Vec<Rc<PathSegmentContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn pathSegment(&self, i: usize) -> Option<Rc<PathSegmentContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}
fn word(&self) -> Option<Rc<WordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token SLASH in current rule
fn SLASH_all(&self) -> Vec<Rc<TerminalNode<'input,ApiDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SLASH, starting from 0.
/// Returns `None` if number of children corresponding to token SLASH is less or equal than `i`.
fn SLASH(&self, i: usize) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(SLASH, i)
}
/// Retrieves first TerminalNode corresponding to token DOT_SLASH
/// Returns `None` if there is no child corresponding to token DOT_SLASH
fn DOT_SLASH(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT_SLASH, 0)
}
/// Retrieves first TerminalNode corresponding to token DOT_DOT_SLASH
/// Returns `None` if there is no child corresponding to token DOT_DOT_SLASH
fn DOT_DOT_SLASH(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT_DOT_SLASH, 0)
}

}

impl<'input> ImportPathContextAttrs<'input> for ImportPathContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
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
			recog.base.set_state(149);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==DOT_SLASH || _la==DOT_DOT_SLASH {
				{
				recog.base.set_state(148);
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
			recog.base.set_state(151);
			recog.pathSegment()?;

			recog.base.set_state(156);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==SLASH {
				{
				{
				recog.base.set_state(152);
				recog.base.match_token(SLASH,&mut recog.err_handler)?;

				/*InvokeRule pathSegment*/
				recog.base.set_state(153);
				recog.pathSegment()?;

				}
				}
				recog.base.set_state(158);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(159);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			/*InvokeRule word*/
			recog.base.set_state(160);
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

impl<'input> ApiDSLParserContext<'input> for PathSegmentContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for PathSegmentContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_pathSegment(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_pathSegment(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for PathSegmentContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_pathSegment(self);
	}
}

impl<'input> CustomRuleContext<'input> for PathSegmentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pathSegment }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pathSegment }
}
antlr_rust::tid!{PathSegmentContextExt<'a>}

impl<'input> PathSegmentContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PathSegmentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PathSegmentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PathSegmentContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<PathSegmentContextExt<'input>>{

fn word_all(&self) ->  Vec<Rc<WordContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn word(&self, i: usize) -> Option<Rc<WordContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token DASH in current rule
fn DASH_all(&self) -> Vec<Rc<TerminalNode<'input,ApiDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DASH, starting from 0.
/// Returns `None` if number of children corresponding to token DASH is less or equal than `i`.
fn DASH(&self, i: usize) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(DASH, i)
}

}

impl<'input> PathSegmentContextAttrs<'input> for PathSegmentContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
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
			recog.base.set_state(162);
			recog.word()?;

			recog.base.set_state(167);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==DASH {
				{
				{
				recog.base.set_state(163);
				recog.base.match_token(DASH,&mut recog.err_handler)?;

				/*InvokeRule word*/
				recog.base.set_state(164);
				recog.word()?;

				}
				}
				recog.base.set_state(169);
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

impl<'input> ApiDSLParserContext<'input> for WordContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for WordContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_word(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_word(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for WordContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_word(self);
	}
}

impl<'input> CustomRuleContext<'input> for WordContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_word }
	//fn type_rule_index() -> usize where Self: Sized { RULE_word }
}
antlr_rust::tid!{WordContextExt<'a>}

impl<'input> WordContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<WordContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,WordContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait WordContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<WordContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token API
/// Returns `None` if there is no child corresponding to token API
fn API(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(API, 0)
}
/// Retrieves first TerminalNode corresponding to token ASYNC
/// Returns `None` if there is no child corresponding to token ASYNC
fn ASYNC(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(ASYNC, 0)
}
/// Retrieves first TerminalNode corresponding to token AUTH
/// Returns `None` if there is no child corresponding to token AUTH
fn AUTH(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(AUTH, 0)
}
/// Retrieves first TerminalNode corresponding to token BASE
/// Returns `None` if there is no child corresponding to token BASE
fn BASE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(BASE, 0)
}
/// Retrieves first TerminalNode corresponding to token BURST
/// Returns `None` if there is no child corresponding to token BURST
fn BURST(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(BURST, 0)
}
/// Retrieves first TerminalNode corresponding to token CACHE
/// Returns `None` if there is no child corresponding to token CACHE
fn CACHE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(CACHE, 0)
}
/// Retrieves first TerminalNode corresponding to token CONFIG
/// Returns `None` if there is no child corresponding to token CONFIG
fn CONFIG(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(CONFIG, 0)
}
/// Retrieves first TerminalNode corresponding to token CONTENT_TYPE
/// Returns `None` if there is no child corresponding to token CONTENT_TYPE
fn CONTENT_TYPE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(CONTENT_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token CORS
/// Returns `None` if there is no child corresponding to token CORS
fn CORS(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(CORS, 0)
}
/// Retrieves first TerminalNode corresponding to token DEFAULT
/// Returns `None` if there is no child corresponding to token DEFAULT
fn DEFAULT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(DEFAULT, 0)
}
/// Retrieves first TerminalNode corresponding to token DEFAULT_SIZE
/// Returns `None` if there is no child corresponding to token DEFAULT_SIZE
fn DEFAULT_SIZE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(DEFAULT_SIZE, 0)
}
/// Retrieves first TerminalNode corresponding to token DEPENDENCIES
/// Returns `None` if there is no child corresponding to token DEPENDENCIES
fn DEPENDENCIES(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(DEPENDENCIES, 0)
}
/// Retrieves first TerminalNode corresponding to token DEPRECATED
/// Returns `None` if there is no child corresponding to token DEPRECATED
fn DEPRECATED(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(DEPRECATED, 0)
}
/// Retrieves first TerminalNode corresponding to token DESCRIPTION
/// Returns `None` if there is no child corresponding to token DESCRIPTION
fn DESCRIPTION(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(DESCRIPTION, 0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
/// Retrieves first TerminalNode corresponding to token ENDPOINT
/// Returns `None` if there is no child corresponding to token ENDPOINT
fn ENDPOINT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(ENDPOINT, 0)
}
/// Retrieves first TerminalNode corresponding to token ERRORS
/// Returns `None` if there is no child corresponding to token ERRORS
fn ERRORS(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(ERRORS, 0)
}
/// Retrieves first TerminalNode corresponding to token EVENT
/// Returns `None` if there is no child corresponding to token EVENT
fn EVENT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(EVENT, 0)
}
/// Retrieves first TerminalNode corresponding to token HEADER
/// Returns `None` if there is no child corresponding to token HEADER
fn HEADER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(HEADER, 0)
}
/// Retrieves first TerminalNode corresponding to token HEADERS
/// Returns `None` if there is no child corresponding to token HEADERS
fn HEADERS(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(HEADERS, 0)
}
/// Retrieves first TerminalNode corresponding to token HEALTH
/// Returns `None` if there is no child corresponding to token HEALTH
fn HEALTH(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(HEALTH, 0)
}
/// Retrieves first TerminalNode corresponding to token IDEMPOTENT
/// Returns `None` if there is no child corresponding to token IDEMPOTENT
fn IDEMPOTENT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDEMPOTENT, 0)
}
/// Retrieves first TerminalNode corresponding to token IMPORT
/// Returns `None` if there is no child corresponding to token IMPORT
fn IMPORT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IMPORT, 0)
}
/// Retrieves first TerminalNode corresponding to token KEY
/// Returns `None` if there is no child corresponding to token KEY
fn KEY(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(KEY, 0)
}
/// Retrieves first TerminalNode corresponding to token LIST
/// Returns `None` if there is no child corresponding to token LIST
fn LIST(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(LIST, 0)
}
/// Retrieves first TerminalNode corresponding to token MAX_AGE
/// Returns `None` if there is no child corresponding to token MAX_AGE
fn MAX_AGE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(MAX_AGE, 0)
}
/// Retrieves first TerminalNode corresponding to token MAX_SIZE
/// Returns `None` if there is no child corresponding to token MAX_SIZE
fn MAX_SIZE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(MAX_SIZE, 0)
}
/// Retrieves first TerminalNode corresponding to token METHOD
/// Returns `None` if there is no child corresponding to token METHOD
fn METHOD(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(METHOD, 0)
}
/// Retrieves first TerminalNode corresponding to token METHODS
/// Returns `None` if there is no child corresponding to token METHODS
fn METHODS(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(METHODS, 0)
}
/// Retrieves first TerminalNode corresponding to token OPTIONAL
/// Returns `None` if there is no child corresponding to token OPTIONAL
fn OPTIONAL(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(OPTIONAL, 0)
}
/// Retrieves first TerminalNode corresponding to token ORIGINS
/// Returns `None` if there is no child corresponding to token ORIGINS
fn ORIGINS(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(ORIGINS, 0)
}
/// Retrieves first TerminalNode corresponding to token PAGINATED
/// Returns `None` if there is no child corresponding to token PAGINATED
fn PAGINATED(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(PAGINATED, 0)
}
/// Retrieves first TerminalNode corresponding to token PAGINATION
/// Returns `None` if there is no child corresponding to token PAGINATION
fn PAGINATION(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(PAGINATION, 0)
}
/// Retrieves first TerminalNode corresponding to token PARAMS
/// Returns `None` if there is no child corresponding to token PARAMS
fn PARAMS(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(PARAMS, 0)
}
/// Retrieves first TerminalNode corresponding to token PARTITIONED_BY
/// Returns `None` if there is no child corresponding to token PARTITIONED_BY
fn PARTITIONED_BY(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(PARTITIONED_BY, 0)
}
/// Retrieves first TerminalNode corresponding to token PATH
/// Returns `None` if there is no child corresponding to token PATH
fn PATH(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(PATH, 0)
}
/// Retrieves first TerminalNode corresponding to token PAYLOAD
/// Returns `None` if there is no child corresponding to token PAYLOAD
fn PAYLOAD(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(PAYLOAD, 0)
}
/// Retrieves first TerminalNode corresponding to token PER
/// Returns `None` if there is no child corresponding to token PER
fn PER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(PER, 0)
}
/// Retrieves first TerminalNode corresponding to token QUERY
/// Returns `None` if there is no child corresponding to token QUERY
fn QUERY(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(QUERY, 0)
}
/// Retrieves first TerminalNode corresponding to token RATE_LIMIT
/// Returns `None` if there is no child corresponding to token RATE_LIMIT
fn RATE_LIMIT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(RATE_LIMIT, 0)
}
/// Retrieves first TerminalNode corresponding to token READY
/// Returns `None` if there is no child corresponding to token READY
fn READY(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(READY, 0)
}
/// Retrieves first TerminalNode corresponding to token REPLACEMENT
/// Returns `None` if there is no child corresponding to token REPLACEMENT
fn REPLACEMENT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(REPLACEMENT, 0)
}
/// Retrieves first TerminalNode corresponding to token REQUEST
/// Returns `None` if there is no child corresponding to token REQUEST
fn REQUEST(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(REQUEST, 0)
}
/// Retrieves first TerminalNode corresponding to token REQUIRED
/// Returns `None` if there is no child corresponding to token REQUIRED
fn REQUIRED(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(REQUIRED, 0)
}
/// Retrieves first TerminalNode corresponding to token RESPONSE
/// Returns `None` if there is no child corresponding to token RESPONSE
fn RESPONSE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(RESPONSE, 0)
}
/// Retrieves first TerminalNode corresponding to token SCOPE
/// Returns `None` if there is no child corresponding to token SCOPE
fn SCOPE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(SCOPE, 0)
}
/// Retrieves first TerminalNode corresponding to token SUNSET
/// Returns `None` if there is no child corresponding to token SUNSET
fn SUNSET(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(SUNSET, 0)
}
/// Retrieves first TerminalNode corresponding to token TARGETS
/// Returns `None` if there is no child corresponding to token TARGETS
fn TARGETS(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(TARGETS, 0)
}
/// Retrieves first TerminalNode corresponding to token TIMEOUT
/// Returns `None` if there is no child corresponding to token TIMEOUT
fn TIMEOUT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(TIMEOUT, 0)
}
/// Retrieves first TerminalNode corresponding to token TOPIC
/// Returns `None` if there is no child corresponding to token TOPIC
fn TOPIC(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(TOPIC, 0)
}
/// Retrieves first TerminalNode corresponding to token USING
/// Returns `None` if there is no child corresponding to token USING
fn USING(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(USING, 0)
}
/// Retrieves first TerminalNode corresponding to token VALIDATE
/// Returns `None` if there is no child corresponding to token VALIDATE
fn VALIDATE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(VALIDATE, 0)
}
/// Retrieves first TerminalNode corresponding to token VERSION
/// Returns `None` if there is no child corresponding to token VERSION
fn VERSION(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(VERSION, 0)
}

}

impl<'input> WordContextAttrs<'input> for WordContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
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
			recog.base.set_state(170);
			_la = recog.base.input.la(1);
			if { !(((((_la - 1)) & !0x3f) == 0 && ((1usize << (_la - 1)) & ((1usize << (API - 1)) | (1usize << (ASYNC - 1)) | (1usize << (AUTH - 1)) | (1usize << (BASE - 1)) | (1usize << (BURST - 1)) | (1usize << (CACHE - 1)) | (1usize << (CONFIG - 1)) | (1usize << (CONTENT_TYPE - 1)) | (1usize << (CORS - 1)) | (1usize << (DEFAULT - 1)) | (1usize << (DEFAULT_SIZE - 1)) | (1usize << (DEPENDENCIES - 1)) | (1usize << (DEPRECATED - 1)) | (1usize << (DESCRIPTION - 1)) | (1usize << (END - 1)) | (1usize << (ENDPOINT - 1)) | (1usize << (ERRORS - 1)) | (1usize << (EVENT - 1)) | (1usize << (HEADER - 1)) | (1usize << (HEADERS - 1)) | (1usize << (HEALTH - 1)) | (1usize << (IDEMPOTENT - 1)) | (1usize << (IMPORT - 1)) | (1usize << (KEY - 1)) | (1usize << (LIST - 1)) | (1usize << (MAX_AGE - 1)) | (1usize << (MAX_SIZE - 1)) | (1usize << (METHOD - 1)) | (1usize << (METHODS - 1)) | (1usize << (OPTIONAL - 1)) | (1usize << (ORIGINS - 1)) | (1usize << (PAGINATED - 1)))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (PAGINATION - 33)) | (1usize << (PARAMS - 33)) | (1usize << (PARTITIONED_BY - 33)) | (1usize << (PATH - 33)) | (1usize << (PAYLOAD - 33)) | (1usize << (PER - 33)) | (1usize << (QUERY - 33)) | (1usize << (RATE_LIMIT - 33)) | (1usize << (READY - 33)) | (1usize << (REPLACEMENT - 33)) | (1usize << (REQUEST - 33)) | (1usize << (REQUIRED - 33)) | (1usize << (RESPONSE - 33)) | (1usize << (SCOPE - 33)) | (1usize << (SUNSET - 33)) | (1usize << (TARGETS - 33)) | (1usize << (TIMEOUT - 33)) | (1usize << (TOPIC - 33)) | (1usize << (USING - 33)) | (1usize << (VALIDATE - 33)) | (1usize << (VERSION - 33)) | (1usize << (IDENTIFIER - 33)))) != 0)) } {
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
//------------------- apiDefinition ----------------
pub type ApiDefinitionContextAll<'input> = ApiDefinitionContext<'input>;


pub type ApiDefinitionContext<'input> = BaseParserRuleContext<'input,ApiDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct ApiDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for ApiDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for ApiDefinitionContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_apiDefinition(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_apiDefinition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for ApiDefinitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_apiDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for ApiDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_apiDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_apiDefinition }
}
antlr_rust::tid!{ApiDefinitionContextExt<'a>}

impl<'input> ApiDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ApiDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ApiDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ApiDefinitionContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<ApiDefinitionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token API
/// Returns `None` if there is no child corresponding to token API
fn API(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(API, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn apiBody(&self) -> Option<Rc<ApiBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}

}

impl<'input> ApiDefinitionContextAttrs<'input> for ApiDefinitionContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn apiDefinition(&mut self,)
	-> Result<Rc<ApiDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ApiDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_apiDefinition);
        let mut _localctx: Rc<ApiDefinitionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(172);
			recog.base.match_token(API,&mut recog.err_handler)?;

			recog.base.set_state(173);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			/*InvokeRule apiBody*/
			recog.base.set_state(174);
			recog.apiBody()?;

			recog.base.set_state(175);
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
//------------------- apiBody ----------------
pub type ApiBodyContextAll<'input> = ApiBodyContext<'input>;


pub type ApiBodyContext<'input> = BaseParserRuleContext<'input,ApiBodyContextExt<'input>>;

#[derive(Clone)]
pub struct ApiBodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for ApiBodyContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for ApiBodyContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_apiBody(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_apiBody(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for ApiBodyContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_apiBody(self);
	}
}

impl<'input> CustomRuleContext<'input> for ApiBodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_apiBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_apiBody }
}
antlr_rust::tid!{ApiBodyContextExt<'a>}

impl<'input> ApiBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ApiBodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ApiBodyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ApiBodyContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<ApiBodyContextExt<'input>>{

fn apiElement_all(&self) ->  Vec<Rc<ApiElementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn apiElement(&self, i: usize) -> Option<Rc<ApiElementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ApiBodyContextAttrs<'input> for ApiBodyContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn apiBody(&mut self,)
	-> Result<Rc<ApiBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ApiBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_apiBody);
        let mut _localctx: Rc<ApiBodyContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(180);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << AUTH) | (1usize << BASE) | (1usize << CONFIG) | (1usize << CONTENT_TYPE) | (1usize << CORS) | (1usize << DEPENDENCIES) | (1usize << DEPRECATED) | (1usize << DESCRIPTION) | (1usize << ENDPOINT) | (1usize << EVENT) | (1usize << HEALTH))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (PAGINATION - 33)) | (1usize << (RATE_LIMIT - 33)) | (1usize << (READY - 33)) | (1usize << (TARGETS - 33)) | (1usize << (VERSION - 33)))) != 0) {
				{
				{
				/*InvokeRule apiElement*/
				recog.base.set_state(177);
				recog.apiElement()?;

				}
				}
				recog.base.set_state(182);
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
//------------------- apiElement ----------------
pub type ApiElementContextAll<'input> = ApiElementContext<'input>;


pub type ApiElementContext<'input> = BaseParserRuleContext<'input,ApiElementContextExt<'input>>;

#[derive(Clone)]
pub struct ApiElementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for ApiElementContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for ApiElementContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_apiElement(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_apiElement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for ApiElementContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_apiElement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ApiElementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_apiElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_apiElement }
}
antlr_rust::tid!{ApiElementContextExt<'a>}

impl<'input> ApiElementContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ApiElementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ApiElementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ApiElementContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<ApiElementContextExt<'input>>{

fn versionDecl(&self) -> Option<Rc<VersionDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn baseDecl(&self) -> Option<Rc<BaseDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn descriptionDecl(&self) -> Option<Rc<DescriptionDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn targetsDecl(&self) -> Option<Rc<TargetsDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn authDefaultDecl(&self) -> Option<Rc<AuthDefaultDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn contentTypeDecl(&self) -> Option<Rc<ContentTypeDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn rateLimitDecl(&self) -> Option<Rc<RateLimitDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn paginationDecl(&self) -> Option<Rc<PaginationDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn configBlock(&self) -> Option<Rc<ConfigBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn corsBlock(&self) -> Option<Rc<CorsBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn endpointDecl(&self) -> Option<Rc<EndpointDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn deprecatedEndpointDecl(&self) -> Option<Rc<DeprecatedEndpointDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn eventDecl(&self) -> Option<Rc<EventDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn dependenciesBlock(&self) -> Option<Rc<DependenciesBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn healthDecl(&self) -> Option<Rc<HealthDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn readyDecl(&self) -> Option<Rc<ReadyDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ApiElementContextAttrs<'input> for ApiElementContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn apiElement(&mut self,)
	-> Result<Rc<ApiElementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ApiElementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_apiElement);
        let mut _localctx: Rc<ApiElementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(199);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 VERSION 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule versionDecl*/
					recog.base.set_state(183);
					recog.versionDecl()?;

					}
				}

			 BASE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule baseDecl*/
					recog.base.set_state(184);
					recog.baseDecl()?;

					}
				}

			 DESCRIPTION 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule descriptionDecl*/
					recog.base.set_state(185);
					recog.descriptionDecl()?;

					}
				}

			 TARGETS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule targetsDecl*/
					recog.base.set_state(186);
					recog.targetsDecl()?;

					}
				}

			 AUTH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule authDefaultDecl*/
					recog.base.set_state(187);
					recog.authDefaultDecl()?;

					}
				}

			 CONTENT_TYPE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule contentTypeDecl*/
					recog.base.set_state(188);
					recog.contentTypeDecl()?;

					}
				}

			 RATE_LIMIT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule rateLimitDecl*/
					recog.base.set_state(189);
					recog.rateLimitDecl()?;

					}
				}

			 PAGINATION 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule paginationDecl*/
					recog.base.set_state(190);
					recog.paginationDecl()?;

					}
				}

			 CONFIG 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					/*InvokeRule configBlock*/
					recog.base.set_state(191);
					recog.configBlock()?;

					}
				}

			 CORS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 10);
					recog.base.enter_outer_alt(None, 10);
					{
					/*InvokeRule corsBlock*/
					recog.base.set_state(192);
					recog.corsBlock()?;

					}
				}

			 ENDPOINT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 11);
					recog.base.enter_outer_alt(None, 11);
					{
					/*InvokeRule endpointDecl*/
					recog.base.set_state(193);
					recog.endpointDecl()?;

					}
				}

			 DEPRECATED 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 12);
					recog.base.enter_outer_alt(None, 12);
					{
					/*InvokeRule deprecatedEndpointDecl*/
					recog.base.set_state(194);
					recog.deprecatedEndpointDecl()?;

					}
				}

			 EVENT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 13);
					recog.base.enter_outer_alt(None, 13);
					{
					/*InvokeRule eventDecl*/
					recog.base.set_state(195);
					recog.eventDecl()?;

					}
				}

			 DEPENDENCIES 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 14);
					recog.base.enter_outer_alt(None, 14);
					{
					/*InvokeRule dependenciesBlock*/
					recog.base.set_state(196);
					recog.dependenciesBlock()?;

					}
				}

			 HEALTH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 15);
					recog.base.enter_outer_alt(None, 15);
					{
					/*InvokeRule healthDecl*/
					recog.base.set_state(197);
					recog.healthDecl()?;

					}
				}

			 READY 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 16);
					recog.base.enter_outer_alt(None, 16);
					{
					/*InvokeRule readyDecl*/
					recog.base.set_state(198);
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
//------------------- versionDecl ----------------
pub type VersionDeclContextAll<'input> = VersionDeclContext<'input>;


pub type VersionDeclContext<'input> = BaseParserRuleContext<'input,VersionDeclContextExt<'input>>;

#[derive(Clone)]
pub struct VersionDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for VersionDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for VersionDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_versionDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_versionDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for VersionDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_versionDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for VersionDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_versionDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_versionDecl }
}
antlr_rust::tid!{VersionDeclContextExt<'a>}

impl<'input> VersionDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VersionDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VersionDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VersionDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<VersionDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token VERSION
/// Returns `None` if there is no child corresponding to token VERSION
fn VERSION(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(VERSION, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}

}

impl<'input> VersionDeclContextAttrs<'input> for VersionDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn versionDecl(&mut self,)
	-> Result<Rc<VersionDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VersionDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_versionDecl);
        let mut _localctx: Rc<VersionDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(201);
			recog.base.match_token(VERSION,&mut recog.err_handler)?;

			recog.base.set_state(202);
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
//------------------- baseDecl ----------------
pub type BaseDeclContextAll<'input> = BaseDeclContext<'input>;


pub type BaseDeclContext<'input> = BaseParserRuleContext<'input,BaseDeclContextExt<'input>>;

#[derive(Clone)]
pub struct BaseDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for BaseDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for BaseDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_baseDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_baseDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for BaseDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_baseDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for BaseDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_baseDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_baseDecl }
}
antlr_rust::tid!{BaseDeclContextExt<'a>}

impl<'input> BaseDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BaseDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BaseDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BaseDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<BaseDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BASE
/// Returns `None` if there is no child corresponding to token BASE
fn BASE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(BASE, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}

}

impl<'input> BaseDeclContextAttrs<'input> for BaseDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn baseDecl(&mut self,)
	-> Result<Rc<BaseDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BaseDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_baseDecl);
        let mut _localctx: Rc<BaseDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(204);
			recog.base.match_token(BASE,&mut recog.err_handler)?;

			recog.base.set_state(205);
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
//------------------- descriptionDecl ----------------
pub type DescriptionDeclContextAll<'input> = DescriptionDeclContext<'input>;


pub type DescriptionDeclContext<'input> = BaseParserRuleContext<'input,DescriptionDeclContextExt<'input>>;

#[derive(Clone)]
pub struct DescriptionDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for DescriptionDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for DescriptionDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_descriptionDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_descriptionDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for DescriptionDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_descriptionDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for DescriptionDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_descriptionDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_descriptionDecl }
}
antlr_rust::tid!{DescriptionDeclContextExt<'a>}

impl<'input> DescriptionDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DescriptionDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DescriptionDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DescriptionDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<DescriptionDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DESCRIPTION
/// Returns `None` if there is no child corresponding to token DESCRIPTION
fn DESCRIPTION(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(DESCRIPTION, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}

}

impl<'input> DescriptionDeclContextAttrs<'input> for DescriptionDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn descriptionDecl(&mut self,)
	-> Result<Rc<DescriptionDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DescriptionDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_descriptionDecl);
        let mut _localctx: Rc<DescriptionDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(207);
			recog.base.match_token(DESCRIPTION,&mut recog.err_handler)?;

			recog.base.set_state(208);
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
//------------------- targetsDecl ----------------
pub type TargetsDeclContextAll<'input> = TargetsDeclContext<'input>;


pub type TargetsDeclContext<'input> = BaseParserRuleContext<'input,TargetsDeclContextExt<'input>>;

#[derive(Clone)]
pub struct TargetsDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for TargetsDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for TargetsDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_targetsDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_targetsDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for TargetsDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_targetsDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for TargetsDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_targetsDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_targetsDecl }
}
antlr_rust::tid!{TargetsDeclContextExt<'a>}

impl<'input> TargetsDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TargetsDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TargetsDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TargetsDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<TargetsDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TARGETS
/// Returns `None` if there is no child corresponding to token TARGETS
fn TARGETS(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(TARGETS, 0)
}
fn identifierList(&self) -> Option<Rc<IdentifierListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TargetsDeclContextAttrs<'input> for TargetsDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn targetsDecl(&mut self,)
	-> Result<Rc<TargetsDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TargetsDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_targetsDecl);
        let mut _localctx: Rc<TargetsDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(210);
			recog.base.match_token(TARGETS,&mut recog.err_handler)?;

			/*InvokeRule identifierList*/
			recog.base.set_state(211);
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
//------------------- authDefaultDecl ----------------
pub type AuthDefaultDeclContextAll<'input> = AuthDefaultDeclContext<'input>;


pub type AuthDefaultDeclContext<'input> = BaseParserRuleContext<'input,AuthDefaultDeclContextExt<'input>>;

#[derive(Clone)]
pub struct AuthDefaultDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for AuthDefaultDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for AuthDefaultDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_authDefaultDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_authDefaultDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for AuthDefaultDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_authDefaultDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for AuthDefaultDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_authDefaultDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_authDefaultDecl }
}
antlr_rust::tid!{AuthDefaultDeclContextExt<'a>}

impl<'input> AuthDefaultDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AuthDefaultDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AuthDefaultDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AuthDefaultDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<AuthDefaultDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token AUTH
/// Returns `None` if there is no child corresponding to token AUTH
fn AUTH(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(AUTH, 0)
}
/// Retrieves first TerminalNode corresponding to token DEFAULT
/// Returns `None` if there is no child corresponding to token DEFAULT
fn DEFAULT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(DEFAULT, 0)
}
fn authScheme(&self) -> Option<Rc<AuthSchemeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AuthDefaultDeclContextAttrs<'input> for AuthDefaultDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn authDefaultDecl(&mut self,)
	-> Result<Rc<AuthDefaultDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AuthDefaultDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_authDefaultDecl);
        let mut _localctx: Rc<AuthDefaultDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(213);
			recog.base.match_token(AUTH,&mut recog.err_handler)?;

			recog.base.set_state(214);
			recog.base.match_token(DEFAULT,&mut recog.err_handler)?;

			/*InvokeRule authScheme*/
			recog.base.set_state(215);
			recog.authScheme()?;

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
//------------------- contentTypeDecl ----------------
pub type ContentTypeDeclContextAll<'input> = ContentTypeDeclContext<'input>;


pub type ContentTypeDeclContext<'input> = BaseParserRuleContext<'input,ContentTypeDeclContextExt<'input>>;

#[derive(Clone)]
pub struct ContentTypeDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for ContentTypeDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for ContentTypeDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_contentTypeDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_contentTypeDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for ContentTypeDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_contentTypeDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for ContentTypeDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_contentTypeDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_contentTypeDecl }
}
antlr_rust::tid!{ContentTypeDeclContextExt<'a>}

impl<'input> ContentTypeDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ContentTypeDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ContentTypeDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ContentTypeDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<ContentTypeDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CONTENT_TYPE
/// Returns `None` if there is no child corresponding to token CONTENT_TYPE
fn CONTENT_TYPE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(CONTENT_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> ContentTypeDeclContextAttrs<'input> for ContentTypeDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn contentTypeDecl(&mut self,)
	-> Result<Rc<ContentTypeDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ContentTypeDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_contentTypeDecl);
        let mut _localctx: Rc<ContentTypeDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(217);
			recog.base.match_token(CONTENT_TYPE,&mut recog.err_handler)?;

			recog.base.set_state(218);
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
//------------------- rateLimitDecl ----------------
pub type RateLimitDeclContextAll<'input> = RateLimitDeclContext<'input>;


pub type RateLimitDeclContext<'input> = BaseParserRuleContext<'input,RateLimitDeclContextExt<'input>>;

#[derive(Clone)]
pub struct RateLimitDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for RateLimitDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for RateLimitDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_rateLimitDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_rateLimitDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for RateLimitDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_rateLimitDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for RateLimitDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_rateLimitDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_rateLimitDecl }
}
antlr_rust::tid!{RateLimitDeclContextExt<'a>}

impl<'input> RateLimitDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RateLimitDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RateLimitDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RateLimitDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<RateLimitDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token RATE_LIMIT
/// Returns `None` if there is no child corresponding to token RATE_LIMIT
fn RATE_LIMIT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(RATE_LIMIT, 0)
}
fn valueOrCfg_all(&self) ->  Vec<Rc<ValueOrCfgContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn valueOrCfg(&self, i: usize) -> Option<Rc<ValueOrCfgContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token PER
/// Returns `None` if there is no child corresponding to token PER
fn PER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(PER, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token BURST
/// Returns `None` if there is no child corresponding to token BURST
fn BURST(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(BURST, 0)
}

}

impl<'input> RateLimitDeclContextAttrs<'input> for RateLimitDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn rateLimitDecl(&mut self,)
	-> Result<Rc<RateLimitDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RateLimitDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_rateLimitDecl);
        let mut _localctx: Rc<RateLimitDeclContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(220);
			recog.base.match_token(RATE_LIMIT,&mut recog.err_handler)?;

			/*InvokeRule valueOrCfg*/
			recog.base.set_state(221);
			recog.valueOrCfg()?;

			recog.base.set_state(222);
			recog.base.match_token(PER,&mut recog.err_handler)?;

			recog.base.set_state(223);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(226);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==BURST {
				{
				recog.base.set_state(224);
				recog.base.match_token(BURST,&mut recog.err_handler)?;

				/*InvokeRule valueOrCfg*/
				recog.base.set_state(225);
				recog.valueOrCfg()?;

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
//------------------- paginationDecl ----------------
pub type PaginationDeclContextAll<'input> = PaginationDeclContext<'input>;


pub type PaginationDeclContext<'input> = BaseParserRuleContext<'input,PaginationDeclContextExt<'input>>;

#[derive(Clone)]
pub struct PaginationDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for PaginationDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for PaginationDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_paginationDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_paginationDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for PaginationDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_paginationDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for PaginationDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_paginationDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_paginationDecl }
}
antlr_rust::tid!{PaginationDeclContextExt<'a>}

impl<'input> PaginationDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PaginationDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PaginationDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PaginationDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<PaginationDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PAGINATION
/// Returns `None` if there is no child corresponding to token PAGINATION
fn PAGINATION(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(PAGINATION, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token DEFAULT_SIZE
/// Returns `None` if there is no child corresponding to token DEFAULT_SIZE
fn DEFAULT_SIZE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(DEFAULT_SIZE, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token INTEGER in current rule
fn INTEGER_all(&self) -> Vec<Rc<TerminalNode<'input,ApiDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token INTEGER, starting from 0.
/// Returns `None` if number of children corresponding to token INTEGER is less or equal than `i`.
fn INTEGER(&self, i: usize) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, i)
}
/// Retrieves first TerminalNode corresponding to token MAX_SIZE
/// Returns `None` if there is no child corresponding to token MAX_SIZE
fn MAX_SIZE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(MAX_SIZE, 0)
}

}

impl<'input> PaginationDeclContextAttrs<'input> for PaginationDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn paginationDecl(&mut self,)
	-> Result<Rc<PaginationDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PaginationDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_paginationDecl);
        let mut _localctx: Rc<PaginationDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(228);
			recog.base.match_token(PAGINATION,&mut recog.err_handler)?;

			recog.base.set_state(229);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(230);
			recog.base.match_token(DEFAULT_SIZE,&mut recog.err_handler)?;

			recog.base.set_state(231);
			recog.base.match_token(INTEGER,&mut recog.err_handler)?;

			recog.base.set_state(232);
			recog.base.match_token(MAX_SIZE,&mut recog.err_handler)?;

			recog.base.set_state(233);
			recog.base.match_token(INTEGER,&mut recog.err_handler)?;

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

impl<'input> ApiDSLParserContext<'input> for ConfigBlockContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for ConfigBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_configBlock(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_configBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for ConfigBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_configBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConfigBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_configBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_configBlock }
}
antlr_rust::tid!{ConfigBlockContextExt<'a>}

impl<'input> ConfigBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConfigBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConfigBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConfigBlockContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<ConfigBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CONFIG
/// Returns `None` if there is no child corresponding to token CONFIG
fn CONFIG(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(CONFIG, 0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
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

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn configBlock(&mut self,)
	-> Result<Rc<ConfigBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConfigBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_configBlock);
        let mut _localctx: Rc<ConfigBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(235);
			recog.base.match_token(CONFIG,&mut recog.err_handler)?;

			recog.base.set_state(239);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==IDENTIFIER {
				{
				{
				/*InvokeRule configDirective*/
				recog.base.set_state(236);
				recog.configDirective()?;

				}
				}
				recog.base.set_state(241);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(242);
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

impl<'input> ApiDSLParserContext<'input> for ConfigDirectiveContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for ConfigDirectiveContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_configDirective(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_configDirective(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for ConfigDirectiveContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_configDirective(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConfigDirectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_configDirective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_configDirective }
}
antlr_rust::tid!{ConfigDirectiveContextExt<'a>}

impl<'input> ConfigDirectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConfigDirectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConfigDirectiveContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConfigDirectiveContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<ConfigDirectiveContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn configValue_all(&self) ->  Vec<Rc<ConfigValueContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn configValue(&self, i: usize) -> Option<Rc<ConfigValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,ApiDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ConfigDirectiveContextAttrs<'input> for ConfigDirectiveContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn configDirective(&mut self,)
	-> Result<Rc<ConfigDirectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConfigDirectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_configDirective);
        let mut _localctx: Rc<ConfigDirectiveContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(244);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			/*InvokeRule configValue*/
			recog.base.set_state(245);
			recog.configValue()?;

			recog.base.set_state(250);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(246);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule configValue*/
				recog.base.set_state(247);
				recog.configValue()?;

				}
				}
				recog.base.set_state(252);
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

impl<'input> ApiDSLParserContext<'input> for ConfigValueContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for ConfigValueContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_configValue(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_configValue(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for ConfigValueContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_configValue(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConfigValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_configValue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_configValue }
}
antlr_rust::tid!{ConfigValueContextExt<'a>}

impl<'input> ConfigValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConfigValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConfigValueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConfigValueContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<ConfigValueContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
/// Retrieves first TerminalNode corresponding to token DECIMAL_LITERAL
/// Returns `None` if there is no child corresponding to token DECIMAL_LITERAL
fn DECIMAL_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(DECIMAL_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token TRUE
/// Returns `None` if there is no child corresponding to token TRUE
fn TRUE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(TRUE, 0)
}
/// Retrieves first TerminalNode corresponding to token FALSE
/// Returns `None` if there is no child corresponding to token FALSE
fn FALSE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(FALSE, 0)
}

}

impl<'input> ConfigValueContextAttrs<'input> for ConfigValueContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn configValue(&mut self,)
	-> Result<Rc<ConfigValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConfigValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_configValue);
        let mut _localctx: Rc<ConfigValueContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(253);
			_la = recog.base.input.la(1);
			if { !(((((_la - 59)) & !0x3f) == 0 && ((1usize << (_la - 59)) & ((1usize << (TRUE - 59)) | (1usize << (FALSE - 59)) | (1usize << (INTEGER - 59)) | (1usize << (DECIMAL_LITERAL - 59)) | (1usize << (STRING_LITERAL - 59)) | (1usize << (IDENTIFIER - 59)))) != 0)) } {
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
//------------------- corsBlock ----------------
pub type CorsBlockContextAll<'input> = CorsBlockContext<'input>;


pub type CorsBlockContext<'input> = BaseParserRuleContext<'input,CorsBlockContextExt<'input>>;

#[derive(Clone)]
pub struct CorsBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for CorsBlockContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for CorsBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_corsBlock(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_corsBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for CorsBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_corsBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for CorsBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_corsBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_corsBlock }
}
antlr_rust::tid!{CorsBlockContextExt<'a>}

impl<'input> CorsBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CorsBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CorsBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CorsBlockContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<CorsBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CORS
/// Returns `None` if there is no child corresponding to token CORS
fn CORS(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(CORS, 0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
fn corsDirective_all(&self) ->  Vec<Rc<CorsDirectiveContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn corsDirective(&self, i: usize) -> Option<Rc<CorsDirectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> CorsBlockContextAttrs<'input> for CorsBlockContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn corsBlock(&mut self,)
	-> Result<Rc<CorsBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CorsBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_corsBlock);
        let mut _localctx: Rc<CorsBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(255);
			recog.base.match_token(CORS,&mut recog.err_handler)?;

			recog.base.set_state(259);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << HEADERS) | (1usize << MAX_AGE) | (1usize << METHODS) | (1usize << ORIGINS))) != 0) {
				{
				{
				/*InvokeRule corsDirective*/
				recog.base.set_state(256);
				recog.corsDirective()?;

				}
				}
				recog.base.set_state(261);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(262);
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
//------------------- corsDirective ----------------
pub type CorsDirectiveContextAll<'input> = CorsDirectiveContext<'input>;


pub type CorsDirectiveContext<'input> = BaseParserRuleContext<'input,CorsDirectiveContextExt<'input>>;

#[derive(Clone)]
pub struct CorsDirectiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for CorsDirectiveContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for CorsDirectiveContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_corsDirective(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_corsDirective(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for CorsDirectiveContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_corsDirective(self);
	}
}

impl<'input> CustomRuleContext<'input> for CorsDirectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_corsDirective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_corsDirective }
}
antlr_rust::tid!{CorsDirectiveContextExt<'a>}

impl<'input> CorsDirectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CorsDirectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CorsDirectiveContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CorsDirectiveContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<CorsDirectiveContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ORIGINS
/// Returns `None` if there is no child corresponding to token ORIGINS
fn ORIGINS(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(ORIGINS, 0)
}
fn valueOrCfgList(&self) -> Option<Rc<ValueOrCfgListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token METHODS
/// Returns `None` if there is no child corresponding to token METHODS
fn METHODS(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(METHODS, 0)
}
fn httpMethodList(&self) -> Option<Rc<HttpMethodListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token HEADERS
/// Returns `None` if there is no child corresponding to token HEADERS
fn HEADERS(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(HEADERS, 0)
}
fn stringList(&self) -> Option<Rc<StringListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token MAX_AGE
/// Returns `None` if there is no child corresponding to token MAX_AGE
fn MAX_AGE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(MAX_AGE, 0)
}
fn valueOrCfg(&self) -> Option<Rc<ValueOrCfgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CorsDirectiveContextAttrs<'input> for CorsDirectiveContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn corsDirective(&mut self,)
	-> Result<Rc<CorsDirectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CorsDirectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_corsDirective);
        let mut _localctx: Rc<CorsDirectiveContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(272);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 ORIGINS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(264);
					recog.base.match_token(ORIGINS,&mut recog.err_handler)?;

					/*InvokeRule valueOrCfgList*/
					recog.base.set_state(265);
					recog.valueOrCfgList()?;

					}
				}

			 METHODS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(266);
					recog.base.match_token(METHODS,&mut recog.err_handler)?;

					/*InvokeRule httpMethodList*/
					recog.base.set_state(267);
					recog.httpMethodList()?;

					}
				}

			 HEADERS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(268);
					recog.base.match_token(HEADERS,&mut recog.err_handler)?;

					/*InvokeRule stringList*/
					recog.base.set_state(269);
					recog.stringList()?;

					}
				}

			 MAX_AGE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(270);
					recog.base.match_token(MAX_AGE,&mut recog.err_handler)?;

					/*InvokeRule valueOrCfg*/
					recog.base.set_state(271);
					recog.valueOrCfg()?;

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
//------------------- endpointDecl ----------------
pub type EndpointDeclContextAll<'input> = EndpointDeclContext<'input>;


pub type EndpointDeclContext<'input> = BaseParserRuleContext<'input,EndpointDeclContextExt<'input>>;

#[derive(Clone)]
pub struct EndpointDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for EndpointDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for EndpointDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_endpointDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_endpointDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for EndpointDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_endpointDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for EndpointDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_endpointDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_endpointDecl }
}
antlr_rust::tid!{EndpointDeclContextExt<'a>}

impl<'input> EndpointDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EndpointDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EndpointDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EndpointDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<EndpointDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ENDPOINT
/// Returns `None` if there is no child corresponding to token ENDPOINT
fn ENDPOINT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(ENDPOINT, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn endpointBody(&self) -> Option<Rc<EndpointBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}

}

impl<'input> EndpointDeclContextAttrs<'input> for EndpointDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn endpointDecl(&mut self,)
	-> Result<Rc<EndpointDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EndpointDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_endpointDecl);
        let mut _localctx: Rc<EndpointDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(274);
			recog.base.match_token(ENDPOINT,&mut recog.err_handler)?;

			recog.base.set_state(275);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			/*InvokeRule endpointBody*/
			recog.base.set_state(276);
			recog.endpointBody()?;

			recog.base.set_state(277);
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
//------------------- deprecatedEndpointDecl ----------------
pub type DeprecatedEndpointDeclContextAll<'input> = DeprecatedEndpointDeclContext<'input>;


pub type DeprecatedEndpointDeclContext<'input> = BaseParserRuleContext<'input,DeprecatedEndpointDeclContextExt<'input>>;

#[derive(Clone)]
pub struct DeprecatedEndpointDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for DeprecatedEndpointDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for DeprecatedEndpointDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_deprecatedEndpointDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_deprecatedEndpointDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for DeprecatedEndpointDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_deprecatedEndpointDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for DeprecatedEndpointDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_deprecatedEndpointDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_deprecatedEndpointDecl }
}
antlr_rust::tid!{DeprecatedEndpointDeclContextExt<'a>}

impl<'input> DeprecatedEndpointDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DeprecatedEndpointDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeprecatedEndpointDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DeprecatedEndpointDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<DeprecatedEndpointDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DEPRECATED
/// Returns `None` if there is no child corresponding to token DEPRECATED
fn DEPRECATED(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(DEPRECATED, 0)
}
/// Retrieves first TerminalNode corresponding to token ENDPOINT
/// Returns `None` if there is no child corresponding to token ENDPOINT
fn ENDPOINT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(ENDPOINT, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn endpointBody(&self) -> Option<Rc<EndpointBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
fn sunsetDecl(&self) -> Option<Rc<SunsetDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn replacementDecl(&self) -> Option<Rc<ReplacementDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DeprecatedEndpointDeclContextAttrs<'input> for DeprecatedEndpointDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn deprecatedEndpointDecl(&mut self,)
	-> Result<Rc<DeprecatedEndpointDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeprecatedEndpointDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_deprecatedEndpointDecl);
        let mut _localctx: Rc<DeprecatedEndpointDeclContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(279);
			recog.base.match_token(DEPRECATED,&mut recog.err_handler)?;

			recog.base.set_state(280);
			recog.base.match_token(ENDPOINT,&mut recog.err_handler)?;

			recog.base.set_state(281);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			/*InvokeRule endpointBody*/
			recog.base.set_state(282);
			recog.endpointBody()?;

			recog.base.set_state(284);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==SUNSET {
				{
				/*InvokeRule sunsetDecl*/
				recog.base.set_state(283);
				recog.sunsetDecl()?;

				}
			}

			recog.base.set_state(287);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==REPLACEMENT {
				{
				/*InvokeRule replacementDecl*/
				recog.base.set_state(286);
				recog.replacementDecl()?;

				}
			}

			recog.base.set_state(289);
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
//------------------- endpointBody ----------------
pub type EndpointBodyContextAll<'input> = EndpointBodyContext<'input>;


pub type EndpointBodyContext<'input> = BaseParserRuleContext<'input,EndpointBodyContextExt<'input>>;

#[derive(Clone)]
pub struct EndpointBodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for EndpointBodyContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for EndpointBodyContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_endpointBody(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_endpointBody(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for EndpointBodyContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_endpointBody(self);
	}
}

impl<'input> CustomRuleContext<'input> for EndpointBodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_endpointBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_endpointBody }
}
antlr_rust::tid!{EndpointBodyContextExt<'a>}

impl<'input> EndpointBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EndpointBodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EndpointBodyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EndpointBodyContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<EndpointBodyContextExt<'input>>{

fn endpointClause_all(&self) ->  Vec<Rc<EndpointClauseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn endpointClause(&self, i: usize) -> Option<Rc<EndpointClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> EndpointBodyContextAttrs<'input> for EndpointBodyContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn endpointBody(&mut self,)
	-> Result<Rc<EndpointBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EndpointBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_endpointBody);
        let mut _localctx: Rc<EndpointBodyContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(294);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ASYNC) | (1usize << AUTH) | (1usize << CACHE) | (1usize << DESCRIPTION) | (1usize << ERRORS) | (1usize << HEADERS) | (1usize << IDEMPOTENT) | (1usize << METHOD))) != 0) || ((((_la - 34)) & !0x3f) == 0 && ((1usize << (_la - 34)) & ((1usize << (PARAMS - 34)) | (1usize << (PATH - 34)) | (1usize << (QUERY - 34)) | (1usize << (RATE_LIMIT - 34)) | (1usize << (REQUEST - 34)) | (1usize << (RESPONSE - 34)) | (1usize << (TIMEOUT - 34)) | (1usize << (VALIDATE - 34)))) != 0) {
				{
				{
				/*InvokeRule endpointClause*/
				recog.base.set_state(291);
				recog.endpointClause()?;

				}
				}
				recog.base.set_state(296);
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
//------------------- endpointClause ----------------
pub type EndpointClauseContextAll<'input> = EndpointClauseContext<'input>;


pub type EndpointClauseContext<'input> = BaseParserRuleContext<'input,EndpointClauseContextExt<'input>>;

#[derive(Clone)]
pub struct EndpointClauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for EndpointClauseContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for EndpointClauseContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_endpointClause(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_endpointClause(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for EndpointClauseContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_endpointClause(self);
	}
}

impl<'input> CustomRuleContext<'input> for EndpointClauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_endpointClause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_endpointClause }
}
antlr_rust::tid!{EndpointClauseContextExt<'a>}

impl<'input> EndpointClauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EndpointClauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EndpointClauseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EndpointClauseContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<EndpointClauseContextExt<'input>>{

fn methodDecl(&self) -> Option<Rc<MethodDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn pathDecl(&self) -> Option<Rc<PathDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn descriptionDecl(&self) -> Option<Rc<DescriptionDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn paramsBlock(&self) -> Option<Rc<ParamsBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn queryBlock(&self) -> Option<Rc<QueryBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn headersBlock(&self) -> Option<Rc<HeadersBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn requestDecl(&self) -> Option<Rc<RequestDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn responseDecl(&self) -> Option<Rc<ResponseDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn errorsBlock(&self) -> Option<Rc<ErrorsBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn authDecl(&self) -> Option<Rc<AuthDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn validateDecl(&self) -> Option<Rc<ValidateDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn rateLimitDecl(&self) -> Option<Rc<RateLimitDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn timeoutDecl(&self) -> Option<Rc<TimeoutDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn cacheDecl(&self) -> Option<Rc<CacheDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn idempotentDecl(&self) -> Option<Rc<IdempotentDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn asyncDecl(&self) -> Option<Rc<AsyncDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EndpointClauseContextAttrs<'input> for EndpointClauseContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn endpointClause(&mut self,)
	-> Result<Rc<EndpointClauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EndpointClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_endpointClause);
        let mut _localctx: Rc<EndpointClauseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(313);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 METHOD 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule methodDecl*/
					recog.base.set_state(297);
					recog.methodDecl()?;

					}
				}

			 PATH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule pathDecl*/
					recog.base.set_state(298);
					recog.pathDecl()?;

					}
				}

			 DESCRIPTION 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule descriptionDecl*/
					recog.base.set_state(299);
					recog.descriptionDecl()?;

					}
				}

			 PARAMS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule paramsBlock*/
					recog.base.set_state(300);
					recog.paramsBlock()?;

					}
				}

			 QUERY 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule queryBlock*/
					recog.base.set_state(301);
					recog.queryBlock()?;

					}
				}

			 HEADERS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule headersBlock*/
					recog.base.set_state(302);
					recog.headersBlock()?;

					}
				}

			 REQUEST 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule requestDecl*/
					recog.base.set_state(303);
					recog.requestDecl()?;

					}
				}

			 RESPONSE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule responseDecl*/
					recog.base.set_state(304);
					recog.responseDecl()?;

					}
				}

			 ERRORS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					/*InvokeRule errorsBlock*/
					recog.base.set_state(305);
					recog.errorsBlock()?;

					}
				}

			 AUTH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 10);
					recog.base.enter_outer_alt(None, 10);
					{
					/*InvokeRule authDecl*/
					recog.base.set_state(306);
					recog.authDecl()?;

					}
				}

			 VALIDATE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 11);
					recog.base.enter_outer_alt(None, 11);
					{
					/*InvokeRule validateDecl*/
					recog.base.set_state(307);
					recog.validateDecl()?;

					}
				}

			 RATE_LIMIT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 12);
					recog.base.enter_outer_alt(None, 12);
					{
					/*InvokeRule rateLimitDecl*/
					recog.base.set_state(308);
					recog.rateLimitDecl()?;

					}
				}

			 TIMEOUT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 13);
					recog.base.enter_outer_alt(None, 13);
					{
					/*InvokeRule timeoutDecl*/
					recog.base.set_state(309);
					recog.timeoutDecl()?;

					}
				}

			 CACHE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 14);
					recog.base.enter_outer_alt(None, 14);
					{
					/*InvokeRule cacheDecl*/
					recog.base.set_state(310);
					recog.cacheDecl()?;

					}
				}

			 IDEMPOTENT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 15);
					recog.base.enter_outer_alt(None, 15);
					{
					/*InvokeRule idempotentDecl*/
					recog.base.set_state(311);
					recog.idempotentDecl()?;

					}
				}

			 ASYNC 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 16);
					recog.base.enter_outer_alt(None, 16);
					{
					/*InvokeRule asyncDecl*/
					recog.base.set_state(312);
					recog.asyncDecl()?;

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
//------------------- methodDecl ----------------
pub type MethodDeclContextAll<'input> = MethodDeclContext<'input>;


pub type MethodDeclContext<'input> = BaseParserRuleContext<'input,MethodDeclContextExt<'input>>;

#[derive(Clone)]
pub struct MethodDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for MethodDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for MethodDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_methodDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_methodDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for MethodDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_methodDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for MethodDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_methodDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_methodDecl }
}
antlr_rust::tid!{MethodDeclContextExt<'a>}

impl<'input> MethodDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MethodDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MethodDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MethodDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<MethodDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token METHOD
/// Returns `None` if there is no child corresponding to token METHOD
fn METHOD(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(METHOD, 0)
}
fn httpMethod(&self) -> Option<Rc<HttpMethodContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MethodDeclContextAttrs<'input> for MethodDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn methodDecl(&mut self,)
	-> Result<Rc<MethodDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MethodDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_methodDecl);
        let mut _localctx: Rc<MethodDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(315);
			recog.base.match_token(METHOD,&mut recog.err_handler)?;

			/*InvokeRule httpMethod*/
			recog.base.set_state(316);
			recog.httpMethod()?;

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
//------------------- httpMethod ----------------
pub type HttpMethodContextAll<'input> = HttpMethodContext<'input>;


pub type HttpMethodContext<'input> = BaseParserRuleContext<'input,HttpMethodContextExt<'input>>;

#[derive(Clone)]
pub struct HttpMethodContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for HttpMethodContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for HttpMethodContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_httpMethod(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_httpMethod(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for HttpMethodContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_httpMethod(self);
	}
}

impl<'input> CustomRuleContext<'input> for HttpMethodContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_httpMethod }
	//fn type_rule_index() -> usize where Self: Sized { RULE_httpMethod }
}
antlr_rust::tid!{HttpMethodContextExt<'a>}

impl<'input> HttpMethodContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<HttpMethodContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HttpMethodContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait HttpMethodContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<HttpMethodContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token GET
/// Returns `None` if there is no child corresponding to token GET
fn GET(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(GET, 0)
}
/// Retrieves first TerminalNode corresponding to token POST
/// Returns `None` if there is no child corresponding to token POST
fn POST(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(POST, 0)
}
/// Retrieves first TerminalNode corresponding to token PUT
/// Returns `None` if there is no child corresponding to token PUT
fn PUT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(PUT, 0)
}
/// Retrieves first TerminalNode corresponding to token PATCH
/// Returns `None` if there is no child corresponding to token PATCH
fn PATCH(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(PATCH, 0)
}
/// Retrieves first TerminalNode corresponding to token DELETE
/// Returns `None` if there is no child corresponding to token DELETE
fn DELETE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(DELETE, 0)
}

}

impl<'input> HttpMethodContextAttrs<'input> for HttpMethodContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn httpMethod(&mut self,)
	-> Result<Rc<HttpMethodContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HttpMethodContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_httpMethod);
        let mut _localctx: Rc<HttpMethodContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(318);
			_la = recog.base.input.la(1);
			if { !(((((_la - 54)) & !0x3f) == 0 && ((1usize << (_la - 54)) & ((1usize << (GET - 54)) | (1usize << (POST - 54)) | (1usize << (PUT - 54)) | (1usize << (PATCH - 54)) | (1usize << (DELETE - 54)))) != 0)) } {
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
//------------------- pathDecl ----------------
pub type PathDeclContextAll<'input> = PathDeclContext<'input>;


pub type PathDeclContext<'input> = BaseParserRuleContext<'input,PathDeclContextExt<'input>>;

#[derive(Clone)]
pub struct PathDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for PathDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for PathDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_pathDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_pathDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for PathDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_pathDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for PathDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pathDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pathDecl }
}
antlr_rust::tid!{PathDeclContextExt<'a>}

impl<'input> PathDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PathDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PathDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PathDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<PathDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PATH
/// Returns `None` if there is no child corresponding to token PATH
fn PATH(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(PATH, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}

}

impl<'input> PathDeclContextAttrs<'input> for PathDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pathDecl(&mut self,)
	-> Result<Rc<PathDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PathDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_pathDecl);
        let mut _localctx: Rc<PathDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(320);
			recog.base.match_token(PATH,&mut recog.err_handler)?;

			recog.base.set_state(321);
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
//------------------- paramsBlock ----------------
pub type ParamsBlockContextAll<'input> = ParamsBlockContext<'input>;


pub type ParamsBlockContext<'input> = BaseParserRuleContext<'input,ParamsBlockContextExt<'input>>;

#[derive(Clone)]
pub struct ParamsBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for ParamsBlockContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for ParamsBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_paramsBlock(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_paramsBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for ParamsBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_paramsBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParamsBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_paramsBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_paramsBlock }
}
antlr_rust::tid!{ParamsBlockContextExt<'a>}

impl<'input> ParamsBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParamsBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParamsBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParamsBlockContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<ParamsBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PARAMS
/// Returns `None` if there is no child corresponding to token PARAMS
fn PARAMS(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(PARAMS, 0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
fn paramDecl_all(&self) ->  Vec<Rc<ParamDeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn paramDecl(&self, i: usize) -> Option<Rc<ParamDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ParamsBlockContextAttrs<'input> for ParamsBlockContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn paramsBlock(&mut self,)
	-> Result<Rc<ParamsBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParamsBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_paramsBlock);
        let mut _localctx: Rc<ParamsBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(323);
			recog.base.match_token(PARAMS,&mut recog.err_handler)?;

			recog.base.set_state(325); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule paramDecl*/
				recog.base.set_state(324);
				recog.paramDecl()?;

				}
				}
				recog.base.set_state(327); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==IDENTIFIER) {break}
			}
			recog.base.set_state(329);
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
//------------------- queryBlock ----------------
pub type QueryBlockContextAll<'input> = QueryBlockContext<'input>;


pub type QueryBlockContext<'input> = BaseParserRuleContext<'input,QueryBlockContextExt<'input>>;

#[derive(Clone)]
pub struct QueryBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for QueryBlockContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for QueryBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_queryBlock(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_queryBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for QueryBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_queryBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for QueryBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_queryBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_queryBlock }
}
antlr_rust::tid!{QueryBlockContextExt<'a>}

impl<'input> QueryBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<QueryBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,QueryBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait QueryBlockContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<QueryBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token QUERY
/// Returns `None` if there is no child corresponding to token QUERY
fn QUERY(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(QUERY, 0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
fn paramDecl_all(&self) ->  Vec<Rc<ParamDeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn paramDecl(&self, i: usize) -> Option<Rc<ParamDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> QueryBlockContextAttrs<'input> for QueryBlockContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn queryBlock(&mut self,)
	-> Result<Rc<QueryBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = QueryBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_queryBlock);
        let mut _localctx: Rc<QueryBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(331);
			recog.base.match_token(QUERY,&mut recog.err_handler)?;

			recog.base.set_state(333); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule paramDecl*/
				recog.base.set_state(332);
				recog.paramDecl()?;

				}
				}
				recog.base.set_state(335); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==IDENTIFIER) {break}
			}
			recog.base.set_state(337);
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
//------------------- headersBlock ----------------
pub type HeadersBlockContextAll<'input> = HeadersBlockContext<'input>;


pub type HeadersBlockContext<'input> = BaseParserRuleContext<'input,HeadersBlockContextExt<'input>>;

#[derive(Clone)]
pub struct HeadersBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for HeadersBlockContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for HeadersBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_headersBlock(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_headersBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for HeadersBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_headersBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for HeadersBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_headersBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_headersBlock }
}
antlr_rust::tid!{HeadersBlockContextExt<'a>}

impl<'input> HeadersBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<HeadersBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HeadersBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait HeadersBlockContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<HeadersBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token HEADERS
/// Returns `None` if there is no child corresponding to token HEADERS
fn HEADERS(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(HEADERS, 0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
fn paramDecl_all(&self) ->  Vec<Rc<ParamDeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn paramDecl(&self, i: usize) -> Option<Rc<ParamDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> HeadersBlockContextAttrs<'input> for HeadersBlockContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn headersBlock(&mut self,)
	-> Result<Rc<HeadersBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HeadersBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_headersBlock);
        let mut _localctx: Rc<HeadersBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(339);
			recog.base.match_token(HEADERS,&mut recog.err_handler)?;

			recog.base.set_state(341); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule paramDecl*/
				recog.base.set_state(340);
				recog.paramDecl()?;

				}
				}
				recog.base.set_state(343); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==IDENTIFIER) {break}
			}
			recog.base.set_state(345);
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
//------------------- paramDecl ----------------
pub type ParamDeclContextAll<'input> = ParamDeclContext<'input>;


pub type ParamDeclContext<'input> = BaseParserRuleContext<'input,ParamDeclContextExt<'input>>;

#[derive(Clone)]
pub struct ParamDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for ParamDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for ParamDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_paramDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_paramDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for ParamDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_paramDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParamDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_paramDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_paramDecl }
}
antlr_rust::tid!{ParamDeclContextExt<'a>}

impl<'input> ParamDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParamDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParamDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParamDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<ParamDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn typeRef(&self) -> Option<Rc<TypeRefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn paramModifier_all(&self) ->  Vec<Rc<ParamModifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn paramModifier(&self, i: usize) -> Option<Rc<ParamModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ParamDeclContextAttrs<'input> for ParamDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn paramDecl(&mut self,)
	-> Result<Rc<ParamDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParamDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_paramDecl);
        let mut _localctx: Rc<ParamDeclContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(347);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			/*InvokeRule typeRef*/
			recog.base.set_state(348);
			recog.typeRef()?;

			recog.base.set_state(352);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==DEFAULT || _la==OPTIONAL || _la==REQUIRED {
				{
				{
				/*InvokeRule paramModifier*/
				recog.base.set_state(349);
				recog.paramModifier()?;

				}
				}
				recog.base.set_state(354);
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
//------------------- paramModifier ----------------
pub type ParamModifierContextAll<'input> = ParamModifierContext<'input>;


pub type ParamModifierContext<'input> = BaseParserRuleContext<'input,ParamModifierContextExt<'input>>;

#[derive(Clone)]
pub struct ParamModifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for ParamModifierContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for ParamModifierContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_paramModifier(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_paramModifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for ParamModifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_paramModifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParamModifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_paramModifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_paramModifier }
}
antlr_rust::tid!{ParamModifierContextExt<'a>}

impl<'input> ParamModifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParamModifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParamModifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParamModifierContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<ParamModifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token REQUIRED
/// Returns `None` if there is no child corresponding to token REQUIRED
fn REQUIRED(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(REQUIRED, 0)
}
/// Retrieves first TerminalNode corresponding to token OPTIONAL
/// Returns `None` if there is no child corresponding to token OPTIONAL
fn OPTIONAL(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(OPTIONAL, 0)
}
/// Retrieves first TerminalNode corresponding to token DEFAULT
/// Returns `None` if there is no child corresponding to token DEFAULT
fn DEFAULT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(DEFAULT, 0)
}
fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ParamModifierContextAttrs<'input> for ParamModifierContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn paramModifier(&mut self,)
	-> Result<Rc<ParamModifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParamModifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_paramModifier);
        let mut _localctx: Rc<ParamModifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(359);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 REQUIRED 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(355);
					recog.base.match_token(REQUIRED,&mut recog.err_handler)?;

					}
				}

			 OPTIONAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(356);
					recog.base.match_token(OPTIONAL,&mut recog.err_handler)?;

					}
				}

			 DEFAULT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(357);
					recog.base.match_token(DEFAULT,&mut recog.err_handler)?;

					/*InvokeRule literal*/
					recog.base.set_state(358);
					recog.literal()?;

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
//------------------- requestDecl ----------------
pub type RequestDeclContextAll<'input> = RequestDeclContext<'input>;


pub type RequestDeclContext<'input> = BaseParserRuleContext<'input,RequestDeclContextExt<'input>>;

#[derive(Clone)]
pub struct RequestDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for RequestDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for RequestDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_requestDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_requestDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for RequestDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_requestDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for RequestDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_requestDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_requestDecl }
}
antlr_rust::tid!{RequestDeclContextExt<'a>}

impl<'input> RequestDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RequestDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RequestDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RequestDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<RequestDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token REQUEST
/// Returns `None` if there is no child corresponding to token REQUEST
fn REQUEST(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(REQUEST, 0)
}
fn schemaRef(&self) -> Option<Rc<SchemaRefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RequestDeclContextAttrs<'input> for RequestDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn requestDecl(&mut self,)
	-> Result<Rc<RequestDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RequestDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_requestDecl);
        let mut _localctx: Rc<RequestDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(361);
			recog.base.match_token(REQUEST,&mut recog.err_handler)?;

			/*InvokeRule schemaRef*/
			recog.base.set_state(362);
			recog.schemaRef()?;

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
//------------------- responseDecl ----------------
pub type ResponseDeclContextAll<'input> = ResponseDeclContext<'input>;


pub type ResponseDeclContext<'input> = BaseParserRuleContext<'input,ResponseDeclContextExt<'input>>;

#[derive(Clone)]
pub struct ResponseDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for ResponseDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for ResponseDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_responseDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_responseDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for ResponseDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_responseDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for ResponseDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_responseDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_responseDecl }
}
antlr_rust::tid!{ResponseDeclContextExt<'a>}

impl<'input> ResponseDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ResponseDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ResponseDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ResponseDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<ResponseDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token RESPONSE
/// Returns `None` if there is no child corresponding to token RESPONSE
fn RESPONSE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(RESPONSE, 0)
}
fn schemaRef(&self) -> Option<Rc<SchemaRefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn responseModifier(&self) -> Option<Rc<ResponseModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ResponseDeclContextAttrs<'input> for ResponseDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn responseDecl(&mut self,)
	-> Result<Rc<ResponseDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ResponseDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_responseDecl);
        let mut _localctx: Rc<ResponseDeclContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(364);
			recog.base.match_token(RESPONSE,&mut recog.err_handler)?;

			recog.base.set_state(366);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LIST || _la==PAGINATED {
				{
				/*InvokeRule responseModifier*/
				recog.base.set_state(365);
				recog.responseModifier()?;

				}
			}

			/*InvokeRule schemaRef*/
			recog.base.set_state(368);
			recog.schemaRef()?;

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
//------------------- responseModifier ----------------
pub type ResponseModifierContextAll<'input> = ResponseModifierContext<'input>;


pub type ResponseModifierContext<'input> = BaseParserRuleContext<'input,ResponseModifierContextExt<'input>>;

#[derive(Clone)]
pub struct ResponseModifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for ResponseModifierContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for ResponseModifierContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_responseModifier(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_responseModifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for ResponseModifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_responseModifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for ResponseModifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_responseModifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_responseModifier }
}
antlr_rust::tid!{ResponseModifierContextExt<'a>}

impl<'input> ResponseModifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ResponseModifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ResponseModifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ResponseModifierContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<ResponseModifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PAGINATED
/// Returns `None` if there is no child corresponding to token PAGINATED
fn PAGINATED(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(PAGINATED, 0)
}
/// Retrieves first TerminalNode corresponding to token LIST
/// Returns `None` if there is no child corresponding to token LIST
fn LIST(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(LIST, 0)
}

}

impl<'input> ResponseModifierContextAttrs<'input> for ResponseModifierContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn responseModifier(&mut self,)
	-> Result<Rc<ResponseModifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ResponseModifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_responseModifier);
        let mut _localctx: Rc<ResponseModifierContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(370);
			_la = recog.base.input.la(1);
			if { !(_la==LIST || _la==PAGINATED) } {
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
//------------------- errorsBlock ----------------
pub type ErrorsBlockContextAll<'input> = ErrorsBlockContext<'input>;


pub type ErrorsBlockContext<'input> = BaseParserRuleContext<'input,ErrorsBlockContextExt<'input>>;

#[derive(Clone)]
pub struct ErrorsBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for ErrorsBlockContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for ErrorsBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_errorsBlock(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_errorsBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for ErrorsBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_errorsBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for ErrorsBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_errorsBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_errorsBlock }
}
antlr_rust::tid!{ErrorsBlockContextExt<'a>}

impl<'input> ErrorsBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ErrorsBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ErrorsBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ErrorsBlockContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<ErrorsBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ERRORS
/// Returns `None` if there is no child corresponding to token ERRORS
fn ERRORS(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(ERRORS, 0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
fn errorMapping_all(&self) ->  Vec<Rc<ErrorMappingContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn errorMapping(&self, i: usize) -> Option<Rc<ErrorMappingContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ErrorsBlockContextAttrs<'input> for ErrorsBlockContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn errorsBlock(&mut self,)
	-> Result<Rc<ErrorsBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ErrorsBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_errorsBlock);
        let mut _localctx: Rc<ErrorsBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(372);
			recog.base.match_token(ERRORS,&mut recog.err_handler)?;

			recog.base.set_state(374); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule errorMapping*/
				recog.base.set_state(373);
				recog.errorMapping()?;

				}
				}
				recog.base.set_state(376); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==INTEGER) {break}
			}
			recog.base.set_state(378);
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
//------------------- errorMapping ----------------
pub type ErrorMappingContextAll<'input> = ErrorMappingContext<'input>;


pub type ErrorMappingContext<'input> = BaseParserRuleContext<'input,ErrorMappingContextExt<'input>>;

#[derive(Clone)]
pub struct ErrorMappingContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for ErrorMappingContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for ErrorMappingContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_errorMapping(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_errorMapping(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for ErrorMappingContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_errorMapping(self);
	}
}

impl<'input> CustomRuleContext<'input> for ErrorMappingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_errorMapping }
	//fn type_rule_index() -> usize where Self: Sized { RULE_errorMapping }
}
antlr_rust::tid!{ErrorMappingContextExt<'a>}

impl<'input> ErrorMappingContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ErrorMappingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ErrorMappingContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ErrorMappingContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<ErrorMappingContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
/// Retrieves first TerminalNode corresponding to token USING
/// Returns `None` if there is no child corresponding to token USING
fn USING(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(USING, 0)
}
fn schemaRef(&self) -> Option<Rc<SchemaRefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ErrorMappingContextAttrs<'input> for ErrorMappingContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn errorMapping(&mut self,)
	-> Result<Rc<ErrorMappingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ErrorMappingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_errorMapping);
        let mut _localctx: Rc<ErrorMappingContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(380);
			recog.base.match_token(INTEGER,&mut recog.err_handler)?;

			recog.base.set_state(381);
			recog.base.match_token(USING,&mut recog.err_handler)?;

			/*InvokeRule schemaRef*/
			recog.base.set_state(382);
			recog.schemaRef()?;

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
//------------------- authDecl ----------------
pub type AuthDeclContextAll<'input> = AuthDeclContext<'input>;


pub type AuthDeclContext<'input> = BaseParserRuleContext<'input,AuthDeclContextExt<'input>>;

#[derive(Clone)]
pub struct AuthDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for AuthDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for AuthDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_authDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_authDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for AuthDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_authDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for AuthDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_authDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_authDecl }
}
antlr_rust::tid!{AuthDeclContextExt<'a>}

impl<'input> AuthDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AuthDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AuthDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AuthDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<AuthDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token AUTH
/// Returns `None` if there is no child corresponding to token AUTH
fn AUTH(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(AUTH, 0)
}
fn authScheme(&self) -> Option<Rc<AuthSchemeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AuthDeclContextAttrs<'input> for AuthDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn authDecl(&mut self,)
	-> Result<Rc<AuthDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AuthDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_authDecl);
        let mut _localctx: Rc<AuthDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(384);
			recog.base.match_token(AUTH,&mut recog.err_handler)?;

			/*InvokeRule authScheme*/
			recog.base.set_state(385);
			recog.authScheme()?;

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
//------------------- authScheme ----------------
pub type AuthSchemeContextAll<'input> = AuthSchemeContext<'input>;


pub type AuthSchemeContext<'input> = BaseParserRuleContext<'input,AuthSchemeContextExt<'input>>;

#[derive(Clone)]
pub struct AuthSchemeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for AuthSchemeContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for AuthSchemeContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_authScheme(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_authScheme(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for AuthSchemeContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_authScheme(self);
	}
}

impl<'input> CustomRuleContext<'input> for AuthSchemeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_authScheme }
	//fn type_rule_index() -> usize where Self: Sized { RULE_authScheme }
}
antlr_rust::tid!{AuthSchemeContextExt<'a>}

impl<'input> AuthSchemeContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AuthSchemeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AuthSchemeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AuthSchemeContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<AuthSchemeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn authSchemeArg_all(&self) ->  Vec<Rc<AuthSchemeArgContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn authSchemeArg(&self, i: usize) -> Option<Rc<AuthSchemeArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> AuthSchemeContextAttrs<'input> for AuthSchemeContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn authScheme(&mut self,)
	-> Result<Rc<AuthSchemeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AuthSchemeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_authScheme);
        let mut _localctx: Rc<AuthSchemeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(387);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(391);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==HEADER || _la==SCOPE {
				{
				{
				/*InvokeRule authSchemeArg*/
				recog.base.set_state(388);
				recog.authSchemeArg()?;

				}
				}
				recog.base.set_state(393);
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
//------------------- authSchemeArg ----------------
pub type AuthSchemeArgContextAll<'input> = AuthSchemeArgContext<'input>;


pub type AuthSchemeArgContext<'input> = BaseParserRuleContext<'input,AuthSchemeArgContextExt<'input>>;

#[derive(Clone)]
pub struct AuthSchemeArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for AuthSchemeArgContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for AuthSchemeArgContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_authSchemeArg(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_authSchemeArg(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for AuthSchemeArgContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_authSchemeArg(self);
	}
}

impl<'input> CustomRuleContext<'input> for AuthSchemeArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_authSchemeArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_authSchemeArg }
}
antlr_rust::tid!{AuthSchemeArgContextExt<'a>}

impl<'input> AuthSchemeArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AuthSchemeArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AuthSchemeArgContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AuthSchemeArgContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<AuthSchemeArgContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SCOPE
/// Returns `None` if there is no child corresponding to token SCOPE
fn SCOPE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(SCOPE, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token HEADER
/// Returns `None` if there is no child corresponding to token HEADER
fn HEADER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(HEADER, 0)
}

}

impl<'input> AuthSchemeArgContextAttrs<'input> for AuthSchemeArgContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn authSchemeArg(&mut self,)
	-> Result<Rc<AuthSchemeArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AuthSchemeArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_authSchemeArg);
        let mut _localctx: Rc<AuthSchemeArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(398);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 SCOPE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(394);
					recog.base.match_token(SCOPE,&mut recog.err_handler)?;

					recog.base.set_state(395);
					recog.base.match_token(STRING_LITERAL,&mut recog.err_handler)?;

					}
				}

			 HEADER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(396);
					recog.base.match_token(HEADER,&mut recog.err_handler)?;

					recog.base.set_state(397);
					recog.base.match_token(STRING_LITERAL,&mut recog.err_handler)?;

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
//------------------- validateDecl ----------------
pub type ValidateDeclContextAll<'input> = ValidateDeclContext<'input>;


pub type ValidateDeclContext<'input> = BaseParserRuleContext<'input,ValidateDeclContextExt<'input>>;

#[derive(Clone)]
pub struct ValidateDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for ValidateDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for ValidateDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_validateDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_validateDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for ValidateDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_validateDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValidateDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_validateDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_validateDecl }
}
antlr_rust::tid!{ValidateDeclContextExt<'a>}

impl<'input> ValidateDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValidateDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValidateDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ValidateDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<ValidateDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token VALIDATE
/// Returns `None` if there is no child corresponding to token VALIDATE
fn VALIDATE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(VALIDATE, 0)
}
/// Retrieves first TerminalNode corresponding to token USING
/// Returns `None` if there is no child corresponding to token USING
fn USING(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(USING, 0)
}
fn qualifiedRef(&self) -> Option<Rc<QualifiedRefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ValidateDeclContextAttrs<'input> for ValidateDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn validateDecl(&mut self,)
	-> Result<Rc<ValidateDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValidateDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_validateDecl);
        let mut _localctx: Rc<ValidateDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(400);
			recog.base.match_token(VALIDATE,&mut recog.err_handler)?;

			recog.base.set_state(401);
			recog.base.match_token(USING,&mut recog.err_handler)?;

			/*InvokeRule qualifiedRef*/
			recog.base.set_state(402);
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
//------------------- timeoutDecl ----------------
pub type TimeoutDeclContextAll<'input> = TimeoutDeclContext<'input>;


pub type TimeoutDeclContext<'input> = BaseParserRuleContext<'input,TimeoutDeclContextExt<'input>>;

#[derive(Clone)]
pub struct TimeoutDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for TimeoutDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for TimeoutDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_timeoutDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_timeoutDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for TimeoutDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_timeoutDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for TimeoutDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_timeoutDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_timeoutDecl }
}
antlr_rust::tid!{TimeoutDeclContextExt<'a>}

impl<'input> TimeoutDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TimeoutDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TimeoutDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TimeoutDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<TimeoutDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TIMEOUT
/// Returns `None` if there is no child corresponding to token TIMEOUT
fn TIMEOUT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(TIMEOUT, 0)
}
fn valueOrCfg(&self) -> Option<Rc<ValueOrCfgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> TimeoutDeclContextAttrs<'input> for TimeoutDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn timeoutDecl(&mut self,)
	-> Result<Rc<TimeoutDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TimeoutDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_timeoutDecl);
        let mut _localctx: Rc<TimeoutDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(404);
			recog.base.match_token(TIMEOUT,&mut recog.err_handler)?;

			/*InvokeRule valueOrCfg*/
			recog.base.set_state(405);
			recog.valueOrCfg()?;

			recog.base.set_state(406);
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
//------------------- cacheDecl ----------------
pub type CacheDeclContextAll<'input> = CacheDeclContext<'input>;


pub type CacheDeclContext<'input> = BaseParserRuleContext<'input,CacheDeclContextExt<'input>>;

#[derive(Clone)]
pub struct CacheDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for CacheDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for CacheDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_cacheDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_cacheDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for CacheDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_cacheDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for CacheDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cacheDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cacheDecl }
}
antlr_rust::tid!{CacheDeclContextExt<'a>}

impl<'input> CacheDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CacheDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CacheDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CacheDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<CacheDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CACHE
/// Returns `None` if there is no child corresponding to token CACHE
fn CACHE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(CACHE, 0)
}
fn valueOrCfg(&self) -> Option<Rc<ValueOrCfgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> CacheDeclContextAttrs<'input> for CacheDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn cacheDecl(&mut self,)
	-> Result<Rc<CacheDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CacheDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_cacheDecl);
        let mut _localctx: Rc<CacheDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(408);
			recog.base.match_token(CACHE,&mut recog.err_handler)?;

			/*InvokeRule valueOrCfg*/
			recog.base.set_state(409);
			recog.valueOrCfg()?;

			recog.base.set_state(410);
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
//------------------- idempotentDecl ----------------
pub type IdempotentDeclContextAll<'input> = IdempotentDeclContext<'input>;


pub type IdempotentDeclContext<'input> = BaseParserRuleContext<'input,IdempotentDeclContextExt<'input>>;

#[derive(Clone)]
pub struct IdempotentDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for IdempotentDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for IdempotentDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_idempotentDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_idempotentDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for IdempotentDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_idempotentDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdempotentDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_idempotentDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_idempotentDecl }
}
antlr_rust::tid!{IdempotentDeclContextExt<'a>}

impl<'input> IdempotentDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IdempotentDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdempotentDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IdempotentDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<IdempotentDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDEMPOTENT
/// Returns `None` if there is no child corresponding to token IDEMPOTENT
fn IDEMPOTENT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDEMPOTENT, 0)
}
/// Retrieves first TerminalNode corresponding to token KEY
/// Returns `None` if there is no child corresponding to token KEY
fn KEY(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(KEY, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}

}

impl<'input> IdempotentDeclContextAttrs<'input> for IdempotentDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn idempotentDecl(&mut self,)
	-> Result<Rc<IdempotentDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdempotentDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_idempotentDecl);
        let mut _localctx: Rc<IdempotentDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(412);
			recog.base.match_token(IDEMPOTENT,&mut recog.err_handler)?;

			recog.base.set_state(413);
			recog.base.match_token(KEY,&mut recog.err_handler)?;

			recog.base.set_state(414);
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
//------------------- asyncDecl ----------------
pub type AsyncDeclContextAll<'input> = AsyncDeclContext<'input>;


pub type AsyncDeclContext<'input> = BaseParserRuleContext<'input,AsyncDeclContextExt<'input>>;

#[derive(Clone)]
pub struct AsyncDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for AsyncDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for AsyncDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_asyncDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_asyncDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for AsyncDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_asyncDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for AsyncDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_asyncDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_asyncDecl }
}
antlr_rust::tid!{AsyncDeclContextExt<'a>}

impl<'input> AsyncDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AsyncDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AsyncDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AsyncDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<AsyncDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ASYNC
/// Returns `None` if there is no child corresponding to token ASYNC
fn ASYNC(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(ASYNC, 0)
}

}

impl<'input> AsyncDeclContextAttrs<'input> for AsyncDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn asyncDecl(&mut self,)
	-> Result<Rc<AsyncDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AsyncDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_asyncDecl);
        let mut _localctx: Rc<AsyncDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(416);
			recog.base.match_token(ASYNC,&mut recog.err_handler)?;

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
//------------------- sunsetDecl ----------------
pub type SunsetDeclContextAll<'input> = SunsetDeclContext<'input>;


pub type SunsetDeclContext<'input> = BaseParserRuleContext<'input,SunsetDeclContextExt<'input>>;

#[derive(Clone)]
pub struct SunsetDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for SunsetDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for SunsetDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_sunsetDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_sunsetDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for SunsetDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_sunsetDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for SunsetDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sunsetDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sunsetDecl }
}
antlr_rust::tid!{SunsetDeclContextExt<'a>}

impl<'input> SunsetDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SunsetDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SunsetDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SunsetDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<SunsetDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SUNSET
/// Returns `None` if there is no child corresponding to token SUNSET
fn SUNSET(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(SUNSET, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}

}

impl<'input> SunsetDeclContextAttrs<'input> for SunsetDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn sunsetDecl(&mut self,)
	-> Result<Rc<SunsetDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SunsetDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_sunsetDecl);
        let mut _localctx: Rc<SunsetDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(418);
			recog.base.match_token(SUNSET,&mut recog.err_handler)?;

			recog.base.set_state(419);
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
//------------------- replacementDecl ----------------
pub type ReplacementDeclContextAll<'input> = ReplacementDeclContext<'input>;


pub type ReplacementDeclContext<'input> = BaseParserRuleContext<'input,ReplacementDeclContextExt<'input>>;

#[derive(Clone)]
pub struct ReplacementDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for ReplacementDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for ReplacementDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_replacementDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_replacementDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for ReplacementDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_replacementDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReplacementDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_replacementDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_replacementDecl }
}
antlr_rust::tid!{ReplacementDeclContextExt<'a>}

impl<'input> ReplacementDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReplacementDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReplacementDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReplacementDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<ReplacementDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token REPLACEMENT
/// Returns `None` if there is no child corresponding to token REPLACEMENT
fn REPLACEMENT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(REPLACEMENT, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> ReplacementDeclContextAttrs<'input> for ReplacementDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn replacementDecl(&mut self,)
	-> Result<Rc<ReplacementDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReplacementDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_replacementDecl);
        let mut _localctx: Rc<ReplacementDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(421);
			recog.base.match_token(REPLACEMENT,&mut recog.err_handler)?;

			recog.base.set_state(422);
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
//------------------- eventDecl ----------------
pub type EventDeclContextAll<'input> = EventDeclContext<'input>;


pub type EventDeclContext<'input> = BaseParserRuleContext<'input,EventDeclContextExt<'input>>;

#[derive(Clone)]
pub struct EventDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for EventDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for EventDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_eventDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_eventDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for EventDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_eventDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for EventDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eventDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eventDecl }
}
antlr_rust::tid!{EventDeclContextExt<'a>}

impl<'input> EventDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EventDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EventDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EventDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<EventDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EVENT
/// Returns `None` if there is no child corresponding to token EVENT
fn EVENT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(EVENT, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn eventBody(&self) -> Option<Rc<EventBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}

}

impl<'input> EventDeclContextAttrs<'input> for EventDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eventDecl(&mut self,)
	-> Result<Rc<EventDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EventDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 96, RULE_eventDecl);
        let mut _localctx: Rc<EventDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(424);
			recog.base.match_token(EVENT,&mut recog.err_handler)?;

			recog.base.set_state(425);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			/*InvokeRule eventBody*/
			recog.base.set_state(426);
			recog.eventBody()?;

			recog.base.set_state(427);
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
//------------------- eventBody ----------------
pub type EventBodyContextAll<'input> = EventBodyContext<'input>;


pub type EventBodyContext<'input> = BaseParserRuleContext<'input,EventBodyContextExt<'input>>;

#[derive(Clone)]
pub struct EventBodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for EventBodyContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for EventBodyContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_eventBody(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_eventBody(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for EventBodyContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_eventBody(self);
	}
}

impl<'input> CustomRuleContext<'input> for EventBodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eventBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eventBody }
}
antlr_rust::tid!{EventBodyContextExt<'a>}

impl<'input> EventBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EventBodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EventBodyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EventBodyContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<EventBodyContextExt<'input>>{

fn eventClause_all(&self) ->  Vec<Rc<EventClauseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn eventClause(&self, i: usize) -> Option<Rc<EventClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> EventBodyContextAttrs<'input> for EventBodyContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eventBody(&mut self,)
	-> Result<Rc<EventBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EventBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 98, RULE_eventBody);
        let mut _localctx: Rc<EventBodyContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(432);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 35)) & !0x3f) == 0 && ((1usize << (_la - 35)) & ((1usize << (PARTITIONED_BY - 35)) | (1usize << (PAYLOAD - 35)) | (1usize << (TOPIC - 35)))) != 0) {
				{
				{
				/*InvokeRule eventClause*/
				recog.base.set_state(429);
				recog.eventClause()?;

				}
				}
				recog.base.set_state(434);
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
//------------------- eventClause ----------------
pub type EventClauseContextAll<'input> = EventClauseContext<'input>;


pub type EventClauseContext<'input> = BaseParserRuleContext<'input,EventClauseContextExt<'input>>;

#[derive(Clone)]
pub struct EventClauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for EventClauseContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for EventClauseContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_eventClause(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_eventClause(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for EventClauseContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_eventClause(self);
	}
}

impl<'input> CustomRuleContext<'input> for EventClauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eventClause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eventClause }
}
antlr_rust::tid!{EventClauseContextExt<'a>}

impl<'input> EventClauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EventClauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EventClauseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EventClauseContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<EventClauseContextExt<'input>>{

fn topicDecl(&self) -> Option<Rc<TopicDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn payloadDecl(&self) -> Option<Rc<PayloadDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn partitionedByDecl(&self) -> Option<Rc<PartitionedByDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EventClauseContextAttrs<'input> for EventClauseContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eventClause(&mut self,)
	-> Result<Rc<EventClauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EventClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 100, RULE_eventClause);
        let mut _localctx: Rc<EventClauseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(438);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 TOPIC 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule topicDecl*/
					recog.base.set_state(435);
					recog.topicDecl()?;

					}
				}

			 PAYLOAD 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule payloadDecl*/
					recog.base.set_state(436);
					recog.payloadDecl()?;

					}
				}

			 PARTITIONED_BY 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule partitionedByDecl*/
					recog.base.set_state(437);
					recog.partitionedByDecl()?;

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
//------------------- topicDecl ----------------
pub type TopicDeclContextAll<'input> = TopicDeclContext<'input>;


pub type TopicDeclContext<'input> = BaseParserRuleContext<'input,TopicDeclContextExt<'input>>;

#[derive(Clone)]
pub struct TopicDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for TopicDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for TopicDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_topicDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_topicDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for TopicDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_topicDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for TopicDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_topicDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_topicDecl }
}
antlr_rust::tid!{TopicDeclContextExt<'a>}

impl<'input> TopicDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TopicDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TopicDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TopicDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<TopicDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TOPIC
/// Returns `None` if there is no child corresponding to token TOPIC
fn TOPIC(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(TOPIC, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}

}

impl<'input> TopicDeclContextAttrs<'input> for TopicDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn topicDecl(&mut self,)
	-> Result<Rc<TopicDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TopicDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 102, RULE_topicDecl);
        let mut _localctx: Rc<TopicDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(440);
			recog.base.match_token(TOPIC,&mut recog.err_handler)?;

			recog.base.set_state(441);
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
//------------------- payloadDecl ----------------
pub type PayloadDeclContextAll<'input> = PayloadDeclContext<'input>;


pub type PayloadDeclContext<'input> = BaseParserRuleContext<'input,PayloadDeclContextExt<'input>>;

#[derive(Clone)]
pub struct PayloadDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for PayloadDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for PayloadDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_payloadDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_payloadDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for PayloadDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_payloadDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for PayloadDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_payloadDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_payloadDecl }
}
antlr_rust::tid!{PayloadDeclContextExt<'a>}

impl<'input> PayloadDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PayloadDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PayloadDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PayloadDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<PayloadDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PAYLOAD
/// Returns `None` if there is no child corresponding to token PAYLOAD
fn PAYLOAD(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(PAYLOAD, 0)
}
fn schemaRef(&self) -> Option<Rc<SchemaRefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PayloadDeclContextAttrs<'input> for PayloadDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn payloadDecl(&mut self,)
	-> Result<Rc<PayloadDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PayloadDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 104, RULE_payloadDecl);
        let mut _localctx: Rc<PayloadDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(443);
			recog.base.match_token(PAYLOAD,&mut recog.err_handler)?;

			/*InvokeRule schemaRef*/
			recog.base.set_state(444);
			recog.schemaRef()?;

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
//------------------- partitionedByDecl ----------------
pub type PartitionedByDeclContextAll<'input> = PartitionedByDeclContext<'input>;


pub type PartitionedByDeclContext<'input> = BaseParserRuleContext<'input,PartitionedByDeclContextExt<'input>>;

#[derive(Clone)]
pub struct PartitionedByDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for PartitionedByDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for PartitionedByDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_partitionedByDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_partitionedByDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for PartitionedByDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_partitionedByDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for PartitionedByDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_partitionedByDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_partitionedByDecl }
}
antlr_rust::tid!{PartitionedByDeclContextExt<'a>}

impl<'input> PartitionedByDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PartitionedByDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PartitionedByDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PartitionedByDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<PartitionedByDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PARTITIONED_BY
/// Returns `None` if there is no child corresponding to token PARTITIONED_BY
fn PARTITIONED_BY(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(PARTITIONED_BY, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> PartitionedByDeclContextAttrs<'input> for PartitionedByDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn partitionedByDecl(&mut self,)
	-> Result<Rc<PartitionedByDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PartitionedByDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 106, RULE_partitionedByDecl);
        let mut _localctx: Rc<PartitionedByDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(446);
			recog.base.match_token(PARTITIONED_BY,&mut recog.err_handler)?;

			recog.base.set_state(447);
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
//------------------- dependenciesBlock ----------------
pub type DependenciesBlockContextAll<'input> = DependenciesBlockContext<'input>;


pub type DependenciesBlockContext<'input> = BaseParserRuleContext<'input,DependenciesBlockContextExt<'input>>;

#[derive(Clone)]
pub struct DependenciesBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for DependenciesBlockContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for DependenciesBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_dependenciesBlock(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_dependenciesBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for DependenciesBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_dependenciesBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for DependenciesBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_dependenciesBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_dependenciesBlock }
}
antlr_rust::tid!{DependenciesBlockContextExt<'a>}

impl<'input> DependenciesBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DependenciesBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DependenciesBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DependenciesBlockContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<DependenciesBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DEPENDENCIES
/// Returns `None` if there is no child corresponding to token DEPENDENCIES
fn DEPENDENCIES(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(DEPENDENCIES, 0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
fn dependencyDecl_all(&self) ->  Vec<Rc<DependencyDeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn dependencyDecl(&self, i: usize) -> Option<Rc<DependencyDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> DependenciesBlockContextAttrs<'input> for DependenciesBlockContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn dependenciesBlock(&mut self,)
	-> Result<Rc<DependenciesBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DependenciesBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 108, RULE_dependenciesBlock);
        let mut _localctx: Rc<DependenciesBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(449);
			recog.base.match_token(DEPENDENCIES,&mut recog.err_handler)?;

			recog.base.set_state(451); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule dependencyDecl*/
				recog.base.set_state(450);
				recog.dependencyDecl()?;

				}
				}
				recog.base.set_state(453); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==IDENTIFIER) {break}
			}
			recog.base.set_state(455);
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
//------------------- dependencyDecl ----------------
pub type DependencyDeclContextAll<'input> = DependencyDeclContext<'input>;


pub type DependencyDeclContext<'input> = BaseParserRuleContext<'input,DependencyDeclContextExt<'input>>;

#[derive(Clone)]
pub struct DependencyDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for DependencyDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for DependencyDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_dependencyDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_dependencyDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for DependencyDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_dependencyDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for DependencyDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_dependencyDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_dependencyDecl }
}
antlr_rust::tid!{DependencyDeclContextExt<'a>}

impl<'input> DependencyDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DependencyDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DependencyDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DependencyDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<DependencyDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token VERSION
/// Returns `None` if there is no child corresponding to token VERSION
fn VERSION(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(VERSION, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}

}

impl<'input> DependencyDeclContextAttrs<'input> for DependencyDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn dependencyDecl(&mut self,)
	-> Result<Rc<DependencyDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DependencyDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 110, RULE_dependencyDecl);
        let mut _localctx: Rc<DependencyDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(457);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(458);
			recog.base.match_token(VERSION,&mut recog.err_handler)?;

			recog.base.set_state(459);
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
//------------------- healthDecl ----------------
pub type HealthDeclContextAll<'input> = HealthDeclContext<'input>;


pub type HealthDeclContext<'input> = BaseParserRuleContext<'input,HealthDeclContextExt<'input>>;

#[derive(Clone)]
pub struct HealthDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for HealthDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for HealthDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_healthDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_healthDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for HealthDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_healthDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for HealthDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_healthDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_healthDecl }
}
antlr_rust::tid!{HealthDeclContextExt<'a>}

impl<'input> HealthDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<HealthDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HealthDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait HealthDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<HealthDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token HEALTH
/// Returns `None` if there is no child corresponding to token HEALTH
fn HEALTH(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(HEALTH, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}

}

impl<'input> HealthDeclContextAttrs<'input> for HealthDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn healthDecl(&mut self,)
	-> Result<Rc<HealthDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HealthDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 112, RULE_healthDecl);
        let mut _localctx: Rc<HealthDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(461);
			recog.base.match_token(HEALTH,&mut recog.err_handler)?;

			recog.base.set_state(462);
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
//------------------- readyDecl ----------------
pub type ReadyDeclContextAll<'input> = ReadyDeclContext<'input>;


pub type ReadyDeclContext<'input> = BaseParserRuleContext<'input,ReadyDeclContextExt<'input>>;

#[derive(Clone)]
pub struct ReadyDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for ReadyDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for ReadyDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_readyDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_readyDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for ReadyDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_readyDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReadyDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_readyDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_readyDecl }
}
antlr_rust::tid!{ReadyDeclContextExt<'a>}

impl<'input> ReadyDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReadyDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReadyDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReadyDeclContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<ReadyDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token READY
/// Returns `None` if there is no child corresponding to token READY
fn READY(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(READY, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}

}

impl<'input> ReadyDeclContextAttrs<'input> for ReadyDeclContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn readyDecl(&mut self,)
	-> Result<Rc<ReadyDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReadyDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 114, RULE_readyDecl);
        let mut _localctx: Rc<ReadyDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(464);
			recog.base.match_token(READY,&mut recog.err_handler)?;

			recog.base.set_state(465);
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

impl<'input> ApiDSLParserContext<'input> for QualifiedAnnotationContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for QualifiedAnnotationContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_qualifiedAnnotation(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_qualifiedAnnotation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for QualifiedAnnotationContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_qualifiedAnnotation(self);
	}
}

impl<'input> CustomRuleContext<'input> for QualifiedAnnotationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_qualifiedAnnotation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_qualifiedAnnotation }
}
antlr_rust::tid!{QualifiedAnnotationContextExt<'a>}

impl<'input> QualifiedAnnotationContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<QualifiedAnnotationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,QualifiedAnnotationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait QualifiedAnnotationContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<QualifiedAnnotationContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,ApiDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}

}

impl<'input> QualifiedAnnotationContextAttrs<'input> for QualifiedAnnotationContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn qualifiedAnnotation(&mut self,)
	-> Result<Rc<QualifiedAnnotationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = QualifiedAnnotationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_qualifiedAnnotation);
        let mut _localctx: Rc<QualifiedAnnotationContextAll> = _localctx;
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
//------------------- valueOrCfg ----------------
pub type ValueOrCfgContextAll<'input> = ValueOrCfgContext<'input>;


pub type ValueOrCfgContext<'input> = BaseParserRuleContext<'input,ValueOrCfgContextExt<'input>>;

#[derive(Clone)]
pub struct ValueOrCfgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for ValueOrCfgContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for ValueOrCfgContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_valueOrCfg(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_valueOrCfg(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for ValueOrCfgContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_valueOrCfg(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueOrCfgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_valueOrCfg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_valueOrCfg }
}
antlr_rust::tid!{ValueOrCfgContextExt<'a>}

impl<'input> ValueOrCfgContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValueOrCfgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValueOrCfgContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ValueOrCfgContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<ValueOrCfgContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
/// Retrieves first TerminalNode corresponding to token DECIMAL_LITERAL
/// Returns `None` if there is no child corresponding to token DECIMAL_LITERAL
fn DECIMAL_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(DECIMAL_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}
fn qualifiedAnnotation(&self) -> Option<Rc<QualifiedAnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ValueOrCfgContextAttrs<'input> for ValueOrCfgContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn valueOrCfg(&mut self,)
	-> Result<Rc<ValueOrCfgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValueOrCfgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_valueOrCfg);
        let mut _localctx: Rc<ValueOrCfgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(475);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 INTEGER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(471);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					}
				}

			 DECIMAL_LITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(472);
					recog.base.match_token(DECIMAL_LITERAL,&mut recog.err_handler)?;

					}
				}

			 STRING_LITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(473);
					recog.base.match_token(STRING_LITERAL,&mut recog.err_handler)?;

					}
				}

			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule qualifiedAnnotation*/
					recog.base.set_state(474);
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
//------------------- valueOrCfgList ----------------
pub type ValueOrCfgListContextAll<'input> = ValueOrCfgListContext<'input>;


pub type ValueOrCfgListContext<'input> = BaseParserRuleContext<'input,ValueOrCfgListContextExt<'input>>;

#[derive(Clone)]
pub struct ValueOrCfgListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for ValueOrCfgListContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for ValueOrCfgListContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_valueOrCfgList(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_valueOrCfgList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for ValueOrCfgListContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_valueOrCfgList(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueOrCfgListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_valueOrCfgList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_valueOrCfgList }
}
antlr_rust::tid!{ValueOrCfgListContextExt<'a>}

impl<'input> ValueOrCfgListContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValueOrCfgListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValueOrCfgListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ValueOrCfgListContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<ValueOrCfgListContextExt<'input>>{

fn valueOrCfg_all(&self) ->  Vec<Rc<ValueOrCfgContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn valueOrCfg(&self, i: usize) -> Option<Rc<ValueOrCfgContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,ApiDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ValueOrCfgListContextAttrs<'input> for ValueOrCfgListContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn valueOrCfgList(&mut self,)
	-> Result<Rc<ValueOrCfgListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValueOrCfgListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 120, RULE_valueOrCfgList);
        let mut _localctx: Rc<ValueOrCfgListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule valueOrCfg*/
			recog.base.set_state(477);
			recog.valueOrCfg()?;

			recog.base.set_state(482);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(478);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule valueOrCfg*/
				recog.base.set_state(479);
				recog.valueOrCfg()?;

				}
				}
				recog.base.set_state(484);
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
//------------------- schemaRef ----------------
pub type SchemaRefContextAll<'input> = SchemaRefContext<'input>;


pub type SchemaRefContext<'input> = BaseParserRuleContext<'input,SchemaRefContextExt<'input>>;

#[derive(Clone)]
pub struct SchemaRefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for SchemaRefContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for SchemaRefContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_schemaRef(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_schemaRef(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for SchemaRefContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_schemaRef(self);
	}
}

impl<'input> CustomRuleContext<'input> for SchemaRefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_schemaRef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_schemaRef }
}
antlr_rust::tid!{SchemaRefContextExt<'a>}

impl<'input> SchemaRefContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SchemaRefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SchemaRefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SchemaRefContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<SchemaRefContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,ApiDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}

}

impl<'input> SchemaRefContextAttrs<'input> for SchemaRefContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn schemaRef(&mut self,)
	-> Result<Rc<SchemaRefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SchemaRefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_schemaRef);
        let mut _localctx: Rc<SchemaRefContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(485);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(488);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==DOT {
				{
				recog.base.set_state(486);
				recog.base.match_token(DOT,&mut recog.err_handler)?;

				recog.base.set_state(487);
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
//------------------- qualifiedRef ----------------
pub type QualifiedRefContextAll<'input> = QualifiedRefContext<'input>;


pub type QualifiedRefContext<'input> = BaseParserRuleContext<'input,QualifiedRefContextExt<'input>>;

#[derive(Clone)]
pub struct QualifiedRefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for QualifiedRefContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for QualifiedRefContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_qualifiedRef(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_qualifiedRef(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for QualifiedRefContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_qualifiedRef(self);
	}
}

impl<'input> CustomRuleContext<'input> for QualifiedRefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_qualifiedRef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_qualifiedRef }
}
antlr_rust::tid!{QualifiedRefContextExt<'a>}

impl<'input> QualifiedRefContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<QualifiedRefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,QualifiedRefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait QualifiedRefContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<QualifiedRefContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,ApiDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}

}

impl<'input> QualifiedRefContextAttrs<'input> for QualifiedRefContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn qualifiedRef(&mut self,)
	-> Result<Rc<QualifiedRefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = QualifiedRefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 124, RULE_qualifiedRef);
        let mut _localctx: Rc<QualifiedRefContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(490);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(491);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			recog.base.set_state(492);
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
//------------------- typeRef ----------------
pub type TypeRefContextAll<'input> = TypeRefContext<'input>;


pub type TypeRefContext<'input> = BaseParserRuleContext<'input,TypeRefContextExt<'input>>;

#[derive(Clone)]
pub struct TypeRefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for TypeRefContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for TypeRefContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_typeRef(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_typeRef(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for TypeRefContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_typeRef(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeRefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeRef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeRef }
}
antlr_rust::tid!{TypeRefContextExt<'a>}

impl<'input> TypeRefContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeRefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeRefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeRefContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<TypeRefContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> TypeRefContextAttrs<'input> for TypeRefContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeRef(&mut self,)
	-> Result<Rc<TypeRefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeRefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_typeRef);
        let mut _localctx: Rc<TypeRefContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(494);
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
//------------------- identifierList ----------------
pub type IdentifierListContextAll<'input> = IdentifierListContext<'input>;


pub type IdentifierListContext<'input> = BaseParserRuleContext<'input,IdentifierListContextExt<'input>>;

#[derive(Clone)]
pub struct IdentifierListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for IdentifierListContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for IdentifierListContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_identifierList(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_identifierList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for IdentifierListContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_identifierList(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdentifierListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_identifierList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_identifierList }
}
antlr_rust::tid!{IdentifierListContextExt<'a>}

impl<'input> IdentifierListContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IdentifierListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdentifierListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IdentifierListContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<IdentifierListContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,ApiDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,ApiDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> IdentifierListContextAttrs<'input> for IdentifierListContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn identifierList(&mut self,)
	-> Result<Rc<IdentifierListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdentifierListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 128, RULE_identifierList);
        let mut _localctx: Rc<IdentifierListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(496);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(501);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(497);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				recog.base.set_state(498);
				recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(503);
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
//------------------- stringList ----------------
pub type StringListContextAll<'input> = StringListContext<'input>;


pub type StringListContext<'input> = BaseParserRuleContext<'input,StringListContextExt<'input>>;

#[derive(Clone)]
pub struct StringListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for StringListContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for StringListContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stringList(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_stringList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for StringListContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_stringList(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stringList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stringList }
}
antlr_rust::tid!{StringListContextExt<'a>}

impl<'input> StringListContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StringListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StringListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StringListContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<StringListContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token STRING_LITERAL in current rule
fn STRING_LITERAL_all(&self) -> Vec<Rc<TerminalNode<'input,ApiDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token STRING_LITERAL, starting from 0.
/// Returns `None` if number of children corresponding to token STRING_LITERAL is less or equal than `i`.
fn STRING_LITERAL(&self, i: usize) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,ApiDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> StringListContextAttrs<'input> for StringListContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stringList(&mut self,)
	-> Result<Rc<StringListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StringListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 130, RULE_stringList);
        let mut _localctx: Rc<StringListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(504);
			recog.base.match_token(STRING_LITERAL,&mut recog.err_handler)?;

			recog.base.set_state(509);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(505);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				recog.base.set_state(506);
				recog.base.match_token(STRING_LITERAL,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(511);
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
//------------------- httpMethodList ----------------
pub type HttpMethodListContextAll<'input> = HttpMethodListContext<'input>;


pub type HttpMethodListContext<'input> = BaseParserRuleContext<'input,HttpMethodListContextExt<'input>>;

#[derive(Clone)]
pub struct HttpMethodListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ApiDSLParserContext<'input> for HttpMethodListContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for HttpMethodListContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_httpMethodList(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_httpMethodList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for HttpMethodListContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_httpMethodList(self);
	}
}

impl<'input> CustomRuleContext<'input> for HttpMethodListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_httpMethodList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_httpMethodList }
}
antlr_rust::tid!{HttpMethodListContextExt<'a>}

impl<'input> HttpMethodListContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<HttpMethodListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HttpMethodListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait HttpMethodListContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<HttpMethodListContextExt<'input>>{

fn httpMethod_all(&self) ->  Vec<Rc<HttpMethodContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn httpMethod(&self, i: usize) -> Option<Rc<HttpMethodContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,ApiDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> HttpMethodListContextAttrs<'input> for HttpMethodListContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn httpMethodList(&mut self,)
	-> Result<Rc<HttpMethodListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HttpMethodListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 132, RULE_httpMethodList);
        let mut _localctx: Rc<HttpMethodListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule httpMethod*/
			recog.base.set_state(512);
			recog.httpMethod()?;

			recog.base.set_state(517);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(513);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule httpMethod*/
				recog.base.set_state(514);
				recog.httpMethod()?;

				}
				}
				recog.base.set_state(519);
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

impl<'input> ApiDSLParserContext<'input> for LiteralContext<'input>{}

impl<'input,'a> Listenable<dyn ApiDSLListener<'input> + 'a> for LiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_literal(self);
		}
		fn exit(&self,listener: &mut (dyn ApiDSLListener<'input> + 'a)) {
			listener.exit_literal(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ApiDSLVisitor<'input> + 'a> for LiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn ApiDSLVisitor<'input> + 'a)) {
		visitor.visit_literal(self);
	}
}

impl<'input> CustomRuleContext<'input> for LiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ApiDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literal }
}
antlr_rust::tid!{LiteralContextExt<'a>}

impl<'input> LiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn ApiDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LiteralContextAttrs<'input>: ApiDSLParserContext<'input> + BorrowMut<LiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
/// Retrieves first TerminalNode corresponding to token DECIMAL_LITERAL
/// Returns `None` if there is no child corresponding to token DECIMAL_LITERAL
fn DECIMAL_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(DECIMAL_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token TRUE
/// Returns `None` if there is no child corresponding to token TRUE
fn TRUE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(TRUE, 0)
}
/// Retrieves first TerminalNode corresponding to token FALSE
/// Returns `None` if there is no child corresponding to token FALSE
fn FALSE(&self) -> Option<Rc<TerminalNode<'input,ApiDSLParserContextType>>> where Self:Sized{
	self.get_token(FALSE, 0)
}

}

impl<'input> LiteralContextAttrs<'input> for LiteralContext<'input>{}

impl<'input, I, H> ApiDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn literal(&mut self,)
	-> Result<Rc<LiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 134, RULE_literal);
        let mut _localctx: Rc<LiteralContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(520);
			_la = recog.base.input.la(1);
			if { !(((((_la - 59)) & !0x3f) == 0 && ((1usize << (_la - 59)) & ((1usize << (TRUE - 59)) | (1usize << (FALSE - 59)) | (1usize << (INTEGER - 59)) | (1usize << (DECIMAL_LITERAL - 59)) | (1usize << (STRING_LITERAL - 59)))) != 0)) } {
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
	\x4a\u{20d}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
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
	\x32\x04\x33\x09\x33\x04\x34\x09\x34\x04\x35\x09\x35\x04\x36\x09\x36\x04\
	\x37\x09\x37\x04\x38\x09\x38\x04\x39\x09\x39\x04\x3a\x09\x3a\x04\x3b\x09\
	\x3b\x04\x3c\x09\x3c\x04\x3d\x09\x3d\x04\x3e\x09\x3e\x04\x3f\x09\x3f\x04\
	\x40\x09\x40\x04\x41\x09\x41\x04\x42\x09\x42\x04\x43\x09\x43\x04\x44\x09\
	\x44\x04\x45\x09\x45\x03\x02\x07\x02\u{8c}\x0a\x02\x0c\x02\x0e\x02\u{8f}\
	\x0b\x02\x03\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\x03\x03\x04\x05\x04\
	\u{98}\x0a\x04\x03\x04\x03\x04\x03\x04\x07\x04\u{9d}\x0a\x04\x0c\x04\x0e\
	\x04\u{a0}\x0b\x04\x03\x04\x03\x04\x03\x04\x03\x05\x03\x05\x03\x05\x07\x05\
	\u{a8}\x0a\x05\x0c\x05\x0e\x05\u{ab}\x0b\x05\x03\x06\x03\x06\x03\x07\x03\
	\x07\x03\x07\x03\x07\x03\x07\x03\x08\x07\x08\u{b5}\x0a\x08\x0c\x08\x0e\x08\
	\u{b8}\x0b\x08\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\
	\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x05\
	\x09\u{ca}\x0a\x09\x03\x0a\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0c\
	\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0d\x03\x0e\x03\x0e\x03\x0e\x03\x0e\
	\x03\x0f\x03\x0f\x03\x0f\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\
	\x05\x10\u{e5}\x0a\x10\x03\x11\x03\x11\x03\x11\x03\x11\x03\x11\x03\x11\x03\
	\x11\x03\x12\x03\x12\x07\x12\u{f0}\x0a\x12\x0c\x12\x0e\x12\u{f3}\x0b\x12\
	\x03\x12\x03\x12\x03\x13\x03\x13\x03\x13\x03\x13\x07\x13\u{fb}\x0a\x13\x0c\
	\x13\x0e\x13\u{fe}\x0b\x13\x03\x14\x03\x14\x03\x15\x03\x15\x07\x15\u{104}\
	\x0a\x15\x0c\x15\x0e\x15\u{107}\x0b\x15\x03\x15\x03\x15\x03\x16\x03\x16\
	\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x05\x16\u{113}\x0a\x16\
	\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x18\x03\x18\x03\x18\x03\x18\
	\x03\x18\x05\x18\u{11f}\x0a\x18\x03\x18\x05\x18\u{122}\x0a\x18\x03\x18\x03\
	\x18\x03\x19\x07\x19\u{127}\x0a\x19\x0c\x19\x0e\x19\u{12a}\x0b\x19\x03\x1a\
	\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\
	\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x05\x1a\u{13c}\x0a\x1a\
	\x03\x1b\x03\x1b\x03\x1b\x03\x1c\x03\x1c\x03\x1d\x03\x1d\x03\x1d\x03\x1e\
	\x03\x1e\x06\x1e\u{148}\x0a\x1e\x0d\x1e\x0e\x1e\u{149}\x03\x1e\x03\x1e\x03\
	\x1f\x03\x1f\x06\x1f\u{150}\x0a\x1f\x0d\x1f\x0e\x1f\u{151}\x03\x1f\x03\x1f\
	\x03\x20\x03\x20\x06\x20\u{158}\x0a\x20\x0d\x20\x0e\x20\u{159}\x03\x20\x03\
	\x20\x03\x21\x03\x21\x03\x21\x07\x21\u{161}\x0a\x21\x0c\x21\x0e\x21\u{164}\
	\x0b\x21\x03\x22\x03\x22\x03\x22\x03\x22\x05\x22\u{16a}\x0a\x22\x03\x23\
	\x03\x23\x03\x23\x03\x24\x03\x24\x05\x24\u{171}\x0a\x24\x03\x24\x03\x24\
	\x03\x25\x03\x25\x03\x26\x03\x26\x06\x26\u{179}\x0a\x26\x0d\x26\x0e\x26\
	\u{17a}\x03\x26\x03\x26\x03\x27\x03\x27\x03\x27\x03\x27\x03\x28\x03\x28\
	\x03\x28\x03\x29\x03\x29\x07\x29\u{188}\x0a\x29\x0c\x29\x0e\x29\u{18b}\x0b\
	\x29\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x05\x2a\u{191}\x0a\x2a\x03\x2b\x03\
	\x2b\x03\x2b\x03\x2b\x03\x2c\x03\x2c\x03\x2c\x03\x2c\x03\x2d\x03\x2d\x03\
	\x2d\x03\x2d\x03\x2e\x03\x2e\x03\x2e\x03\x2e\x03\x2f\x03\x2f\x03\x30\x03\
	\x30\x03\x30\x03\x31\x03\x31\x03\x31\x03\x32\x03\x32\x03\x32\x03\x32\x03\
	\x32\x03\x33\x07\x33\u{1b1}\x0a\x33\x0c\x33\x0e\x33\u{1b4}\x0b\x33\x03\x34\
	\x03\x34\x03\x34\x05\x34\u{1b9}\x0a\x34\x03\x35\x03\x35\x03\x35\x03\x36\
	\x03\x36\x03\x36\x03\x37\x03\x37\x03\x37\x03\x38\x03\x38\x06\x38\u{1c6}\
	\x0a\x38\x0d\x38\x0e\x38\u{1c7}\x03\x38\x03\x38\x03\x39\x03\x39\x03\x39\
	\x03\x39\x03\x3a\x03\x3a\x03\x3a\x03\x3b\x03\x3b\x03\x3b\x03\x3c\x03\x3c\
	\x03\x3c\x03\x3c\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x05\x3d\u{1de}\x0a\x3d\
	\x03\x3e\x03\x3e\x03\x3e\x07\x3e\u{1e3}\x0a\x3e\x0c\x3e\x0e\x3e\u{1e6}\x0b\
	\x3e\x03\x3f\x03\x3f\x03\x3f\x05\x3f\u{1eb}\x0a\x3f\x03\x40\x03\x40\x03\
	\x40\x03\x40\x03\x41\x03\x41\x03\x42\x03\x42\x03\x42\x07\x42\u{1f6}\x0a\
	\x42\x0c\x42\x0e\x42\u{1f9}\x0b\x42\x03\x43\x03\x43\x03\x43\x07\x43\u{1fe}\
	\x0a\x43\x0c\x43\x0e\x43\u{201}\x0b\x43\x03\x44\x03\x44\x03\x44\x07\x44\
	\u{206}\x0a\x44\x0c\x44\x0e\x44\u{209}\x0b\x44\x03\x45\x03\x45\x03\x45\x02\
	\x02\x46\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\
	\x22\x24\x26\x28\x2a\x2c\x2e\x30\x32\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\
	\x46\x48\x4a\x4c\x4e\x50\x52\x54\x56\x58\x5a\x5c\x5e\x60\x62\x64\x66\x68\
	\x6a\x6c\x6e\x70\x72\x74\x76\x78\x7a\x7c\x7e\u{80}\u{82}\u{84}\u{86}\u{88}\
	\x02\x08\x03\x02\x47\x48\x04\x02\x03\x37\x42\x42\x03\x02\x3d\x42\x03\x02\
	\x38\x3c\x04\x02\x1b\x1b\x22\x22\x03\x02\x3d\x41\x02\u{20b}\x02\u{8d}\x03\
	\x02\x02\x02\x04\u{93}\x03\x02\x02\x02\x06\u{97}\x03\x02\x02\x02\x08\u{a4}\
	\x03\x02\x02\x02\x0a\u{ac}\x03\x02\x02\x02\x0c\u{ae}\x03\x02\x02\x02\x0e\
	\u{b6}\x03\x02\x02\x02\x10\u{c9}\x03\x02\x02\x02\x12\u{cb}\x03\x02\x02\x02\
	\x14\u{ce}\x03\x02\x02\x02\x16\u{d1}\x03\x02\x02\x02\x18\u{d4}\x03\x02\x02\
	\x02\x1a\u{d7}\x03\x02\x02\x02\x1c\u{db}\x03\x02\x02\x02\x1e\u{de}\x03\x02\
	\x02\x02\x20\u{e6}\x03\x02\x02\x02\x22\u{ed}\x03\x02\x02\x02\x24\u{f6}\x03\
	\x02\x02\x02\x26\u{ff}\x03\x02\x02\x02\x28\u{101}\x03\x02\x02\x02\x2a\u{112}\
	\x03\x02\x02\x02\x2c\u{114}\x03\x02\x02\x02\x2e\u{119}\x03\x02\x02\x02\x30\
	\u{128}\x03\x02\x02\x02\x32\u{13b}\x03\x02\x02\x02\x34\u{13d}\x03\x02\x02\
	\x02\x36\u{140}\x03\x02\x02\x02\x38\u{142}\x03\x02\x02\x02\x3a\u{145}\x03\
	\x02\x02\x02\x3c\u{14d}\x03\x02\x02\x02\x3e\u{155}\x03\x02\x02\x02\x40\u{15d}\
	\x03\x02\x02\x02\x42\u{169}\x03\x02\x02\x02\x44\u{16b}\x03\x02\x02\x02\x46\
	\u{16e}\x03\x02\x02\x02\x48\u{174}\x03\x02\x02\x02\x4a\u{176}\x03\x02\x02\
	\x02\x4c\u{17e}\x03\x02\x02\x02\x4e\u{182}\x03\x02\x02\x02\x50\u{185}\x03\
	\x02\x02\x02\x52\u{190}\x03\x02\x02\x02\x54\u{192}\x03\x02\x02\x02\x56\u{196}\
	\x03\x02\x02\x02\x58\u{19a}\x03\x02\x02\x02\x5a\u{19e}\x03\x02\x02\x02\x5c\
	\u{1a2}\x03\x02\x02\x02\x5e\u{1a4}\x03\x02\x02\x02\x60\u{1a7}\x03\x02\x02\
	\x02\x62\u{1aa}\x03\x02\x02\x02\x64\u{1b2}\x03\x02\x02\x02\x66\u{1b8}\x03\
	\x02\x02\x02\x68\u{1ba}\x03\x02\x02\x02\x6a\u{1bd}\x03\x02\x02\x02\x6c\u{1c0}\
	\x03\x02\x02\x02\x6e\u{1c3}\x03\x02\x02\x02\x70\u{1cb}\x03\x02\x02\x02\x72\
	\u{1cf}\x03\x02\x02\x02\x74\u{1d2}\x03\x02\x02\x02\x76\u{1d5}\x03\x02\x02\
	\x02\x78\u{1dd}\x03\x02\x02\x02\x7a\u{1df}\x03\x02\x02\x02\x7c\u{1e7}\x03\
	\x02\x02\x02\x7e\u{1ec}\x03\x02\x02\x02\u{80}\u{1f0}\x03\x02\x02\x02\u{82}\
	\u{1f2}\x03\x02\x02\x02\u{84}\u{1fa}\x03\x02\x02\x02\u{86}\u{202}\x03\x02\
	\x02\x02\u{88}\u{20a}\x03\x02\x02\x02\u{8a}\u{8c}\x05\x04\x03\x02\u{8b}\
	\u{8a}\x03\x02\x02\x02\u{8c}\u{8f}\x03\x02\x02\x02\u{8d}\u{8b}\x03\x02\x02\
	\x02\u{8d}\u{8e}\x03\x02\x02\x02\u{8e}\u{90}\x03\x02\x02\x02\u{8f}\u{8d}\
	\x03\x02\x02\x02\u{90}\u{91}\x05\x0c\x07\x02\u{91}\u{92}\x07\x02\x02\x03\
	\u{92}\x03\x03\x02\x02\x02\u{93}\u{94}\x07\x19\x02\x02\u{94}\u{95}\x05\x06\
	\x04\x02\u{95}\x05\x03\x02\x02\x02\u{96}\u{98}\x09\x02\x02\x02\u{97}\u{96}\
	\x03\x02\x02\x02\u{97}\u{98}\x03\x02\x02\x02\u{98}\u{99}\x03\x02\x02\x02\
	\u{99}\u{9e}\x05\x08\x05\x02\u{9a}\u{9b}\x07\x45\x02\x02\u{9b}\u{9d}\x05\
	\x08\x05\x02\u{9c}\u{9a}\x03\x02\x02\x02\u{9d}\u{a0}\x03\x02\x02\x02\u{9e}\
	\u{9c}\x03\x02\x02\x02\u{9e}\u{9f}\x03\x02\x02\x02\u{9f}\u{a1}\x03\x02\x02\
	\x02\u{a0}\u{9e}\x03\x02\x02\x02\u{a1}\u{a2}\x07\x43\x02\x02\u{a2}\u{a3}\
	\x05\x0a\x06\x02\u{a3}\x07\x03\x02\x02\x02\u{a4}\u{a9}\x05\x0a\x06\x02\u{a5}\
	\u{a6}\x07\x46\x02\x02\u{a6}\u{a8}\x05\x0a\x06\x02\u{a7}\u{a5}\x03\x02\x02\
	\x02\u{a8}\u{ab}\x03\x02\x02\x02\u{a9}\u{a7}\x03\x02\x02\x02\u{a9}\u{aa}\
	\x03\x02\x02\x02\u{aa}\x09\x03\x02\x02\x02\u{ab}\u{a9}\x03\x02\x02\x02\u{ac}\
	\u{ad}\x09\x03\x02\x02\u{ad}\x0b\x03\x02\x02\x02\u{ae}\u{af}\x07\x03\x02\
	\x02\u{af}\u{b0}\x07\x42\x02\x02\u{b0}\u{b1}\x05\x0e\x08\x02\u{b1}\u{b2}\
	\x07\x11\x02\x02\u{b2}\x0d\x03\x02\x02\x02\u{b3}\u{b5}\x05\x10\x09\x02\u{b4}\
	\u{b3}\x03\x02\x02\x02\u{b5}\u{b8}\x03\x02\x02\x02\u{b6}\u{b4}\x03\x02\x02\
	\x02\u{b6}\u{b7}\x03\x02\x02\x02\u{b7}\x0f\x03\x02\x02\x02\u{b8}\u{b6}\x03\
	\x02\x02\x02\u{b9}\u{ca}\x05\x12\x0a\x02\u{ba}\u{ca}\x05\x14\x0b\x02\u{bb}\
	\u{ca}\x05\x16\x0c\x02\u{bc}\u{ca}\x05\x18\x0d\x02\u{bd}\u{ca}\x05\x1a\x0e\
	\x02\u{be}\u{ca}\x05\x1c\x0f\x02\u{bf}\u{ca}\x05\x1e\x10\x02\u{c0}\u{ca}\
	\x05\x20\x11\x02\u{c1}\u{ca}\x05\x22\x12\x02\u{c2}\u{ca}\x05\x28\x15\x02\
	\u{c3}\u{ca}\x05\x2c\x17\x02\u{c4}\u{ca}\x05\x2e\x18\x02\u{c5}\u{ca}\x05\
	\x62\x32\x02\u{c6}\u{ca}\x05\x6e\x38\x02\u{c7}\u{ca}\x05\x72\x3a\x02\u{c8}\
	\u{ca}\x05\x74\x3b\x02\u{c9}\u{b9}\x03\x02\x02\x02\u{c9}\u{ba}\x03\x02\x02\
	\x02\u{c9}\u{bb}\x03\x02\x02\x02\u{c9}\u{bc}\x03\x02\x02\x02\u{c9}\u{bd}\
	\x03\x02\x02\x02\u{c9}\u{be}\x03\x02\x02\x02\u{c9}\u{bf}\x03\x02\x02\x02\
	\u{c9}\u{c0}\x03\x02\x02\x02\u{c9}\u{c1}\x03\x02\x02\x02\u{c9}\u{c2}\x03\
	\x02\x02\x02\u{c9}\u{c3}\x03\x02\x02\x02\u{c9}\u{c4}\x03\x02\x02\x02\u{c9}\
	\u{c5}\x03\x02\x02\x02\u{c9}\u{c6}\x03\x02\x02\x02\u{c9}\u{c7}\x03\x02\x02\
	\x02\u{c9}\u{c8}\x03\x02\x02\x02\u{ca}\x11\x03\x02\x02\x02\u{cb}\u{cc}\x07\
	\x37\x02\x02\u{cc}\u{cd}\x07\x41\x02\x02\u{cd}\x13\x03\x02\x02\x02\u{ce}\
	\u{cf}\x07\x06\x02\x02\u{cf}\u{d0}\x07\x41\x02\x02\u{d0}\x15\x03\x02\x02\
	\x02\u{d1}\u{d2}\x07\x10\x02\x02\u{d2}\u{d3}\x07\x41\x02\x02\u{d3}\x17\x03\
	\x02\x02\x02\u{d4}\u{d5}\x07\x32\x02\x02\u{d5}\u{d6}\x05\u{82}\x42\x02\u{d6}\
	\x19\x03\x02\x02\x02\u{d7}\u{d8}\x07\x05\x02\x02\u{d8}\u{d9}\x07\x0c\x02\
	\x02\u{d9}\u{da}\x05\x50\x29\x02\u{da}\x1b\x03\x02\x02\x02\u{db}\u{dc}\x07\
	\x0a\x02\x02\u{dc}\u{dd}\x07\x42\x02\x02\u{dd}\x1d\x03\x02\x02\x02\u{de}\
	\u{df}\x07\x2a\x02\x02\u{df}\u{e0}\x05\x78\x3d\x02\u{e0}\u{e1}\x07\x28\x02\
	\x02\u{e1}\u{e4}\x07\x42\x02\x02\u{e2}\u{e3}\x07\x07\x02\x02\u{e3}\u{e5}\
	\x05\x78\x3d\x02\u{e4}\u{e2}\x03\x02\x02\x02\u{e4}\u{e5}\x03\x02\x02\x02\
	\u{e5}\x1f\x03\x02\x02\x02\u{e6}\u{e7}\x07\x23\x02\x02\u{e7}\u{e8}\x07\x42\
	\x02\x02\u{e8}\u{e9}\x07\x0d\x02\x02\u{e9}\u{ea}\x07\x3f\x02\x02\u{ea}\u{eb}\
	\x07\x1d\x02\x02\u{eb}\u{ec}\x07\x3f\x02\x02\u{ec}\x21\x03\x02\x02\x02\u{ed}\
	\u{f1}\x07\x09\x02\x02\u{ee}\u{f0}\x05\x24\x13\x02\u{ef}\u{ee}\x03\x02\x02\
	\x02\u{f0}\u{f3}\x03\x02\x02\x02\u{f1}\u{ef}\x03\x02\x02\x02\u{f1}\u{f2}\
	\x03\x02\x02\x02\u{f2}\u{f4}\x03\x02\x02\x02\u{f3}\u{f1}\x03\x02\x02\x02\
	\u{f4}\u{f5}\x07\x11\x02\x02\u{f5}\x23\x03\x02\x02\x02\u{f6}\u{f7}\x07\x42\
	\x02\x02\u{f7}\u{fc}\x05\x26\x14\x02\u{f8}\u{f9}\x07\x44\x02\x02\u{f9}\u{fb}\
	\x05\x26\x14\x02\u{fa}\u{f8}\x03\x02\x02\x02\u{fb}\u{fe}\x03\x02\x02\x02\
	\u{fc}\u{fa}\x03\x02\x02\x02\u{fc}\u{fd}\x03\x02\x02\x02\u{fd}\x25\x03\x02\
	\x02\x02\u{fe}\u{fc}\x03\x02\x02\x02\u{ff}\u{100}\x09\x04\x02\x02\u{100}\
	\x27\x03\x02\x02\x02\u{101}\u{105}\x07\x0b\x02\x02\u{102}\u{104}\x05\x2a\
	\x16\x02\u{103}\u{102}\x03\x02\x02\x02\u{104}\u{107}\x03\x02\x02\x02\u{105}\
	\u{103}\x03\x02\x02\x02\u{105}\u{106}\x03\x02\x02\x02\u{106}\u{108}\x03\
	\x02\x02\x02\u{107}\u{105}\x03\x02\x02\x02\u{108}\u{109}\x07\x11\x02\x02\
	\u{109}\x29\x03\x02\x02\x02\u{10a}\u{10b}\x07\x21\x02\x02\u{10b}\u{113}\
	\x05\x7a\x3e\x02\u{10c}\u{10d}\x07\x1f\x02\x02\u{10d}\u{113}\x05\u{86}\x44\
	\x02\u{10e}\u{10f}\x07\x16\x02\x02\u{10f}\u{113}\x05\u{84}\x43\x02\u{110}\
	\u{111}\x07\x1c\x02\x02\u{111}\u{113}\x05\x78\x3d\x02\u{112}\u{10a}\x03\
	\x02\x02\x02\u{112}\u{10c}\x03\x02\x02\x02\u{112}\u{10e}\x03\x02\x02\x02\
	\u{112}\u{110}\x03\x02\x02\x02\u{113}\x2b\x03\x02\x02\x02\u{114}\u{115}\
	\x07\x12\x02\x02\u{115}\u{116}\x07\x42\x02\x02\u{116}\u{117}\x05\x30\x19\
	\x02\u{117}\u{118}\x07\x11\x02\x02\u{118}\x2d\x03\x02\x02\x02\u{119}\u{11a}\
	\x07\x0f\x02\x02\u{11a}\u{11b}\x07\x12\x02\x02\u{11b}\u{11c}\x07\x42\x02\
	\x02\u{11c}\u{11e}\x05\x30\x19\x02\u{11d}\u{11f}\x05\x5e\x30\x02\u{11e}\
	\u{11d}\x03\x02\x02\x02\u{11e}\u{11f}\x03\x02\x02\x02\u{11f}\u{121}\x03\
	\x02\x02\x02\u{120}\u{122}\x05\x60\x31\x02\u{121}\u{120}\x03\x02\x02\x02\
	\u{121}\u{122}\x03\x02\x02\x02\u{122}\u{123}\x03\x02\x02\x02\u{123}\u{124}\
	\x07\x11\x02\x02\u{124}\x2f\x03\x02\x02\x02\u{125}\u{127}\x05\x32\x1a\x02\
	\u{126}\u{125}\x03\x02\x02\x02\u{127}\u{12a}\x03\x02\x02\x02\u{128}\u{126}\
	\x03\x02\x02\x02\u{128}\u{129}\x03\x02\x02\x02\u{129}\x31\x03\x02\x02\x02\
	\u{12a}\u{128}\x03\x02\x02\x02\u{12b}\u{13c}\x05\x34\x1b\x02\u{12c}\u{13c}\
	\x05\x38\x1d\x02\u{12d}\u{13c}\x05\x16\x0c\x02\u{12e}\u{13c}\x05\x3a\x1e\
	\x02\u{12f}\u{13c}\x05\x3c\x1f\x02\u{130}\u{13c}\x05\x3e\x20\x02\u{131}\
	\u{13c}\x05\x44\x23\x02\u{132}\u{13c}\x05\x46\x24\x02\u{133}\u{13c}\x05\
	\x4a\x26\x02\u{134}\u{13c}\x05\x4e\x28\x02\u{135}\u{13c}\x05\x54\x2b\x02\
	\u{136}\u{13c}\x05\x1e\x10\x02\u{137}\u{13c}\x05\x56\x2c\x02\u{138}\u{13c}\
	\x05\x58\x2d\x02\u{139}\u{13c}\x05\x5a\x2e\x02\u{13a}\u{13c}\x05\x5c\x2f\
	\x02\u{13b}\u{12b}\x03\x02\x02\x02\u{13b}\u{12c}\x03\x02\x02\x02\u{13b}\
	\u{12d}\x03\x02\x02\x02\u{13b}\u{12e}\x03\x02\x02\x02\u{13b}\u{12f}\x03\
	\x02\x02\x02\u{13b}\u{130}\x03\x02\x02\x02\u{13b}\u{131}\x03\x02\x02\x02\
	\u{13b}\u{132}\x03\x02\x02\x02\u{13b}\u{133}\x03\x02\x02\x02\u{13b}\u{134}\
	\x03\x02\x02\x02\u{13b}\u{135}\x03\x02\x02\x02\u{13b}\u{136}\x03\x02\x02\
	\x02\u{13b}\u{137}\x03\x02\x02\x02\u{13b}\u{138}\x03\x02\x02\x02\u{13b}\
	\u{139}\x03\x02\x02\x02\u{13b}\u{13a}\x03\x02\x02\x02\u{13c}\x33\x03\x02\
	\x02\x02\u{13d}\u{13e}\x07\x1e\x02\x02\u{13e}\u{13f}\x05\x36\x1c\x02\u{13f}\
	\x35\x03\x02\x02\x02\u{140}\u{141}\x09\x05\x02\x02\u{141}\x37\x03\x02\x02\
	\x02\u{142}\u{143}\x07\x26\x02\x02\u{143}\u{144}\x07\x41\x02\x02\u{144}\
	\x39\x03\x02\x02\x02\u{145}\u{147}\x07\x24\x02\x02\u{146}\u{148}\x05\x40\
	\x21\x02\u{147}\u{146}\x03\x02\x02\x02\u{148}\u{149}\x03\x02\x02\x02\u{149}\
	\u{147}\x03\x02\x02\x02\u{149}\u{14a}\x03\x02\x02\x02\u{14a}\u{14b}\x03\
	\x02\x02\x02\u{14b}\u{14c}\x07\x11\x02\x02\u{14c}\x3b\x03\x02\x02\x02\u{14d}\
	\u{14f}\x07\x29\x02\x02\u{14e}\u{150}\x05\x40\x21\x02\u{14f}\u{14e}\x03\
	\x02\x02\x02\u{150}\u{151}\x03\x02\x02\x02\u{151}\u{14f}\x03\x02\x02\x02\
	\u{151}\u{152}\x03\x02\x02\x02\u{152}\u{153}\x03\x02\x02\x02\u{153}\u{154}\
	\x07\x11\x02\x02\u{154}\x3d\x03\x02\x02\x02\u{155}\u{157}\x07\x16\x02\x02\
	\u{156}\u{158}\x05\x40\x21\x02\u{157}\u{156}\x03\x02\x02\x02\u{158}\u{159}\
	\x03\x02\x02\x02\u{159}\u{157}\x03\x02\x02\x02\u{159}\u{15a}\x03\x02\x02\
	\x02\u{15a}\u{15b}\x03\x02\x02\x02\u{15b}\u{15c}\x07\x11\x02\x02\u{15c}\
	\x3f\x03\x02\x02\x02\u{15d}\u{15e}\x07\x42\x02\x02\u{15e}\u{162}\x05\u{80}\
	\x41\x02\u{15f}\u{161}\x05\x42\x22\x02\u{160}\u{15f}\x03\x02\x02\x02\u{161}\
	\u{164}\x03\x02\x02\x02\u{162}\u{160}\x03\x02\x02\x02\u{162}\u{163}\x03\
	\x02\x02\x02\u{163}\x41\x03\x02\x02\x02\u{164}\u{162}\x03\x02\x02\x02\u{165}\
	\u{16a}\x07\x2e\x02\x02\u{166}\u{16a}\x07\x20\x02\x02\u{167}\u{168}\x07\
	\x0c\x02\x02\u{168}\u{16a}\x05\u{88}\x45\x02\u{169}\u{165}\x03\x02\x02\x02\
	\u{169}\u{166}\x03\x02\x02\x02\u{169}\u{167}\x03\x02\x02\x02\u{16a}\x43\
	\x03\x02\x02\x02\u{16b}\u{16c}\x07\x2d\x02\x02\u{16c}\u{16d}\x05\x7c\x3f\
	\x02\u{16d}\x45\x03\x02\x02\x02\u{16e}\u{170}\x07\x2f\x02\x02\u{16f}\u{171}\
	\x05\x48\x25\x02\u{170}\u{16f}\x03\x02\x02\x02\u{170}\u{171}\x03\x02\x02\
	\x02\u{171}\u{172}\x03\x02\x02\x02\u{172}\u{173}\x05\x7c\x3f\x02\u{173}\
	\x47\x03\x02\x02\x02\u{174}\u{175}\x09\x06\x02\x02\u{175}\x49\x03\x02\x02\
	\x02\u{176}\u{178}\x07\x13\x02\x02\u{177}\u{179}\x05\x4c\x27\x02\u{178}\
	\u{177}\x03\x02\x02\x02\u{179}\u{17a}\x03\x02\x02\x02\u{17a}\u{178}\x03\
	\x02\x02\x02\u{17a}\u{17b}\x03\x02\x02\x02\u{17b}\u{17c}\x03\x02\x02\x02\
	\u{17c}\u{17d}\x07\x11\x02\x02\u{17d}\x4b\x03\x02\x02\x02\u{17e}\u{17f}\
	\x07\x3f\x02\x02\u{17f}\u{180}\x07\x35\x02\x02\u{180}\u{181}\x05\x7c\x3f\
	\x02\u{181}\x4d\x03\x02\x02\x02\u{182}\u{183}\x07\x05\x02\x02\u{183}\u{184}\
	\x05\x50\x29\x02\u{184}\x4f\x03\x02\x02\x02\u{185}\u{189}\x07\x42\x02\x02\
	\u{186}\u{188}\x05\x52\x2a\x02\u{187}\u{186}\x03\x02\x02\x02\u{188}\u{18b}\
	\x03\x02\x02\x02\u{189}\u{187}\x03\x02\x02\x02\u{189}\u{18a}\x03\x02\x02\
	\x02\u{18a}\x51\x03\x02\x02\x02\u{18b}\u{189}\x03\x02\x02\x02\u{18c}\u{18d}\
	\x07\x30\x02\x02\u{18d}\u{191}\x07\x41\x02\x02\u{18e}\u{18f}\x07\x15\x02\
	\x02\u{18f}\u{191}\x07\x41\x02\x02\u{190}\u{18c}\x03\x02\x02\x02\u{190}\
	\u{18e}\x03\x02\x02\x02\u{191}\x53\x03\x02\x02\x02\u{192}\u{193}\x07\x36\
	\x02\x02\u{193}\u{194}\x07\x35\x02\x02\u{194}\u{195}\x05\x7e\x40\x02\u{195}\
	\x55\x03\x02\x02\x02\u{196}\u{197}\x07\x33\x02\x02\u{197}\u{198}\x05\x78\
	\x3d\x02\u{198}\u{199}\x07\x42\x02\x02\u{199}\x57\x03\x02\x02\x02\u{19a}\
	\u{19b}\x07\x08\x02\x02\u{19b}\u{19c}\x05\x78\x3d\x02\u{19c}\u{19d}\x07\
	\x42\x02\x02\u{19d}\x59\x03\x02\x02\x02\u{19e}\u{19f}\x07\x18\x02\x02\u{19f}\
	\u{1a0}\x07\x1a\x02\x02\u{1a0}\u{1a1}\x07\x41\x02\x02\u{1a1}\x5b\x03\x02\
	\x02\x02\u{1a2}\u{1a3}\x07\x04\x02\x02\u{1a3}\x5d\x03\x02\x02\x02\u{1a4}\
	\u{1a5}\x07\x31\x02\x02\u{1a5}\u{1a6}\x07\x41\x02\x02\u{1a6}\x5f\x03\x02\
	\x02\x02\u{1a7}\u{1a8}\x07\x2c\x02\x02\u{1a8}\u{1a9}\x07\x42\x02\x02\u{1a9}\
	\x61\x03\x02\x02\x02\u{1aa}\u{1ab}\x07\x14\x02\x02\u{1ab}\u{1ac}\x07\x42\
	\x02\x02\u{1ac}\u{1ad}\x05\x64\x33\x02\u{1ad}\u{1ae}\x07\x11\x02\x02\u{1ae}\
	\x63\x03\x02\x02\x02\u{1af}\u{1b1}\x05\x66\x34\x02\u{1b0}\u{1af}\x03\x02\
	\x02\x02\u{1b1}\u{1b4}\x03\x02\x02\x02\u{1b2}\u{1b0}\x03\x02\x02\x02\u{1b2}\
	\u{1b3}\x03\x02\x02\x02\u{1b3}\x65\x03\x02\x02\x02\u{1b4}\u{1b2}\x03\x02\
	\x02\x02\u{1b5}\u{1b9}\x05\x68\x35\x02\u{1b6}\u{1b9}\x05\x6a\x36\x02\u{1b7}\
	\u{1b9}\x05\x6c\x37\x02\u{1b8}\u{1b5}\x03\x02\x02\x02\u{1b8}\u{1b6}\x03\
	\x02\x02\x02\u{1b8}\u{1b7}\x03\x02\x02\x02\u{1b9}\x67\x03\x02\x02\x02\u{1ba}\
	\u{1bb}\x07\x34\x02\x02\u{1bb}\u{1bc}\x07\x41\x02\x02\u{1bc}\x69\x03\x02\
	\x02\x02\u{1bd}\u{1be}\x07\x27\x02\x02\u{1be}\u{1bf}\x05\x7c\x3f\x02\u{1bf}\
	\x6b\x03\x02\x02\x02\u{1c0}\u{1c1}\x07\x25\x02\x02\u{1c1}\u{1c2}\x07\x42\
	\x02\x02\u{1c2}\x6d\x03\x02\x02\x02\u{1c3}\u{1c5}\x07\x0e\x02\x02\u{1c4}\
	\u{1c6}\x05\x70\x39\x02\u{1c5}\u{1c4}\x03\x02\x02\x02\u{1c6}\u{1c7}\x03\
	\x02\x02\x02\u{1c7}\u{1c5}\x03\x02\x02\x02\u{1c7}\u{1c8}\x03\x02\x02\x02\
	\u{1c8}\u{1c9}\x03\x02\x02\x02\u{1c9}\u{1ca}\x07\x11\x02\x02\u{1ca}\x6f\
	\x03\x02\x02\x02\u{1cb}\u{1cc}\x07\x42\x02\x02\u{1cc}\u{1cd}\x07\x37\x02\
	\x02\u{1cd}\u{1ce}\x07\x41\x02\x02\u{1ce}\x71\x03\x02\x02\x02\u{1cf}\u{1d0}\
	\x07\x17\x02\x02\u{1d0}\u{1d1}\x07\x41\x02\x02\u{1d1}\x73\x03\x02\x02\x02\
	\u{1d2}\u{1d3}\x07\x2b\x02\x02\u{1d3}\u{1d4}\x07\x41\x02\x02\u{1d4}\x75\
	\x03\x02\x02\x02\u{1d5}\u{1d6}\x07\x42\x02\x02\u{1d6}\u{1d7}\x07\x43\x02\
	\x02\u{1d7}\u{1d8}\x07\x42\x02\x02\u{1d8}\x77\x03\x02\x02\x02\u{1d9}\u{1de}\
	\x07\x3f\x02\x02\u{1da}\u{1de}\x07\x40\x02\x02\u{1db}\u{1de}\x07\x41\x02\
	\x02\u{1dc}\u{1de}\x05\x76\x3c\x02\u{1dd}\u{1d9}\x03\x02\x02\x02\u{1dd}\
	\u{1da}\x03\x02\x02\x02\u{1dd}\u{1db}\x03\x02\x02\x02\u{1dd}\u{1dc}\x03\
	\x02\x02\x02\u{1de}\x79\x03\x02\x02\x02\u{1df}\u{1e4}\x05\x78\x3d\x02\u{1e0}\
	\u{1e1}\x07\x44\x02\x02\u{1e1}\u{1e3}\x05\x78\x3d\x02\u{1e2}\u{1e0}\x03\
	\x02\x02\x02\u{1e3}\u{1e6}\x03\x02\x02\x02\u{1e4}\u{1e2}\x03\x02\x02\x02\
	\u{1e4}\u{1e5}\x03\x02\x02\x02\u{1e5}\x7b\x03\x02\x02\x02\u{1e6}\u{1e4}\
	\x03\x02\x02\x02\u{1e7}\u{1ea}\x07\x42\x02\x02\u{1e8}\u{1e9}\x07\x43\x02\
	\x02\u{1e9}\u{1eb}\x07\x42\x02\x02\u{1ea}\u{1e8}\x03\x02\x02\x02\u{1ea}\
	\u{1eb}\x03\x02\x02\x02\u{1eb}\x7d\x03\x02\x02\x02\u{1ec}\u{1ed}\x07\x42\
	\x02\x02\u{1ed}\u{1ee}\x07\x43\x02\x02\u{1ee}\u{1ef}\x07\x42\x02\x02\u{1ef}\
	\x7f\x03\x02\x02\x02\u{1f0}\u{1f1}\x07\x42\x02\x02\u{1f1}\u{81}\x03\x02\
	\x02\x02\u{1f2}\u{1f7}\x07\x42\x02\x02\u{1f3}\u{1f4}\x07\x44\x02\x02\u{1f4}\
	\u{1f6}\x07\x42\x02\x02\u{1f5}\u{1f3}\x03\x02\x02\x02\u{1f6}\u{1f9}\x03\
	\x02\x02\x02\u{1f7}\u{1f5}\x03\x02\x02\x02\u{1f7}\u{1f8}\x03\x02\x02\x02\
	\u{1f8}\u{83}\x03\x02\x02\x02\u{1f9}\u{1f7}\x03\x02\x02\x02\u{1fa}\u{1ff}\
	\x07\x41\x02\x02\u{1fb}\u{1fc}\x07\x44\x02\x02\u{1fc}\u{1fe}\x07\x41\x02\
	\x02\u{1fd}\u{1fb}\x03\x02\x02\x02\u{1fe}\u{201}\x03\x02\x02\x02\u{1ff}\
	\u{1fd}\x03\x02\x02\x02\u{1ff}\u{200}\x03\x02\x02\x02\u{200}\u{85}\x03\x02\
	\x02\x02\u{201}\u{1ff}\x03\x02\x02\x02\u{202}\u{207}\x05\x36\x1c\x02\u{203}\
	\u{204}\x07\x44\x02\x02\u{204}\u{206}\x05\x36\x1c\x02\u{205}\u{203}\x03\
	\x02\x02\x02\u{206}\u{209}\x03\x02\x02\x02\u{207}\u{205}\x03\x02\x02\x02\
	\u{207}\u{208}\x03\x02\x02\x02\u{208}\u{87}\x03\x02\x02\x02\u{209}\u{207}\
	\x03\x02\x02\x02\u{20a}\u{20b}\x09\x07\x02\x02\u{20b}\u{89}\x03\x02\x02\
	\x02\x23\u{8d}\u{97}\u{9e}\u{a9}\u{b6}\u{c9}\u{e4}\u{f1}\u{fc}\u{105}\u{112}\
	\u{11e}\u{121}\u{128}\u{13b}\u{149}\u{151}\u{159}\u{162}\u{169}\u{170}\u{17a}\
	\u{189}\u{190}\u{1b2}\u{1b8}\u{1c7}\u{1dd}\u{1e4}\u{1ea}\u{1f7}\u{1ff}\u{207}";

