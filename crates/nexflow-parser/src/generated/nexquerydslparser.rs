// Generated from grammar/nexflow/NexQueryDSL.g4 by ANTLR 4.8
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
use super::nexquerydsllistener::*;
use super::nexquerydslvisitor::*;

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

		pub const PROGRAMS:isize=1; 
		pub const PARAGRAPHS:isize=2; 
		pub const FIELDS:isize=3; 
		pub const COPYBOOKS:isize=4; 
		pub const FILES:isize=5; 
		pub const TABLES:isize=6; 
		pub const RULES:isize=7; 
		pub const CALLING:isize=8; 
		pub const CALLED_BY:isize=9; 
		pub const PERFORMING:isize=10; 
		pub const PERFORMED_BY:isize=11; 
		pub const READING:isize=12; 
		pub const READ_BY:isize=13; 
		pub const WRITING:isize=14; 
		pub const WRITTEN_BY:isize=15; 
		pub const USING:isize=16; 
		pub const USED_BY:isize=17; 
		pub const ACCESSING:isize=18; 
		pub const ACCESSED_BY:isize=19; 
		pub const CONTAINING:isize=20; 
		pub const WITHIN:isize=21; 
		pub const TRACE:isize=22; 
		pub const RANK:isize=23; 
		pub const SIMILAR:isize=24; 
		pub const FIND_DEAD:isize=25; 
		pub const DEPS:isize=26; 
		pub const IMPACT:isize=27; 
		pub const COMPARE:isize=28; 
		pub const DISCOVER_PROCESSES:isize=29; 
		pub const ESTIMATE_COST:isize=30; 
		pub const SAVE:isize=31; 
		pub const RUN:isize=32; 
		pub const BY:isize=33; 
		pub const TOP:isize=34; 
		pub const IN:isize=35; 
		pub const WITH:isize=36; 
		pub const DEPTH:isize=37; 
		pub const LEVEL:isize=38; 
		pub const ORDER:isize=39; 
		pub const SCOPE:isize=40; 
		pub const THRESHOLD:isize=41; 
		pub const AS:isize=42; 
		pub const EACH:isize=43; 
		pub const AND:isize=44; 
		pub const OR:isize=45; 
		pub const NOT:isize=46; 
		pub const HAS:isize=47; 
		pub const EQ:isize=48; 
		pub const NE:isize=49; 
		pub const GT:isize=50; 
		pub const LT:isize=51; 
		pub const GE:isize=52; 
		pub const LE:isize=53; 
		pub const GLOB:isize=54; 
		pub const REGEX:isize=55; 
		pub const SEMICOLON:isize=56; 
		pub const COMMA:isize=57; 
		pub const DOT:isize=58; 
		pub const LPAREN:isize=59; 
		pub const RPAREN:isize=60; 
		pub const LBRACKET:isize=61; 
		pub const RBRACKET:isize=62; 
		pub const NUMBER:isize=63; 
		pub const STRING:isize=64; 
		pub const IDENTIFIER:isize=65; 
		pub const CONTINUATION:isize=66; 
		pub const LINE_COMMENT:isize=67; 
		pub const WS:isize=68;
	pub const RULE_program:usize = 0; 
	pub const RULE_statement:usize = 1; 
	pub const RULE_clause:usize = 2; 
	pub const RULE_traverseClause:usize = 3; 
	pub const RULE_expandClause:usize = 4; 
	pub const RULE_verbClause:usize = 5; 
	pub const RULE_nodeType:usize = 6; 
	pub const RULE_traversalVerb:usize = 7; 
	pub const RULE_domainVerb:usize = 8; 
	pub const RULE_target:usize = 9; 
	pub const RULE_list:usize = 10; 
	pub const RULE_listItem:usize = 11; 
	pub const RULE_filter:usize = 12; 
	pub const RULE_filterExpr:usize = 13; 
	pub const RULE_predicate:usize = 14; 
	pub const RULE_fieldRef:usize = 15; 
	pub const RULE_compareOp:usize = 16; 
	pub const RULE_value:usize = 17; 
	pub const RULE_modifier:usize = 18; 
	pub const RULE_modifierKeyword:usize = 19; 
	pub const RULE_modifierValue:usize = 20;
	pub const ruleNames: [&'static str; 21] =  [
		"program", "statement", "clause", "traverseClause", "expandClause", "verbClause", 
		"nodeType", "traversalVerb", "domainVerb", "target", "list", "listItem", 
		"filter", "filterExpr", "predicate", "fieldRef", "compareOp", "value", 
		"modifier", "modifierKeyword", "modifierValue"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;67] = [
		None, Some("'programs'"), Some("'paragraphs'"), Some("'fields'"), Some("'copybooks'"), 
		Some("'files'"), Some("'tables'"), Some("'rules'"), Some("'calling'"), 
		Some("'called-by'"), Some("'performing'"), Some("'performed-by'"), Some("'reading'"), 
		Some("'read-by'"), Some("'writing'"), Some("'written-by'"), Some("'using'"), 
		Some("'used-by'"), Some("'accessing'"), Some("'accessed-by'"), Some("'containing'"), 
		Some("'within'"), Some("'trace'"), Some("'rank'"), Some("'similar'"), 
		Some("'find-dead'"), Some("'deps'"), Some("'impact'"), Some("'compare'"), 
		Some("'discover-processes'"), Some("'estimate-cost'"), Some("'save'"), 
		Some("'run'"), Some("'by'"), Some("'top'"), Some("'in'"), Some("'with'"), 
		Some("'depth'"), Some("'level'"), Some("'order'"), Some("'scope'"), Some("'threshold'"), 
		Some("'as'"), Some("'each'"), Some("'and'"), Some("'or'"), Some("'not'"), 
		Some("'has'"), Some("'='"), Some("'!='"), Some("'>'"), Some("'<'"), Some("'>='"), 
		Some("'<='"), Some("'~'"), Some("'~~'"), Some("';'"), Some("','"), Some("'.'"), 
		Some("'('"), Some("')'"), Some("'['"), Some("']'"), None, None, None, 
		Some("'..'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;69]  = [
		None, Some("PROGRAMS"), Some("PARAGRAPHS"), Some("FIELDS"), Some("COPYBOOKS"), 
		Some("FILES"), Some("TABLES"), Some("RULES"), Some("CALLING"), Some("CALLED_BY"), 
		Some("PERFORMING"), Some("PERFORMED_BY"), Some("READING"), Some("READ_BY"), 
		Some("WRITING"), Some("WRITTEN_BY"), Some("USING"), Some("USED_BY"), Some("ACCESSING"), 
		Some("ACCESSED_BY"), Some("CONTAINING"), Some("WITHIN"), Some("TRACE"), 
		Some("RANK"), Some("SIMILAR"), Some("FIND_DEAD"), Some("DEPS"), Some("IMPACT"), 
		Some("COMPARE"), Some("DISCOVER_PROCESSES"), Some("ESTIMATE_COST"), Some("SAVE"), 
		Some("RUN"), Some("BY"), Some("TOP"), Some("IN"), Some("WITH"), Some("DEPTH"), 
		Some("LEVEL"), Some("ORDER"), Some("SCOPE"), Some("THRESHOLD"), Some("AS"), 
		Some("EACH"), Some("AND"), Some("OR"), Some("NOT"), Some("HAS"), Some("EQ"), 
		Some("NE"), Some("GT"), Some("LT"), Some("GE"), Some("LE"), Some("GLOB"), 
		Some("REGEX"), Some("SEMICOLON"), Some("COMMA"), Some("DOT"), Some("LPAREN"), 
		Some("RPAREN"), Some("LBRACKET"), Some("RBRACKET"), Some("NUMBER"), Some("STRING"), 
		Some("IDENTIFIER"), Some("CONTINUATION"), Some("LINE_COMMENT"), Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,NexQueryDSLParserExt<'input>, I, NexQueryDSLParserContextType , dyn NexQueryDSLListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type NexQueryDSLTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, NexQueryDSLParserContextType , dyn NexQueryDSLListener<'input> + 'a>;

/// Parser for NexQueryDSL grammar
pub struct NexQueryDSLParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> NexQueryDSLParser<'input, I, H>
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
				NexQueryDSLParserExt{
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

impl<'input, I> NexQueryDSLParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> NexQueryDSLParser<'input, I, DefaultErrorStrategy<'input,NexQueryDSLParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for NexQueryDSLParser
pub trait NexQueryDSLParserContext<'input>:
	for<'x> Listenable<dyn NexQueryDSLListener<'input> + 'x > + 
	for<'x> Visitable<dyn NexQueryDSLVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=NexQueryDSLParserContextType>
{}

antlr_rust::coerce_from!{ 'input : NexQueryDSLParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn NexQueryDSLParserContext<'input> + 'input
where
    T: NexQueryDSLVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn NexQueryDSLVisitor<'input> + 'x))
    }
}

impl<'input> NexQueryDSLParserContext<'input> for TerminalNode<'input,NexQueryDSLParserContextType> {}
impl<'input> NexQueryDSLParserContext<'input> for ErrorNode<'input,NexQueryDSLParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn NexQueryDSLParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn NexQueryDSLListener<'input> + 'input }

pub struct NexQueryDSLParserContextType;
antlr_rust::tid!{NexQueryDSLParserContextType}

impl<'input> ParserNodeType<'input> for NexQueryDSLParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn NexQueryDSLParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct NexQueryDSLParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> NexQueryDSLParserExt<'input>{
}
antlr_rust::tid! { NexQueryDSLParserExt<'a> }

impl<'input> TokenAware<'input> for NexQueryDSLParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for NexQueryDSLParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for NexQueryDSLParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "NexQueryDSL.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn NexQueryDSLParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					13 => NexQueryDSLParser::<'input,I,_>::filterExpr_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> NexQueryDSLParser<'input, I, DefaultErrorStrategy<'input,NexQueryDSLParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn filterExpr_sempred(_localctx: Option<&FilterExprContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 5)
				}
				1=>{
					recog.precpred(None, 4)
				}
			_ => true
		}
	}
}
//------------------- program ----------------
pub type ProgramContextAll<'input> = ProgramContext<'input>;


pub type ProgramContext<'input> = BaseParserRuleContext<'input,ProgramContextExt<'input>>;

#[derive(Clone)]
pub struct ProgramContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> NexQueryDSLParserContext<'input> for ProgramContext<'input>{}

impl<'input,'a> Listenable<dyn NexQueryDSLListener<'input> + 'a> for ProgramContext<'input>{
		fn enter(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_program(self);
		}
		fn exit(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.exit_program(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn NexQueryDSLVisitor<'input> + 'a> for ProgramContext<'input>{
	fn accept(&self,visitor: &mut (dyn NexQueryDSLVisitor<'input> + 'a)) {
		visitor.visit_program(self);
	}
}

impl<'input> CustomRuleContext<'input> for ProgramContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = NexQueryDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_program }
	//fn type_rule_index() -> usize where Self: Sized { RULE_program }
}
antlr_rust::tid!{ProgramContextExt<'a>}

impl<'input> ProgramContextExt<'input>{
	fn new(parent: Option<Rc<dyn NexQueryDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProgramContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProgramContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ProgramContextAttrs<'input>: NexQueryDSLParserContext<'input> + BorrowMut<ProgramContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ProgramContextAttrs<'input> for ProgramContext<'input>{}

impl<'input, I, H> NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn program(&mut self,)
	-> Result<Rc<ProgramContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProgramContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_program);
        let mut _localctx: Rc<ProgramContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(43); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule statement*/
				recog.base.set_state(42);
				recog.statement()?;

				}
				}
				recog.base.set_state(45); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(((((_la - 1)) & !0x3f) == 0 && ((1usize << (_la - 1)) & ((1usize << (PROGRAMS - 1)) | (1usize << (PARAGRAPHS - 1)) | (1usize << (FIELDS - 1)) | (1usize << (COPYBOOKS - 1)) | (1usize << (FILES - 1)) | (1usize << (TABLES - 1)) | (1usize << (RULES - 1)) | (1usize << (TRACE - 1)) | (1usize << (RANK - 1)) | (1usize << (SIMILAR - 1)) | (1usize << (FIND_DEAD - 1)) | (1usize << (DEPS - 1)) | (1usize << (IMPACT - 1)) | (1usize << (COMPARE - 1)) | (1usize << (DISCOVER_PROCESSES - 1)) | (1usize << (ESTIMATE_COST - 1)) | (1usize << (SAVE - 1)) | (1usize << (RUN - 1)))) != 0)) {break}
			}
			recog.base.set_state(47);
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
//------------------- statement ----------------
pub type StatementContextAll<'input> = StatementContext<'input>;


pub type StatementContext<'input> = BaseParserRuleContext<'input,StatementContextExt<'input>>;

#[derive(Clone)]
pub struct StatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> NexQueryDSLParserContext<'input> for StatementContext<'input>{}

impl<'input,'a> Listenable<dyn NexQueryDSLListener<'input> + 'a> for StatementContext<'input>{
		fn enter(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_statement(self);
		}
		fn exit(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.exit_statement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn NexQueryDSLVisitor<'input> + 'a> for StatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn NexQueryDSLVisitor<'input> + 'a)) {
		visitor.visit_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = NexQueryDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr_rust::tid!{StatementContextExt<'a>}

impl<'input> StatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn NexQueryDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StatementContextAttrs<'input>: NexQueryDSLParserContext<'input> + BorrowMut<StatementContextExt<'input>>{

fn clause_all(&self) ->  Vec<Rc<ClauseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn clause(&self, i: usize) -> Option<Rc<ClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}

}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input>{}

impl<'input, I, H> NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn statement(&mut self,)
	-> Result<Rc<StatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(50); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					{
					/*InvokeRule clause*/
					recog.base.set_state(49);
					recog.clause()?;

					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(52); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(1,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
			}
			recog.base.set_state(55);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==SEMICOLON {
				{
				recog.base.set_state(54);
				recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

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
//------------------- clause ----------------
pub type ClauseContextAll<'input> = ClauseContext<'input>;


pub type ClauseContext<'input> = BaseParserRuleContext<'input,ClauseContextExt<'input>>;

#[derive(Clone)]
pub struct ClauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> NexQueryDSLParserContext<'input> for ClauseContext<'input>{}

impl<'input,'a> Listenable<dyn NexQueryDSLListener<'input> + 'a> for ClauseContext<'input>{
		fn enter(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_clause(self);
		}
		fn exit(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.exit_clause(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn NexQueryDSLVisitor<'input> + 'a> for ClauseContext<'input>{
	fn accept(&self,visitor: &mut (dyn NexQueryDSLVisitor<'input> + 'a)) {
		visitor.visit_clause(self);
	}
}

impl<'input> CustomRuleContext<'input> for ClauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = NexQueryDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_clause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_clause }
}
antlr_rust::tid!{ClauseContextExt<'a>}

impl<'input> ClauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn NexQueryDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ClauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ClauseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ClauseContextAttrs<'input>: NexQueryDSLParserContext<'input> + BorrowMut<ClauseContextExt<'input>>{

fn traverseClause(&self) -> Option<Rc<TraverseClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expandClause(&self) -> Option<Rc<ExpandClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn verbClause(&self) -> Option<Rc<VerbClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ClauseContextAttrs<'input> for ClauseContext<'input>{}

impl<'input, I, H> NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn clause(&mut self,)
	-> Result<Rc<ClauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_clause);
        let mut _localctx: Rc<ClauseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(60);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(3,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule traverseClause*/
					recog.base.set_state(57);
					recog.traverseClause()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule expandClause*/
					recog.base.set_state(58);
					recog.expandClause()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule verbClause*/
					recog.base.set_state(59);
					recog.verbClause()?;

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
//------------------- traverseClause ----------------
pub type TraverseClauseContextAll<'input> = TraverseClauseContext<'input>;


pub type TraverseClauseContext<'input> = BaseParserRuleContext<'input,TraverseClauseContextExt<'input>>;

#[derive(Clone)]
pub struct TraverseClauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> NexQueryDSLParserContext<'input> for TraverseClauseContext<'input>{}

impl<'input,'a> Listenable<dyn NexQueryDSLListener<'input> + 'a> for TraverseClauseContext<'input>{
		fn enter(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_traverseClause(self);
		}
		fn exit(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.exit_traverseClause(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn NexQueryDSLVisitor<'input> + 'a> for TraverseClauseContext<'input>{
	fn accept(&self,visitor: &mut (dyn NexQueryDSLVisitor<'input> + 'a)) {
		visitor.visit_traverseClause(self);
	}
}

impl<'input> CustomRuleContext<'input> for TraverseClauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = NexQueryDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_traverseClause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_traverseClause }
}
antlr_rust::tid!{TraverseClauseContextExt<'a>}

impl<'input> TraverseClauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn NexQueryDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TraverseClauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TraverseClauseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TraverseClauseContextAttrs<'input>: NexQueryDSLParserContext<'input> + BorrowMut<TraverseClauseContextExt<'input>>{

fn nodeType(&self) -> Option<Rc<NodeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn traversalVerb(&self) -> Option<Rc<TraversalVerbContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn target(&self) -> Option<Rc<TargetContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn filter(&self) -> Option<Rc<FilterContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TraverseClauseContextAttrs<'input> for TraverseClauseContext<'input>{}

impl<'input, I, H> NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn traverseClause(&mut self,)
	-> Result<Rc<TraverseClauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TraverseClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_traverseClause);
        let mut _localctx: Rc<TraverseClauseContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule nodeType*/
			recog.base.set_state(62);
			recog.nodeType()?;

			/*InvokeRule traversalVerb*/
			recog.base.set_state(63);
			recog.traversalVerb()?;

			/*InvokeRule target*/
			recog.base.set_state(64);
			recog.target()?;

			recog.base.set_state(66);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LPAREN {
				{
				/*InvokeRule filter*/
				recog.base.set_state(65);
				recog.filter()?;

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
//------------------- expandClause ----------------
pub type ExpandClauseContextAll<'input> = ExpandClauseContext<'input>;


pub type ExpandClauseContext<'input> = BaseParserRuleContext<'input,ExpandClauseContextExt<'input>>;

#[derive(Clone)]
pub struct ExpandClauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> NexQueryDSLParserContext<'input> for ExpandClauseContext<'input>{}

impl<'input,'a> Listenable<dyn NexQueryDSLListener<'input> + 'a> for ExpandClauseContext<'input>{
		fn enter(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expandClause(self);
		}
		fn exit(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.exit_expandClause(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn NexQueryDSLVisitor<'input> + 'a> for ExpandClauseContext<'input>{
	fn accept(&self,visitor: &mut (dyn NexQueryDSLVisitor<'input> + 'a)) {
		visitor.visit_expandClause(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpandClauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = NexQueryDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expandClause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expandClause }
}
antlr_rust::tid!{ExpandClauseContextExt<'a>}

impl<'input> ExpandClauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn NexQueryDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpandClauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpandClauseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpandClauseContextAttrs<'input>: NexQueryDSLParserContext<'input> + BorrowMut<ExpandClauseContextExt<'input>>{

fn nodeType(&self) -> Option<Rc<NodeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn filter(&self) -> Option<Rc<FilterContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExpandClauseContextAttrs<'input> for ExpandClauseContext<'input>{}

impl<'input, I, H> NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expandClause(&mut self,)
	-> Result<Rc<ExpandClauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpandClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_expandClause);
        let mut _localctx: Rc<ExpandClauseContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule nodeType*/
			recog.base.set_state(68);
			recog.nodeType()?;

			recog.base.set_state(70);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LPAREN {
				{
				/*InvokeRule filter*/
				recog.base.set_state(69);
				recog.filter()?;

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
//------------------- verbClause ----------------
pub type VerbClauseContextAll<'input> = VerbClauseContext<'input>;


pub type VerbClauseContext<'input> = BaseParserRuleContext<'input,VerbClauseContextExt<'input>>;

#[derive(Clone)]
pub struct VerbClauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> NexQueryDSLParserContext<'input> for VerbClauseContext<'input>{}

impl<'input,'a> Listenable<dyn NexQueryDSLListener<'input> + 'a> for VerbClauseContext<'input>{
		fn enter(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_verbClause(self);
		}
		fn exit(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.exit_verbClause(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn NexQueryDSLVisitor<'input> + 'a> for VerbClauseContext<'input>{
	fn accept(&self,visitor: &mut (dyn NexQueryDSLVisitor<'input> + 'a)) {
		visitor.visit_verbClause(self);
	}
}

impl<'input> CustomRuleContext<'input> for VerbClauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = NexQueryDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_verbClause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_verbClause }
}
antlr_rust::tid!{VerbClauseContextExt<'a>}

impl<'input> VerbClauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn NexQueryDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VerbClauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VerbClauseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VerbClauseContextAttrs<'input>: NexQueryDSLParserContext<'input> + BorrowMut<VerbClauseContextExt<'input>>{

fn domainVerb(&self) -> Option<Rc<DomainVerbContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn target(&self) -> Option<Rc<TargetContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn modifier_all(&self) ->  Vec<Rc<ModifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn modifier(&self, i: usize) -> Option<Rc<ModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> VerbClauseContextAttrs<'input> for VerbClauseContext<'input>{}

impl<'input, I, H> NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn verbClause(&mut self,)
	-> Result<Rc<VerbClauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VerbClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_verbClause);
        let mut _localctx: Rc<VerbClauseContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule domainVerb*/
			recog.base.set_state(72);
			recog.domainVerb()?;

			recog.base.set_state(74);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(6,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule target*/
					recog.base.set_state(73);
					recog.target()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(79);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (BY - 33)) | (1usize << (TOP - 33)) | (1usize << (IN - 33)) | (1usize << (WITH - 33)) | (1usize << (DEPTH - 33)) | (1usize << (LEVEL - 33)) | (1usize << (ORDER - 33)) | (1usize << (SCOPE - 33)) | (1usize << (THRESHOLD - 33)) | (1usize << (AS - 33)))) != 0) {
				{
				{
				/*InvokeRule modifier*/
				recog.base.set_state(76);
				recog.modifier()?;

				}
				}
				recog.base.set_state(81);
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
//------------------- nodeType ----------------
pub type NodeTypeContextAll<'input> = NodeTypeContext<'input>;


pub type NodeTypeContext<'input> = BaseParserRuleContext<'input,NodeTypeContextExt<'input>>;

#[derive(Clone)]
pub struct NodeTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> NexQueryDSLParserContext<'input> for NodeTypeContext<'input>{}

impl<'input,'a> Listenable<dyn NexQueryDSLListener<'input> + 'a> for NodeTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_nodeType(self);
		}
		fn exit(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.exit_nodeType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn NexQueryDSLVisitor<'input> + 'a> for NodeTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn NexQueryDSLVisitor<'input> + 'a)) {
		visitor.visit_nodeType(self);
	}
}

impl<'input> CustomRuleContext<'input> for NodeTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = NexQueryDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nodeType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nodeType }
}
antlr_rust::tid!{NodeTypeContextExt<'a>}

impl<'input> NodeTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn NexQueryDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NodeTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NodeTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NodeTypeContextAttrs<'input>: NexQueryDSLParserContext<'input> + BorrowMut<NodeTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PROGRAMS
/// Returns `None` if there is no child corresponding to token PROGRAMS
fn PROGRAMS(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(PROGRAMS, 0)
}
/// Retrieves first TerminalNode corresponding to token PARAGRAPHS
/// Returns `None` if there is no child corresponding to token PARAGRAPHS
fn PARAGRAPHS(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(PARAGRAPHS, 0)
}
/// Retrieves first TerminalNode corresponding to token FIELDS
/// Returns `None` if there is no child corresponding to token FIELDS
fn FIELDS(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(FIELDS, 0)
}
/// Retrieves first TerminalNode corresponding to token COPYBOOKS
/// Returns `None` if there is no child corresponding to token COPYBOOKS
fn COPYBOOKS(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(COPYBOOKS, 0)
}
/// Retrieves first TerminalNode corresponding to token FILES
/// Returns `None` if there is no child corresponding to token FILES
fn FILES(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(FILES, 0)
}
/// Retrieves first TerminalNode corresponding to token TABLES
/// Returns `None` if there is no child corresponding to token TABLES
fn TABLES(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(TABLES, 0)
}
/// Retrieves first TerminalNode corresponding to token RULES
/// Returns `None` if there is no child corresponding to token RULES
fn RULES(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(RULES, 0)
}

}

impl<'input> NodeTypeContextAttrs<'input> for NodeTypeContext<'input>{}

impl<'input, I, H> NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn nodeType(&mut self,)
	-> Result<Rc<NodeTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NodeTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_nodeType);
        let mut _localctx: Rc<NodeTypeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(82);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << PROGRAMS) | (1usize << PARAGRAPHS) | (1usize << FIELDS) | (1usize << COPYBOOKS) | (1usize << FILES) | (1usize << TABLES) | (1usize << RULES))) != 0)) } {
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
//------------------- traversalVerb ----------------
pub type TraversalVerbContextAll<'input> = TraversalVerbContext<'input>;


pub type TraversalVerbContext<'input> = BaseParserRuleContext<'input,TraversalVerbContextExt<'input>>;

#[derive(Clone)]
pub struct TraversalVerbContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> NexQueryDSLParserContext<'input> for TraversalVerbContext<'input>{}

impl<'input,'a> Listenable<dyn NexQueryDSLListener<'input> + 'a> for TraversalVerbContext<'input>{
		fn enter(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_traversalVerb(self);
		}
		fn exit(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.exit_traversalVerb(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn NexQueryDSLVisitor<'input> + 'a> for TraversalVerbContext<'input>{
	fn accept(&self,visitor: &mut (dyn NexQueryDSLVisitor<'input> + 'a)) {
		visitor.visit_traversalVerb(self);
	}
}

impl<'input> CustomRuleContext<'input> for TraversalVerbContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = NexQueryDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_traversalVerb }
	//fn type_rule_index() -> usize where Self: Sized { RULE_traversalVerb }
}
antlr_rust::tid!{TraversalVerbContextExt<'a>}

impl<'input> TraversalVerbContextExt<'input>{
	fn new(parent: Option<Rc<dyn NexQueryDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TraversalVerbContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TraversalVerbContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TraversalVerbContextAttrs<'input>: NexQueryDSLParserContext<'input> + BorrowMut<TraversalVerbContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CALLING
/// Returns `None` if there is no child corresponding to token CALLING
fn CALLING(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(CALLING, 0)
}
/// Retrieves first TerminalNode corresponding to token CALLED_BY
/// Returns `None` if there is no child corresponding to token CALLED_BY
fn CALLED_BY(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(CALLED_BY, 0)
}
/// Retrieves first TerminalNode corresponding to token PERFORMING
/// Returns `None` if there is no child corresponding to token PERFORMING
fn PERFORMING(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(PERFORMING, 0)
}
/// Retrieves first TerminalNode corresponding to token PERFORMED_BY
/// Returns `None` if there is no child corresponding to token PERFORMED_BY
fn PERFORMED_BY(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(PERFORMED_BY, 0)
}
/// Retrieves first TerminalNode corresponding to token READING
/// Returns `None` if there is no child corresponding to token READING
fn READING(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(READING, 0)
}
/// Retrieves first TerminalNode corresponding to token READ_BY
/// Returns `None` if there is no child corresponding to token READ_BY
fn READ_BY(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(READ_BY, 0)
}
/// Retrieves first TerminalNode corresponding to token WRITING
/// Returns `None` if there is no child corresponding to token WRITING
fn WRITING(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(WRITING, 0)
}
/// Retrieves first TerminalNode corresponding to token WRITTEN_BY
/// Returns `None` if there is no child corresponding to token WRITTEN_BY
fn WRITTEN_BY(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(WRITTEN_BY, 0)
}
/// Retrieves first TerminalNode corresponding to token USING
/// Returns `None` if there is no child corresponding to token USING
fn USING(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(USING, 0)
}
/// Retrieves first TerminalNode corresponding to token USED_BY
/// Returns `None` if there is no child corresponding to token USED_BY
fn USED_BY(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(USED_BY, 0)
}
/// Retrieves first TerminalNode corresponding to token ACCESSING
/// Returns `None` if there is no child corresponding to token ACCESSING
fn ACCESSING(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(ACCESSING, 0)
}
/// Retrieves first TerminalNode corresponding to token ACCESSED_BY
/// Returns `None` if there is no child corresponding to token ACCESSED_BY
fn ACCESSED_BY(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(ACCESSED_BY, 0)
}
/// Retrieves first TerminalNode corresponding to token CONTAINING
/// Returns `None` if there is no child corresponding to token CONTAINING
fn CONTAINING(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(CONTAINING, 0)
}
/// Retrieves first TerminalNode corresponding to token WITHIN
/// Returns `None` if there is no child corresponding to token WITHIN
fn WITHIN(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(WITHIN, 0)
}

}

impl<'input> TraversalVerbContextAttrs<'input> for TraversalVerbContext<'input>{}

impl<'input, I, H> NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn traversalVerb(&mut self,)
	-> Result<Rc<TraversalVerbContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TraversalVerbContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_traversalVerb);
        let mut _localctx: Rc<TraversalVerbContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(84);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << CALLING) | (1usize << CALLED_BY) | (1usize << PERFORMING) | (1usize << PERFORMED_BY) | (1usize << READING) | (1usize << READ_BY) | (1usize << WRITING) | (1usize << WRITTEN_BY) | (1usize << USING) | (1usize << USED_BY) | (1usize << ACCESSING) | (1usize << ACCESSED_BY) | (1usize << CONTAINING) | (1usize << WITHIN))) != 0)) } {
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
//------------------- domainVerb ----------------
pub type DomainVerbContextAll<'input> = DomainVerbContext<'input>;


pub type DomainVerbContext<'input> = BaseParserRuleContext<'input,DomainVerbContextExt<'input>>;

#[derive(Clone)]
pub struct DomainVerbContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> NexQueryDSLParserContext<'input> for DomainVerbContext<'input>{}

impl<'input,'a> Listenable<dyn NexQueryDSLListener<'input> + 'a> for DomainVerbContext<'input>{
		fn enter(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_domainVerb(self);
		}
		fn exit(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.exit_domainVerb(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn NexQueryDSLVisitor<'input> + 'a> for DomainVerbContext<'input>{
	fn accept(&self,visitor: &mut (dyn NexQueryDSLVisitor<'input> + 'a)) {
		visitor.visit_domainVerb(self);
	}
}

impl<'input> CustomRuleContext<'input> for DomainVerbContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = NexQueryDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_domainVerb }
	//fn type_rule_index() -> usize where Self: Sized { RULE_domainVerb }
}
antlr_rust::tid!{DomainVerbContextExt<'a>}

impl<'input> DomainVerbContextExt<'input>{
	fn new(parent: Option<Rc<dyn NexQueryDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DomainVerbContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DomainVerbContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DomainVerbContextAttrs<'input>: NexQueryDSLParserContext<'input> + BorrowMut<DomainVerbContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TRACE
/// Returns `None` if there is no child corresponding to token TRACE
fn TRACE(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(TRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RANK
/// Returns `None` if there is no child corresponding to token RANK
fn RANK(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(RANK, 0)
}
/// Retrieves first TerminalNode corresponding to token SIMILAR
/// Returns `None` if there is no child corresponding to token SIMILAR
fn SIMILAR(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(SIMILAR, 0)
}
/// Retrieves first TerminalNode corresponding to token FIND_DEAD
/// Returns `None` if there is no child corresponding to token FIND_DEAD
fn FIND_DEAD(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(FIND_DEAD, 0)
}
/// Retrieves first TerminalNode corresponding to token DEPS
/// Returns `None` if there is no child corresponding to token DEPS
fn DEPS(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(DEPS, 0)
}
/// Retrieves first TerminalNode corresponding to token IMPACT
/// Returns `None` if there is no child corresponding to token IMPACT
fn IMPACT(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(IMPACT, 0)
}
/// Retrieves first TerminalNode corresponding to token COMPARE
/// Returns `None` if there is no child corresponding to token COMPARE
fn COMPARE(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(COMPARE, 0)
}
/// Retrieves first TerminalNode corresponding to token DISCOVER_PROCESSES
/// Returns `None` if there is no child corresponding to token DISCOVER_PROCESSES
fn DISCOVER_PROCESSES(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(DISCOVER_PROCESSES, 0)
}
/// Retrieves first TerminalNode corresponding to token ESTIMATE_COST
/// Returns `None` if there is no child corresponding to token ESTIMATE_COST
fn ESTIMATE_COST(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(ESTIMATE_COST, 0)
}
/// Retrieves first TerminalNode corresponding to token SAVE
/// Returns `None` if there is no child corresponding to token SAVE
fn SAVE(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(SAVE, 0)
}
/// Retrieves first TerminalNode corresponding to token RUN
/// Returns `None` if there is no child corresponding to token RUN
fn RUN(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(RUN, 0)
}

}

impl<'input> DomainVerbContextAttrs<'input> for DomainVerbContext<'input>{}

impl<'input, I, H> NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn domainVerb(&mut self,)
	-> Result<Rc<DomainVerbContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DomainVerbContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_domainVerb);
        let mut _localctx: Rc<DomainVerbContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(86);
			_la = recog.base.input.la(1);
			if { !(((((_la - 22)) & !0x3f) == 0 && ((1usize << (_la - 22)) & ((1usize << (TRACE - 22)) | (1usize << (RANK - 22)) | (1usize << (SIMILAR - 22)) | (1usize << (FIND_DEAD - 22)) | (1usize << (DEPS - 22)) | (1usize << (IMPACT - 22)) | (1usize << (COMPARE - 22)) | (1usize << (DISCOVER_PROCESSES - 22)) | (1usize << (ESTIMATE_COST - 22)) | (1usize << (SAVE - 22)) | (1usize << (RUN - 22)))) != 0)) } {
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
//------------------- target ----------------
pub type TargetContextAll<'input> = TargetContext<'input>;


pub type TargetContext<'input> = BaseParserRuleContext<'input,TargetContextExt<'input>>;

#[derive(Clone)]
pub struct TargetContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> NexQueryDSLParserContext<'input> for TargetContext<'input>{}

impl<'input,'a> Listenable<dyn NexQueryDSLListener<'input> + 'a> for TargetContext<'input>{
		fn enter(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_target(self);
		}
		fn exit(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.exit_target(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn NexQueryDSLVisitor<'input> + 'a> for TargetContext<'input>{
	fn accept(&self,visitor: &mut (dyn NexQueryDSLVisitor<'input> + 'a)) {
		visitor.visit_target(self);
	}
}

impl<'input> CustomRuleContext<'input> for TargetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = NexQueryDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_target }
	//fn type_rule_index() -> usize where Self: Sized { RULE_target }
}
antlr_rust::tid!{TargetContextExt<'a>}

impl<'input> TargetContextExt<'input>{
	fn new(parent: Option<Rc<dyn NexQueryDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TargetContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TargetContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TargetContextAttrs<'input>: NexQueryDSLParserContext<'input> + BorrowMut<TargetContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EACH
/// Returns `None` if there is no child corresponding to token EACH
fn EACH(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(EACH, 0)
}
fn list(&self) -> Option<Rc<ListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn nodeType(&self) -> Option<Rc<NodeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> TargetContextAttrs<'input> for TargetContext<'input>{}

impl<'input, I, H> NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn target(&mut self,)
	-> Result<Rc<TargetContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TargetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_target);
        let mut _localctx: Rc<TargetContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(92);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 EACH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(88);
					recog.base.match_token(EACH,&mut recog.err_handler)?;

					}
				}

			 LBRACKET 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule list*/
					recog.base.set_state(89);
					recog.list()?;

					}
				}

			 PROGRAMS | PARAGRAPHS | FIELDS | COPYBOOKS | FILES | TABLES | RULES 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule nodeType*/
					recog.base.set_state(90);
					recog.nodeType()?;

					}
				}

			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(91);
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
//------------------- list ----------------
pub type ListContextAll<'input> = ListContext<'input>;


pub type ListContext<'input> = BaseParserRuleContext<'input,ListContextExt<'input>>;

#[derive(Clone)]
pub struct ListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> NexQueryDSLParserContext<'input> for ListContext<'input>{}

impl<'input,'a> Listenable<dyn NexQueryDSLListener<'input> + 'a> for ListContext<'input>{
		fn enter(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_list(self);
		}
		fn exit(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.exit_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn NexQueryDSLVisitor<'input> + 'a> for ListContext<'input>{
	fn accept(&self,visitor: &mut (dyn NexQueryDSLVisitor<'input> + 'a)) {
		visitor.visit_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for ListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = NexQueryDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_list }
}
antlr_rust::tid!{ListContextExt<'a>}

impl<'input> ListContextExt<'input>{
	fn new(parent: Option<Rc<dyn NexQueryDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ListContextAttrs<'input>: NexQueryDSLParserContext<'input> + BorrowMut<ListContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACKET
/// Returns `None` if there is no child corresponding to token LBRACKET
fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(LBRACKET, 0)
}
fn listItem_all(&self) ->  Vec<Rc<ListItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn listItem(&self, i: usize) -> Option<Rc<ListItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RBRACKET
/// Returns `None` if there is no child corresponding to token RBRACKET
fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(RBRACKET, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ListContextAttrs<'input> for ListContext<'input>{}

impl<'input, I, H> NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn list(&mut self,)
	-> Result<Rc<ListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_list);
        let mut _localctx: Rc<ListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(94);
			recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

			/*InvokeRule listItem*/
			recog.base.set_state(95);
			recog.listItem()?;

			recog.base.set_state(100);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(96);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule listItem*/
				recog.base.set_state(97);
				recog.listItem()?;

				}
				}
				recog.base.set_state(102);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(103);
			recog.base.match_token(RBRACKET,&mut recog.err_handler)?;

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
//------------------- listItem ----------------
pub type ListItemContextAll<'input> = ListItemContext<'input>;


pub type ListItemContext<'input> = BaseParserRuleContext<'input,ListItemContextExt<'input>>;

#[derive(Clone)]
pub struct ListItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> NexQueryDSLParserContext<'input> for ListItemContext<'input>{}

impl<'input,'a> Listenable<dyn NexQueryDSLListener<'input> + 'a> for ListItemContext<'input>{
		fn enter(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_listItem(self);
		}
		fn exit(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.exit_listItem(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn NexQueryDSLVisitor<'input> + 'a> for ListItemContext<'input>{
	fn accept(&self,visitor: &mut (dyn NexQueryDSLVisitor<'input> + 'a)) {
		visitor.visit_listItem(self);
	}
}

impl<'input> CustomRuleContext<'input> for ListItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = NexQueryDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listItem }
}
antlr_rust::tid!{ListItemContextExt<'a>}

impl<'input> ListItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn NexQueryDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ListItemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListItemContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ListItemContextAttrs<'input>: NexQueryDSLParserContext<'input> + BorrowMut<ListItemContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}

}

impl<'input> ListItemContextAttrs<'input> for ListItemContext<'input>{}

impl<'input, I, H> NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn listItem(&mut self,)
	-> Result<Rc<ListItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ListItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_listItem);
        let mut _localctx: Rc<ListItemContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(105);
			_la = recog.base.input.la(1);
			if { !(_la==STRING || _la==IDENTIFIER) } {
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
//------------------- filter ----------------
pub type FilterContextAll<'input> = FilterContext<'input>;


pub type FilterContext<'input> = BaseParserRuleContext<'input,FilterContextExt<'input>>;

#[derive(Clone)]
pub struct FilterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> NexQueryDSLParserContext<'input> for FilterContext<'input>{}

impl<'input,'a> Listenable<dyn NexQueryDSLListener<'input> + 'a> for FilterContext<'input>{
		fn enter(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_filter(self);
		}
		fn exit(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.exit_filter(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn NexQueryDSLVisitor<'input> + 'a> for FilterContext<'input>{
	fn accept(&self,visitor: &mut (dyn NexQueryDSLVisitor<'input> + 'a)) {
		visitor.visit_filter(self);
	}
}

impl<'input> CustomRuleContext<'input> for FilterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = NexQueryDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_filter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_filter }
}
antlr_rust::tid!{FilterContextExt<'a>}

impl<'input> FilterContextExt<'input>{
	fn new(parent: Option<Rc<dyn NexQueryDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FilterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FilterContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FilterContextAttrs<'input>: NexQueryDSLParserContext<'input> + BorrowMut<FilterContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn filterExpr(&self) -> Option<Rc<FilterExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> FilterContextAttrs<'input> for FilterContext<'input>{}

impl<'input, I, H> NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn filter(&mut self,)
	-> Result<Rc<FilterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FilterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_filter);
        let mut _localctx: Rc<FilterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(107);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule filterExpr*/
			recog.base.set_state(108);
			recog.filterExpr_rec(0)?;

			recog.base.set_state(109);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

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
//------------------- filterExpr ----------------
pub type FilterExprContextAll<'input> = FilterExprContext<'input>;


pub type FilterExprContext<'input> = BaseParserRuleContext<'input,FilterExprContextExt<'input>>;

#[derive(Clone)]
pub struct FilterExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> NexQueryDSLParserContext<'input> for FilterExprContext<'input>{}

impl<'input,'a> Listenable<dyn NexQueryDSLListener<'input> + 'a> for FilterExprContext<'input>{
		fn enter(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_filterExpr(self);
		}
		fn exit(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.exit_filterExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn NexQueryDSLVisitor<'input> + 'a> for FilterExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn NexQueryDSLVisitor<'input> + 'a)) {
		visitor.visit_filterExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for FilterExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = NexQueryDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_filterExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_filterExpr }
}
antlr_rust::tid!{FilterExprContextExt<'a>}

impl<'input> FilterExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn NexQueryDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FilterExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FilterExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FilterExprContextAttrs<'input>: NexQueryDSLParserContext<'input> + BorrowMut<FilterExprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token NOT
/// Returns `None` if there is no child corresponding to token NOT
fn NOT(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(NOT, 0)
}
fn filterExpr_all(&self) ->  Vec<Rc<FilterExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn filterExpr(&self, i: usize) -> Option<Rc<FilterExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn predicate(&self) -> Option<Rc<PredicateContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token AND
/// Returns `None` if there is no child corresponding to token AND
fn AND(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(AND, 0)
}
/// Retrieves first TerminalNode corresponding to token OR
/// Returns `None` if there is no child corresponding to token OR
fn OR(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(OR, 0)
}

}

impl<'input> FilterExprContextAttrs<'input> for FilterExprContext<'input>{}

impl<'input, I, H> NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  filterExpr(&mut self,)
	-> Result<Rc<FilterExprContextAll<'input>>,ANTLRError> {
		self.filterExpr_rec(0)
	}

	fn filterExpr_rec(&mut self, _p: isize)
	-> Result<Rc<FilterExprContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = FilterExprContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 26, RULE_filterExpr, _p);
	    let mut _localctx: Rc<FilterExprContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 26;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(119);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 NOT 
				=> {
					{
					recog.base.set_state(112);
					recog.base.match_token(NOT,&mut recog.err_handler)?;

					/*InvokeRule filterExpr*/
					recog.base.set_state(113);
					recog.filterExpr_rec(3)?;

					}
				}

			 LPAREN 
				=> {
					{
					recog.base.set_state(114);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule filterExpr*/
					recog.base.set_state(115);
					recog.filterExpr_rec(0)?;

					recog.base.set_state(116);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}

			 IDENTIFIER 
				=> {
					{
					/*InvokeRule predicate*/
					recog.base.set_state(118);
					recog.predicate()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(129);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(12,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(127);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(11,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = FilterExprContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_filterExpr);
							_localctx = tmp;
							recog.base.set_state(121);
							if !({recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(122);
							recog.base.match_token(AND,&mut recog.err_handler)?;

							/*InvokeRule filterExpr*/
							recog.base.set_state(123);
							recog.filterExpr_rec(6)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = FilterExprContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_filterExpr);
							_localctx = tmp;
							recog.base.set_state(124);
							if !({recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(125);
							recog.base.match_token(OR,&mut recog.err_handler)?;

							/*InvokeRule filterExpr*/
							recog.base.set_state(126);
							recog.filterExpr_rec(5)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(131);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(12,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

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

impl<'input> NexQueryDSLParserContext<'input> for PredicateContext<'input>{}

impl<'input,'a> Listenable<dyn NexQueryDSLListener<'input> + 'a> for PredicateContext<'input>{
		fn enter(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_predicate(self);
		}
		fn exit(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.exit_predicate(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn NexQueryDSLVisitor<'input> + 'a> for PredicateContext<'input>{
	fn accept(&self,visitor: &mut (dyn NexQueryDSLVisitor<'input> + 'a)) {
		visitor.visit_predicate(self);
	}
}

impl<'input> CustomRuleContext<'input> for PredicateContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = NexQueryDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_predicate }
	//fn type_rule_index() -> usize where Self: Sized { RULE_predicate }
}
antlr_rust::tid!{PredicateContextExt<'a>}

impl<'input> PredicateContextExt<'input>{
	fn new(parent: Option<Rc<dyn NexQueryDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PredicateContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PredicateContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PredicateContextAttrs<'input>: NexQueryDSLParserContext<'input> + BorrowMut<PredicateContextExt<'input>>{

fn fieldRef(&self) -> Option<Rc<FieldRefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn compareOp(&self) -> Option<Rc<CompareOpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn value(&self) -> Option<Rc<ValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PredicateContextAttrs<'input> for PredicateContext<'input>{}

impl<'input, I, H> NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn predicate(&mut self,)
	-> Result<Rc<PredicateContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PredicateContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_predicate);
        let mut _localctx: Rc<PredicateContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fieldRef*/
			recog.base.set_state(132);
			recog.fieldRef()?;

			/*InvokeRule compareOp*/
			recog.base.set_state(133);
			recog.compareOp()?;

			/*InvokeRule value*/
			recog.base.set_state(134);
			recog.value()?;

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
//------------------- fieldRef ----------------
pub type FieldRefContextAll<'input> = FieldRefContext<'input>;


pub type FieldRefContext<'input> = BaseParserRuleContext<'input,FieldRefContextExt<'input>>;

#[derive(Clone)]
pub struct FieldRefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> NexQueryDSLParserContext<'input> for FieldRefContext<'input>{}

impl<'input,'a> Listenable<dyn NexQueryDSLListener<'input> + 'a> for FieldRefContext<'input>{
		fn enter(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fieldRef(self);
		}
		fn exit(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.exit_fieldRef(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn NexQueryDSLVisitor<'input> + 'a> for FieldRefContext<'input>{
	fn accept(&self,visitor: &mut (dyn NexQueryDSLVisitor<'input> + 'a)) {
		visitor.visit_fieldRef(self);
	}
}

impl<'input> CustomRuleContext<'input> for FieldRefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = NexQueryDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fieldRef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fieldRef }
}
antlr_rust::tid!{FieldRefContextExt<'a>}

impl<'input> FieldRefContextExt<'input>{
	fn new(parent: Option<Rc<dyn NexQueryDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FieldRefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FieldRefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FieldRefContextAttrs<'input>: NexQueryDSLParserContext<'input> + BorrowMut<FieldRefContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}

}

impl<'input> FieldRefContextAttrs<'input> for FieldRefContext<'input>{}

impl<'input, I, H> NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fieldRef(&mut self,)
	-> Result<Rc<FieldRefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FieldRefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_fieldRef);
        let mut _localctx: Rc<FieldRefContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(136);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(139);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==DOT {
				{
				recog.base.set_state(137);
				recog.base.match_token(DOT,&mut recog.err_handler)?;

				recog.base.set_state(138);
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
//------------------- compareOp ----------------
pub type CompareOpContextAll<'input> = CompareOpContext<'input>;


pub type CompareOpContext<'input> = BaseParserRuleContext<'input,CompareOpContextExt<'input>>;

#[derive(Clone)]
pub struct CompareOpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> NexQueryDSLParserContext<'input> for CompareOpContext<'input>{}

impl<'input,'a> Listenable<dyn NexQueryDSLListener<'input> + 'a> for CompareOpContext<'input>{
		fn enter(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_compareOp(self);
		}
		fn exit(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.exit_compareOp(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn NexQueryDSLVisitor<'input> + 'a> for CompareOpContext<'input>{
	fn accept(&self,visitor: &mut (dyn NexQueryDSLVisitor<'input> + 'a)) {
		visitor.visit_compareOp(self);
	}
}

impl<'input> CustomRuleContext<'input> for CompareOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = NexQueryDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compareOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compareOp }
}
antlr_rust::tid!{CompareOpContextExt<'a>}

impl<'input> CompareOpContextExt<'input>{
	fn new(parent: Option<Rc<dyn NexQueryDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CompareOpContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompareOpContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CompareOpContextAttrs<'input>: NexQueryDSLParserContext<'input> + BorrowMut<CompareOpContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EQ
/// Returns `None` if there is no child corresponding to token EQ
fn EQ(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(EQ, 0)
}
/// Retrieves first TerminalNode corresponding to token NE
/// Returns `None` if there is no child corresponding to token NE
fn NE(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(NE, 0)
}
/// Retrieves first TerminalNode corresponding to token GT
/// Returns `None` if there is no child corresponding to token GT
fn GT(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(GT, 0)
}
/// Retrieves first TerminalNode corresponding to token LT
/// Returns `None` if there is no child corresponding to token LT
fn LT(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(LT, 0)
}
/// Retrieves first TerminalNode corresponding to token GE
/// Returns `None` if there is no child corresponding to token GE
fn GE(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(GE, 0)
}
/// Retrieves first TerminalNode corresponding to token LE
/// Returns `None` if there is no child corresponding to token LE
fn LE(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(LE, 0)
}
/// Retrieves first TerminalNode corresponding to token GLOB
/// Returns `None` if there is no child corresponding to token GLOB
fn GLOB(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(GLOB, 0)
}
/// Retrieves first TerminalNode corresponding to token REGEX
/// Returns `None` if there is no child corresponding to token REGEX
fn REGEX(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(REGEX, 0)
}
/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(IN, 0)
}
/// Retrieves first TerminalNode corresponding to token HAS
/// Returns `None` if there is no child corresponding to token HAS
fn HAS(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(HAS, 0)
}

}

impl<'input> CompareOpContextAttrs<'input> for CompareOpContext<'input>{}

impl<'input, I, H> NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compareOp(&mut self,)
	-> Result<Rc<CompareOpContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CompareOpContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_compareOp);
        let mut _localctx: Rc<CompareOpContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(141);
			_la = recog.base.input.la(1);
			if { !(((((_la - 35)) & !0x3f) == 0 && ((1usize << (_la - 35)) & ((1usize << (IN - 35)) | (1usize << (HAS - 35)) | (1usize << (EQ - 35)) | (1usize << (NE - 35)) | (1usize << (GT - 35)) | (1usize << (LT - 35)) | (1usize << (GE - 35)) | (1usize << (LE - 35)) | (1usize << (GLOB - 35)) | (1usize << (REGEX - 35)))) != 0)) } {
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
//------------------- value ----------------
pub type ValueContextAll<'input> = ValueContext<'input>;


pub type ValueContext<'input> = BaseParserRuleContext<'input,ValueContextExt<'input>>;

#[derive(Clone)]
pub struct ValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> NexQueryDSLParserContext<'input> for ValueContext<'input>{}

impl<'input,'a> Listenable<dyn NexQueryDSLListener<'input> + 'a> for ValueContext<'input>{
		fn enter(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_value(self);
		}
		fn exit(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.exit_value(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn NexQueryDSLVisitor<'input> + 'a> for ValueContext<'input>{
	fn accept(&self,visitor: &mut (dyn NexQueryDSLVisitor<'input> + 'a)) {
		visitor.visit_value(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = NexQueryDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value }
}
antlr_rust::tid!{ValueContextExt<'a>}

impl<'input> ValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn NexQueryDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ValueContextAttrs<'input>: NexQueryDSLParserContext<'input> + BorrowMut<ValueContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}
fn list(&self) -> Option<Rc<ListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ValueContextAttrs<'input> for ValueContext<'input>{}

impl<'input, I, H> NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn value(&mut self,)
	-> Result<Rc<ValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_value);
        let mut _localctx: Rc<ValueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(146);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 STRING 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(143);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}

			 NUMBER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(144);
					recog.base.match_token(NUMBER,&mut recog.err_handler)?;

					}
				}

			 LBRACKET 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule list*/
					recog.base.set_state(145);
					recog.list()?;

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
//------------------- modifier ----------------
pub type ModifierContextAll<'input> = ModifierContext<'input>;


pub type ModifierContext<'input> = BaseParserRuleContext<'input,ModifierContextExt<'input>>;

#[derive(Clone)]
pub struct ModifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> NexQueryDSLParserContext<'input> for ModifierContext<'input>{}

impl<'input,'a> Listenable<dyn NexQueryDSLListener<'input> + 'a> for ModifierContext<'input>{
		fn enter(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_modifier(self);
		}
		fn exit(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.exit_modifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn NexQueryDSLVisitor<'input> + 'a> for ModifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn NexQueryDSLVisitor<'input> + 'a)) {
		visitor.visit_modifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for ModifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = NexQueryDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_modifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_modifier }
}
antlr_rust::tid!{ModifierContextExt<'a>}

impl<'input> ModifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn NexQueryDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ModifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ModifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ModifierContextAttrs<'input>: NexQueryDSLParserContext<'input> + BorrowMut<ModifierContextExt<'input>>{

fn modifierKeyword(&self) -> Option<Rc<ModifierKeywordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn modifierValue(&self) -> Option<Rc<ModifierValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ModifierContextAttrs<'input> for ModifierContext<'input>{}

impl<'input, I, H> NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn modifier(&mut self,)
	-> Result<Rc<ModifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ModifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_modifier);
        let mut _localctx: Rc<ModifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule modifierKeyword*/
			recog.base.set_state(148);
			recog.modifierKeyword()?;

			/*InvokeRule modifierValue*/
			recog.base.set_state(149);
			recog.modifierValue()?;

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
//------------------- modifierKeyword ----------------
pub type ModifierKeywordContextAll<'input> = ModifierKeywordContext<'input>;


pub type ModifierKeywordContext<'input> = BaseParserRuleContext<'input,ModifierKeywordContextExt<'input>>;

#[derive(Clone)]
pub struct ModifierKeywordContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> NexQueryDSLParserContext<'input> for ModifierKeywordContext<'input>{}

impl<'input,'a> Listenable<dyn NexQueryDSLListener<'input> + 'a> for ModifierKeywordContext<'input>{
		fn enter(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_modifierKeyword(self);
		}
		fn exit(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.exit_modifierKeyword(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn NexQueryDSLVisitor<'input> + 'a> for ModifierKeywordContext<'input>{
	fn accept(&self,visitor: &mut (dyn NexQueryDSLVisitor<'input> + 'a)) {
		visitor.visit_modifierKeyword(self);
	}
}

impl<'input> CustomRuleContext<'input> for ModifierKeywordContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = NexQueryDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_modifierKeyword }
	//fn type_rule_index() -> usize where Self: Sized { RULE_modifierKeyword }
}
antlr_rust::tid!{ModifierKeywordContextExt<'a>}

impl<'input> ModifierKeywordContextExt<'input>{
	fn new(parent: Option<Rc<dyn NexQueryDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ModifierKeywordContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ModifierKeywordContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ModifierKeywordContextAttrs<'input>: NexQueryDSLParserContext<'input> + BorrowMut<ModifierKeywordContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BY
/// Returns `None` if there is no child corresponding to token BY
fn BY(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(BY, 0)
}
/// Retrieves first TerminalNode corresponding to token TOP
/// Returns `None` if there is no child corresponding to token TOP
fn TOP(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(TOP, 0)
}
/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(IN, 0)
}
/// Retrieves first TerminalNode corresponding to token WITH
/// Returns `None` if there is no child corresponding to token WITH
fn WITH(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(WITH, 0)
}
/// Retrieves first TerminalNode corresponding to token DEPTH
/// Returns `None` if there is no child corresponding to token DEPTH
fn DEPTH(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(DEPTH, 0)
}
/// Retrieves first TerminalNode corresponding to token LEVEL
/// Returns `None` if there is no child corresponding to token LEVEL
fn LEVEL(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(LEVEL, 0)
}
/// Retrieves first TerminalNode corresponding to token ORDER
/// Returns `None` if there is no child corresponding to token ORDER
fn ORDER(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(ORDER, 0)
}
/// Retrieves first TerminalNode corresponding to token SCOPE
/// Returns `None` if there is no child corresponding to token SCOPE
fn SCOPE(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(SCOPE, 0)
}
/// Retrieves first TerminalNode corresponding to token THRESHOLD
/// Returns `None` if there is no child corresponding to token THRESHOLD
fn THRESHOLD(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(THRESHOLD, 0)
}
/// Retrieves first TerminalNode corresponding to token AS
/// Returns `None` if there is no child corresponding to token AS
fn AS(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(AS, 0)
}

}

impl<'input> ModifierKeywordContextAttrs<'input> for ModifierKeywordContext<'input>{}

impl<'input, I, H> NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn modifierKeyword(&mut self,)
	-> Result<Rc<ModifierKeywordContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ModifierKeywordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_modifierKeyword);
        let mut _localctx: Rc<ModifierKeywordContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(151);
			_la = recog.base.input.la(1);
			if { !(((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (BY - 33)) | (1usize << (TOP - 33)) | (1usize << (IN - 33)) | (1usize << (WITH - 33)) | (1usize << (DEPTH - 33)) | (1usize << (LEVEL - 33)) | (1usize << (ORDER - 33)) | (1usize << (SCOPE - 33)) | (1usize << (THRESHOLD - 33)) | (1usize << (AS - 33)))) != 0)) } {
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
//------------------- modifierValue ----------------
pub type ModifierValueContextAll<'input> = ModifierValueContext<'input>;


pub type ModifierValueContext<'input> = BaseParserRuleContext<'input,ModifierValueContextExt<'input>>;

#[derive(Clone)]
pub struct ModifierValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> NexQueryDSLParserContext<'input> for ModifierValueContext<'input>{}

impl<'input,'a> Listenable<dyn NexQueryDSLListener<'input> + 'a> for ModifierValueContext<'input>{
		fn enter(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_modifierValue(self);
		}
		fn exit(&self,listener: &mut (dyn NexQueryDSLListener<'input> + 'a)) {
			listener.exit_modifierValue(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn NexQueryDSLVisitor<'input> + 'a> for ModifierValueContext<'input>{
	fn accept(&self,visitor: &mut (dyn NexQueryDSLVisitor<'input> + 'a)) {
		visitor.visit_modifierValue(self);
	}
}

impl<'input> CustomRuleContext<'input> for ModifierValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = NexQueryDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_modifierValue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_modifierValue }
}
antlr_rust::tid!{ModifierValueContextExt<'a>}

impl<'input> ModifierValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn NexQueryDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ModifierValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ModifierValueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ModifierValueContextAttrs<'input>: NexQueryDSLParserContext<'input> + BorrowMut<ModifierValueContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}
fn nodeType(&self) -> Option<Rc<NodeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,NexQueryDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}

}

impl<'input> ModifierValueContextAttrs<'input> for ModifierValueContext<'input>{}

impl<'input, I, H> NexQueryDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn modifierValue(&mut self,)
	-> Result<Rc<ModifierValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ModifierValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_modifierValue);
        let mut _localctx: Rc<ModifierValueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(157);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(153);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

			 NUMBER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(154);
					recog.base.match_token(NUMBER,&mut recog.err_handler)?;

					}
				}

			 PROGRAMS | PARAGRAPHS | FIELDS | COPYBOOKS | FILES | TABLES | RULES 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule nodeType*/
					recog.base.set_state(155);
					recog.nodeType()?;

					}
				}

			 STRING 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(156);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

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
	\x46\u{a2}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\x0a\
	\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\x0e\
	\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\x13\
	\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x03\x02\x06\x02\
	\x2e\x0a\x02\x0d\x02\x0e\x02\x2f\x03\x02\x03\x02\x03\x03\x06\x03\x35\x0a\
	\x03\x0d\x03\x0e\x03\x36\x03\x03\x05\x03\x3a\x0a\x03\x03\x04\x03\x04\x03\
	\x04\x05\x04\x3f\x0a\x04\x03\x05\x03\x05\x03\x05\x03\x05\x05\x05\x45\x0a\
	\x05\x03\x06\x03\x06\x05\x06\x49\x0a\x06\x03\x07\x03\x07\x05\x07\x4d\x0a\
	\x07\x03\x07\x07\x07\x50\x0a\x07\x0c\x07\x0e\x07\x53\x0b\x07\x03\x08\x03\
	\x08\x03\x09\x03\x09\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x05\
	\x0b\x5f\x0a\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x07\x0c\x65\x0a\x0c\x0c\
	\x0c\x0e\x0c\x68\x0b\x0c\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0e\x03\x0e\
	\x03\x0e\x03\x0e\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\
	\x03\x0f\x05\x0f\x7a\x0a\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\
	\x0f\x07\x0f\u{82}\x0a\x0f\x0c\x0f\x0e\x0f\u{85}\x0b\x0f\x03\x10\x03\x10\
	\x03\x10\x03\x10\x03\x11\x03\x11\x03\x11\x05\x11\u{8e}\x0a\x11\x03\x12\x03\
	\x12\x03\x13\x03\x13\x03\x13\x05\x13\u{95}\x0a\x13\x03\x14\x03\x14\x03\x14\
	\x03\x15\x03\x15\x03\x16\x03\x16\x03\x16\x03\x16\x05\x16\u{a0}\x0a\x16\x03\
	\x16\x02\x03\x1c\x17\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\
	\x1c\x1e\x20\x22\x24\x26\x28\x2a\x02\x08\x03\x02\x03\x09\x03\x02\x0a\x17\
	\x03\x02\x18\x22\x03\x02\x42\x43\x04\x02\x25\x25\x31\x39\x03\x02\x23\x2c\
	\x02\u{a3}\x02\x2d\x03\x02\x02\x02\x04\x34\x03\x02\x02\x02\x06\x3e\x03\x02\
	\x02\x02\x08\x40\x03\x02\x02\x02\x0a\x46\x03\x02\x02\x02\x0c\x4a\x03\x02\
	\x02\x02\x0e\x54\x03\x02\x02\x02\x10\x56\x03\x02\x02\x02\x12\x58\x03\x02\
	\x02\x02\x14\x5e\x03\x02\x02\x02\x16\x60\x03\x02\x02\x02\x18\x6b\x03\x02\
	\x02\x02\x1a\x6d\x03\x02\x02\x02\x1c\x79\x03\x02\x02\x02\x1e\u{86}\x03\x02\
	\x02\x02\x20\u{8a}\x03\x02\x02\x02\x22\u{8f}\x03\x02\x02\x02\x24\u{94}\x03\
	\x02\x02\x02\x26\u{96}\x03\x02\x02\x02\x28\u{99}\x03\x02\x02\x02\x2a\u{9f}\
	\x03\x02\x02\x02\x2c\x2e\x05\x04\x03\x02\x2d\x2c\x03\x02\x02\x02\x2e\x2f\
	\x03\x02\x02\x02\x2f\x2d\x03\x02\x02\x02\x2f\x30\x03\x02\x02\x02\x30\x31\
	\x03\x02\x02\x02\x31\x32\x07\x02\x02\x03\x32\x03\x03\x02\x02\x02\x33\x35\
	\x05\x06\x04\x02\x34\x33\x03\x02\x02\x02\x35\x36\x03\x02\x02\x02\x36\x34\
	\x03\x02\x02\x02\x36\x37\x03\x02\x02\x02\x37\x39\x03\x02\x02\x02\x38\x3a\
	\x07\x3a\x02\x02\x39\x38\x03\x02\x02\x02\x39\x3a\x03\x02\x02\x02\x3a\x05\
	\x03\x02\x02\x02\x3b\x3f\x05\x08\x05\x02\x3c\x3f\x05\x0a\x06\x02\x3d\x3f\
	\x05\x0c\x07\x02\x3e\x3b\x03\x02\x02\x02\x3e\x3c\x03\x02\x02\x02\x3e\x3d\
	\x03\x02\x02\x02\x3f\x07\x03\x02\x02\x02\x40\x41\x05\x0e\x08\x02\x41\x42\
	\x05\x10\x09\x02\x42\x44\x05\x14\x0b\x02\x43\x45\x05\x1a\x0e\x02\x44\x43\
	\x03\x02\x02\x02\x44\x45\x03\x02\x02\x02\x45\x09\x03\x02\x02\x02\x46\x48\
	\x05\x0e\x08\x02\x47\x49\x05\x1a\x0e\x02\x48\x47\x03\x02\x02\x02\x48\x49\
	\x03\x02\x02\x02\x49\x0b\x03\x02\x02\x02\x4a\x4c\x05\x12\x0a\x02\x4b\x4d\
	\x05\x14\x0b\x02\x4c\x4b\x03\x02\x02\x02\x4c\x4d\x03\x02\x02\x02\x4d\x51\
	\x03\x02\x02\x02\x4e\x50\x05\x26\x14\x02\x4f\x4e\x03\x02\x02\x02\x50\x53\
	\x03\x02\x02\x02\x51\x4f\x03\x02\x02\x02\x51\x52\x03\x02\x02\x02\x52\x0d\
	\x03\x02\x02\x02\x53\x51\x03\x02\x02\x02\x54\x55\x09\x02\x02\x02\x55\x0f\
	\x03\x02\x02\x02\x56\x57\x09\x03\x02\x02\x57\x11\x03\x02\x02\x02\x58\x59\
	\x09\x04\x02\x02\x59\x13\x03\x02\x02\x02\x5a\x5f\x07\x2d\x02\x02\x5b\x5f\
	\x05\x16\x0c\x02\x5c\x5f\x05\x0e\x08\x02\x5d\x5f\x07\x43\x02\x02\x5e\x5a\
	\x03\x02\x02\x02\x5e\x5b\x03\x02\x02\x02\x5e\x5c\x03\x02\x02\x02\x5e\x5d\
	\x03\x02\x02\x02\x5f\x15\x03\x02\x02\x02\x60\x61\x07\x3f\x02\x02\x61\x66\
	\x05\x18\x0d\x02\x62\x63\x07\x3b\x02\x02\x63\x65\x05\x18\x0d\x02\x64\x62\
	\x03\x02\x02\x02\x65\x68\x03\x02\x02\x02\x66\x64\x03\x02\x02\x02\x66\x67\
	\x03\x02\x02\x02\x67\x69\x03\x02\x02\x02\x68\x66\x03\x02\x02\x02\x69\x6a\
	\x07\x40\x02\x02\x6a\x17\x03\x02\x02\x02\x6b\x6c\x09\x05\x02\x02\x6c\x19\
	\x03\x02\x02\x02\x6d\x6e\x07\x3d\x02\x02\x6e\x6f\x05\x1c\x0f\x02\x6f\x70\
	\x07\x3e\x02\x02\x70\x1b\x03\x02\x02\x02\x71\x72\x08\x0f\x01\x02\x72\x73\
	\x07\x30\x02\x02\x73\x7a\x05\x1c\x0f\x05\x74\x75\x07\x3d\x02\x02\x75\x76\
	\x05\x1c\x0f\x02\x76\x77\x07\x3e\x02\x02\x77\x7a\x03\x02\x02\x02\x78\x7a\
	\x05\x1e\x10\x02\x79\x71\x03\x02\x02\x02\x79\x74\x03\x02\x02\x02\x79\x78\
	\x03\x02\x02\x02\x7a\u{83}\x03\x02\x02\x02\x7b\x7c\x0c\x07\x02\x02\x7c\x7d\
	\x07\x2e\x02\x02\x7d\u{82}\x05\x1c\x0f\x08\x7e\x7f\x0c\x06\x02\x02\x7f\u{80}\
	\x07\x2f\x02\x02\u{80}\u{82}\x05\x1c\x0f\x07\u{81}\x7b\x03\x02\x02\x02\u{81}\
	\x7e\x03\x02\x02\x02\u{82}\u{85}\x03\x02\x02\x02\u{83}\u{81}\x03\x02\x02\
	\x02\u{83}\u{84}\x03\x02\x02\x02\u{84}\x1d\x03\x02\x02\x02\u{85}\u{83}\x03\
	\x02\x02\x02\u{86}\u{87}\x05\x20\x11\x02\u{87}\u{88}\x05\x22\x12\x02\u{88}\
	\u{89}\x05\x24\x13\x02\u{89}\x1f\x03\x02\x02\x02\u{8a}\u{8d}\x07\x43\x02\
	\x02\u{8b}\u{8c}\x07\x3c\x02\x02\u{8c}\u{8e}\x07\x43\x02\x02\u{8d}\u{8b}\
	\x03\x02\x02\x02\u{8d}\u{8e}\x03\x02\x02\x02\u{8e}\x21\x03\x02\x02\x02\u{8f}\
	\u{90}\x09\x06\x02\x02\u{90}\x23\x03\x02\x02\x02\u{91}\u{95}\x07\x42\x02\
	\x02\u{92}\u{95}\x07\x41\x02\x02\u{93}\u{95}\x05\x16\x0c\x02\u{94}\u{91}\
	\x03\x02\x02\x02\u{94}\u{92}\x03\x02\x02\x02\u{94}\u{93}\x03\x02\x02\x02\
	\u{95}\x25\x03\x02\x02\x02\u{96}\u{97}\x05\x28\x15\x02\u{97}\u{98}\x05\x2a\
	\x16\x02\u{98}\x27\x03\x02\x02\x02\u{99}\u{9a}\x09\x07\x02\x02\u{9a}\x29\
	\x03\x02\x02\x02\u{9b}\u{a0}\x07\x43\x02\x02\u{9c}\u{a0}\x07\x41\x02\x02\
	\u{9d}\u{a0}\x05\x0e\x08\x02\u{9e}\u{a0}\x07\x42\x02\x02\u{9f}\u{9b}\x03\
	\x02\x02\x02\u{9f}\u{9c}\x03\x02\x02\x02\u{9f}\u{9d}\x03\x02\x02\x02\u{9f}\
	\u{9e}\x03\x02\x02\x02\u{a0}\x2b\x03\x02\x02\x02\x12\x2f\x36\x39\x3e\x44\
	\x48\x4c\x51\x5e\x66\x79\u{81}\u{83}\u{8d}\u{94}\u{9f}";

