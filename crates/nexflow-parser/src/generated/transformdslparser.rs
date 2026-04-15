// Generated from /Users/chandramohn/workspace/nexcore/grammar/nexflow/TransformDSL.g4 by ANTLR 4.8
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
use super::transformdsllistener::*;
use super::transformdslvisitor::*;

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

		pub const T__0:isize=1; 
		pub const T__1:isize=2; 
		pub const T__2:isize=3; 
		pub const T__3:isize=4; 
		pub const T__4:isize=5; 
		pub const T__5:isize=6; 
		pub const T__6:isize=7; 
		pub const T__7:isize=8; 
		pub const T__8:isize=9; 
		pub const T__9:isize=10; 
		pub const T__10:isize=11; 
		pub const T__11:isize=12; 
		pub const T__12:isize=13; 
		pub const T__13:isize=14; 
		pub const T__14:isize=15; 
		pub const T__15:isize=16; 
		pub const T__16:isize=17; 
		pub const T__17:isize=18; 
		pub const T__18:isize=19; 
		pub const T__19:isize=20; 
		pub const T__20:isize=21; 
		pub const T__21:isize=22; 
		pub const T__22:isize=23; 
		pub const T__23:isize=24; 
		pub const T__24:isize=25; 
		pub const T__25:isize=26; 
		pub const T__26:isize=27; 
		pub const T__27:isize=28; 
		pub const T__28:isize=29; 
		pub const T__29:isize=30; 
		pub const T__30:isize=31; 
		pub const T__31:isize=32; 
		pub const T__32:isize=33; 
		pub const T__33:isize=34; 
		pub const T__34:isize=35; 
		pub const T__35:isize=36; 
		pub const T__36:isize=37; 
		pub const T__37:isize=38; 
		pub const T__38:isize=39; 
		pub const T__39:isize=40; 
		pub const T__40:isize=41; 
		pub const T__41:isize=42; 
		pub const T__42:isize=43; 
		pub const T__43:isize=44; 
		pub const T__44:isize=45; 
		pub const T__45:isize=46; 
		pub const T__46:isize=47; 
		pub const T__47:isize=48; 
		pub const T__48:isize=49; 
		pub const T__49:isize=50; 
		pub const T__50:isize=51; 
		pub const T__51:isize=52; 
		pub const T__52:isize=53; 
		pub const T__53:isize=54; 
		pub const T__54:isize=55; 
		pub const T__55:isize=56; 
		pub const T__56:isize=57; 
		pub const T__57:isize=58; 
		pub const T__58:isize=59; 
		pub const T__59:isize=60; 
		pub const T__60:isize=61; 
		pub const T__61:isize=62; 
		pub const T__62:isize=63; 
		pub const T__63:isize=64; 
		pub const T__64:isize=65; 
		pub const T__65:isize=66; 
		pub const T__66:isize=67; 
		pub const T__67:isize=68; 
		pub const T__68:isize=69; 
		pub const T__69:isize=70; 
		pub const T__70:isize=71; 
		pub const T__71:isize=72; 
		pub const T__72:isize=73; 
		pub const T__73:isize=74; 
		pub const T__74:isize=75; 
		pub const T__75:isize=76; 
		pub const T__76:isize=77; 
		pub const T__77:isize=78; 
		pub const T__78:isize=79; 
		pub const T__79:isize=80; 
		pub const T__80:isize=81; 
		pub const T__81:isize=82; 
		pub const T__82:isize=83; 
		pub const T__83:isize=84; 
		pub const T__84:isize=85; 
		pub const T__85:isize=86; 
		pub const T__86:isize=87; 
		pub const T__87:isize=88; 
		pub const T__88:isize=89; 
		pub const T__89:isize=90; 
		pub const T__90:isize=91; 
		pub const T__91:isize=92; 
		pub const T__92:isize=93; 
		pub const T__93:isize=94; 
		pub const T__94:isize=95; 
		pub const T__95:isize=96; 
		pub const T__96:isize=97; 
		pub const T__97:isize=98; 
		pub const T__98:isize=99; 
		pub const T__99:isize=100; 
		pub const T__100:isize=101; 
		pub const T__101:isize=102; 
		pub const T__102:isize=103; 
		pub const T__103:isize=104; 
		pub const T__104:isize=105; 
		pub const T__105:isize=106; 
		pub const T__106:isize=107; 
		pub const T__107:isize=108; 
		pub const T__108:isize=109; 
		pub const T__109:isize=110; 
		pub const T__110:isize=111; 
		pub const T__111:isize=112; 
		pub const T__112:isize=113; 
		pub const T__113:isize=114; 
		pub const T__114:isize=115; 
		pub const T__115:isize=116; 
		pub const T__116:isize=117; 
		pub const T__117:isize=118; 
		pub const T__118:isize=119; 
		pub const T__119:isize=120; 
		pub const T__120:isize=121; 
		pub const T__121:isize=122; 
		pub const VERSION_NUMBER:isize=123; 
		pub const INTEGER:isize=124; 
		pub const DECIMAL:isize=125; 
		pub const DURATION_LITERAL:isize=126; 
		pub const BOOLEAN:isize=127; 
		pub const IMPORT:isize=128; 
		pub const DEFAULT_KW:isize=129; 
		pub const IDENTIFIER:isize=130; 
		pub const UPPER_IDENTIFIER:isize=131; 
		pub const STRING:isize=132; 
		pub const COLON:isize=133; 
		pub const COMMA:isize=134; 
		pub const DOTDOT:isize=135; 
		pub const DOT:isize=136; 
		pub const LBRACKET:isize=137; 
		pub const RBRACKET:isize=138; 
		pub const LPAREN:isize=139; 
		pub const RPAREN:isize=140; 
		pub const LBRACE:isize=141; 
		pub const RBRACE:isize=142; 
		pub const LANGLE:isize=143; 
		pub const RANGLE:isize=144; 
		pub const EQ:isize=145; 
		pub const NE:isize=146; 
		pub const LE:isize=147; 
		pub const GE:isize=148; 
		pub const NULLSAFE_EQ:isize=149; 
		pub const PLUS:isize=150; 
		pub const MINUS:isize=151; 
		pub const STAR:isize=152; 
		pub const SLASH:isize=153; 
		pub const PERCENT:isize=154; 
		pub const ARROW:isize=155; 
		pub const OPTIONAL_CHAIN:isize=156; 
		pub const COALESCE:isize=157; 
		pub const COMMENT:isize=158; 
		pub const BLOCK_COMMENT:isize=159; 
		pub const WS:isize=160;
	pub const RULE_program:usize = 0; 
	pub const RULE_importStatement:usize = 1; 
	pub const RULE_importPath:usize = 2; 
	pub const RULE_importPathSegment:usize = 3; 
	pub const RULE_importFileExtension:usize = 4; 
	pub const RULE_transformDef:usize = 5; 
	pub const RULE_idempotentDecl:usize = 6; 
	pub const RULE_lookupDecl:usize = 7; 
	pub const RULE_lookupsBlock:usize = 8; 
	pub const RULE_lookupFieldDecl:usize = 9; 
	pub const RULE_stateDecl:usize = 10; 
	pub const RULE_paramsBlock:usize = 11; 
	pub const RULE_paramDecl:usize = 12; 
	pub const RULE_paramQualifiers:usize = 13; 
	pub const RULE_paramDefault:usize = 14; 
	pub const RULE_transformName:usize = 15; 
	pub const RULE_transformMetadata:usize = 16; 
	pub const RULE_versionDecl:usize = 17; 
	pub const RULE_descriptionDecl:usize = 18; 
	pub const RULE_previousVersionDecl:usize = 19; 
	pub const RULE_compatibilityDecl:usize = 20; 
	pub const RULE_compatibilityMode:usize = 21; 
	pub const RULE_purityDecl:usize = 22; 
	pub const RULE_cacheDecl:usize = 23; 
	pub const RULE_cacheTtl:usize = 24; 
	pub const RULE_cacheKey:usize = 25; 
	pub const RULE_transformBlockDef:usize = 26; 
	pub const RULE_useBlock:usize = 27; 
	pub const RULE_inputSpec:usize = 28; 
	pub const RULE_inputFieldDecl:usize = 29; 
	pub const RULE_outputSpec:usize = 30; 
	pub const RULE_outputFieldDecl:usize = 31; 
	pub const RULE_fieldType:usize = 32; 
	pub const RULE_baseType:usize = 33; 
	pub const RULE_collectionType:usize = 34; 
	pub const RULE_constraint:usize = 35; 
	pub const RULE_constraintSpec:usize = 36; 
	pub const RULE_rangeSpec:usize = 37; 
	pub const RULE_lengthSpec:usize = 38; 
	pub const RULE_valueList:usize = 39; 
	pub const RULE_qualifiers:usize = 40; 
	pub const RULE_qualifier:usize = 41; 
	pub const RULE_applyBlock:usize = 42; 
	pub const RULE_statement:usize = 43; 
	pub const RULE_assignment:usize = 44; 
	pub const RULE_letAssignment:usize = 45; 
	pub const RULE_mappingsBlock:usize = 46; 
	pub const RULE_mapping:usize = 47; 
	pub const RULE_composeBlock:usize = 48; 
	pub const RULE_composeType:usize = 49; 
	pub const RULE_composeRef:usize = 50; 
	pub const RULE_thenBlock:usize = 51; 
	pub const RULE_validateInputBlock:usize = 52; 
	pub const RULE_validateOutputBlock:usize = 53; 
	pub const RULE_validationRule:usize = 54; 
	pub const RULE_validationMessage:usize = 55; 
	pub const RULE_validationMessageObject:usize = 56; 
	pub const RULE_severityLevel:usize = 57; 
	pub const RULE_invariantBlock:usize = 58; 
	pub const RULE_onErrorBlock:usize = 59; 
	pub const RULE_errorStatement:usize = 60; 
	pub const RULE_errorAction:usize = 61; 
	pub const RULE_logErrorCall:usize = 62; 
	pub const RULE_emitStatement:usize = 63; 
	pub const RULE_emitMode:usize = 64; 
	pub const RULE_rejectStatement:usize = 65; 
	pub const RULE_rejectArg:usize = 66; 
	pub const RULE_errorActionType:usize = 67; 
	pub const RULE_logLevel:usize = 68; 
	pub const RULE_onInvalidBlock:usize = 69; 
	pub const RULE_invalidAction:usize = 70; 
	pub const RULE_onChangeBlock:usize = 71; 
	pub const RULE_recalculateBlock:usize = 72; 
	pub const RULE_expression:usize = 73; 
	pub const RULE_primaryExpression:usize = 74; 
	pub const RULE_objectLiteral:usize = 75; 
	pub const RULE_objectField:usize = 76; 
	pub const RULE_objectFieldName:usize = 77; 
	pub const RULE_lambdaExpression:usize = 78; 
	pub const RULE_listElements:usize = 79; 
	pub const RULE_whenExpression:usize = 80; 
	pub const RULE_indexExpression:usize = 81; 
	pub const RULE_optionalChainExpression:usize = 82; 
	pub const RULE_binaryOp:usize = 83; 
	pub const RULE_arithmeticOp:usize = 84; 
	pub const RULE_comparisonOp:usize = 85; 
	pub const RULE_logicalOp:usize = 86; 
	pub const RULE_unaryOp:usize = 87; 
	pub const RULE_functionCall:usize = 88; 
	pub const RULE_functionName:usize = 89; 
	pub const RULE_listLiteral:usize = 90; 
	pub const RULE_fieldPath:usize = 91; 
	pub const RULE_fieldOrKeyword:usize = 92; 
	pub const RULE_fieldArray:usize = 93; 
	pub const RULE_fieldName:usize = 94; 
	pub const RULE_duration:usize = 95; 
	pub const RULE_timeUnit:usize = 96; 
	pub const RULE_literal:usize = 97; 
	pub const RULE_numberLiteral:usize = 98;
	pub const ruleNames: [&'static str; 99] =  [
		"program", "importStatement", "importPath", "importPathSegment", "importFileExtension", 
		"transformDef", "idempotentDecl", "lookupDecl", "lookupsBlock", "lookupFieldDecl", 
		"stateDecl", "paramsBlock", "paramDecl", "paramQualifiers", "paramDefault", 
		"transformName", "transformMetadata", "versionDecl", "descriptionDecl", 
		"previousVersionDecl", "compatibilityDecl", "compatibilityMode", "purityDecl", 
		"cacheDecl", "cacheTtl", "cacheKey", "transformBlockDef", "useBlock", 
		"inputSpec", "inputFieldDecl", "outputSpec", "outputFieldDecl", "fieldType", 
		"baseType", "collectionType", "constraint", "constraintSpec", "rangeSpec", 
		"lengthSpec", "valueList", "qualifiers", "qualifier", "applyBlock", "statement", 
		"assignment", "letAssignment", "mappingsBlock", "mapping", "composeBlock", 
		"composeType", "composeRef", "thenBlock", "validateInputBlock", "validateOutputBlock", 
		"validationRule", "validationMessage", "validationMessageObject", "severityLevel", 
		"invariantBlock", "onErrorBlock", "errorStatement", "errorAction", "logErrorCall", 
		"emitStatement", "emitMode", "rejectStatement", "rejectArg", "errorActionType", 
		"logLevel", "onInvalidBlock", "invalidAction", "onChangeBlock", "recalculateBlock", 
		"expression", "primaryExpression", "objectLiteral", "objectField", "objectFieldName", 
		"lambdaExpression", "listElements", "whenExpression", "indexExpression", 
		"optionalChainExpression", "binaryOp", "arithmeticOp", "comparisonOp", 
		"logicalOp", "unaryOp", "functionCall", "functionName", "listLiteral", 
		"fieldPath", "fieldOrKeyword", "fieldArray", "fieldName", "duration", 
		"timeUnit", "literal", "numberLiteral"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;158] = [
		None, Some("'schema'"), Some("'transform'"), Some("'flow'"), Some("'rules'"), 
		Some("'end'"), Some("'idempotent'"), Some("'lookup'"), Some("'lookups'"), 
		Some("'state'"), Some("'params'"), Some("'required'"), Some("'optional'"), 
		Some("'version'"), Some("'description'"), Some("'previous_version'"), 
		Some("'compatibility'"), Some("'backward'"), Some("'forward'"), Some("'full'"), 
		Some("'none'"), Some("'pure'"), Some("'cache'"), Some("'ttl'"), Some("'key'"), 
		Some("'transform_block'"), Some("'use'"), Some("'input'"), Some("'output'"), 
		Some("'string'"), Some("'integer'"), Some("'decimal'"), Some("'boolean'"), 
		Some("'date'"), Some("'timestamp'"), Some("'uuid'"), Some("'bytes'"), 
		Some("'list'"), Some("'set'"), Some("'map'"), Some("'range'"), Some("'length'"), 
		Some("'pattern'"), Some("'values'"), Some("'precision'"), Some("'scale'"), 
		Some("'nullable'"), Some("'apply'"), Some("'='"), Some("'let'"), Some("'mappings'"), 
		Some("'compose'"), Some("'sequential'"), Some("'parallel'"), Some("'conditional'"), 
		Some("'when'"), Some("'otherwise'"), Some("'then'"), Some("'validate_input'"), 
		Some("'validate_output'"), Some("'require'"), Some("'else'"), Some("'message'"), 
		Some("'code'"), Some("'severity'"), Some("'error'"), Some("'warning'"), 
		Some("'info'"), Some("'invariant'"), Some("'on_error'"), Some("'action'"), 
		Some("'log_level'"), Some("'emit_to'"), Some("'error_code'"), Some("'log_error'"), 
		Some("'emit'"), Some("'with'"), Some("'defaults'"), Some("'partial'"), 
		Some("'data'"), Some("'reject'"), Some("'skip'"), Some("'use_default'"), 
		Some("'raise'"), Some("'debug'"), Some("'on_invalid'"), Some("'error_message'"), 
		Some("'emit_all_errors'"), Some("'on_change'"), Some("'recalculate'"), 
		Some("'between'"), Some("'and'"), Some("'not'"), Some("'in'"), Some("'is'"), 
		Some("'null'"), Some("'matches'"), Some("'or'"), Some("'any'"), Some("'all'"), 
		Some("'sum'"), Some("'count'"), Some("'avg'"), Some("'max'"), Some("'min'"), 
		Some("'filter'"), Some("'find'"), Some("'distinct'"), Some("'current_business_date'"), 
		Some("'previous_business_date'"), Some("'next_business_date'"), Some("'add_business_days'"), 
		Some("'is_business_day'"), Some("'is_holiday'"), Some("'business_days_between'"), 
		Some("'seconds'"), Some("'second'"), Some("'minutes'"), Some("'minute'"), 
		Some("'hours'"), Some("'hour'"), Some("'days'"), Some("'day'"), None, 
		None, None, None, None, Some("'import'"), Some("'default'"), None, None, 
		None, Some("':'"), Some("','"), Some("'..'"), Some("'.'"), Some("'['"), 
		Some("']'"), Some("'('"), Some("')'"), Some("'{'"), Some("'}'"), Some("'<'"), 
		Some("'>'"), None, Some("'!='"), Some("'<='"), Some("'>='"), Some("'=?'"), 
		Some("'+'"), Some("'-'"), Some("'*'"), Some("'/'"), Some("'%'"), Some("'->'"), 
		Some("'?.'"), Some("'??'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;161]  = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, Some("VERSION_NUMBER"), Some("INTEGER"), Some("DECIMAL"), 
		Some("DURATION_LITERAL"), Some("BOOLEAN"), Some("IMPORT"), Some("DEFAULT_KW"), 
		Some("IDENTIFIER"), Some("UPPER_IDENTIFIER"), Some("STRING"), Some("COLON"), 
		Some("COMMA"), Some("DOTDOT"), Some("DOT"), Some("LBRACKET"), Some("RBRACKET"), 
		Some("LPAREN"), Some("RPAREN"), Some("LBRACE"), Some("RBRACE"), Some("LANGLE"), 
		Some("RANGLE"), Some("EQ"), Some("NE"), Some("LE"), Some("GE"), Some("NULLSAFE_EQ"), 
		Some("PLUS"), Some("MINUS"), Some("STAR"), Some("SLASH"), Some("PERCENT"), 
		Some("ARROW"), Some("OPTIONAL_CHAIN"), Some("COALESCE"), Some("COMMENT"), 
		Some("BLOCK_COMMENT"), Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,TransformDSLParserExt<'input>, I, TransformDSLParserContextType , dyn TransformDSLListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type TransformDSLTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, TransformDSLParserContextType , dyn TransformDSLListener<'input> + 'a>;

/// Parser for TransformDSL grammar
pub struct TransformDSLParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> TransformDSLParser<'input, I, H>
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
				TransformDSLParserExt{
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

impl<'input, I> TransformDSLParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> TransformDSLParser<'input, I, DefaultErrorStrategy<'input,TransformDSLParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for TransformDSLParser
pub trait TransformDSLParserContext<'input>:
	for<'x> Listenable<dyn TransformDSLListener<'input> + 'x > + 
	for<'x> Visitable<dyn TransformDSLVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=TransformDSLParserContextType>
{}

antlr_rust::coerce_from!{ 'input : TransformDSLParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn TransformDSLParserContext<'input> + 'input
where
    T: TransformDSLVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn TransformDSLVisitor<'input> + 'x))
    }
}

impl<'input> TransformDSLParserContext<'input> for TerminalNode<'input,TransformDSLParserContextType> {}
impl<'input> TransformDSLParserContext<'input> for ErrorNode<'input,TransformDSLParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn TransformDSLParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn TransformDSLListener<'input> + 'input }

pub struct TransformDSLParserContextType;
antlr_rust::tid!{TransformDSLParserContextType}

impl<'input> ParserNodeType<'input> for TransformDSLParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn TransformDSLParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct TransformDSLParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> TransformDSLParserExt<'input>{
}
antlr_rust::tid! { TransformDSLParserExt<'a> }

impl<'input> TokenAware<'input> for TransformDSLParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for TransformDSLParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for TransformDSLParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "TransformDSL.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn TransformDSLParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					73 => TransformDSLParser::<'input,I,_>::expression_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> TransformDSLParser<'input, I, DefaultErrorStrategy<'input,TransformDSLParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn expression_sempred(_localctx: Option<&ExpressionContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 13)
				}
				1=>{
					recog.precpred(None, 12)
				}
				2=>{
					recog.precpred(None, 11)
				}
				3=>{
					recog.precpred(None, 10)
				}
				4=>{
					recog.precpred(None, 9)
				}
				5=>{
					recog.precpred(None, 8)
				}
				6=>{
					recog.precpred(None, 7)
				}
				7=>{
					recog.precpred(None, 6)
				}
				8=>{
					recog.precpred(None, 5)
				}
				9=>{
					recog.precpred(None, 4)
				}
				10=>{
					recog.precpred(None, 3)
				}
				11=>{
					recog.precpred(None, 2)
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

impl<'input> TransformDSLParserContext<'input> for ProgramContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ProgramContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_program(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_program(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ProgramContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_program(self);
	}
}

impl<'input> CustomRuleContext<'input> for ProgramContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_program }
	//fn type_rule_index() -> usize where Self: Sized { RULE_program }
}
antlr_rust::tid!{ProgramContextExt<'a>}

impl<'input> ProgramContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProgramContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProgramContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ProgramContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ProgramContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn importStatement_all(&self) ->  Vec<Rc<ImportStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn importStatement(&self, i: usize) -> Option<Rc<ImportStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn transformDef_all(&self) ->  Vec<Rc<TransformDefContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn transformDef(&self, i: usize) -> Option<Rc<TransformDefContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn transformBlockDef_all(&self) ->  Vec<Rc<TransformBlockDefContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn transformBlockDef(&self, i: usize) -> Option<Rc<TransformBlockDefContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ProgramContextAttrs<'input> for ProgramContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
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
			recog.base.set_state(201);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==IMPORT {
				{
				{
				/*InvokeRule importStatement*/
				recog.base.set_state(198);
				recog.importStatement()?;

				}
				}
				recog.base.set_state(203);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(206); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				recog.base.set_state(206);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.base.input.la(1) {
				 T__1 
					=> {
						{
						/*InvokeRule transformDef*/
						recog.base.set_state(204);
						recog.transformDef()?;

						}
					}

				 T__24 
					=> {
						{
						/*InvokeRule transformBlockDef*/
						recog.base.set_state(205);
						recog.transformBlockDef()?;

						}
					}

					_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				}
				recog.base.set_state(208); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==T__1 || _la==T__24) {break}
			}
			recog.base.set_state(210);
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

impl<'input> TransformDSLParserContext<'input> for ImportStatementContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ImportStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_importStatement(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_importStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ImportStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_importStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importStatement }
}
antlr_rust::tid!{ImportStatementContextExt<'a>}

impl<'input> ImportStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportStatementContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ImportStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IMPORT
/// Returns `None` if there is no child corresponding to token IMPORT
fn IMPORT(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(IMPORT, 0)
}
fn importPath(&self) -> Option<Rc<ImportPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ImportStatementContextAttrs<'input> for ImportStatementContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
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
			recog.base.set_state(212);
			recog.base.match_token(IMPORT,&mut recog.err_handler)?;

			/*InvokeRule importPath*/
			recog.base.set_state(213);
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

impl<'input> TransformDSLParserContext<'input> for ImportPathContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ImportPathContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_importPath(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_importPath(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ImportPathContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_importPath(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportPathContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importPath }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importPath }
}
antlr_rust::tid!{ImportPathContextExt<'a>}

impl<'input> ImportPathContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportPathContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportPathContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportPathContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ImportPathContextExt<'input>>{

fn importFileExtension(&self) -> Option<Rc<ImportFileExtensionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn importPathSegment_all(&self) ->  Vec<Rc<ImportPathSegmentContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn importPathSegment(&self, i: usize) -> Option<Rc<ImportPathSegmentContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ImportPathContextAttrs<'input> for ImportPathContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
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
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(216); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					{
					/*InvokeRule importPathSegment*/
					recog.base.set_state(215);
					recog.importPathSegment()?;

					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(218); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(3,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
			}
			/*InvokeRule importFileExtension*/
			recog.base.set_state(220);
			recog.importFileExtension()?;

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
//------------------- importPathSegment ----------------
pub type ImportPathSegmentContextAll<'input> = ImportPathSegmentContext<'input>;


pub type ImportPathSegmentContext<'input> = BaseParserRuleContext<'input,ImportPathSegmentContextExt<'input>>;

#[derive(Clone)]
pub struct ImportPathSegmentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ImportPathSegmentContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ImportPathSegmentContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_importPathSegment(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_importPathSegment(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ImportPathSegmentContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_importPathSegment(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportPathSegmentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importPathSegment }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importPathSegment }
}
antlr_rust::tid!{ImportPathSegmentContextExt<'a>}

impl<'input> ImportPathSegmentContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportPathSegmentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportPathSegmentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportPathSegmentContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ImportPathSegmentContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DOTDOT
/// Returns `None` if there is no child corresponding to token DOTDOT
fn DOTDOT(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(DOTDOT, 0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}
/// Retrieves first TerminalNode corresponding to token SLASH
/// Returns `None` if there is no child corresponding to token SLASH
fn SLASH(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(SLASH, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token UPPER_IDENTIFIER
/// Returns `None` if there is no child corresponding to token UPPER_IDENTIFIER
fn UPPER_IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(UPPER_IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}

}

impl<'input> ImportPathSegmentContextAttrs<'input> for ImportPathSegmentContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn importPathSegment(&mut self,)
	-> Result<Rc<ImportPathSegmentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ImportPathSegmentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_importPathSegment);
        let mut _localctx: Rc<ImportPathSegmentContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(222);
			_la = recog.base.input.la(1);
			if { !(((((_la - 130)) & !0x3f) == 0 && ((1usize << (_la - 130)) & ((1usize << (IDENTIFIER - 130)) | (1usize << (UPPER_IDENTIFIER - 130)) | (1usize << (DOTDOT - 130)) | (1usize << (DOT - 130)) | (1usize << (MINUS - 130)) | (1usize << (SLASH - 130)))) != 0)) } {
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
//------------------- importFileExtension ----------------
pub type ImportFileExtensionContextAll<'input> = ImportFileExtensionContext<'input>;


pub type ImportFileExtensionContext<'input> = BaseParserRuleContext<'input,ImportFileExtensionContextExt<'input>>;

#[derive(Clone)]
pub struct ImportFileExtensionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ImportFileExtensionContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ImportFileExtensionContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_importFileExtension(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_importFileExtension(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ImportFileExtensionContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_importFileExtension(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportFileExtensionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importFileExtension }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importFileExtension }
}
antlr_rust::tid!{ImportFileExtensionContextExt<'a>}

impl<'input> ImportFileExtensionContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportFileExtensionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportFileExtensionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportFileExtensionContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ImportFileExtensionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}

}

impl<'input> ImportFileExtensionContextAttrs<'input> for ImportFileExtensionContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn importFileExtension(&mut self,)
	-> Result<Rc<ImportFileExtensionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ImportFileExtensionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_importFileExtension);
        let mut _localctx: Rc<ImportFileExtensionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(224);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			recog.base.set_state(225);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__0) | (1usize << T__1) | (1usize << T__2) | (1usize << T__3))) != 0)) } {
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
//------------------- transformDef ----------------
pub type TransformDefContextAll<'input> = TransformDefContext<'input>;


pub type TransformDefContext<'input> = BaseParserRuleContext<'input,TransformDefContextExt<'input>>;

#[derive(Clone)]
pub struct TransformDefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for TransformDefContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for TransformDefContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_transformDef(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_transformDef(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for TransformDefContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_transformDef(self);
	}
}

impl<'input> CustomRuleContext<'input> for TransformDefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_transformDef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_transformDef }
}
antlr_rust::tid!{TransformDefContextExt<'a>}

impl<'input> TransformDefContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TransformDefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TransformDefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TransformDefContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<TransformDefContextExt<'input>>{

fn transformName(&self) -> Option<Rc<TransformNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn inputSpec(&self) -> Option<Rc<InputSpecContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn outputSpec(&self) -> Option<Rc<OutputSpecContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn applyBlock(&self) -> Option<Rc<ApplyBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn transformMetadata(&self) -> Option<Rc<TransformMetadataContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn purityDecl(&self) -> Option<Rc<PurityDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn idempotentDecl(&self) -> Option<Rc<IdempotentDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn cacheDecl(&self) -> Option<Rc<CacheDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lookupDecl(&self) -> Option<Rc<LookupDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lookupsBlock(&self) -> Option<Rc<LookupsBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stateDecl(&self) -> Option<Rc<StateDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn paramsBlock(&self) -> Option<Rc<ParamsBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn validateInputBlock(&self) -> Option<Rc<ValidateInputBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn validateOutputBlock(&self) -> Option<Rc<ValidateOutputBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn onErrorBlock(&self) -> Option<Rc<OnErrorBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TransformDefContextAttrs<'input> for TransformDefContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn transformDef(&mut self,)
	-> Result<Rc<TransformDefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TransformDefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_transformDef);
        let mut _localctx: Rc<TransformDefContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(227);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule transformName*/
			recog.base.set_state(228);
			recog.transformName()?;

			recog.base.set_state(230);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(4,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule transformMetadata*/
					recog.base.set_state(229);
					recog.transformMetadata()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(233);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__20 {
				{
				/*InvokeRule purityDecl*/
				recog.base.set_state(232);
				recog.purityDecl()?;

				}
			}

			recog.base.set_state(236);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__5 {
				{
				/*InvokeRule idempotentDecl*/
				recog.base.set_state(235);
				recog.idempotentDecl()?;

				}
			}

			recog.base.set_state(239);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__21 {
				{
				/*InvokeRule cacheDecl*/
				recog.base.set_state(238);
				recog.cacheDecl()?;

				}
			}

			/*InvokeRule inputSpec*/
			recog.base.set_state(241);
			recog.inputSpec()?;

			recog.base.set_state(243);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__6 {
				{
				/*InvokeRule lookupDecl*/
				recog.base.set_state(242);
				recog.lookupDecl()?;

				}
			}

			recog.base.set_state(246);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__7 {
				{
				/*InvokeRule lookupsBlock*/
				recog.base.set_state(245);
				recog.lookupsBlock()?;

				}
			}

			recog.base.set_state(249);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__8 {
				{
				/*InvokeRule stateDecl*/
				recog.base.set_state(248);
				recog.stateDecl()?;

				}
			}

			recog.base.set_state(252);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__9 {
				{
				/*InvokeRule paramsBlock*/
				recog.base.set_state(251);
				recog.paramsBlock()?;

				}
			}

			/*InvokeRule outputSpec*/
			recog.base.set_state(254);
			recog.outputSpec()?;

			recog.base.set_state(256);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__57 {
				{
				/*InvokeRule validateInputBlock*/
				recog.base.set_state(255);
				recog.validateInputBlock()?;

				}
			}

			/*InvokeRule applyBlock*/
			recog.base.set_state(258);
			recog.applyBlock()?;

			recog.base.set_state(260);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__58 {
				{
				/*InvokeRule validateOutputBlock*/
				recog.base.set_state(259);
				recog.validateOutputBlock()?;

				}
			}

			recog.base.set_state(263);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__68 {
				{
				/*InvokeRule onErrorBlock*/
				recog.base.set_state(262);
				recog.onErrorBlock()?;

				}
			}

			recog.base.set_state(265);
			recog.base.match_token(T__4,&mut recog.err_handler)?;

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

impl<'input> TransformDSLParserContext<'input> for IdempotentDeclContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for IdempotentDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_idempotentDecl(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_idempotentDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for IdempotentDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_idempotentDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdempotentDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_idempotentDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_idempotentDecl }
}
antlr_rust::tid!{IdempotentDeclContextExt<'a>}

impl<'input> IdempotentDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IdempotentDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdempotentDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IdempotentDeclContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<IdempotentDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token BOOLEAN
/// Returns `None` if there is no child corresponding to token BOOLEAN
fn BOOLEAN(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(BOOLEAN, 0)
}

}

impl<'input> IdempotentDeclContextAttrs<'input> for IdempotentDeclContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn idempotentDecl(&mut self,)
	-> Result<Rc<IdempotentDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdempotentDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_idempotentDecl);
        let mut _localctx: Rc<IdempotentDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(267);
			recog.base.match_token(T__5,&mut recog.err_handler)?;

			recog.base.set_state(268);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(269);
			recog.base.match_token(BOOLEAN,&mut recog.err_handler)?;

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
//------------------- lookupDecl ----------------
pub type LookupDeclContextAll<'input> = LookupDeclContext<'input>;


pub type LookupDeclContext<'input> = BaseParserRuleContext<'input,LookupDeclContextExt<'input>>;

#[derive(Clone)]
pub struct LookupDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for LookupDeclContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for LookupDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lookupDecl(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_lookupDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for LookupDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_lookupDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for LookupDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lookupDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lookupDecl }
}
antlr_rust::tid!{LookupDeclContextExt<'a>}

impl<'input> LookupDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LookupDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LookupDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LookupDeclContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<LookupDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> LookupDeclContextAttrs<'input> for LookupDeclContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lookupDecl(&mut self,)
	-> Result<Rc<LookupDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LookupDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_lookupDecl);
        let mut _localctx: Rc<LookupDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(271);
			recog.base.match_token(T__6,&mut recog.err_handler)?;

			recog.base.set_state(272);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(273);
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
//------------------- lookupsBlock ----------------
pub type LookupsBlockContextAll<'input> = LookupsBlockContext<'input>;


pub type LookupsBlockContext<'input> = BaseParserRuleContext<'input,LookupsBlockContextExt<'input>>;

#[derive(Clone)]
pub struct LookupsBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for LookupsBlockContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for LookupsBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lookupsBlock(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_lookupsBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for LookupsBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_lookupsBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for LookupsBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lookupsBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lookupsBlock }
}
antlr_rust::tid!{LookupsBlockContextExt<'a>}

impl<'input> LookupsBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LookupsBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LookupsBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LookupsBlockContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<LookupsBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn lookupFieldDecl_all(&self) ->  Vec<Rc<LookupFieldDeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn lookupFieldDecl(&self, i: usize) -> Option<Rc<LookupFieldDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> LookupsBlockContextAttrs<'input> for LookupsBlockContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lookupsBlock(&mut self,)
	-> Result<Rc<LookupsBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LookupsBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_lookupsBlock);
        let mut _localctx: Rc<LookupsBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(275);
			recog.base.match_token(T__7,&mut recog.err_handler)?;

			recog.base.set_state(276);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(278); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule lookupFieldDecl*/
				recog.base.set_state(277);
				recog.lookupFieldDecl()?;

				}
				}
				recog.base.set_state(280); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==IDENTIFIER) {break}
			}
			recog.base.set_state(283);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__4 {
				{
				recog.base.set_state(282);
				recog.base.match_token(T__4,&mut recog.err_handler)?;

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
//------------------- lookupFieldDecl ----------------
pub type LookupFieldDeclContextAll<'input> = LookupFieldDeclContext<'input>;


pub type LookupFieldDeclContext<'input> = BaseParserRuleContext<'input,LookupFieldDeclContextExt<'input>>;

#[derive(Clone)]
pub struct LookupFieldDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for LookupFieldDeclContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for LookupFieldDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lookupFieldDecl(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_lookupFieldDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for LookupFieldDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_lookupFieldDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for LookupFieldDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lookupFieldDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lookupFieldDecl }
}
antlr_rust::tid!{LookupFieldDeclContextExt<'a>}

impl<'input> LookupFieldDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LookupFieldDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LookupFieldDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LookupFieldDeclContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<LookupFieldDeclContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}

}

impl<'input> LookupFieldDeclContextAttrs<'input> for LookupFieldDeclContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lookupFieldDecl(&mut self,)
	-> Result<Rc<LookupFieldDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LookupFieldDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_lookupFieldDecl);
        let mut _localctx: Rc<LookupFieldDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(285);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(286);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(287);
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
//------------------- stateDecl ----------------
pub type StateDeclContextAll<'input> = StateDeclContext<'input>;


pub type StateDeclContext<'input> = BaseParserRuleContext<'input,StateDeclContextExt<'input>>;

#[derive(Clone)]
pub struct StateDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for StateDeclContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for StateDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stateDecl(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_stateDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for StateDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_stateDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for StateDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stateDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stateDecl }
}
antlr_rust::tid!{StateDeclContextExt<'a>}

impl<'input> StateDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StateDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StateDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StateDeclContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<StateDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> StateDeclContextAttrs<'input> for StateDeclContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stateDecl(&mut self,)
	-> Result<Rc<StateDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StateDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_stateDecl);
        let mut _localctx: Rc<StateDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(289);
			recog.base.match_token(T__8,&mut recog.err_handler)?;

			recog.base.set_state(290);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(291);
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
//------------------- paramsBlock ----------------
pub type ParamsBlockContextAll<'input> = ParamsBlockContext<'input>;


pub type ParamsBlockContext<'input> = BaseParserRuleContext<'input,ParamsBlockContextExt<'input>>;

#[derive(Clone)]
pub struct ParamsBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ParamsBlockContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ParamsBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_paramsBlock(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_paramsBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ParamsBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_paramsBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParamsBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_paramsBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_paramsBlock }
}
antlr_rust::tid!{ParamsBlockContextExt<'a>}

impl<'input> ParamsBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParamsBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParamsBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParamsBlockContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ParamsBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn paramDecl_all(&self) ->  Vec<Rc<ParamDeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn paramDecl(&self, i: usize) -> Option<Rc<ParamDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ParamsBlockContextAttrs<'input> for ParamsBlockContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn paramsBlock(&mut self,)
	-> Result<Rc<ParamsBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParamsBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_paramsBlock);
        let mut _localctx: Rc<ParamsBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(293);
			recog.base.match_token(T__9,&mut recog.err_handler)?;

			recog.base.set_state(294);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(296); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule paramDecl*/
				recog.base.set_state(295);
				recog.paramDecl()?;

				}
				}
				recog.base.set_state(298); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==IDENTIFIER) {break}
			}
			recog.base.set_state(301);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__4 {
				{
				recog.base.set_state(300);
				recog.base.match_token(T__4,&mut recog.err_handler)?;

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
//------------------- paramDecl ----------------
pub type ParamDeclContextAll<'input> = ParamDeclContext<'input>;


pub type ParamDeclContext<'input> = BaseParserRuleContext<'input,ParamDeclContextExt<'input>>;

#[derive(Clone)]
pub struct ParamDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ParamDeclContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ParamDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_paramDecl(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_paramDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ParamDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_paramDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParamDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_paramDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_paramDecl }
}
antlr_rust::tid!{ParamDeclContextExt<'a>}

impl<'input> ParamDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParamDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParamDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParamDeclContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ParamDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn fieldType(&self) -> Option<Rc<FieldTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn paramQualifiers(&self) -> Option<Rc<ParamQualifiersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ParamDeclContextAttrs<'input> for ParamDeclContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn paramDecl(&mut self,)
	-> Result<Rc<ParamDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParamDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_paramDecl);
        let mut _localctx: Rc<ParamDeclContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(303);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(304);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule fieldType*/
			recog.base.set_state(305);
			recog.fieldType()?;

			recog.base.set_state(307);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__10 || _la==T__11 || _la==DEFAULT_KW {
				{
				/*InvokeRule paramQualifiers*/
				recog.base.set_state(306);
				recog.paramQualifiers()?;

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
//------------------- paramQualifiers ----------------
pub type ParamQualifiersContextAll<'input> = ParamQualifiersContext<'input>;


pub type ParamQualifiersContext<'input> = BaseParserRuleContext<'input,ParamQualifiersContextExt<'input>>;

#[derive(Clone)]
pub struct ParamQualifiersContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ParamQualifiersContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ParamQualifiersContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_paramQualifiers(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_paramQualifiers(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ParamQualifiersContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_paramQualifiers(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParamQualifiersContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_paramQualifiers }
	//fn type_rule_index() -> usize where Self: Sized { RULE_paramQualifiers }
}
antlr_rust::tid!{ParamQualifiersContextExt<'a>}

impl<'input> ParamQualifiersContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParamQualifiersContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParamQualifiersContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParamQualifiersContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ParamQualifiersContextExt<'input>>{

fn paramDefault(&self) -> Option<Rc<ParamDefaultContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ParamQualifiersContextAttrs<'input> for ParamQualifiersContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn paramQualifiers(&mut self,)
	-> Result<Rc<ParamQualifiersContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParamQualifiersContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_paramQualifiers);
        let mut _localctx: Rc<ParamQualifiersContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(318);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__10 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(309);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

					recog.base.set_state(311);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==DEFAULT_KW {
						{
						/*InvokeRule paramDefault*/
						recog.base.set_state(310);
						recog.paramDefault()?;

						}
					}

					}
				}

			 T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(313);
					recog.base.match_token(T__11,&mut recog.err_handler)?;

					recog.base.set_state(315);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==DEFAULT_KW {
						{
						/*InvokeRule paramDefault*/
						recog.base.set_state(314);
						recog.paramDefault()?;

						}
					}

					}
				}

			 DEFAULT_KW 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule paramDefault*/
					recog.base.set_state(317);
					recog.paramDefault()?;

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
//------------------- paramDefault ----------------
pub type ParamDefaultContextAll<'input> = ParamDefaultContext<'input>;


pub type ParamDefaultContext<'input> = BaseParserRuleContext<'input,ParamDefaultContextExt<'input>>;

#[derive(Clone)]
pub struct ParamDefaultContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ParamDefaultContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ParamDefaultContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_paramDefault(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_paramDefault(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ParamDefaultContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_paramDefault(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParamDefaultContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_paramDefault }
	//fn type_rule_index() -> usize where Self: Sized { RULE_paramDefault }
}
antlr_rust::tid!{ParamDefaultContextExt<'a>}

impl<'input> ParamDefaultContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParamDefaultContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParamDefaultContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParamDefaultContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ParamDefaultContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DEFAULT_KW
/// Returns `None` if there is no child corresponding to token DEFAULT_KW
fn DEFAULT_KW(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(DEFAULT_KW, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ParamDefaultContextAttrs<'input> for ParamDefaultContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn paramDefault(&mut self,)
	-> Result<Rc<ParamDefaultContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParamDefaultContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_paramDefault);
        let mut _localctx: Rc<ParamDefaultContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(320);
			recog.base.match_token(DEFAULT_KW,&mut recog.err_handler)?;

			recog.base.set_state(321);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(322);
			recog.expression_rec(0)?;

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
//------------------- transformName ----------------
pub type TransformNameContextAll<'input> = TransformNameContext<'input>;


pub type TransformNameContext<'input> = BaseParserRuleContext<'input,TransformNameContextExt<'input>>;

#[derive(Clone)]
pub struct TransformNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for TransformNameContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for TransformNameContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_transformName(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_transformName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for TransformNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_transformName(self);
	}
}

impl<'input> CustomRuleContext<'input> for TransformNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_transformName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_transformName }
}
antlr_rust::tid!{TransformNameContextExt<'a>}

impl<'input> TransformNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TransformNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TransformNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TransformNameContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<TransformNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> TransformNameContextAttrs<'input> for TransformNameContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn transformName(&mut self,)
	-> Result<Rc<TransformNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TransformNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_transformName);
        let mut _localctx: Rc<TransformNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(324);
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
//------------------- transformMetadata ----------------
pub type TransformMetadataContextAll<'input> = TransformMetadataContext<'input>;


pub type TransformMetadataContext<'input> = BaseParserRuleContext<'input,TransformMetadataContextExt<'input>>;

#[derive(Clone)]
pub struct TransformMetadataContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for TransformMetadataContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for TransformMetadataContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_transformMetadata(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_transformMetadata(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for TransformMetadataContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_transformMetadata(self);
	}
}

impl<'input> CustomRuleContext<'input> for TransformMetadataContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_transformMetadata }
	//fn type_rule_index() -> usize where Self: Sized { RULE_transformMetadata }
}
antlr_rust::tid!{TransformMetadataContextExt<'a>}

impl<'input> TransformMetadataContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TransformMetadataContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TransformMetadataContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TransformMetadataContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<TransformMetadataContextExt<'input>>{

fn versionDecl(&self) -> Option<Rc<VersionDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn descriptionDecl(&self) -> Option<Rc<DescriptionDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn previousVersionDecl(&self) -> Option<Rc<PreviousVersionDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn compatibilityDecl(&self) -> Option<Rc<CompatibilityDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TransformMetadataContextAttrs<'input> for TransformMetadataContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn transformMetadata(&mut self,)
	-> Result<Rc<TransformMetadataContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TransformMetadataContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_transformMetadata);
        let mut _localctx: Rc<TransformMetadataContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(327);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__12 {
				{
				/*InvokeRule versionDecl*/
				recog.base.set_state(326);
				recog.versionDecl()?;

				}
			}

			recog.base.set_state(330);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__13 {
				{
				/*InvokeRule descriptionDecl*/
				recog.base.set_state(329);
				recog.descriptionDecl()?;

				}
			}

			recog.base.set_state(333);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__14 {
				{
				/*InvokeRule previousVersionDecl*/
				recog.base.set_state(332);
				recog.previousVersionDecl()?;

				}
			}

			recog.base.set_state(336);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__15 {
				{
				/*InvokeRule compatibilityDecl*/
				recog.base.set_state(335);
				recog.compatibilityDecl()?;

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
//------------------- versionDecl ----------------
pub type VersionDeclContextAll<'input> = VersionDeclContext<'input>;


pub type VersionDeclContext<'input> = BaseParserRuleContext<'input,VersionDeclContextExt<'input>>;

#[derive(Clone)]
pub struct VersionDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for VersionDeclContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for VersionDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_versionDecl(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_versionDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for VersionDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_versionDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for VersionDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_versionDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_versionDecl }
}
antlr_rust::tid!{VersionDeclContextExt<'a>}

impl<'input> VersionDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VersionDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VersionDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VersionDeclContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<VersionDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token VERSION_NUMBER
/// Returns `None` if there is no child corresponding to token VERSION_NUMBER
fn VERSION_NUMBER(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(VERSION_NUMBER, 0)
}

}

impl<'input> VersionDeclContextAttrs<'input> for VersionDeclContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn versionDecl(&mut self,)
	-> Result<Rc<VersionDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VersionDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_versionDecl);
        let mut _localctx: Rc<VersionDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(338);
			recog.base.match_token(T__12,&mut recog.err_handler)?;

			recog.base.set_state(339);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(340);
			recog.base.match_token(VERSION_NUMBER,&mut recog.err_handler)?;

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

impl<'input> TransformDSLParserContext<'input> for DescriptionDeclContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for DescriptionDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_descriptionDecl(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_descriptionDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for DescriptionDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_descriptionDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for DescriptionDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_descriptionDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_descriptionDecl }
}
antlr_rust::tid!{DescriptionDeclContextExt<'a>}

impl<'input> DescriptionDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DescriptionDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DescriptionDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DescriptionDeclContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<DescriptionDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}

}

impl<'input> DescriptionDeclContextAttrs<'input> for DescriptionDeclContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn descriptionDecl(&mut self,)
	-> Result<Rc<DescriptionDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DescriptionDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_descriptionDecl);
        let mut _localctx: Rc<DescriptionDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(342);
			recog.base.match_token(T__13,&mut recog.err_handler)?;

			recog.base.set_state(343);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(344);
			recog.base.match_token(STRING,&mut recog.err_handler)?;

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
//------------------- previousVersionDecl ----------------
pub type PreviousVersionDeclContextAll<'input> = PreviousVersionDeclContext<'input>;


pub type PreviousVersionDeclContext<'input> = BaseParserRuleContext<'input,PreviousVersionDeclContextExt<'input>>;

#[derive(Clone)]
pub struct PreviousVersionDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for PreviousVersionDeclContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for PreviousVersionDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_previousVersionDecl(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_previousVersionDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for PreviousVersionDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_previousVersionDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for PreviousVersionDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_previousVersionDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_previousVersionDecl }
}
antlr_rust::tid!{PreviousVersionDeclContextExt<'a>}

impl<'input> PreviousVersionDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PreviousVersionDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PreviousVersionDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PreviousVersionDeclContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<PreviousVersionDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token VERSION_NUMBER
/// Returns `None` if there is no child corresponding to token VERSION_NUMBER
fn VERSION_NUMBER(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(VERSION_NUMBER, 0)
}

}

impl<'input> PreviousVersionDeclContextAttrs<'input> for PreviousVersionDeclContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn previousVersionDecl(&mut self,)
	-> Result<Rc<PreviousVersionDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PreviousVersionDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_previousVersionDecl);
        let mut _localctx: Rc<PreviousVersionDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(346);
			recog.base.match_token(T__14,&mut recog.err_handler)?;

			recog.base.set_state(347);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(348);
			recog.base.match_token(VERSION_NUMBER,&mut recog.err_handler)?;

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
//------------------- compatibilityDecl ----------------
pub type CompatibilityDeclContextAll<'input> = CompatibilityDeclContext<'input>;


pub type CompatibilityDeclContext<'input> = BaseParserRuleContext<'input,CompatibilityDeclContextExt<'input>>;

#[derive(Clone)]
pub struct CompatibilityDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for CompatibilityDeclContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for CompatibilityDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_compatibilityDecl(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_compatibilityDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for CompatibilityDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_compatibilityDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for CompatibilityDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compatibilityDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compatibilityDecl }
}
antlr_rust::tid!{CompatibilityDeclContextExt<'a>}

impl<'input> CompatibilityDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CompatibilityDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompatibilityDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CompatibilityDeclContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<CompatibilityDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn compatibilityMode(&self) -> Option<Rc<CompatibilityModeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CompatibilityDeclContextAttrs<'input> for CompatibilityDeclContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compatibilityDecl(&mut self,)
	-> Result<Rc<CompatibilityDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CompatibilityDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_compatibilityDecl);
        let mut _localctx: Rc<CompatibilityDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(350);
			recog.base.match_token(T__15,&mut recog.err_handler)?;

			recog.base.set_state(351);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule compatibilityMode*/
			recog.base.set_state(352);
			recog.compatibilityMode()?;

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
//------------------- compatibilityMode ----------------
pub type CompatibilityModeContextAll<'input> = CompatibilityModeContext<'input>;


pub type CompatibilityModeContext<'input> = BaseParserRuleContext<'input,CompatibilityModeContextExt<'input>>;

#[derive(Clone)]
pub struct CompatibilityModeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for CompatibilityModeContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for CompatibilityModeContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_compatibilityMode(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_compatibilityMode(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for CompatibilityModeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_compatibilityMode(self);
	}
}

impl<'input> CustomRuleContext<'input> for CompatibilityModeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compatibilityMode }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compatibilityMode }
}
antlr_rust::tid!{CompatibilityModeContextExt<'a>}

impl<'input> CompatibilityModeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CompatibilityModeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompatibilityModeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CompatibilityModeContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<CompatibilityModeContextExt<'input>>{


}

impl<'input> CompatibilityModeContextAttrs<'input> for CompatibilityModeContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compatibilityMode(&mut self,)
	-> Result<Rc<CompatibilityModeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CompatibilityModeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_compatibilityMode);
        let mut _localctx: Rc<CompatibilityModeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(354);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__16) | (1usize << T__17) | (1usize << T__18) | (1usize << T__19))) != 0)) } {
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
//------------------- purityDecl ----------------
pub type PurityDeclContextAll<'input> = PurityDeclContext<'input>;


pub type PurityDeclContext<'input> = BaseParserRuleContext<'input,PurityDeclContextExt<'input>>;

#[derive(Clone)]
pub struct PurityDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for PurityDeclContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for PurityDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_purityDecl(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_purityDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for PurityDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_purityDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for PurityDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_purityDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_purityDecl }
}
antlr_rust::tid!{PurityDeclContextExt<'a>}

impl<'input> PurityDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PurityDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PurityDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PurityDeclContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<PurityDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token BOOLEAN
/// Returns `None` if there is no child corresponding to token BOOLEAN
fn BOOLEAN(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(BOOLEAN, 0)
}

}

impl<'input> PurityDeclContextAttrs<'input> for PurityDeclContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn purityDecl(&mut self,)
	-> Result<Rc<PurityDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PurityDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_purityDecl);
        let mut _localctx: Rc<PurityDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(356);
			recog.base.match_token(T__20,&mut recog.err_handler)?;

			recog.base.set_state(357);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(358);
			recog.base.match_token(BOOLEAN,&mut recog.err_handler)?;

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

impl<'input> TransformDSLParserContext<'input> for CacheDeclContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for CacheDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_cacheDecl(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_cacheDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for CacheDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_cacheDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for CacheDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cacheDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cacheDecl }
}
antlr_rust::tid!{CacheDeclContextExt<'a>}

impl<'input> CacheDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CacheDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CacheDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CacheDeclContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<CacheDeclContextExt<'input>>{

fn cacheTtl(&self) -> Option<Rc<CacheTtlContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn cacheKey(&self) -> Option<Rc<CacheKeyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn duration(&self) -> Option<Rc<DurationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CacheDeclContextAttrs<'input> for CacheDeclContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn cacheDecl(&mut self,)
	-> Result<Rc<CacheDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CacheDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_cacheDecl);
        let mut _localctx: Rc<CacheDeclContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(373);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(30,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(360);
					recog.base.match_token(T__21,&mut recog.err_handler)?;

					recog.base.set_state(362);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__22 {
						{
						/*InvokeRule cacheTtl*/
						recog.base.set_state(361);
						recog.cacheTtl()?;

						}
					}

					recog.base.set_state(365);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__23 {
						{
						/*InvokeRule cacheKey*/
						recog.base.set_state(364);
						recog.cacheKey()?;

						}
					}

					recog.base.set_state(368);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__4 {
						{
						recog.base.set_state(367);
						recog.base.match_token(T__4,&mut recog.err_handler)?;

						}
					}

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(370);
					recog.base.match_token(T__21,&mut recog.err_handler)?;

					recog.base.set_state(371);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule duration*/
					recog.base.set_state(372);
					recog.duration()?;

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
//------------------- cacheTtl ----------------
pub type CacheTtlContextAll<'input> = CacheTtlContext<'input>;


pub type CacheTtlContext<'input> = BaseParserRuleContext<'input,CacheTtlContextExt<'input>>;

#[derive(Clone)]
pub struct CacheTtlContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for CacheTtlContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for CacheTtlContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_cacheTtl(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_cacheTtl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for CacheTtlContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_cacheTtl(self);
	}
}

impl<'input> CustomRuleContext<'input> for CacheTtlContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cacheTtl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cacheTtl }
}
antlr_rust::tid!{CacheTtlContextExt<'a>}

impl<'input> CacheTtlContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CacheTtlContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CacheTtlContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CacheTtlContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<CacheTtlContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn duration(&self) -> Option<Rc<DurationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CacheTtlContextAttrs<'input> for CacheTtlContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn cacheTtl(&mut self,)
	-> Result<Rc<CacheTtlContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CacheTtlContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_cacheTtl);
        let mut _localctx: Rc<CacheTtlContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(375);
			recog.base.match_token(T__22,&mut recog.err_handler)?;

			recog.base.set_state(376);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule duration*/
			recog.base.set_state(377);
			recog.duration()?;

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
//------------------- cacheKey ----------------
pub type CacheKeyContextAll<'input> = CacheKeyContext<'input>;


pub type CacheKeyContext<'input> = BaseParserRuleContext<'input,CacheKeyContextExt<'input>>;

#[derive(Clone)]
pub struct CacheKeyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for CacheKeyContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for CacheKeyContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_cacheKey(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_cacheKey(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for CacheKeyContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_cacheKey(self);
	}
}

impl<'input> CustomRuleContext<'input> for CacheKeyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cacheKey }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cacheKey }
}
antlr_rust::tid!{CacheKeyContextExt<'a>}

impl<'input> CacheKeyContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CacheKeyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CacheKeyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CacheKeyContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<CacheKeyContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn fieldArray(&self) -> Option<Rc<FieldArrayContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CacheKeyContextAttrs<'input> for CacheKeyContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn cacheKey(&mut self,)
	-> Result<Rc<CacheKeyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CacheKeyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_cacheKey);
        let mut _localctx: Rc<CacheKeyContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(379);
			recog.base.match_token(T__23,&mut recog.err_handler)?;

			recog.base.set_state(380);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule fieldArray*/
			recog.base.set_state(381);
			recog.fieldArray()?;

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
//------------------- transformBlockDef ----------------
pub type TransformBlockDefContextAll<'input> = TransformBlockDefContext<'input>;


pub type TransformBlockDefContext<'input> = BaseParserRuleContext<'input,TransformBlockDefContextExt<'input>>;

#[derive(Clone)]
pub struct TransformBlockDefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for TransformBlockDefContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for TransformBlockDefContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_transformBlockDef(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_transformBlockDef(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for TransformBlockDefContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_transformBlockDef(self);
	}
}

impl<'input> CustomRuleContext<'input> for TransformBlockDefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_transformBlockDef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_transformBlockDef }
}
antlr_rust::tid!{TransformBlockDefContextExt<'a>}

impl<'input> TransformBlockDefContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TransformBlockDefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TransformBlockDefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TransformBlockDefContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<TransformBlockDefContextExt<'input>>{

fn transformName(&self) -> Option<Rc<TransformNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn inputSpec(&self) -> Option<Rc<InputSpecContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn outputSpec(&self) -> Option<Rc<OutputSpecContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mappingsBlock(&self) -> Option<Rc<MappingsBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn composeBlock(&self) -> Option<Rc<ComposeBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn transformMetadata(&self) -> Option<Rc<TransformMetadataContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn useBlock(&self) -> Option<Rc<UseBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn validateInputBlock(&self) -> Option<Rc<ValidateInputBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn invariantBlock(&self) -> Option<Rc<InvariantBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn validateOutputBlock(&self) -> Option<Rc<ValidateOutputBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn onChangeBlock(&self) -> Option<Rc<OnChangeBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn onErrorBlock(&self) -> Option<Rc<OnErrorBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TransformBlockDefContextAttrs<'input> for TransformBlockDefContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn transformBlockDef(&mut self,)
	-> Result<Rc<TransformBlockDefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TransformBlockDefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_transformBlockDef);
        let mut _localctx: Rc<TransformBlockDefContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(383);
			recog.base.match_token(T__24,&mut recog.err_handler)?;

			/*InvokeRule transformName*/
			recog.base.set_state(384);
			recog.transformName()?;

			recog.base.set_state(386);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(31,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule transformMetadata*/
					recog.base.set_state(385);
					recog.transformMetadata()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(389);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__25 {
				{
				/*InvokeRule useBlock*/
				recog.base.set_state(388);
				recog.useBlock()?;

				}
			}

			/*InvokeRule inputSpec*/
			recog.base.set_state(391);
			recog.inputSpec()?;

			/*InvokeRule outputSpec*/
			recog.base.set_state(392);
			recog.outputSpec()?;

			recog.base.set_state(394);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__57 {
				{
				/*InvokeRule validateInputBlock*/
				recog.base.set_state(393);
				recog.validateInputBlock()?;

				}
			}

			recog.base.set_state(397);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__67 {
				{
				/*InvokeRule invariantBlock*/
				recog.base.set_state(396);
				recog.invariantBlock()?;

				}
			}

			recog.base.set_state(401);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__49 
				=> {
					{
					/*InvokeRule mappingsBlock*/
					recog.base.set_state(399);
					recog.mappingsBlock()?;

					}
				}

			 T__50 
				=> {
					{
					/*InvokeRule composeBlock*/
					recog.base.set_state(400);
					recog.composeBlock()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(404);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__58 {
				{
				/*InvokeRule validateOutputBlock*/
				recog.base.set_state(403);
				recog.validateOutputBlock()?;

				}
			}

			recog.base.set_state(407);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__87 {
				{
				/*InvokeRule onChangeBlock*/
				recog.base.set_state(406);
				recog.onChangeBlock()?;

				}
			}

			recog.base.set_state(410);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__68 {
				{
				/*InvokeRule onErrorBlock*/
				recog.base.set_state(409);
				recog.onErrorBlock()?;

				}
			}

			recog.base.set_state(412);
			recog.base.match_token(T__4,&mut recog.err_handler)?;

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
//------------------- useBlock ----------------
pub type UseBlockContextAll<'input> = UseBlockContext<'input>;


pub type UseBlockContext<'input> = BaseParserRuleContext<'input,UseBlockContextExt<'input>>;

#[derive(Clone)]
pub struct UseBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for UseBlockContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for UseBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_useBlock(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_useBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for UseBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_useBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for UseBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_useBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_useBlock }
}
antlr_rust::tid!{UseBlockContextExt<'a>}

impl<'input> UseBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UseBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UseBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UseBlockContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<UseBlockContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}

}

impl<'input> UseBlockContextAttrs<'input> for UseBlockContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn useBlock(&mut self,)
	-> Result<Rc<UseBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UseBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_useBlock);
        let mut _localctx: Rc<UseBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(414);
			recog.base.match_token(T__25,&mut recog.err_handler)?;

			recog.base.set_state(416); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				recog.base.set_state(415);
				recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(418); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==IDENTIFIER) {break}
			}
			recog.base.set_state(420);
			recog.base.match_token(T__4,&mut recog.err_handler)?;

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
//------------------- inputSpec ----------------
pub type InputSpecContextAll<'input> = InputSpecContext<'input>;


pub type InputSpecContext<'input> = BaseParserRuleContext<'input,InputSpecContextExt<'input>>;

#[derive(Clone)]
pub struct InputSpecContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for InputSpecContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for InputSpecContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_inputSpec(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_inputSpec(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for InputSpecContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_inputSpec(self);
	}
}

impl<'input> CustomRuleContext<'input> for InputSpecContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inputSpec }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inputSpec }
}
antlr_rust::tid!{InputSpecContextExt<'a>}

impl<'input> InputSpecContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InputSpecContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InputSpecContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InputSpecContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<InputSpecContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn fieldType(&self) -> Option<Rc<FieldTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn qualifiers(&self) -> Option<Rc<QualifiersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn inputFieldDecl_all(&self) ->  Vec<Rc<InputFieldDeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn inputFieldDecl(&self, i: usize) -> Option<Rc<InputFieldDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> InputSpecContextAttrs<'input> for InputSpecContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inputSpec(&mut self,)
	-> Result<Rc<InputSpecContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InputSpecContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_inputSpec);
        let mut _localctx: Rc<InputSpecContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(436);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(42,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(422);
					recog.base.match_token(T__26,&mut recog.err_handler)?;

					recog.base.set_state(423);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule fieldType*/
					recog.base.set_state(424);
					recog.fieldType()?;

					recog.base.set_state(426);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==COMMA {
						{
						/*InvokeRule qualifiers*/
						recog.base.set_state(425);
						recog.qualifiers()?;

						}
					}

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(428);
					recog.base.match_token(T__26,&mut recog.err_handler)?;

					recog.base.set_state(430); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						/*InvokeRule inputFieldDecl*/
						recog.base.set_state(429);
						recog.inputFieldDecl()?;

						}
						}
						recog.base.set_state(432); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !(_la==IDENTIFIER) {break}
					}
					recog.base.set_state(434);
					recog.base.match_token(T__4,&mut recog.err_handler)?;

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
//------------------- inputFieldDecl ----------------
pub type InputFieldDeclContextAll<'input> = InputFieldDeclContext<'input>;


pub type InputFieldDeclContext<'input> = BaseParserRuleContext<'input,InputFieldDeclContextExt<'input>>;

#[derive(Clone)]
pub struct InputFieldDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for InputFieldDeclContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for InputFieldDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_inputFieldDecl(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_inputFieldDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for InputFieldDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_inputFieldDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for InputFieldDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inputFieldDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inputFieldDecl }
}
antlr_rust::tid!{InputFieldDeclContextExt<'a>}

impl<'input> InputFieldDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InputFieldDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InputFieldDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InputFieldDeclContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<InputFieldDeclContextExt<'input>>{

fn fieldName(&self) -> Option<Rc<FieldNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn fieldType(&self) -> Option<Rc<FieldTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn qualifiers(&self) -> Option<Rc<QualifiersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> InputFieldDeclContextAttrs<'input> for InputFieldDeclContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inputFieldDecl(&mut self,)
	-> Result<Rc<InputFieldDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InputFieldDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_inputFieldDecl);
        let mut _localctx: Rc<InputFieldDeclContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fieldName*/
			recog.base.set_state(438);
			recog.fieldName()?;

			recog.base.set_state(439);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule fieldType*/
			recog.base.set_state(440);
			recog.fieldType()?;

			recog.base.set_state(442);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==COMMA {
				{
				/*InvokeRule qualifiers*/
				recog.base.set_state(441);
				recog.qualifiers()?;

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
//------------------- outputSpec ----------------
pub type OutputSpecContextAll<'input> = OutputSpecContext<'input>;


pub type OutputSpecContext<'input> = BaseParserRuleContext<'input,OutputSpecContextExt<'input>>;

#[derive(Clone)]
pub struct OutputSpecContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for OutputSpecContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for OutputSpecContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_outputSpec(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_outputSpec(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for OutputSpecContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_outputSpec(self);
	}
}

impl<'input> CustomRuleContext<'input> for OutputSpecContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_outputSpec }
	//fn type_rule_index() -> usize where Self: Sized { RULE_outputSpec }
}
antlr_rust::tid!{OutputSpecContextExt<'a>}

impl<'input> OutputSpecContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OutputSpecContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OutputSpecContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OutputSpecContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<OutputSpecContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn fieldType(&self) -> Option<Rc<FieldTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn qualifiers(&self) -> Option<Rc<QualifiersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn outputFieldDecl_all(&self) ->  Vec<Rc<OutputFieldDeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn outputFieldDecl(&self, i: usize) -> Option<Rc<OutputFieldDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OutputSpecContextAttrs<'input> for OutputSpecContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn outputSpec(&mut self,)
	-> Result<Rc<OutputSpecContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OutputSpecContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_outputSpec);
        let mut _localctx: Rc<OutputSpecContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(458);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(46,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(444);
					recog.base.match_token(T__27,&mut recog.err_handler)?;

					recog.base.set_state(445);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule fieldType*/
					recog.base.set_state(446);
					recog.fieldType()?;

					recog.base.set_state(448);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==COMMA {
						{
						/*InvokeRule qualifiers*/
						recog.base.set_state(447);
						recog.qualifiers()?;

						}
					}

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(450);
					recog.base.match_token(T__27,&mut recog.err_handler)?;

					recog.base.set_state(452); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						/*InvokeRule outputFieldDecl*/
						recog.base.set_state(451);
						recog.outputFieldDecl()?;

						}
						}
						recog.base.set_state(454); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !(_la==IDENTIFIER) {break}
					}
					recog.base.set_state(456);
					recog.base.match_token(T__4,&mut recog.err_handler)?;

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
//------------------- outputFieldDecl ----------------
pub type OutputFieldDeclContextAll<'input> = OutputFieldDeclContext<'input>;


pub type OutputFieldDeclContext<'input> = BaseParserRuleContext<'input,OutputFieldDeclContextExt<'input>>;

#[derive(Clone)]
pub struct OutputFieldDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for OutputFieldDeclContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for OutputFieldDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_outputFieldDecl(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_outputFieldDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for OutputFieldDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_outputFieldDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for OutputFieldDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_outputFieldDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_outputFieldDecl }
}
antlr_rust::tid!{OutputFieldDeclContextExt<'a>}

impl<'input> OutputFieldDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OutputFieldDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OutputFieldDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OutputFieldDeclContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<OutputFieldDeclContextExt<'input>>{

fn fieldName(&self) -> Option<Rc<FieldNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn fieldType(&self) -> Option<Rc<FieldTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn qualifiers(&self) -> Option<Rc<QualifiersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OutputFieldDeclContextAttrs<'input> for OutputFieldDeclContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn outputFieldDecl(&mut self,)
	-> Result<Rc<OutputFieldDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OutputFieldDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_outputFieldDecl);
        let mut _localctx: Rc<OutputFieldDeclContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fieldName*/
			recog.base.set_state(460);
			recog.fieldName()?;

			recog.base.set_state(461);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule fieldType*/
			recog.base.set_state(462);
			recog.fieldType()?;

			recog.base.set_state(464);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==COMMA {
				{
				/*InvokeRule qualifiers*/
				recog.base.set_state(463);
				recog.qualifiers()?;

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
//------------------- fieldType ----------------
pub type FieldTypeContextAll<'input> = FieldTypeContext<'input>;


pub type FieldTypeContext<'input> = BaseParserRuleContext<'input,FieldTypeContextExt<'input>>;

#[derive(Clone)]
pub struct FieldTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for FieldTypeContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for FieldTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fieldType(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_fieldType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for FieldTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_fieldType(self);
	}
}

impl<'input> CustomRuleContext<'input> for FieldTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fieldType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fieldType }
}
antlr_rust::tid!{FieldTypeContextExt<'a>}

impl<'input> FieldTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FieldTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FieldTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FieldTypeContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<FieldTypeContextExt<'input>>{

fn baseType(&self) -> Option<Rc<BaseTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn constraint_all(&self) ->  Vec<Rc<ConstraintContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn constraint(&self, i: usize) -> Option<Rc<ConstraintContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn collectionType(&self) -> Option<Rc<CollectionTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token UPPER_IDENTIFIER
/// Returns `None` if there is no child corresponding to token UPPER_IDENTIFIER
fn UPPER_IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(UPPER_IDENTIFIER, 0)
}

}

impl<'input> FieldTypeContextAttrs<'input> for FieldTypeContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fieldType(&mut self,)
	-> Result<Rc<FieldTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FieldTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_fieldType);
        let mut _localctx: Rc<FieldTypeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(476);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__28 | T__29 | T__30 | T__31 | T__32 | T__33 | T__34 | T__35 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule baseType*/
					recog.base.set_state(466);
					recog.baseType()?;

					recog.base.set_state(470);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==LBRACKET {
						{
						{
						/*InvokeRule constraint*/
						recog.base.set_state(467);
						recog.constraint()?;

						}
						}
						recog.base.set_state(472);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

			 T__36 | T__37 | T__38 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule collectionType*/
					recog.base.set_state(473);
					recog.collectionType()?;

					}
				}

			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(474);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

			 UPPER_IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(475);
					recog.base.match_token(UPPER_IDENTIFIER,&mut recog.err_handler)?;

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
//------------------- baseType ----------------
pub type BaseTypeContextAll<'input> = BaseTypeContext<'input>;


pub type BaseTypeContext<'input> = BaseParserRuleContext<'input,BaseTypeContextExt<'input>>;

#[derive(Clone)]
pub struct BaseTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for BaseTypeContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for BaseTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_baseType(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_baseType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for BaseTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_baseType(self);
	}
}

impl<'input> CustomRuleContext<'input> for BaseTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_baseType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_baseType }
}
antlr_rust::tid!{BaseTypeContextExt<'a>}

impl<'input> BaseTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BaseTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BaseTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BaseTypeContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<BaseTypeContextExt<'input>>{


}

impl<'input> BaseTypeContextAttrs<'input> for BaseTypeContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn baseType(&mut self,)
	-> Result<Rc<BaseTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BaseTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_baseType);
        let mut _localctx: Rc<BaseTypeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(478);
			_la = recog.base.input.la(1);
			if { !(((((_la - 29)) & !0x3f) == 0 && ((1usize << (_la - 29)) & ((1usize << (T__28 - 29)) | (1usize << (T__29 - 29)) | (1usize << (T__30 - 29)) | (1usize << (T__31 - 29)) | (1usize << (T__32 - 29)) | (1usize << (T__33 - 29)) | (1usize << (T__34 - 29)) | (1usize << (T__35 - 29)))) != 0)) } {
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
//------------------- collectionType ----------------
pub type CollectionTypeContextAll<'input> = CollectionTypeContext<'input>;


pub type CollectionTypeContext<'input> = BaseParserRuleContext<'input,CollectionTypeContextExt<'input>>;

#[derive(Clone)]
pub struct CollectionTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for CollectionTypeContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for CollectionTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_collectionType(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_collectionType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for CollectionTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_collectionType(self);
	}
}

impl<'input> CustomRuleContext<'input> for CollectionTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_collectionType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_collectionType }
}
antlr_rust::tid!{CollectionTypeContextExt<'a>}

impl<'input> CollectionTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CollectionTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CollectionTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CollectionTypeContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<CollectionTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LANGLE
/// Returns `None` if there is no child corresponding to token LANGLE
fn LANGLE(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(LANGLE, 0)
}
fn fieldType_all(&self) ->  Vec<Rc<FieldTypeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fieldType(&self, i: usize) -> Option<Rc<FieldTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RANGLE
/// Returns `None` if there is no child corresponding to token RANGLE
fn RANGLE(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(RANGLE, 0)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}

}

impl<'input> CollectionTypeContextAttrs<'input> for CollectionTypeContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn collectionType(&mut self,)
	-> Result<Rc<CollectionTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CollectionTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_collectionType);
        let mut _localctx: Rc<CollectionTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(497);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__36 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(480);
					recog.base.match_token(T__36,&mut recog.err_handler)?;

					recog.base.set_state(481);
					recog.base.match_token(LANGLE,&mut recog.err_handler)?;

					/*InvokeRule fieldType*/
					recog.base.set_state(482);
					recog.fieldType()?;

					recog.base.set_state(483);
					recog.base.match_token(RANGLE,&mut recog.err_handler)?;

					}
				}

			 T__37 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(485);
					recog.base.match_token(T__37,&mut recog.err_handler)?;

					recog.base.set_state(486);
					recog.base.match_token(LANGLE,&mut recog.err_handler)?;

					/*InvokeRule fieldType*/
					recog.base.set_state(487);
					recog.fieldType()?;

					recog.base.set_state(488);
					recog.base.match_token(RANGLE,&mut recog.err_handler)?;

					}
				}

			 T__38 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(490);
					recog.base.match_token(T__38,&mut recog.err_handler)?;

					recog.base.set_state(491);
					recog.base.match_token(LANGLE,&mut recog.err_handler)?;

					/*InvokeRule fieldType*/
					recog.base.set_state(492);
					recog.fieldType()?;

					recog.base.set_state(493);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule fieldType*/
					recog.base.set_state(494);
					recog.fieldType()?;

					recog.base.set_state(495);
					recog.base.match_token(RANGLE,&mut recog.err_handler)?;

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
//------------------- constraint ----------------
pub type ConstraintContextAll<'input> = ConstraintContext<'input>;


pub type ConstraintContext<'input> = BaseParserRuleContext<'input,ConstraintContextExt<'input>>;

#[derive(Clone)]
pub struct ConstraintContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ConstraintContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ConstraintContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_constraint(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_constraint(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ConstraintContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_constraint(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConstraintContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constraint }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constraint }
}
antlr_rust::tid!{ConstraintContextExt<'a>}

impl<'input> ConstraintContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConstraintContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConstraintContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConstraintContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ConstraintContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACKET
/// Returns `None` if there is no child corresponding to token LBRACKET
fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(LBRACKET, 0)
}
fn constraintSpec_all(&self) ->  Vec<Rc<ConstraintSpecContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn constraintSpec(&self, i: usize) -> Option<Rc<ConstraintSpecContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RBRACKET
/// Returns `None` if there is no child corresponding to token RBRACKET
fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(RBRACKET, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ConstraintContextAttrs<'input> for ConstraintContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn constraint(&mut self,)
	-> Result<Rc<ConstraintContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConstraintContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_constraint);
        let mut _localctx: Rc<ConstraintContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(499);
			recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

			/*InvokeRule constraintSpec*/
			recog.base.set_state(500);
			recog.constraintSpec()?;

			recog.base.set_state(505);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(501);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule constraintSpec*/
				recog.base.set_state(502);
				recog.constraintSpec()?;

				}
				}
				recog.base.set_state(507);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(508);
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
//------------------- constraintSpec ----------------
pub type ConstraintSpecContextAll<'input> = ConstraintSpecContext<'input>;


pub type ConstraintSpecContext<'input> = BaseParserRuleContext<'input,ConstraintSpecContextExt<'input>>;

#[derive(Clone)]
pub struct ConstraintSpecContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ConstraintSpecContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ConstraintSpecContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_constraintSpec(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_constraintSpec(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ConstraintSpecContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_constraintSpec(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConstraintSpecContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constraintSpec }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constraintSpec }
}
antlr_rust::tid!{ConstraintSpecContextExt<'a>}

impl<'input> ConstraintSpecContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConstraintSpecContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConstraintSpecContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConstraintSpecContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ConstraintSpecContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token COLON in current rule
fn COLON_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COLON, starting from 0.
/// Returns `None` if number of children corresponding to token COLON is less or equal than `i`.
fn COLON(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, i)
}
fn rangeSpec(&self) -> Option<Rc<RangeSpecContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lengthSpec(&self) -> Option<Rc<LengthSpecContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
fn valueList(&self) -> Option<Rc<ValueListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token INTEGER in current rule
fn INTEGER_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token INTEGER, starting from 0.
/// Returns `None` if number of children corresponding to token INTEGER is less or equal than `i`.
fn INTEGER(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, i)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}

}

impl<'input> ConstraintSpecContextAttrs<'input> for ConstraintSpecContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn constraintSpec(&mut self,)
	-> Result<Rc<ConstraintSpecContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConstraintSpecContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_constraintSpec);
        let mut _localctx: Rc<ConstraintSpecContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(534);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__39 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(510);
					recog.base.match_token(T__39,&mut recog.err_handler)?;

					recog.base.set_state(511);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule rangeSpec*/
					recog.base.set_state(512);
					recog.rangeSpec()?;

					}
				}

			 T__40 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(513);
					recog.base.match_token(T__40,&mut recog.err_handler)?;

					recog.base.set_state(514);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule lengthSpec*/
					recog.base.set_state(515);
					recog.lengthSpec()?;

					}
				}

			 T__41 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(516);
					recog.base.match_token(T__41,&mut recog.err_handler)?;

					recog.base.set_state(517);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(518);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}

			 T__42 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(519);
					recog.base.match_token(T__42,&mut recog.err_handler)?;

					recog.base.set_state(520);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule valueList*/
					recog.base.set_state(521);
					recog.valueList()?;

					}
				}

			 T__43 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(522);
					recog.base.match_token(T__43,&mut recog.err_handler)?;

					recog.base.set_state(523);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(524);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					recog.base.set_state(529);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(52,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(525);
							recog.base.match_token(COMMA,&mut recog.err_handler)?;

							recog.base.set_state(526);
							recog.base.match_token(T__44,&mut recog.err_handler)?;

							recog.base.set_state(527);
							recog.base.match_token(COLON,&mut recog.err_handler)?;

							recog.base.set_state(528);
							recog.base.match_token(INTEGER,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}

			 T__44 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(531);
					recog.base.match_token(T__44,&mut recog.err_handler)?;

					recog.base.set_state(532);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(533);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

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
//------------------- rangeSpec ----------------
pub type RangeSpecContextAll<'input> = RangeSpecContext<'input>;


pub type RangeSpecContext<'input> = BaseParserRuleContext<'input,RangeSpecContextExt<'input>>;

#[derive(Clone)]
pub struct RangeSpecContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for RangeSpecContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for RangeSpecContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_rangeSpec(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_rangeSpec(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for RangeSpecContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_rangeSpec(self);
	}
}

impl<'input> CustomRuleContext<'input> for RangeSpecContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_rangeSpec }
	//fn type_rule_index() -> usize where Self: Sized { RULE_rangeSpec }
}
antlr_rust::tid!{RangeSpecContextExt<'a>}

impl<'input> RangeSpecContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RangeSpecContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RangeSpecContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RangeSpecContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<RangeSpecContextExt<'input>>{

fn numberLiteral_all(&self) ->  Vec<Rc<NumberLiteralContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn numberLiteral(&self, i: usize) -> Option<Rc<NumberLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token DOTDOT
/// Returns `None` if there is no child corresponding to token DOTDOT
fn DOTDOT(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(DOTDOT, 0)
}

}

impl<'input> RangeSpecContextAttrs<'input> for RangeSpecContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn rangeSpec(&mut self,)
	-> Result<Rc<RangeSpecContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RangeSpecContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_rangeSpec);
        let mut _localctx: Rc<RangeSpecContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(545);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(54,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule numberLiteral*/
					recog.base.set_state(536);
					recog.numberLiteral()?;

					recog.base.set_state(537);
					recog.base.match_token(DOTDOT,&mut recog.err_handler)?;

					/*InvokeRule numberLiteral*/
					recog.base.set_state(538);
					recog.numberLiteral()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(540);
					recog.base.match_token(DOTDOT,&mut recog.err_handler)?;

					/*InvokeRule numberLiteral*/
					recog.base.set_state(541);
					recog.numberLiteral()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule numberLiteral*/
					recog.base.set_state(542);
					recog.numberLiteral()?;

					recog.base.set_state(543);
					recog.base.match_token(DOTDOT,&mut recog.err_handler)?;

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
//------------------- lengthSpec ----------------
pub type LengthSpecContextAll<'input> = LengthSpecContext<'input>;


pub type LengthSpecContext<'input> = BaseParserRuleContext<'input,LengthSpecContextExt<'input>>;

#[derive(Clone)]
pub struct LengthSpecContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for LengthSpecContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for LengthSpecContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lengthSpec(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_lengthSpec(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for LengthSpecContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_lengthSpec(self);
	}
}

impl<'input> CustomRuleContext<'input> for LengthSpecContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lengthSpec }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lengthSpec }
}
antlr_rust::tid!{LengthSpecContextExt<'a>}

impl<'input> LengthSpecContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LengthSpecContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LengthSpecContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LengthSpecContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<LengthSpecContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token INTEGER in current rule
fn INTEGER_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token INTEGER, starting from 0.
/// Returns `None` if number of children corresponding to token INTEGER is less or equal than `i`.
fn INTEGER(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, i)
}
/// Retrieves first TerminalNode corresponding to token DOTDOT
/// Returns `None` if there is no child corresponding to token DOTDOT
fn DOTDOT(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(DOTDOT, 0)
}

}

impl<'input> LengthSpecContextAttrs<'input> for LengthSpecContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lengthSpec(&mut self,)
	-> Result<Rc<LengthSpecContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LengthSpecContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_lengthSpec);
        let mut _localctx: Rc<LengthSpecContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(555);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(55,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(547);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(548);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					recog.base.set_state(549);
					recog.base.match_token(DOTDOT,&mut recog.err_handler)?;

					recog.base.set_state(550);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(551);
					recog.base.match_token(DOTDOT,&mut recog.err_handler)?;

					recog.base.set_state(552);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(553);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					recog.base.set_state(554);
					recog.base.match_token(DOTDOT,&mut recog.err_handler)?;

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
//------------------- valueList ----------------
pub type ValueListContextAll<'input> = ValueListContext<'input>;


pub type ValueListContext<'input> = BaseParserRuleContext<'input,ValueListContextExt<'input>>;

#[derive(Clone)]
pub struct ValueListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ValueListContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ValueListContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_valueList(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_valueList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ValueListContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_valueList(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_valueList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_valueList }
}
antlr_rust::tid!{ValueListContextExt<'a>}

impl<'input> ValueListContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValueListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValueListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ValueListContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ValueListContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}
/// Retrieves all `TerminalNode`s corresponding to token STRING in current rule
fn STRING_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token STRING, starting from 0.
/// Returns `None` if number of children corresponding to token STRING is less or equal than `i`.
fn STRING(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, i)
}

}

impl<'input> ValueListContextAttrs<'input> for ValueListContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn valueList(&mut self,)
	-> Result<Rc<ValueListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValueListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_valueList);
        let mut _localctx: Rc<ValueListContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			recog.base.set_state(573);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(557);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(562);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(56,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(558);
							recog.base.match_token(COMMA,&mut recog.err_handler)?;

							recog.base.set_state(559);
							recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

							}
							} 
						}
						recog.base.set_state(564);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(56,&mut recog.base)?;
					}
					}
				}

			 STRING 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(565);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					recog.base.set_state(570);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(57,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(566);
							recog.base.match_token(COMMA,&mut recog.err_handler)?;

							recog.base.set_state(567);
							recog.base.match_token(STRING,&mut recog.err_handler)?;

							}
							} 
						}
						recog.base.set_state(572);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(57,&mut recog.base)?;
					}
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
//------------------- qualifiers ----------------
pub type QualifiersContextAll<'input> = QualifiersContext<'input>;


pub type QualifiersContext<'input> = BaseParserRuleContext<'input,QualifiersContextExt<'input>>;

#[derive(Clone)]
pub struct QualifiersContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for QualifiersContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for QualifiersContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_qualifiers(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_qualifiers(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for QualifiersContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_qualifiers(self);
	}
}

impl<'input> CustomRuleContext<'input> for QualifiersContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_qualifiers }
	//fn type_rule_index() -> usize where Self: Sized { RULE_qualifiers }
}
antlr_rust::tid!{QualifiersContextExt<'a>}

impl<'input> QualifiersContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<QualifiersContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,QualifiersContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait QualifiersContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<QualifiersContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}
fn qualifier_all(&self) ->  Vec<Rc<QualifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn qualifier(&self, i: usize) -> Option<Rc<QualifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> QualifiersContextAttrs<'input> for QualifiersContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn qualifiers(&mut self,)
	-> Result<Rc<QualifiersContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = QualifiersContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_qualifiers);
        let mut _localctx: Rc<QualifiersContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(575);
			recog.base.match_token(COMMA,&mut recog.err_handler)?;

			/*InvokeRule qualifier*/
			recog.base.set_state(576);
			recog.qualifier()?;

			recog.base.set_state(581);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(577);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule qualifier*/
				recog.base.set_state(578);
				recog.qualifier()?;

				}
				}
				recog.base.set_state(583);
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
//------------------- qualifier ----------------
pub type QualifierContextAll<'input> = QualifierContext<'input>;


pub type QualifierContext<'input> = BaseParserRuleContext<'input,QualifierContextExt<'input>>;

#[derive(Clone)]
pub struct QualifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for QualifierContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for QualifierContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_qualifier(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_qualifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for QualifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_qualifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for QualifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_qualifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_qualifier }
}
antlr_rust::tid!{QualifierContextExt<'a>}

impl<'input> QualifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<QualifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,QualifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait QualifierContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<QualifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DEFAULT_KW
/// Returns `None` if there is no child corresponding to token DEFAULT_KW
fn DEFAULT_KW(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(DEFAULT_KW, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> QualifierContextAttrs<'input> for QualifierContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn qualifier(&mut self,)
	-> Result<Rc<QualifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = QualifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_qualifier);
        let mut _localctx: Rc<QualifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(589);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__45 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(584);
					recog.base.match_token(T__45,&mut recog.err_handler)?;

					}
				}

			 T__10 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(585);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

					}
				}

			 DEFAULT_KW 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(586);
					recog.base.match_token(DEFAULT_KW,&mut recog.err_handler)?;

					recog.base.set_state(587);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(588);
					recog.expression_rec(0)?;

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
//------------------- applyBlock ----------------
pub type ApplyBlockContextAll<'input> = ApplyBlockContext<'input>;


pub type ApplyBlockContext<'input> = BaseParserRuleContext<'input,ApplyBlockContextExt<'input>>;

#[derive(Clone)]
pub struct ApplyBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ApplyBlockContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ApplyBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_applyBlock(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_applyBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ApplyBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_applyBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for ApplyBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_applyBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_applyBlock }
}
antlr_rust::tid!{ApplyBlockContextExt<'a>}

impl<'input> ApplyBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ApplyBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ApplyBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ApplyBlockContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ApplyBlockContextExt<'input>>{

fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ApplyBlockContextAttrs<'input> for ApplyBlockContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn applyBlock(&mut self,)
	-> Result<Rc<ApplyBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ApplyBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_applyBlock);
        let mut _localctx: Rc<ApplyBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(591);
			recog.base.match_token(T__46,&mut recog.err_handler)?;

			recog.base.set_state(593); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule statement*/
				recog.base.set_state(592);
				recog.statement()?;

				}
				}
				recog.base.set_state(595); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9) | (1usize << T__23) | (1usize << T__26) | (1usize << T__27))) != 0) || ((((_la - 40)) & !0x3f) == 0 && ((1usize << (_la - 40)) & ((1usize << (T__39 - 40)) | (1usize << (T__40 - 40)) | (1usize << (T__41 - 40)) | (1usize << (T__42 - 40)) | (1usize << (T__48 - 40)) | (1usize << (T__61 - 40)) | (1usize << (T__62 - 40)) | (1usize << (T__63 - 40)) | (1usize << (T__64 - 40)) | (1usize << (T__69 - 40)))) != 0) || _la==T__78 || _la==IDENTIFIER) {break}
			}
			recog.base.set_state(597);
			recog.base.match_token(T__4,&mut recog.err_handler)?;

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

impl<'input> TransformDSLParserContext<'input> for StatementContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for StatementContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_statement(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_statement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for StatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr_rust::tid!{StatementContextExt<'a>}

impl<'input> StatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StatementContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<StatementContextExt<'input>>{

fn assignment(&self) -> Option<Rc<AssignmentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn letAssignment(&self) -> Option<Rc<LetAssignmentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn statement(&mut self,)
	-> Result<Rc<StatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(601);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__6 | T__7 | T__8 | T__9 | T__23 | T__26 | T__27 | T__39 | T__40 | T__41 |
			 T__42 | T__61 | T__62 | T__63 | T__64 | T__69 | T__78 | IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule assignment*/
					recog.base.set_state(599);
					recog.assignment()?;

					}
				}

			 T__48 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule letAssignment*/
					recog.base.set_state(600);
					recog.letAssignment()?;

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
//------------------- assignment ----------------
pub type AssignmentContextAll<'input> = AssignmentContext<'input>;


pub type AssignmentContext<'input> = BaseParserRuleContext<'input,AssignmentContextExt<'input>>;

#[derive(Clone)]
pub struct AssignmentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for AssignmentContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for AssignmentContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_assignment(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_assignment(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for AssignmentContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_assignment(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssignmentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assignment }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assignment }
}
antlr_rust::tid!{AssignmentContextExt<'a>}

impl<'input> AssignmentContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssignmentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssignmentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssignmentContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<AssignmentContextExt<'input>>{

fn fieldPath(&self) -> Option<Rc<FieldPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> AssignmentContextAttrs<'input> for AssignmentContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assignment(&mut self,)
	-> Result<Rc<AssignmentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssignmentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_assignment);
        let mut _localctx: Rc<AssignmentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(610);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(63,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fieldPath*/
					recog.base.set_state(603);
					recog.fieldPath()?;

					recog.base.set_state(604);
					recog.base.match_token(T__47,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(605);
					recog.expression_rec(0)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(607);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(608);
					recog.base.match_token(T__47,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(609);
					recog.expression_rec(0)?;

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
//------------------- letAssignment ----------------
pub type LetAssignmentContextAll<'input> = LetAssignmentContext<'input>;


pub type LetAssignmentContext<'input> = BaseParserRuleContext<'input,LetAssignmentContextExt<'input>>;

#[derive(Clone)]
pub struct LetAssignmentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for LetAssignmentContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for LetAssignmentContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_letAssignment(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_letAssignment(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for LetAssignmentContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_letAssignment(self);
	}
}

impl<'input> CustomRuleContext<'input> for LetAssignmentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_letAssignment }
	//fn type_rule_index() -> usize where Self: Sized { RULE_letAssignment }
}
antlr_rust::tid!{LetAssignmentContextExt<'a>}

impl<'input> LetAssignmentContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LetAssignmentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LetAssignmentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LetAssignmentContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<LetAssignmentContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> LetAssignmentContextAttrs<'input> for LetAssignmentContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn letAssignment(&mut self,)
	-> Result<Rc<LetAssignmentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LetAssignmentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_letAssignment);
        let mut _localctx: Rc<LetAssignmentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(612);
			recog.base.match_token(T__48,&mut recog.err_handler)?;

			recog.base.set_state(613);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(614);
			recog.base.match_token(T__47,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(615);
			recog.expression_rec(0)?;

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
//------------------- mappingsBlock ----------------
pub type MappingsBlockContextAll<'input> = MappingsBlockContext<'input>;


pub type MappingsBlockContext<'input> = BaseParserRuleContext<'input,MappingsBlockContextExt<'input>>;

#[derive(Clone)]
pub struct MappingsBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for MappingsBlockContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for MappingsBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mappingsBlock(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_mappingsBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for MappingsBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_mappingsBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for MappingsBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mappingsBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mappingsBlock }
}
antlr_rust::tid!{MappingsBlockContextExt<'a>}

impl<'input> MappingsBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MappingsBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MappingsBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MappingsBlockContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<MappingsBlockContextExt<'input>>{

fn mapping_all(&self) ->  Vec<Rc<MappingContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn mapping(&self, i: usize) -> Option<Rc<MappingContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> MappingsBlockContextAttrs<'input> for MappingsBlockContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mappingsBlock(&mut self,)
	-> Result<Rc<MappingsBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MappingsBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_mappingsBlock);
        let mut _localctx: Rc<MappingsBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(617);
			recog.base.match_token(T__49,&mut recog.err_handler)?;

			recog.base.set_state(619); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule mapping*/
				recog.base.set_state(618);
				recog.mapping()?;

				}
				}
				recog.base.set_state(621); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9) | (1usize << T__23) | (1usize << T__26) | (1usize << T__27))) != 0) || ((((_la - 40)) & !0x3f) == 0 && ((1usize << (_la - 40)) & ((1usize << (T__39 - 40)) | (1usize << (T__40 - 40)) | (1usize << (T__41 - 40)) | (1usize << (T__42 - 40)) | (1usize << (T__61 - 40)) | (1usize << (T__62 - 40)) | (1usize << (T__63 - 40)) | (1usize << (T__64 - 40)) | (1usize << (T__69 - 40)))) != 0) || _la==T__78 || _la==IDENTIFIER) {break}
			}
			recog.base.set_state(623);
			recog.base.match_token(T__4,&mut recog.err_handler)?;

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
//------------------- mapping ----------------
pub type MappingContextAll<'input> = MappingContext<'input>;


pub type MappingContext<'input> = BaseParserRuleContext<'input,MappingContextExt<'input>>;

#[derive(Clone)]
pub struct MappingContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for MappingContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for MappingContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mapping(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_mapping(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for MappingContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_mapping(self);
	}
}

impl<'input> CustomRuleContext<'input> for MappingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mapping }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mapping }
}
antlr_rust::tid!{MappingContextExt<'a>}

impl<'input> MappingContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MappingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MappingContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MappingContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<MappingContextExt<'input>>{

fn fieldPath(&self) -> Option<Rc<FieldPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MappingContextAttrs<'input> for MappingContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mapping(&mut self,)
	-> Result<Rc<MappingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MappingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_mapping);
        let mut _localctx: Rc<MappingContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fieldPath*/
			recog.base.set_state(625);
			recog.fieldPath()?;

			recog.base.set_state(626);
			recog.base.match_token(T__47,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(627);
			recog.expression_rec(0)?;

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
//------------------- composeBlock ----------------
pub type ComposeBlockContextAll<'input> = ComposeBlockContext<'input>;


pub type ComposeBlockContext<'input> = BaseParserRuleContext<'input,ComposeBlockContextExt<'input>>;

#[derive(Clone)]
pub struct ComposeBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ComposeBlockContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ComposeBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_composeBlock(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_composeBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ComposeBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_composeBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComposeBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_composeBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_composeBlock }
}
antlr_rust::tid!{ComposeBlockContextExt<'a>}

impl<'input> ComposeBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComposeBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComposeBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComposeBlockContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ComposeBlockContextExt<'input>>{

fn composeType(&self) -> Option<Rc<ComposeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn composeRef_all(&self) ->  Vec<Rc<ComposeRefContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn composeRef(&self, i: usize) -> Option<Rc<ComposeRefContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn thenBlock(&self) -> Option<Rc<ThenBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ComposeBlockContextAttrs<'input> for ComposeBlockContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn composeBlock(&mut self,)
	-> Result<Rc<ComposeBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComposeBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 96, RULE_composeBlock);
        let mut _localctx: Rc<ComposeBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(629);
			recog.base.match_token(T__50,&mut recog.err_handler)?;

			recog.base.set_state(631);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 52)) & !0x3f) == 0 && ((1usize << (_la - 52)) & ((1usize << (T__51 - 52)) | (1usize << (T__52 - 52)) | (1usize << (T__53 - 52)))) != 0) {
				{
				/*InvokeRule composeType*/
				recog.base.set_state(630);
				recog.composeType()?;

				}
			}

			recog.base.set_state(634); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule composeRef*/
				recog.base.set_state(633);
				recog.composeRef()?;

				}
				}
				recog.base.set_state(636); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==T__54 || _la==T__55 || _la==IDENTIFIER) {break}
			}
			recog.base.set_state(638);
			recog.base.match_token(T__4,&mut recog.err_handler)?;

			recog.base.set_state(640);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__56 {
				{
				/*InvokeRule thenBlock*/
				recog.base.set_state(639);
				recog.thenBlock()?;

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
//------------------- composeType ----------------
pub type ComposeTypeContextAll<'input> = ComposeTypeContext<'input>;


pub type ComposeTypeContext<'input> = BaseParserRuleContext<'input,ComposeTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ComposeTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ComposeTypeContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ComposeTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_composeType(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_composeType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ComposeTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_composeType(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComposeTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_composeType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_composeType }
}
antlr_rust::tid!{ComposeTypeContextExt<'a>}

impl<'input> ComposeTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComposeTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComposeTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComposeTypeContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ComposeTypeContextExt<'input>>{


}

impl<'input> ComposeTypeContextAttrs<'input> for ComposeTypeContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn composeType(&mut self,)
	-> Result<Rc<ComposeTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComposeTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 98, RULE_composeType);
        let mut _localctx: Rc<ComposeTypeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(642);
			_la = recog.base.input.la(1);
			if { !(((((_la - 52)) & !0x3f) == 0 && ((1usize << (_la - 52)) & ((1usize << (T__51 - 52)) | (1usize << (T__52 - 52)) | (1usize << (T__53 - 52)))) != 0)) } {
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
//------------------- composeRef ----------------
pub type ComposeRefContextAll<'input> = ComposeRefContext<'input>;


pub type ComposeRefContext<'input> = BaseParserRuleContext<'input,ComposeRefContextExt<'input>>;

#[derive(Clone)]
pub struct ComposeRefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ComposeRefContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ComposeRefContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_composeRef(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_composeRef(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ComposeRefContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_composeRef(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComposeRefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_composeRef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_composeRef }
}
antlr_rust::tid!{ComposeRefContextExt<'a>}

impl<'input> ComposeRefContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComposeRefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComposeRefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComposeRefContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ComposeRefContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}

}

impl<'input> ComposeRefContextAttrs<'input> for ComposeRefContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn composeRef(&mut self,)
	-> Result<Rc<ComposeRefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComposeRefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 100, RULE_composeRef);
        let mut _localctx: Rc<ComposeRefContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(653);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(644);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

			 T__54 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(645);
					recog.base.match_token(T__54,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(646);
					recog.expression_rec(0)?;

					recog.base.set_state(647);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(648);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

			 T__55 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(650);
					recog.base.match_token(T__55,&mut recog.err_handler)?;

					recog.base.set_state(651);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(652);
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
//------------------- thenBlock ----------------
pub type ThenBlockContextAll<'input> = ThenBlockContext<'input>;


pub type ThenBlockContext<'input> = BaseParserRuleContext<'input,ThenBlockContextExt<'input>>;

#[derive(Clone)]
pub struct ThenBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ThenBlockContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ThenBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thenBlock(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_thenBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ThenBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_thenBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for ThenBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thenBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thenBlock }
}
antlr_rust::tid!{ThenBlockContextExt<'a>}

impl<'input> ThenBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ThenBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ThenBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ThenBlockContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ThenBlockContextExt<'input>>{

fn composeType(&self) -> Option<Rc<ComposeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn composeRef_all(&self) ->  Vec<Rc<ComposeRefContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn composeRef(&self, i: usize) -> Option<Rc<ComposeRefContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ThenBlockContextAttrs<'input> for ThenBlockContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thenBlock(&mut self,)
	-> Result<Rc<ThenBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ThenBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 102, RULE_thenBlock);
        let mut _localctx: Rc<ThenBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(655);
			recog.base.match_token(T__56,&mut recog.err_handler)?;

			recog.base.set_state(657);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 52)) & !0x3f) == 0 && ((1usize << (_la - 52)) & ((1usize << (T__51 - 52)) | (1usize << (T__52 - 52)) | (1usize << (T__53 - 52)))) != 0) {
				{
				/*InvokeRule composeType*/
				recog.base.set_state(656);
				recog.composeType()?;

				}
			}

			recog.base.set_state(660); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule composeRef*/
				recog.base.set_state(659);
				recog.composeRef()?;

				}
				}
				recog.base.set_state(662); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==T__54 || _la==T__55 || _la==IDENTIFIER) {break}
			}
			recog.base.set_state(664);
			recog.base.match_token(T__4,&mut recog.err_handler)?;

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
//------------------- validateInputBlock ----------------
pub type ValidateInputBlockContextAll<'input> = ValidateInputBlockContext<'input>;


pub type ValidateInputBlockContext<'input> = BaseParserRuleContext<'input,ValidateInputBlockContextExt<'input>>;

#[derive(Clone)]
pub struct ValidateInputBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ValidateInputBlockContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ValidateInputBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_validateInputBlock(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_validateInputBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ValidateInputBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_validateInputBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValidateInputBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_validateInputBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_validateInputBlock }
}
antlr_rust::tid!{ValidateInputBlockContextExt<'a>}

impl<'input> ValidateInputBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValidateInputBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValidateInputBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ValidateInputBlockContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ValidateInputBlockContextExt<'input>>{

fn validationRule_all(&self) ->  Vec<Rc<ValidationRuleContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn validationRule(&self, i: usize) -> Option<Rc<ValidationRuleContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ValidateInputBlockContextAttrs<'input> for ValidateInputBlockContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn validateInputBlock(&mut self,)
	-> Result<Rc<ValidateInputBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValidateInputBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 104, RULE_validateInputBlock);
        let mut _localctx: Rc<ValidateInputBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(666);
			recog.base.match_token(T__57,&mut recog.err_handler)?;

			recog.base.set_state(668); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule validationRule*/
				recog.base.set_state(667);
				recog.validationRule()?;

				}
				}
				recog.base.set_state(670); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9) | (1usize << T__19) | (1usize << T__23) | (1usize << T__26) | (1usize << T__27) | (1usize << T__28) | (1usize << T__29) | (1usize << T__30))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (T__32 - 33)) | (1usize << (T__36 - 33)) | (1usize << (T__37 - 33)) | (1usize << (T__38 - 33)) | (1usize << (T__39 - 33)) | (1usize << (T__40 - 33)) | (1usize << (T__41 - 33)) | (1usize << (T__42 - 33)) | (1usize << (T__54 - 33)) | (1usize << (T__59 - 33)) | (1usize << (T__61 - 33)) | (1usize << (T__62 - 33)) | (1usize << (T__63 - 33)))) != 0) || ((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (T__64 - 65)) | (1usize << (T__69 - 65)) | (1usize << (T__78 - 65)) | (1usize << (T__91 - 65)) | (1usize << (T__94 - 65)))) != 0) || ((((_la - 98)) & !0x3f) == 0 && ((1usize << (_la - 98)) & ((1usize << (T__97 - 98)) | (1usize << (T__98 - 98)) | (1usize << (T__99 - 98)) | (1usize << (T__100 - 98)) | (1usize << (T__101 - 98)) | (1usize << (T__102 - 98)) | (1usize << (T__103 - 98)) | (1usize << (T__104 - 98)) | (1usize << (T__105 - 98)) | (1usize << (T__106 - 98)) | (1usize << (T__107 - 98)) | (1usize << (T__108 - 98)) | (1usize << (T__109 - 98)) | (1usize << (T__110 - 98)) | (1usize << (T__111 - 98)) | (1usize << (T__112 - 98)) | (1usize << (T__113 - 98)) | (1usize << (INTEGER - 98)) | (1usize << (DECIMAL - 98)) | (1usize << (BOOLEAN - 98)))) != 0) || ((((_la - 130)) & !0x3f) == 0 && ((1usize << (_la - 130)) & ((1usize << (IDENTIFIER - 130)) | (1usize << (STRING - 130)) | (1usize << (LBRACKET - 130)) | (1usize << (LPAREN - 130)) | (1usize << (LBRACE - 130)) | (1usize << (MINUS - 130)))) != 0)) {break}
			}
			recog.base.set_state(672);
			recog.base.match_token(T__4,&mut recog.err_handler)?;

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
//------------------- validateOutputBlock ----------------
pub type ValidateOutputBlockContextAll<'input> = ValidateOutputBlockContext<'input>;


pub type ValidateOutputBlockContext<'input> = BaseParserRuleContext<'input,ValidateOutputBlockContextExt<'input>>;

#[derive(Clone)]
pub struct ValidateOutputBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ValidateOutputBlockContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ValidateOutputBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_validateOutputBlock(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_validateOutputBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ValidateOutputBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_validateOutputBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValidateOutputBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_validateOutputBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_validateOutputBlock }
}
antlr_rust::tid!{ValidateOutputBlockContextExt<'a>}

impl<'input> ValidateOutputBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValidateOutputBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValidateOutputBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ValidateOutputBlockContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ValidateOutputBlockContextExt<'input>>{

fn validationRule_all(&self) ->  Vec<Rc<ValidationRuleContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn validationRule(&self, i: usize) -> Option<Rc<ValidationRuleContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ValidateOutputBlockContextAttrs<'input> for ValidateOutputBlockContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn validateOutputBlock(&mut self,)
	-> Result<Rc<ValidateOutputBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValidateOutputBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 106, RULE_validateOutputBlock);
        let mut _localctx: Rc<ValidateOutputBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(674);
			recog.base.match_token(T__58,&mut recog.err_handler)?;

			recog.base.set_state(676); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule validationRule*/
				recog.base.set_state(675);
				recog.validationRule()?;

				}
				}
				recog.base.set_state(678); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9) | (1usize << T__19) | (1usize << T__23) | (1usize << T__26) | (1usize << T__27) | (1usize << T__28) | (1usize << T__29) | (1usize << T__30))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (T__32 - 33)) | (1usize << (T__36 - 33)) | (1usize << (T__37 - 33)) | (1usize << (T__38 - 33)) | (1usize << (T__39 - 33)) | (1usize << (T__40 - 33)) | (1usize << (T__41 - 33)) | (1usize << (T__42 - 33)) | (1usize << (T__54 - 33)) | (1usize << (T__59 - 33)) | (1usize << (T__61 - 33)) | (1usize << (T__62 - 33)) | (1usize << (T__63 - 33)))) != 0) || ((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (T__64 - 65)) | (1usize << (T__69 - 65)) | (1usize << (T__78 - 65)) | (1usize << (T__91 - 65)) | (1usize << (T__94 - 65)))) != 0) || ((((_la - 98)) & !0x3f) == 0 && ((1usize << (_la - 98)) & ((1usize << (T__97 - 98)) | (1usize << (T__98 - 98)) | (1usize << (T__99 - 98)) | (1usize << (T__100 - 98)) | (1usize << (T__101 - 98)) | (1usize << (T__102 - 98)) | (1usize << (T__103 - 98)) | (1usize << (T__104 - 98)) | (1usize << (T__105 - 98)) | (1usize << (T__106 - 98)) | (1usize << (T__107 - 98)) | (1usize << (T__108 - 98)) | (1usize << (T__109 - 98)) | (1usize << (T__110 - 98)) | (1usize << (T__111 - 98)) | (1usize << (T__112 - 98)) | (1usize << (T__113 - 98)) | (1usize << (INTEGER - 98)) | (1usize << (DECIMAL - 98)) | (1usize << (BOOLEAN - 98)))) != 0) || ((((_la - 130)) & !0x3f) == 0 && ((1usize << (_la - 130)) & ((1usize << (IDENTIFIER - 130)) | (1usize << (STRING - 130)) | (1usize << (LBRACKET - 130)) | (1usize << (LPAREN - 130)) | (1usize << (LBRACE - 130)) | (1usize << (MINUS - 130)))) != 0)) {break}
			}
			recog.base.set_state(680);
			recog.base.match_token(T__4,&mut recog.err_handler)?;

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
//------------------- validationRule ----------------
pub type ValidationRuleContextAll<'input> = ValidationRuleContext<'input>;


pub type ValidationRuleContext<'input> = BaseParserRuleContext<'input,ValidationRuleContextExt<'input>>;

#[derive(Clone)]
pub struct ValidationRuleContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ValidationRuleContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ValidationRuleContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_validationRule(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_validationRule(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ValidationRuleContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_validationRule(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValidationRuleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_validationRule }
	//fn type_rule_index() -> usize where Self: Sized { RULE_validationRule }
}
antlr_rust::tid!{ValidationRuleContextExt<'a>}

impl<'input> ValidationRuleContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValidationRuleContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValidationRuleContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ValidationRuleContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ValidationRuleContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn validationMessage(&self) -> Option<Rc<ValidationMessageContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn validationRule_all(&self) ->  Vec<Rc<ValidationRuleContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn validationRule(&self, i: usize) -> Option<Rc<ValidationRuleContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ValidationRuleContextAttrs<'input> for ValidationRuleContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn validationRule(&mut self,)
	-> Result<Rc<ValidationRuleContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValidationRuleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 108, RULE_validationRule);
        let mut _localctx: Rc<ValidationRuleContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(701);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(74,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule expression*/
					recog.base.set_state(682);
					recog.expression_rec(0)?;

					recog.base.set_state(683);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule validationMessage*/
					recog.base.set_state(684);
					recog.validationMessage()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(686);
					recog.base.match_token(T__54,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(687);
					recog.expression_rec(0)?;

					recog.base.set_state(688);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(690); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						/*InvokeRule validationRule*/
						recog.base.set_state(689);
						recog.validationRule()?;

						}
						}
						recog.base.set_state(692); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9) | (1usize << T__19) | (1usize << T__23) | (1usize << T__26) | (1usize << T__27) | (1usize << T__28) | (1usize << T__29) | (1usize << T__30))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (T__32 - 33)) | (1usize << (T__36 - 33)) | (1usize << (T__37 - 33)) | (1usize << (T__38 - 33)) | (1usize << (T__39 - 33)) | (1usize << (T__40 - 33)) | (1usize << (T__41 - 33)) | (1usize << (T__42 - 33)) | (1usize << (T__54 - 33)) | (1usize << (T__59 - 33)) | (1usize << (T__61 - 33)) | (1usize << (T__62 - 33)) | (1usize << (T__63 - 33)))) != 0) || ((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (T__64 - 65)) | (1usize << (T__69 - 65)) | (1usize << (T__78 - 65)) | (1usize << (T__91 - 65)) | (1usize << (T__94 - 65)))) != 0) || ((((_la - 98)) & !0x3f) == 0 && ((1usize << (_la - 98)) & ((1usize << (T__97 - 98)) | (1usize << (T__98 - 98)) | (1usize << (T__99 - 98)) | (1usize << (T__100 - 98)) | (1usize << (T__101 - 98)) | (1usize << (T__102 - 98)) | (1usize << (T__103 - 98)) | (1usize << (T__104 - 98)) | (1usize << (T__105 - 98)) | (1usize << (T__106 - 98)) | (1usize << (T__107 - 98)) | (1usize << (T__108 - 98)) | (1usize << (T__109 - 98)) | (1usize << (T__110 - 98)) | (1usize << (T__111 - 98)) | (1usize << (T__112 - 98)) | (1usize << (T__113 - 98)) | (1usize << (INTEGER - 98)) | (1usize << (DECIMAL - 98)) | (1usize << (BOOLEAN - 98)))) != 0) || ((((_la - 130)) & !0x3f) == 0 && ((1usize << (_la - 130)) & ((1usize << (IDENTIFIER - 130)) | (1usize << (STRING - 130)) | (1usize << (LBRACKET - 130)) | (1usize << (LPAREN - 130)) | (1usize << (LBRACE - 130)) | (1usize << (MINUS - 130)))) != 0)) {break}
					}
					recog.base.set_state(694);
					recog.base.match_token(T__4,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(696);
					recog.base.match_token(T__59,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(697);
					recog.expression_rec(0)?;

					recog.base.set_state(698);
					recog.base.match_token(T__60,&mut recog.err_handler)?;

					/*InvokeRule validationMessage*/
					recog.base.set_state(699);
					recog.validationMessage()?;

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
//------------------- validationMessage ----------------
pub type ValidationMessageContextAll<'input> = ValidationMessageContext<'input>;


pub type ValidationMessageContext<'input> = BaseParserRuleContext<'input,ValidationMessageContextExt<'input>>;

#[derive(Clone)]
pub struct ValidationMessageContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ValidationMessageContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ValidationMessageContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_validationMessage(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_validationMessage(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ValidationMessageContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_validationMessage(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValidationMessageContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_validationMessage }
	//fn type_rule_index() -> usize where Self: Sized { RULE_validationMessage }
}
antlr_rust::tid!{ValidationMessageContextExt<'a>}

impl<'input> ValidationMessageContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValidationMessageContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValidationMessageContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ValidationMessageContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ValidationMessageContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
fn validationMessageObject(&self) -> Option<Rc<ValidationMessageObjectContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ValidationMessageContextAttrs<'input> for ValidationMessageContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn validationMessage(&mut self,)
	-> Result<Rc<ValidationMessageContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValidationMessageContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 110, RULE_validationMessage);
        let mut _localctx: Rc<ValidationMessageContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(705);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 STRING 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(703);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}

			 T__61 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule validationMessageObject*/
					recog.base.set_state(704);
					recog.validationMessageObject()?;

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
//------------------- validationMessageObject ----------------
pub type ValidationMessageObjectContextAll<'input> = ValidationMessageObjectContext<'input>;


pub type ValidationMessageObjectContext<'input> = BaseParserRuleContext<'input,ValidationMessageObjectContextExt<'input>>;

#[derive(Clone)]
pub struct ValidationMessageObjectContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ValidationMessageObjectContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ValidationMessageObjectContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_validationMessageObject(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_validationMessageObject(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ValidationMessageObjectContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_validationMessageObject(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValidationMessageObjectContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_validationMessageObject }
	//fn type_rule_index() -> usize where Self: Sized { RULE_validationMessageObject }
}
antlr_rust::tid!{ValidationMessageObjectContextExt<'a>}

impl<'input> ValidationMessageObjectContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValidationMessageObjectContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValidationMessageObjectContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ValidationMessageObjectContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ValidationMessageObjectContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token COLON in current rule
fn COLON_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COLON, starting from 0.
/// Returns `None` if number of children corresponding to token COLON is less or equal than `i`.
fn COLON(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, i)
}
/// Retrieves all `TerminalNode`s corresponding to token STRING in current rule
fn STRING_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token STRING, starting from 0.
/// Returns `None` if number of children corresponding to token STRING is less or equal than `i`.
fn STRING(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, i)
}
fn severityLevel(&self) -> Option<Rc<SeverityLevelContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ValidationMessageObjectContextAttrs<'input> for ValidationMessageObjectContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn validationMessageObject(&mut self,)
	-> Result<Rc<ValidationMessageObjectContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValidationMessageObjectContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 112, RULE_validationMessageObject);
        let mut _localctx: Rc<ValidationMessageObjectContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(707);
			recog.base.match_token(T__61,&mut recog.err_handler)?;

			recog.base.set_state(708);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(709);
			recog.base.match_token(STRING,&mut recog.err_handler)?;

			recog.base.set_state(713);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(76,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(710);
					recog.base.match_token(T__62,&mut recog.err_handler)?;

					recog.base.set_state(711);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(712);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			recog.base.set_state(718);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(77,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(715);
					recog.base.match_token(T__63,&mut recog.err_handler)?;

					recog.base.set_state(716);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule severityLevel*/
					recog.base.set_state(717);
					recog.severityLevel()?;

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
//------------------- severityLevel ----------------
pub type SeverityLevelContextAll<'input> = SeverityLevelContext<'input>;


pub type SeverityLevelContext<'input> = BaseParserRuleContext<'input,SeverityLevelContextExt<'input>>;

#[derive(Clone)]
pub struct SeverityLevelContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for SeverityLevelContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for SeverityLevelContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_severityLevel(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_severityLevel(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for SeverityLevelContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_severityLevel(self);
	}
}

impl<'input> CustomRuleContext<'input> for SeverityLevelContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_severityLevel }
	//fn type_rule_index() -> usize where Self: Sized { RULE_severityLevel }
}
antlr_rust::tid!{SeverityLevelContextExt<'a>}

impl<'input> SeverityLevelContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SeverityLevelContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SeverityLevelContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SeverityLevelContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<SeverityLevelContextExt<'input>>{


}

impl<'input> SeverityLevelContextAttrs<'input> for SeverityLevelContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn severityLevel(&mut self,)
	-> Result<Rc<SeverityLevelContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SeverityLevelContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 114, RULE_severityLevel);
        let mut _localctx: Rc<SeverityLevelContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(720);
			_la = recog.base.input.la(1);
			if { !(((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (T__64 - 65)) | (1usize << (T__65 - 65)) | (1usize << (T__66 - 65)))) != 0)) } {
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
//------------------- invariantBlock ----------------
pub type InvariantBlockContextAll<'input> = InvariantBlockContext<'input>;


pub type InvariantBlockContext<'input> = BaseParserRuleContext<'input,InvariantBlockContextExt<'input>>;

#[derive(Clone)]
pub struct InvariantBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for InvariantBlockContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for InvariantBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_invariantBlock(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_invariantBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for InvariantBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_invariantBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for InvariantBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_invariantBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_invariantBlock }
}
antlr_rust::tid!{InvariantBlockContextExt<'a>}

impl<'input> InvariantBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InvariantBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InvariantBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InvariantBlockContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<InvariantBlockContextExt<'input>>{

fn validationRule_all(&self) ->  Vec<Rc<ValidationRuleContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn validationRule(&self, i: usize) -> Option<Rc<ValidationRuleContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> InvariantBlockContextAttrs<'input> for InvariantBlockContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn invariantBlock(&mut self,)
	-> Result<Rc<InvariantBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InvariantBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_invariantBlock);
        let mut _localctx: Rc<InvariantBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(722);
			recog.base.match_token(T__67,&mut recog.err_handler)?;

			recog.base.set_state(724); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule validationRule*/
				recog.base.set_state(723);
				recog.validationRule()?;

				}
				}
				recog.base.set_state(726); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9) | (1usize << T__19) | (1usize << T__23) | (1usize << T__26) | (1usize << T__27) | (1usize << T__28) | (1usize << T__29) | (1usize << T__30))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (T__32 - 33)) | (1usize << (T__36 - 33)) | (1usize << (T__37 - 33)) | (1usize << (T__38 - 33)) | (1usize << (T__39 - 33)) | (1usize << (T__40 - 33)) | (1usize << (T__41 - 33)) | (1usize << (T__42 - 33)) | (1usize << (T__54 - 33)) | (1usize << (T__59 - 33)) | (1usize << (T__61 - 33)) | (1usize << (T__62 - 33)) | (1usize << (T__63 - 33)))) != 0) || ((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (T__64 - 65)) | (1usize << (T__69 - 65)) | (1usize << (T__78 - 65)) | (1usize << (T__91 - 65)) | (1usize << (T__94 - 65)))) != 0) || ((((_la - 98)) & !0x3f) == 0 && ((1usize << (_la - 98)) & ((1usize << (T__97 - 98)) | (1usize << (T__98 - 98)) | (1usize << (T__99 - 98)) | (1usize << (T__100 - 98)) | (1usize << (T__101 - 98)) | (1usize << (T__102 - 98)) | (1usize << (T__103 - 98)) | (1usize << (T__104 - 98)) | (1usize << (T__105 - 98)) | (1usize << (T__106 - 98)) | (1usize << (T__107 - 98)) | (1usize << (T__108 - 98)) | (1usize << (T__109 - 98)) | (1usize << (T__110 - 98)) | (1usize << (T__111 - 98)) | (1usize << (T__112 - 98)) | (1usize << (T__113 - 98)) | (1usize << (INTEGER - 98)) | (1usize << (DECIMAL - 98)) | (1usize << (BOOLEAN - 98)))) != 0) || ((((_la - 130)) & !0x3f) == 0 && ((1usize << (_la - 130)) & ((1usize << (IDENTIFIER - 130)) | (1usize << (STRING - 130)) | (1usize << (LBRACKET - 130)) | (1usize << (LPAREN - 130)) | (1usize << (LBRACE - 130)) | (1usize << (MINUS - 130)))) != 0)) {break}
			}
			recog.base.set_state(728);
			recog.base.match_token(T__4,&mut recog.err_handler)?;

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

impl<'input> TransformDSLParserContext<'input> for OnErrorBlockContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for OnErrorBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_onErrorBlock(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_onErrorBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for OnErrorBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_onErrorBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for OnErrorBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_onErrorBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_onErrorBlock }
}
antlr_rust::tid!{OnErrorBlockContextExt<'a>}

impl<'input> OnErrorBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OnErrorBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OnErrorBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OnErrorBlockContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<OnErrorBlockContextExt<'input>>{

fn errorStatement_all(&self) ->  Vec<Rc<ErrorStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn errorStatement(&self, i: usize) -> Option<Rc<ErrorStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OnErrorBlockContextAttrs<'input> for OnErrorBlockContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn onErrorBlock(&mut self,)
	-> Result<Rc<OnErrorBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OnErrorBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_onErrorBlock);
        let mut _localctx: Rc<OnErrorBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(730);
			recog.base.match_token(T__68,&mut recog.err_handler)?;

			recog.base.set_state(732); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule errorStatement*/
				recog.base.set_state(731);
				recog.errorStatement()?;

				}
				}
				recog.base.set_state(734); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(((((_la - 70)) & !0x3f) == 0 && ((1usize << (_la - 70)) & ((1usize << (T__69 - 70)) | (1usize << (T__70 - 70)) | (1usize << (T__71 - 70)) | (1usize << (T__72 - 70)) | (1usize << (T__73 - 70)) | (1usize << (T__74 - 70)) | (1usize << (T__79 - 70)))) != 0) || _la==DEFAULT_KW) {break}
			}
			recog.base.set_state(736);
			recog.base.match_token(T__4,&mut recog.err_handler)?;

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
//------------------- errorStatement ----------------
pub type ErrorStatementContextAll<'input> = ErrorStatementContext<'input>;


pub type ErrorStatementContext<'input> = BaseParserRuleContext<'input,ErrorStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ErrorStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ErrorStatementContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ErrorStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_errorStatement(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_errorStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ErrorStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_errorStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ErrorStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_errorStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_errorStatement }
}
antlr_rust::tid!{ErrorStatementContextExt<'a>}

impl<'input> ErrorStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ErrorStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ErrorStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ErrorStatementContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ErrorStatementContextExt<'input>>{

fn errorAction(&self) -> Option<Rc<ErrorActionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn logErrorCall(&self) -> Option<Rc<LogErrorCallContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn emitStatement(&self) -> Option<Rc<EmitStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn rejectStatement(&self) -> Option<Rc<RejectStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ErrorStatementContextAttrs<'input> for ErrorStatementContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn errorStatement(&mut self,)
	-> Result<Rc<ErrorStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ErrorStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 120, RULE_errorStatement);
        let mut _localctx: Rc<ErrorStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(742);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__69 | T__70 | T__71 | T__72 | DEFAULT_KW 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule errorAction*/
					recog.base.set_state(738);
					recog.errorAction()?;

					}
				}

			 T__73 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule logErrorCall*/
					recog.base.set_state(739);
					recog.logErrorCall()?;

					}
				}

			 T__74 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule emitStatement*/
					recog.base.set_state(740);
					recog.emitStatement()?;

					}
				}

			 T__79 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule rejectStatement*/
					recog.base.set_state(741);
					recog.rejectStatement()?;

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
//------------------- errorAction ----------------
pub type ErrorActionContextAll<'input> = ErrorActionContext<'input>;


pub type ErrorActionContext<'input> = BaseParserRuleContext<'input,ErrorActionContextExt<'input>>;

#[derive(Clone)]
pub struct ErrorActionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ErrorActionContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ErrorActionContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_errorAction(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_errorAction(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ErrorActionContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_errorAction(self);
	}
}

impl<'input> CustomRuleContext<'input> for ErrorActionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_errorAction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_errorAction }
}
antlr_rust::tid!{ErrorActionContextExt<'a>}

impl<'input> ErrorActionContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ErrorActionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ErrorActionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ErrorActionContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ErrorActionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn errorActionType(&self) -> Option<Rc<ErrorActionTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DEFAULT_KW
/// Returns `None` if there is no child corresponding to token DEFAULT_KW
fn DEFAULT_KW(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(DEFAULT_KW, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn logLevel(&self) -> Option<Rc<LogLevelContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}

}

impl<'input> ErrorActionContextAttrs<'input> for ErrorActionContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn errorAction(&mut self,)
	-> Result<Rc<ErrorActionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ErrorActionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_errorAction);
        let mut _localctx: Rc<ErrorActionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(759);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__69 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(744);
					recog.base.match_token(T__69,&mut recog.err_handler)?;

					recog.base.set_state(745);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule errorActionType*/
					recog.base.set_state(746);
					recog.errorActionType()?;

					}
				}

			 DEFAULT_KW 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(747);
					recog.base.match_token(DEFAULT_KW,&mut recog.err_handler)?;

					recog.base.set_state(748);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(749);
					recog.expression_rec(0)?;

					}
				}

			 T__70 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(750);
					recog.base.match_token(T__70,&mut recog.err_handler)?;

					recog.base.set_state(751);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule logLevel*/
					recog.base.set_state(752);
					recog.logLevel()?;

					}
				}

			 T__71 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(753);
					recog.base.match_token(T__71,&mut recog.err_handler)?;

					recog.base.set_state(754);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(755);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

			 T__72 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(756);
					recog.base.match_token(T__72,&mut recog.err_handler)?;

					recog.base.set_state(757);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(758);
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
//------------------- logErrorCall ----------------
pub type LogErrorCallContextAll<'input> = LogErrorCallContext<'input>;


pub type LogErrorCallContext<'input> = BaseParserRuleContext<'input,LogErrorCallContextExt<'input>>;

#[derive(Clone)]
pub struct LogErrorCallContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for LogErrorCallContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for LogErrorCallContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_logErrorCall(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_logErrorCall(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for LogErrorCallContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_logErrorCall(self);
	}
}

impl<'input> CustomRuleContext<'input> for LogErrorCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logErrorCall }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logErrorCall }
}
antlr_rust::tid!{LogErrorCallContextExt<'a>}

impl<'input> LogErrorCallContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LogErrorCallContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LogErrorCallContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LogErrorCallContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<LogErrorCallContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> LogErrorCallContextAttrs<'input> for LogErrorCallContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn logErrorCall(&mut self,)
	-> Result<Rc<LogErrorCallContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LogErrorCallContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 124, RULE_logErrorCall);
        let mut _localctx: Rc<LogErrorCallContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(761);
			recog.base.match_token(T__73,&mut recog.err_handler)?;

			recog.base.set_state(762);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			recog.base.set_state(763);
			recog.base.match_token(STRING,&mut recog.err_handler)?;

			recog.base.set_state(764);
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
//------------------- emitStatement ----------------
pub type EmitStatementContextAll<'input> = EmitStatementContext<'input>;


pub type EmitStatementContext<'input> = BaseParserRuleContext<'input,EmitStatementContextExt<'input>>;

#[derive(Clone)]
pub struct EmitStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for EmitStatementContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for EmitStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_emitStatement(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_emitStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for EmitStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_emitStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for EmitStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_emitStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_emitStatement }
}
antlr_rust::tid!{EmitStatementContextExt<'a>}

impl<'input> EmitStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EmitStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EmitStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EmitStatementContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<EmitStatementContextExt<'input>>{

fn emitMode(&self) -> Option<Rc<EmitModeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EmitStatementContextAttrs<'input> for EmitStatementContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn emitStatement(&mut self,)
	-> Result<Rc<EmitStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EmitStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_emitStatement);
        let mut _localctx: Rc<EmitStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(766);
			recog.base.match_token(T__74,&mut recog.err_handler)?;

			recog.base.set_state(767);
			recog.base.match_token(T__75,&mut recog.err_handler)?;

			/*InvokeRule emitMode*/
			recog.base.set_state(768);
			recog.emitMode()?;

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
//------------------- emitMode ----------------
pub type EmitModeContextAll<'input> = EmitModeContext<'input>;


pub type EmitModeContext<'input> = BaseParserRuleContext<'input,EmitModeContextExt<'input>>;

#[derive(Clone)]
pub struct EmitModeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for EmitModeContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for EmitModeContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_emitMode(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_emitMode(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for EmitModeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_emitMode(self);
	}
}

impl<'input> CustomRuleContext<'input> for EmitModeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_emitMode }
	//fn type_rule_index() -> usize where Self: Sized { RULE_emitMode }
}
antlr_rust::tid!{EmitModeContextExt<'a>}

impl<'input> EmitModeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EmitModeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EmitModeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EmitModeContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<EmitModeContextExt<'input>>{


}

impl<'input> EmitModeContextAttrs<'input> for EmitModeContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn emitMode(&mut self,)
	-> Result<Rc<EmitModeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EmitModeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 128, RULE_emitMode);
        let mut _localctx: Rc<EmitModeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(775);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__76 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(770);
					recog.base.match_token(T__76,&mut recog.err_handler)?;

					}
				}

			 T__77 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(771);
					recog.base.match_token(T__77,&mut recog.err_handler)?;

					recog.base.set_state(773);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__78 {
						{
						recog.base.set_state(772);
						recog.base.match_token(T__78,&mut recog.err_handler)?;

						}
					}

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
//------------------- rejectStatement ----------------
pub type RejectStatementContextAll<'input> = RejectStatementContext<'input>;


pub type RejectStatementContext<'input> = BaseParserRuleContext<'input,RejectStatementContextExt<'input>>;

#[derive(Clone)]
pub struct RejectStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for RejectStatementContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for RejectStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_rejectStatement(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_rejectStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for RejectStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_rejectStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for RejectStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_rejectStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_rejectStatement }
}
antlr_rust::tid!{RejectStatementContextExt<'a>}

impl<'input> RejectStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RejectStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RejectStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RejectStatementContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<RejectStatementContextExt<'input>>{

fn rejectArg(&self) -> Option<Rc<RejectArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RejectStatementContextAttrs<'input> for RejectStatementContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn rejectStatement(&mut self,)
	-> Result<Rc<RejectStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RejectStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 130, RULE_rejectStatement);
        let mut _localctx: Rc<RejectStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(777);
			recog.base.match_token(T__79,&mut recog.err_handler)?;

			recog.base.set_state(778);
			recog.base.match_token(T__75,&mut recog.err_handler)?;

			/*InvokeRule rejectArg*/
			recog.base.set_state(779);
			recog.rejectArg()?;

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
//------------------- rejectArg ----------------
pub type RejectArgContextAll<'input> = RejectArgContext<'input>;


pub type RejectArgContext<'input> = BaseParserRuleContext<'input,RejectArgContextExt<'input>>;

#[derive(Clone)]
pub struct RejectArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for RejectArgContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for RejectArgContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_rejectArg(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_rejectArg(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for RejectArgContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_rejectArg(self);
	}
}

impl<'input> CustomRuleContext<'input> for RejectArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_rejectArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_rejectArg }
}
antlr_rust::tid!{RejectArgContextExt<'a>}

impl<'input> RejectArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RejectArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RejectArgContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RejectArgContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<RejectArgContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}

}

impl<'input> RejectArgContextAttrs<'input> for RejectArgContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn rejectArg(&mut self,)
	-> Result<Rc<RejectArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RejectArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 132, RULE_rejectArg);
        let mut _localctx: Rc<RejectArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(784);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__62 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(781);
					recog.base.match_token(T__62,&mut recog.err_handler)?;

					recog.base.set_state(782);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}

			 STRING 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(783);
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
//------------------- errorActionType ----------------
pub type ErrorActionTypeContextAll<'input> = ErrorActionTypeContext<'input>;


pub type ErrorActionTypeContext<'input> = BaseParserRuleContext<'input,ErrorActionTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ErrorActionTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ErrorActionTypeContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ErrorActionTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_errorActionType(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_errorActionType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ErrorActionTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_errorActionType(self);
	}
}

impl<'input> CustomRuleContext<'input> for ErrorActionTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_errorActionType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_errorActionType }
}
antlr_rust::tid!{ErrorActionTypeContextExt<'a>}

impl<'input> ErrorActionTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ErrorActionTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ErrorActionTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ErrorActionTypeContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ErrorActionTypeContextExt<'input>>{


}

impl<'input> ErrorActionTypeContextAttrs<'input> for ErrorActionTypeContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn errorActionType(&mut self,)
	-> Result<Rc<ErrorActionTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ErrorActionTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 134, RULE_errorActionType);
        let mut _localctx: Rc<ErrorActionTypeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(786);
			_la = recog.base.input.la(1);
			if { !(((((_la - 80)) & !0x3f) == 0 && ((1usize << (_la - 80)) & ((1usize << (T__79 - 80)) | (1usize << (T__80 - 80)) | (1usize << (T__81 - 80)) | (1usize << (T__82 - 80)))) != 0)) } {
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
//------------------- logLevel ----------------
pub type LogLevelContextAll<'input> = LogLevelContext<'input>;


pub type LogLevelContext<'input> = BaseParserRuleContext<'input,LogLevelContextExt<'input>>;

#[derive(Clone)]
pub struct LogLevelContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for LogLevelContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for LogLevelContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_logLevel(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_logLevel(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for LogLevelContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_logLevel(self);
	}
}

impl<'input> CustomRuleContext<'input> for LogLevelContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logLevel }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logLevel }
}
antlr_rust::tid!{LogLevelContextExt<'a>}

impl<'input> LogLevelContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LogLevelContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LogLevelContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LogLevelContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<LogLevelContextExt<'input>>{


}

impl<'input> LogLevelContextAttrs<'input> for LogLevelContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn logLevel(&mut self,)
	-> Result<Rc<LogLevelContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LogLevelContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 136, RULE_logLevel);
        let mut _localctx: Rc<LogLevelContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(788);
			_la = recog.base.input.la(1);
			if { !(((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (T__64 - 65)) | (1usize << (T__65 - 65)) | (1usize << (T__66 - 65)) | (1usize << (T__83 - 65)))) != 0)) } {
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
//------------------- onInvalidBlock ----------------
pub type OnInvalidBlockContextAll<'input> = OnInvalidBlockContext<'input>;


pub type OnInvalidBlockContext<'input> = BaseParserRuleContext<'input,OnInvalidBlockContextExt<'input>>;

#[derive(Clone)]
pub struct OnInvalidBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for OnInvalidBlockContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for OnInvalidBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_onInvalidBlock(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_onInvalidBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for OnInvalidBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_onInvalidBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for OnInvalidBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_onInvalidBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_onInvalidBlock }
}
antlr_rust::tid!{OnInvalidBlockContextExt<'a>}

impl<'input> OnInvalidBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OnInvalidBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OnInvalidBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OnInvalidBlockContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<OnInvalidBlockContextExt<'input>>{

fn invalidAction_all(&self) ->  Vec<Rc<InvalidActionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn invalidAction(&self, i: usize) -> Option<Rc<InvalidActionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OnInvalidBlockContextAttrs<'input> for OnInvalidBlockContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn onInvalidBlock(&mut self,)
	-> Result<Rc<OnInvalidBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OnInvalidBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 138, RULE_onInvalidBlock);
        let mut _localctx: Rc<OnInvalidBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(790);
			recog.base.match_token(T__84,&mut recog.err_handler)?;

			recog.base.set_state(792); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule invalidAction*/
				recog.base.set_state(791);
				recog.invalidAction()?;

				}
				}
				recog.base.set_state(794); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(((((_la - 72)) & !0x3f) == 0 && ((1usize << (_la - 72)) & ((1usize << (T__71 - 72)) | (1usize << (T__72 - 72)) | (1usize << (T__79 - 72)) | (1usize << (T__85 - 72)) | (1usize << (T__86 - 72)))) != 0)) {break}
			}
			recog.base.set_state(796);
			recog.base.match_token(T__4,&mut recog.err_handler)?;

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
//------------------- invalidAction ----------------
pub type InvalidActionContextAll<'input> = InvalidActionContext<'input>;


pub type InvalidActionContext<'input> = BaseParserRuleContext<'input,InvalidActionContextExt<'input>>;

#[derive(Clone)]
pub struct InvalidActionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for InvalidActionContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for InvalidActionContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_invalidAction(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_invalidAction(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for InvalidActionContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_invalidAction(self);
	}
}

impl<'input> CustomRuleContext<'input> for InvalidActionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_invalidAction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_invalidAction }
}
antlr_rust::tid!{InvalidActionContextExt<'a>}

impl<'input> InvalidActionContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InvalidActionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InvalidActionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InvalidActionContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<InvalidActionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token BOOLEAN
/// Returns `None` if there is no child corresponding to token BOOLEAN
fn BOOLEAN(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(BOOLEAN, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> InvalidActionContextAttrs<'input> for InvalidActionContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn invalidAction(&mut self,)
	-> Result<Rc<InvalidActionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InvalidActionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 140, RULE_invalidAction);
        let mut _localctx: Rc<InvalidActionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(813);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__79 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(798);
					recog.base.match_token(T__79,&mut recog.err_handler)?;

					recog.base.set_state(799);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(800);
					recog.base.match_token(BOOLEAN,&mut recog.err_handler)?;

					}
				}

			 T__72 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(801);
					recog.base.match_token(T__72,&mut recog.err_handler)?;

					recog.base.set_state(802);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(803);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}

			 T__85 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(804);
					recog.base.match_token(T__85,&mut recog.err_handler)?;

					recog.base.set_state(805);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(806);
					recog.expression_rec(0)?;

					}
				}

			 T__71 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(807);
					recog.base.match_token(T__71,&mut recog.err_handler)?;

					recog.base.set_state(808);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(809);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

			 T__86 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(810);
					recog.base.match_token(T__86,&mut recog.err_handler)?;

					recog.base.set_state(811);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(812);
					recog.base.match_token(BOOLEAN,&mut recog.err_handler)?;

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
//------------------- onChangeBlock ----------------
pub type OnChangeBlockContextAll<'input> = OnChangeBlockContext<'input>;


pub type OnChangeBlockContext<'input> = BaseParserRuleContext<'input,OnChangeBlockContextExt<'input>>;

#[derive(Clone)]
pub struct OnChangeBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for OnChangeBlockContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for OnChangeBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_onChangeBlock(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_onChangeBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for OnChangeBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_onChangeBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for OnChangeBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_onChangeBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_onChangeBlock }
}
antlr_rust::tid!{OnChangeBlockContextExt<'a>}

impl<'input> OnChangeBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OnChangeBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OnChangeBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OnChangeBlockContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<OnChangeBlockContextExt<'input>>{

fn fieldArray(&self) -> Option<Rc<FieldArrayContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn recalculateBlock(&self) -> Option<Rc<RecalculateBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OnChangeBlockContextAttrs<'input> for OnChangeBlockContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn onChangeBlock(&mut self,)
	-> Result<Rc<OnChangeBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OnChangeBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 142, RULE_onChangeBlock);
        let mut _localctx: Rc<OnChangeBlockContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(815);
			recog.base.match_token(T__87,&mut recog.err_handler)?;

			/*InvokeRule fieldArray*/
			recog.base.set_state(816);
			recog.fieldArray()?;

			/*InvokeRule recalculateBlock*/
			recog.base.set_state(817);
			recog.recalculateBlock()?;

			recog.base.set_state(818);
			recog.base.match_token(T__4,&mut recog.err_handler)?;

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
//------------------- recalculateBlock ----------------
pub type RecalculateBlockContextAll<'input> = RecalculateBlockContext<'input>;


pub type RecalculateBlockContext<'input> = BaseParserRuleContext<'input,RecalculateBlockContextExt<'input>>;

#[derive(Clone)]
pub struct RecalculateBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for RecalculateBlockContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for RecalculateBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_recalculateBlock(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_recalculateBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for RecalculateBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_recalculateBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for RecalculateBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_recalculateBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_recalculateBlock }
}
antlr_rust::tid!{RecalculateBlockContextExt<'a>}

impl<'input> RecalculateBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RecalculateBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RecalculateBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RecalculateBlockContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<RecalculateBlockContextExt<'input>>{

fn assignment_all(&self) ->  Vec<Rc<AssignmentContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn assignment(&self, i: usize) -> Option<Rc<AssignmentContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> RecalculateBlockContextAttrs<'input> for RecalculateBlockContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn recalculateBlock(&mut self,)
	-> Result<Rc<RecalculateBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RecalculateBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 144, RULE_recalculateBlock);
        let mut _localctx: Rc<RecalculateBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(820);
			recog.base.match_token(T__88,&mut recog.err_handler)?;

			recog.base.set_state(822); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule assignment*/
				recog.base.set_state(821);
				recog.assignment()?;

				}
				}
				recog.base.set_state(824); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9) | (1usize << T__23) | (1usize << T__26) | (1usize << T__27))) != 0) || ((((_la - 40)) & !0x3f) == 0 && ((1usize << (_la - 40)) & ((1usize << (T__39 - 40)) | (1usize << (T__40 - 40)) | (1usize << (T__41 - 40)) | (1usize << (T__42 - 40)) | (1usize << (T__61 - 40)) | (1usize << (T__62 - 40)) | (1usize << (T__63 - 40)) | (1usize << (T__64 - 40)) | (1usize << (T__69 - 40)))) != 0) || _la==T__78 || _la==IDENTIFIER) {break}
			}
			recog.base.set_state(826);
			recog.base.match_token(T__4,&mut recog.err_handler)?;

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

impl<'input> TransformDSLParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expression(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::tid!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{

fn unaryOp(&self) -> Option<Rc<UnaryOpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn primaryExpression(&self) -> Option<Rc<PrimaryExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn arithmeticOp(&self) -> Option<Rc<ArithmeticOpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn comparisonOp(&self) -> Option<Rc<ComparisonOpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn logicalOp(&self) -> Option<Rc<LogicalOpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DEFAULT_KW
/// Returns `None` if there is no child corresponding to token DEFAULT_KW
fn DEFAULT_KW(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(DEFAULT_KW, 0)
}
/// Retrieves first TerminalNode corresponding to token COALESCE
/// Returns `None` if there is no child corresponding to token COALESCE
fn COALESCE(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COALESCE, 0)
}
fn listLiteral(&self) -> Option<Rc<ListLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LBRACKET
/// Returns `None` if there is no child corresponding to token LBRACKET
fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(LBRACKET, 0)
}
fn listElements(&self) -> Option<Rc<ListElementsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RBRACKET
/// Returns `None` if there is no child corresponding to token RBRACKET
fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(RBRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}

}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		self.expression_rec(0)
	}

	fn expression_rec(&mut self, _p: isize)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 146, RULE_expression, _p);
	    let mut _localctx: Rc<ExpressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 146;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(833);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(88,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule unaryOp*/
					recog.base.set_state(829);
					recog.unaryOp()?;

					/*InvokeRule expression*/
					recog.base.set_state(830);
					recog.expression_rec(14)?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule primaryExpression*/
					recog.base.set_state(832);
					recog.primaryExpression()?;

					}
				}

				_ => {}
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(888);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(90,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(886);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(89,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(835);
							if !({recog.precpred(None, 13)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 13)".to_owned()), None))?;
							}
							/*InvokeRule arithmeticOp*/
							recog.base.set_state(836);
							recog.arithmeticOp()?;

							/*InvokeRule expression*/
							recog.base.set_state(837);
							recog.expression_rec(14)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(839);
							if !({recog.precpred(None, 12)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 12)".to_owned()), None))?;
							}
							/*InvokeRule comparisonOp*/
							recog.base.set_state(840);
							recog.comparisonOp()?;

							/*InvokeRule expression*/
							recog.base.set_state(841);
							recog.expression_rec(13)?;

							}
						}
					,
						3 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(843);
							if !({recog.precpred(None, 11)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 11)".to_owned()), None))?;
							}
							/*InvokeRule logicalOp*/
							recog.base.set_state(844);
							recog.logicalOp()?;

							/*InvokeRule expression*/
							recog.base.set_state(845);
							recog.expression_rec(12)?;

							}
						}
					,
						4 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(847);
							if !({recog.precpred(None, 10)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 10)".to_owned()), None))?;
							}
							recog.base.set_state(848);
							_la = recog.base.input.la(1);
							if { !(_la==DEFAULT_KW || _la==COALESCE) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expression*/
							recog.base.set_state(849);
							recog.expression_rec(11)?;

							}
						}
					,
						5 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(850);
							if !({recog.precpred(None, 9)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 9)".to_owned()), None))?;
							}
							recog.base.set_state(851);
							recog.base.match_token(T__89,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(852);
							recog.expression_rec(0)?;

							recog.base.set_state(853);
							recog.base.match_token(T__90,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(854);
							recog.expression_rec(10)?;

							}
						}
					,
						6 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(856);
							if !({recog.precpred(None, 8)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 8)".to_owned()), None))?;
							}
							recog.base.set_state(857);
							recog.base.match_token(T__91,&mut recog.err_handler)?;

							recog.base.set_state(858);
							recog.base.match_token(T__89,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(859);
							recog.expression_rec(0)?;

							recog.base.set_state(860);
							recog.base.match_token(T__90,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(861);
							recog.expression_rec(9)?;

							}
						}
					,
						7 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(863);
							if !({recog.precpred(None, 7)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 7)".to_owned()), None))?;
							}
							recog.base.set_state(864);
							recog.base.match_token(T__92,&mut recog.err_handler)?;

							/*InvokeRule listLiteral*/
							recog.base.set_state(865);
							recog.listLiteral()?;

							}
						}
					,
						8 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(866);
							if !({recog.precpred(None, 6)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
							}
							recog.base.set_state(867);
							recog.base.match_token(T__92,&mut recog.err_handler)?;

							recog.base.set_state(868);
							recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

							/*InvokeRule listElements*/
							recog.base.set_state(869);
							recog.listElements()?;

							recog.base.set_state(870);
							recog.base.match_token(RBRACKET,&mut recog.err_handler)?;

							}
						}
					,
						9 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(872);
							if !({recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(873);
							recog.base.match_token(T__91,&mut recog.err_handler)?;

							recog.base.set_state(874);
							recog.base.match_token(T__92,&mut recog.err_handler)?;

							/*InvokeRule listLiteral*/
							recog.base.set_state(875);
							recog.listLiteral()?;

							}
						}
					,
						10 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(876);
							if !({recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(877);
							recog.base.match_token(T__93,&mut recog.err_handler)?;

							recog.base.set_state(878);
							recog.base.match_token(T__94,&mut recog.err_handler)?;

							}
						}
					,
						11 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(879);
							if !({recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(880);
							recog.base.match_token(T__93,&mut recog.err_handler)?;

							recog.base.set_state(881);
							recog.base.match_token(T__91,&mut recog.err_handler)?;

							recog.base.set_state(882);
							recog.base.match_token(T__94,&mut recog.err_handler)?;

							}
						}
					,
						12 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(883);
							if !({recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(884);
							recog.base.match_token(T__95,&mut recog.err_handler)?;

							recog.base.set_state(885);
							recog.base.match_token(STRING,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(890);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(90,&mut recog.base)?;
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
//------------------- primaryExpression ----------------
pub type PrimaryExpressionContextAll<'input> = PrimaryExpressionContext<'input>;


pub type PrimaryExpressionContext<'input> = BaseParserRuleContext<'input,PrimaryExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct PrimaryExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for PrimaryExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for PrimaryExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_primaryExpression(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_primaryExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for PrimaryExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_primaryExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for PrimaryExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primaryExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primaryExpression }
}
antlr_rust::tid!{PrimaryExpressionContextExt<'a>}

impl<'input> PrimaryExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PrimaryExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrimaryExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PrimaryExpressionContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<PrimaryExpressionContextExt<'input>>{

fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldPath(&self) -> Option<Rc<FieldPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functionCall(&self) -> Option<Rc<FunctionCallContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn whenExpression(&self) -> Option<Rc<WhenExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn indexExpression(&self) -> Option<Rc<IndexExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn optionalChainExpression(&self) -> Option<Rc<OptionalChainExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn objectLiteral(&self) -> Option<Rc<ObjectLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lambdaExpression(&self) -> Option<Rc<LambdaExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn listLiteral(&self) -> Option<Rc<ListLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PrimaryExpressionContextAttrs<'input> for PrimaryExpressionContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn primaryExpression(&mut self,)
	-> Result<Rc<PrimaryExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrimaryExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 148, RULE_primaryExpression);
        let mut _localctx: Rc<PrimaryExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(904);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(91,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule literal*/
					recog.base.set_state(891);
					recog.literal()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule fieldPath*/
					recog.base.set_state(892);
					recog.fieldPath()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule functionCall*/
					recog.base.set_state(893);
					recog.functionCall()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(894);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(895);
					recog.expression_rec(0)?;

					recog.base.set_state(896);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule whenExpression*/
					recog.base.set_state(898);
					recog.whenExpression()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule indexExpression*/
					recog.base.set_state(899);
					recog.indexExpression()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule optionalChainExpression*/
					recog.base.set_state(900);
					recog.optionalChainExpression()?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule objectLiteral*/
					recog.base.set_state(901);
					recog.objectLiteral()?;

					}
				}
			,
				9 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					/*InvokeRule lambdaExpression*/
					recog.base.set_state(902);
					recog.lambdaExpression()?;

					}
				}
			,
				10 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 10);
					recog.base.enter_outer_alt(None, 10);
					{
					/*InvokeRule listLiteral*/
					recog.base.set_state(903);
					recog.listLiteral()?;

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
//------------------- objectLiteral ----------------
pub type ObjectLiteralContextAll<'input> = ObjectLiteralContext<'input>;


pub type ObjectLiteralContext<'input> = BaseParserRuleContext<'input,ObjectLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct ObjectLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ObjectLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ObjectLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_objectLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_objectLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ObjectLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_objectLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for ObjectLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_objectLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_objectLiteral }
}
antlr_rust::tid!{ObjectLiteralContextExt<'a>}

impl<'input> ObjectLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ObjectLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ObjectLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ObjectLiteralContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ObjectLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
fn objectField_all(&self) ->  Vec<Rc<ObjectFieldContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn objectField(&self, i: usize) -> Option<Rc<ObjectFieldContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ObjectLiteralContextAttrs<'input> for ObjectLiteralContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn objectLiteral(&mut self,)
	-> Result<Rc<ObjectLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ObjectLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 150, RULE_objectLiteral);
        let mut _localctx: Rc<ObjectLiteralContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(919);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(93,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(906);
					recog.base.match_token(LBRACE,&mut recog.err_handler)?;

					/*InvokeRule objectField*/
					recog.base.set_state(907);
					recog.objectField()?;

					recog.base.set_state(912);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==COMMA {
						{
						{
						recog.base.set_state(908);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						/*InvokeRule objectField*/
						recog.base.set_state(909);
						recog.objectField()?;

						}
						}
						recog.base.set_state(914);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(915);
					recog.base.match_token(RBRACE,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(917);
					recog.base.match_token(LBRACE,&mut recog.err_handler)?;

					recog.base.set_state(918);
					recog.base.match_token(RBRACE,&mut recog.err_handler)?;

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
//------------------- objectField ----------------
pub type ObjectFieldContextAll<'input> = ObjectFieldContext<'input>;


pub type ObjectFieldContext<'input> = BaseParserRuleContext<'input,ObjectFieldContextExt<'input>>;

#[derive(Clone)]
pub struct ObjectFieldContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ObjectFieldContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ObjectFieldContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_objectField(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_objectField(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ObjectFieldContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_objectField(self);
	}
}

impl<'input> CustomRuleContext<'input> for ObjectFieldContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_objectField }
	//fn type_rule_index() -> usize where Self: Sized { RULE_objectField }
}
antlr_rust::tid!{ObjectFieldContextExt<'a>}

impl<'input> ObjectFieldContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ObjectFieldContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ObjectFieldContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ObjectFieldContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ObjectFieldContextExt<'input>>{

fn objectFieldName(&self) -> Option<Rc<ObjectFieldNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ObjectFieldContextAttrs<'input> for ObjectFieldContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn objectField(&mut self,)
	-> Result<Rc<ObjectFieldContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ObjectFieldContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 152, RULE_objectField);
        let mut _localctx: Rc<ObjectFieldContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule objectFieldName*/
			recog.base.set_state(921);
			recog.objectFieldName()?;

			recog.base.set_state(922);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(923);
			recog.expression_rec(0)?;

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
//------------------- objectFieldName ----------------
pub type ObjectFieldNameContextAll<'input> = ObjectFieldNameContext<'input>;


pub type ObjectFieldNameContext<'input> = BaseParserRuleContext<'input,ObjectFieldNameContextExt<'input>>;

#[derive(Clone)]
pub struct ObjectFieldNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ObjectFieldNameContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ObjectFieldNameContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_objectFieldName(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_objectFieldName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ObjectFieldNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_objectFieldName(self);
	}
}

impl<'input> CustomRuleContext<'input> for ObjectFieldNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_objectFieldName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_objectFieldName }
}
antlr_rust::tid!{ObjectFieldNameContextExt<'a>}

impl<'input> ObjectFieldNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ObjectFieldNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ObjectFieldNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ObjectFieldNameContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ObjectFieldNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}

}

impl<'input> ObjectFieldNameContextAttrs<'input> for ObjectFieldNameContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn objectFieldName(&mut self,)
	-> Result<Rc<ObjectFieldNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ObjectFieldNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 154, RULE_objectFieldName);
        let mut _localctx: Rc<ObjectFieldNameContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(925);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__6) | (1usize << T__8) | (1usize << T__9) | (1usize << T__23) | (1usize << T__26) | (1usize << T__27))) != 0) || ((((_la - 40)) & !0x3f) == 0 && ((1usize << (_la - 40)) & ((1usize << (T__39 - 40)) | (1usize << (T__40 - 40)) | (1usize << (T__41 - 40)) | (1usize << (T__42 - 40)) | (1usize << (T__61 - 40)) | (1usize << (T__62 - 40)) | (1usize << (T__63 - 40)) | (1usize << (T__64 - 40)) | (1usize << (T__69 - 40)))) != 0) || _la==T__78 || _la==IDENTIFIER || _la==STRING) } {
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
//------------------- lambdaExpression ----------------
pub type LambdaExpressionContextAll<'input> = LambdaExpressionContext<'input>;


pub type LambdaExpressionContext<'input> = BaseParserRuleContext<'input,LambdaExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct LambdaExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for LambdaExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for LambdaExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lambdaExpression(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_lambdaExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for LambdaExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_lambdaExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for LambdaExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lambdaExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lambdaExpression }
}
antlr_rust::tid!{LambdaExpressionContextExt<'a>}

impl<'input> LambdaExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LambdaExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LambdaExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LambdaExpressionContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<LambdaExpressionContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves first TerminalNode corresponding to token ARROW
/// Returns `None` if there is no child corresponding to token ARROW
fn ARROW(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(ARROW, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> LambdaExpressionContextAttrs<'input> for LambdaExpressionContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lambdaExpression(&mut self,)
	-> Result<Rc<LambdaExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LambdaExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 156, RULE_lambdaExpression);
        let mut _localctx: Rc<LambdaExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(942);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(927);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(928);
					recog.base.match_token(ARROW,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(929);
					recog.expression_rec(0)?;

					}
				}

			 LPAREN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(930);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					recog.base.set_state(931);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(936);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==COMMA {
						{
						{
						recog.base.set_state(932);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						recog.base.set_state(933);
						recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(938);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(939);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					recog.base.set_state(940);
					recog.base.match_token(ARROW,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(941);
					recog.expression_rec(0)?;

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
//------------------- listElements ----------------
pub type ListElementsContextAll<'input> = ListElementsContext<'input>;


pub type ListElementsContext<'input> = BaseParserRuleContext<'input,ListElementsContextExt<'input>>;

#[derive(Clone)]
pub struct ListElementsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ListElementsContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ListElementsContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_listElements(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_listElements(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ListElementsContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_listElements(self);
	}
}

impl<'input> CustomRuleContext<'input> for ListElementsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listElements }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listElements }
}
antlr_rust::tid!{ListElementsContextExt<'a>}

impl<'input> ListElementsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ListElementsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListElementsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ListElementsContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ListElementsContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ListElementsContextAttrs<'input> for ListElementsContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn listElements(&mut self,)
	-> Result<Rc<ListElementsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ListElementsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 158, RULE_listElements);
        let mut _localctx: Rc<ListElementsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(944);
			recog.expression_rec(0)?;

			recog.base.set_state(949);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(945);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(946);
				recog.expression_rec(0)?;

				}
				}
				recog.base.set_state(951);
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
//------------------- whenExpression ----------------
pub type WhenExpressionContextAll<'input> = WhenExpressionContext<'input>;


pub type WhenExpressionContext<'input> = BaseParserRuleContext<'input,WhenExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct WhenExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for WhenExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for WhenExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_whenExpression(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_whenExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for WhenExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_whenExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for WhenExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_whenExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_whenExpression }
}
antlr_rust::tid!{WhenExpressionContextExt<'a>}

impl<'input> WhenExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<WhenExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,WhenExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait WhenExpressionContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<WhenExpressionContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COLON in current rule
fn COLON_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COLON, starting from 0.
/// Returns `None` if number of children corresponding to token COLON is less or equal than `i`.
fn COLON(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, i)
}

}

impl<'input> WhenExpressionContextAttrs<'input> for WhenExpressionContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn whenExpression(&mut self,)
	-> Result<Rc<WhenExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = WhenExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 160, RULE_whenExpression);
        let mut _localctx: Rc<WhenExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(987);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(99,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(952);
					recog.base.match_token(T__54,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(953);
					recog.expression_rec(0)?;

					recog.base.set_state(954);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(955);
					recog.expression_rec(0)?;

					recog.base.set_state(963);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__54 {
						{
						{
						recog.base.set_state(956);
						recog.base.match_token(T__54,&mut recog.err_handler)?;

						/*InvokeRule expression*/
						recog.base.set_state(957);
						recog.expression_rec(0)?;

						recog.base.set_state(958);
						recog.base.match_token(COLON,&mut recog.err_handler)?;

						/*InvokeRule expression*/
						recog.base.set_state(959);
						recog.expression_rec(0)?;

						}
						}
						recog.base.set_state(965);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(966);
					recog.base.match_token(T__55,&mut recog.err_handler)?;

					recog.base.set_state(967);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(968);
					recog.expression_rec(0)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(970);
					recog.base.match_token(T__54,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(971);
					recog.expression_rec(0)?;

					recog.base.set_state(972);
					recog.base.match_token(T__56,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(973);
					recog.expression_rec(0)?;

					recog.base.set_state(981);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__54 {
						{
						{
						recog.base.set_state(974);
						recog.base.match_token(T__54,&mut recog.err_handler)?;

						/*InvokeRule expression*/
						recog.base.set_state(975);
						recog.expression_rec(0)?;

						recog.base.set_state(976);
						recog.base.match_token(T__56,&mut recog.err_handler)?;

						/*InvokeRule expression*/
						recog.base.set_state(977);
						recog.expression_rec(0)?;

						}
						}
						recog.base.set_state(983);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(984);
					recog.base.match_token(T__55,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(985);
					recog.expression_rec(0)?;

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
//------------------- indexExpression ----------------
pub type IndexExpressionContextAll<'input> = IndexExpressionContext<'input>;


pub type IndexExpressionContext<'input> = BaseParserRuleContext<'input,IndexExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct IndexExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for IndexExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for IndexExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_indexExpression(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_indexExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for IndexExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_indexExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for IndexExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_indexExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_indexExpression }
}
antlr_rust::tid!{IndexExpressionContextExt<'a>}

impl<'input> IndexExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IndexExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IndexExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IndexExpressionContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<IndexExpressionContextExt<'input>>{

fn fieldPath(&self) -> Option<Rc<FieldPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LBRACKET
/// Returns `None` if there is no child corresponding to token LBRACKET
fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(LBRACKET, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RBRACKET
/// Returns `None` if there is no child corresponding to token RBRACKET
fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(RBRACKET, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
/// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, i)
}
fn fieldOrKeyword_all(&self) ->  Vec<Rc<FieldOrKeywordContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fieldOrKeyword(&self, i: usize) -> Option<Rc<FieldOrKeywordContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> IndexExpressionContextAttrs<'input> for IndexExpressionContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn indexExpression(&mut self,)
	-> Result<Rc<IndexExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IndexExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 162, RULE_indexExpression);
        let mut _localctx: Rc<IndexExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fieldPath*/
			recog.base.set_state(989);
			recog.fieldPath()?;

			recog.base.set_state(990);
			recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(991);
			recog.expression_rec(0)?;

			recog.base.set_state(992);
			recog.base.match_token(RBRACKET,&mut recog.err_handler)?;

			recog.base.set_state(997);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(100,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(993);
					recog.base.match_token(DOT,&mut recog.err_handler)?;

					/*InvokeRule fieldOrKeyword*/
					recog.base.set_state(994);
					recog.fieldOrKeyword()?;

					}
					} 
				}
				recog.base.set_state(999);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(100,&mut recog.base)?;
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
//------------------- optionalChainExpression ----------------
pub type OptionalChainExpressionContextAll<'input> = OptionalChainExpressionContext<'input>;


pub type OptionalChainExpressionContext<'input> = BaseParserRuleContext<'input,OptionalChainExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OptionalChainExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for OptionalChainExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for OptionalChainExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_optionalChainExpression(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_optionalChainExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for OptionalChainExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_optionalChainExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for OptionalChainExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_optionalChainExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_optionalChainExpression }
}
antlr_rust::tid!{OptionalChainExpressionContextExt<'a>}

impl<'input> OptionalChainExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OptionalChainExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OptionalChainExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OptionalChainExpressionContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<OptionalChainExpressionContextExt<'input>>{

fn fieldPath(&self) -> Option<Rc<FieldPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token OPTIONAL_CHAIN in current rule
fn OPTIONAL_CHAIN_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token OPTIONAL_CHAIN, starting from 0.
/// Returns `None` if number of children corresponding to token OPTIONAL_CHAIN is less or equal than `i`.
fn OPTIONAL_CHAIN(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(OPTIONAL_CHAIN, i)
}
fn fieldOrKeyword_all(&self) ->  Vec<Rc<FieldOrKeywordContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fieldOrKeyword(&self, i: usize) -> Option<Rc<FieldOrKeywordContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OptionalChainExpressionContextAttrs<'input> for OptionalChainExpressionContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn optionalChainExpression(&mut self,)
	-> Result<Rc<OptionalChainExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OptionalChainExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 164, RULE_optionalChainExpression);
        let mut _localctx: Rc<OptionalChainExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fieldPath*/
			recog.base.set_state(1000);
			recog.fieldPath()?;

			recog.base.set_state(1003); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					{
					recog.base.set_state(1001);
					recog.base.match_token(OPTIONAL_CHAIN,&mut recog.err_handler)?;

					/*InvokeRule fieldOrKeyword*/
					recog.base.set_state(1002);
					recog.fieldOrKeyword()?;

					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(1005); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(101,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
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
//------------------- binaryOp ----------------
pub type BinaryOpContextAll<'input> = BinaryOpContext<'input>;


pub type BinaryOpContext<'input> = BaseParserRuleContext<'input,BinaryOpContextExt<'input>>;

#[derive(Clone)]
pub struct BinaryOpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for BinaryOpContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for BinaryOpContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_binaryOp(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_binaryOp(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for BinaryOpContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_binaryOp(self);
	}
}

impl<'input> CustomRuleContext<'input> for BinaryOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_binaryOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_binaryOp }
}
antlr_rust::tid!{BinaryOpContextExt<'a>}

impl<'input> BinaryOpContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BinaryOpContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BinaryOpContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BinaryOpContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<BinaryOpContextExt<'input>>{

fn arithmeticOp(&self) -> Option<Rc<ArithmeticOpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn comparisonOp(&self) -> Option<Rc<ComparisonOpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn logicalOp(&self) -> Option<Rc<LogicalOpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> BinaryOpContextAttrs<'input> for BinaryOpContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn binaryOp(&mut self,)
	-> Result<Rc<BinaryOpContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BinaryOpContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 166, RULE_binaryOp);
        let mut _localctx: Rc<BinaryOpContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1010);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 PLUS | MINUS | STAR | SLASH | PERCENT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule arithmeticOp*/
					recog.base.set_state(1007);
					recog.arithmeticOp()?;

					}
				}

			 T__47 | LANGLE | RANGLE | EQ | NE | LE | GE | NULLSAFE_EQ 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule comparisonOp*/
					recog.base.set_state(1008);
					recog.comparisonOp()?;

					}
				}

			 T__90 | T__96 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule logicalOp*/
					recog.base.set_state(1009);
					recog.logicalOp()?;

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
//------------------- arithmeticOp ----------------
pub type ArithmeticOpContextAll<'input> = ArithmeticOpContext<'input>;


pub type ArithmeticOpContext<'input> = BaseParserRuleContext<'input,ArithmeticOpContextExt<'input>>;

#[derive(Clone)]
pub struct ArithmeticOpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ArithmeticOpContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ArithmeticOpContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_arithmeticOp(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_arithmeticOp(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ArithmeticOpContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_arithmeticOp(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArithmeticOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arithmeticOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arithmeticOp }
}
antlr_rust::tid!{ArithmeticOpContextExt<'a>}

impl<'input> ArithmeticOpContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ArithmeticOpContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArithmeticOpContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ArithmeticOpContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ArithmeticOpContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PLUS
/// Returns `None` if there is no child corresponding to token PLUS
fn PLUS(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(PLUS, 0)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}
/// Retrieves first TerminalNode corresponding to token STAR
/// Returns `None` if there is no child corresponding to token STAR
fn STAR(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(STAR, 0)
}
/// Retrieves first TerminalNode corresponding to token SLASH
/// Returns `None` if there is no child corresponding to token SLASH
fn SLASH(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(SLASH, 0)
}
/// Retrieves first TerminalNode corresponding to token PERCENT
/// Returns `None` if there is no child corresponding to token PERCENT
fn PERCENT(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(PERCENT, 0)
}

}

impl<'input> ArithmeticOpContextAttrs<'input> for ArithmeticOpContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn arithmeticOp(&mut self,)
	-> Result<Rc<ArithmeticOpContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArithmeticOpContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 168, RULE_arithmeticOp);
        let mut _localctx: Rc<ArithmeticOpContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1012);
			_la = recog.base.input.la(1);
			if { !(((((_la - 150)) & !0x3f) == 0 && ((1usize << (_la - 150)) & ((1usize << (PLUS - 150)) | (1usize << (MINUS - 150)) | (1usize << (STAR - 150)) | (1usize << (SLASH - 150)) | (1usize << (PERCENT - 150)))) != 0)) } {
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
//------------------- comparisonOp ----------------
pub type ComparisonOpContextAll<'input> = ComparisonOpContext<'input>;


pub type ComparisonOpContext<'input> = BaseParserRuleContext<'input,ComparisonOpContextExt<'input>>;

#[derive(Clone)]
pub struct ComparisonOpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ComparisonOpContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ComparisonOpContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_comparisonOp(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_comparisonOp(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ComparisonOpContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_comparisonOp(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComparisonOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comparisonOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comparisonOp }
}
antlr_rust::tid!{ComparisonOpContextExt<'a>}

impl<'input> ComparisonOpContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComparisonOpContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComparisonOpContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComparisonOpContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ComparisonOpContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EQ
/// Returns `None` if there is no child corresponding to token EQ
fn EQ(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(EQ, 0)
}
/// Retrieves first TerminalNode corresponding to token NE
/// Returns `None` if there is no child corresponding to token NE
fn NE(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(NE, 0)
}
/// Retrieves first TerminalNode corresponding to token LANGLE
/// Returns `None` if there is no child corresponding to token LANGLE
fn LANGLE(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(LANGLE, 0)
}
/// Retrieves first TerminalNode corresponding to token RANGLE
/// Returns `None` if there is no child corresponding to token RANGLE
fn RANGLE(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(RANGLE, 0)
}
/// Retrieves first TerminalNode corresponding to token LE
/// Returns `None` if there is no child corresponding to token LE
fn LE(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(LE, 0)
}
/// Retrieves first TerminalNode corresponding to token GE
/// Returns `None` if there is no child corresponding to token GE
fn GE(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(GE, 0)
}
/// Retrieves first TerminalNode corresponding to token NULLSAFE_EQ
/// Returns `None` if there is no child corresponding to token NULLSAFE_EQ
fn NULLSAFE_EQ(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(NULLSAFE_EQ, 0)
}

}

impl<'input> ComparisonOpContextAttrs<'input> for ComparisonOpContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn comparisonOp(&mut self,)
	-> Result<Rc<ComparisonOpContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComparisonOpContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 170, RULE_comparisonOp);
        let mut _localctx: Rc<ComparisonOpContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1014);
			_la = recog.base.input.la(1);
			if { !(_la==T__47 || ((((_la - 143)) & !0x3f) == 0 && ((1usize << (_la - 143)) & ((1usize << (LANGLE - 143)) | (1usize << (RANGLE - 143)) | (1usize << (EQ - 143)) | (1usize << (NE - 143)) | (1usize << (LE - 143)) | (1usize << (GE - 143)) | (1usize << (NULLSAFE_EQ - 143)))) != 0)) } {
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
//------------------- logicalOp ----------------
pub type LogicalOpContextAll<'input> = LogicalOpContext<'input>;


pub type LogicalOpContext<'input> = BaseParserRuleContext<'input,LogicalOpContextExt<'input>>;

#[derive(Clone)]
pub struct LogicalOpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for LogicalOpContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for LogicalOpContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_logicalOp(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_logicalOp(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for LogicalOpContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_logicalOp(self);
	}
}

impl<'input> CustomRuleContext<'input> for LogicalOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicalOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicalOp }
}
antlr_rust::tid!{LogicalOpContextExt<'a>}

impl<'input> LogicalOpContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LogicalOpContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LogicalOpContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LogicalOpContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<LogicalOpContextExt<'input>>{


}

impl<'input> LogicalOpContextAttrs<'input> for LogicalOpContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn logicalOp(&mut self,)
	-> Result<Rc<LogicalOpContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LogicalOpContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 172, RULE_logicalOp);
        let mut _localctx: Rc<LogicalOpContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1016);
			_la = recog.base.input.la(1);
			if { !(_la==T__90 || _la==T__96) } {
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
//------------------- unaryOp ----------------
pub type UnaryOpContextAll<'input> = UnaryOpContext<'input>;


pub type UnaryOpContext<'input> = BaseParserRuleContext<'input,UnaryOpContextExt<'input>>;

#[derive(Clone)]
pub struct UnaryOpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for UnaryOpContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for UnaryOpContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_unaryOp(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_unaryOp(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for UnaryOpContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_unaryOp(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnaryOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unaryOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unaryOp }
}
antlr_rust::tid!{UnaryOpContextExt<'a>}

impl<'input> UnaryOpContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UnaryOpContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UnaryOpContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UnaryOpContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<UnaryOpContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}

}

impl<'input> UnaryOpContextAttrs<'input> for UnaryOpContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn unaryOp(&mut self,)
	-> Result<Rc<UnaryOpContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UnaryOpContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 174, RULE_unaryOp);
        let mut _localctx: Rc<UnaryOpContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1018);
			_la = recog.base.input.la(1);
			if { !(_la==T__91 || _la==MINUS) } {
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
//------------------- functionCall ----------------
pub type FunctionCallContextAll<'input> = FunctionCallContext<'input>;


pub type FunctionCallContext<'input> = BaseParserRuleContext<'input,FunctionCallContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionCallContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for FunctionCallContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for FunctionCallContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_functionCall(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_functionCall(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for FunctionCallContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_functionCall(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionCall }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionCall }
}
antlr_rust::tid!{FunctionCallContextExt<'a>}

impl<'input> FunctionCallContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionCallContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionCallContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionCallContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<FunctionCallContextExt<'input>>{

fn functionName(&self) -> Option<Rc<FunctionNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}
fn fieldPath(&self) -> Option<Rc<FieldPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}

}

impl<'input> FunctionCallContextAttrs<'input> for FunctionCallContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionCall(&mut self,)
	-> Result<Rc<FunctionCallContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionCallContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 176, RULE_functionCall);
        let mut _localctx: Rc<FunctionCallContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1050);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(107,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule functionName*/
					recog.base.set_state(1020);
					recog.functionName()?;

					recog.base.set_state(1021);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					recog.base.set_state(1030);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9) | (1usize << T__19) | (1usize << T__23) | (1usize << T__26) | (1usize << T__27) | (1usize << T__28) | (1usize << T__29) | (1usize << T__30))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (T__32 - 33)) | (1usize << (T__36 - 33)) | (1usize << (T__37 - 33)) | (1usize << (T__38 - 33)) | (1usize << (T__39 - 33)) | (1usize << (T__40 - 33)) | (1usize << (T__41 - 33)) | (1usize << (T__42 - 33)) | (1usize << (T__54 - 33)) | (1usize << (T__61 - 33)) | (1usize << (T__62 - 33)) | (1usize << (T__63 - 33)))) != 0) || ((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (T__64 - 65)) | (1usize << (T__69 - 65)) | (1usize << (T__78 - 65)) | (1usize << (T__91 - 65)) | (1usize << (T__94 - 65)))) != 0) || ((((_la - 98)) & !0x3f) == 0 && ((1usize << (_la - 98)) & ((1usize << (T__97 - 98)) | (1usize << (T__98 - 98)) | (1usize << (T__99 - 98)) | (1usize << (T__100 - 98)) | (1usize << (T__101 - 98)) | (1usize << (T__102 - 98)) | (1usize << (T__103 - 98)) | (1usize << (T__104 - 98)) | (1usize << (T__105 - 98)) | (1usize << (T__106 - 98)) | (1usize << (T__107 - 98)) | (1usize << (T__108 - 98)) | (1usize << (T__109 - 98)) | (1usize << (T__110 - 98)) | (1usize << (T__111 - 98)) | (1usize << (T__112 - 98)) | (1usize << (T__113 - 98)) | (1usize << (INTEGER - 98)) | (1usize << (DECIMAL - 98)) | (1usize << (BOOLEAN - 98)))) != 0) || ((((_la - 130)) & !0x3f) == 0 && ((1usize << (_la - 130)) & ((1usize << (IDENTIFIER - 130)) | (1usize << (STRING - 130)) | (1usize << (LBRACKET - 130)) | (1usize << (LPAREN - 130)) | (1usize << (LBRACE - 130)) | (1usize << (MINUS - 130)))) != 0) {
						{
						/*InvokeRule expression*/
						recog.base.set_state(1022);
						recog.expression_rec(0)?;

						recog.base.set_state(1027);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						while _la==COMMA {
							{
							{
							recog.base.set_state(1023);
							recog.base.match_token(COMMA,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(1024);
							recog.expression_rec(0)?;

							}
							}
							recog.base.set_state(1029);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
						}
						}
					}

					recog.base.set_state(1032);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule fieldPath*/
					recog.base.set_state(1034);
					recog.fieldPath()?;

					recog.base.set_state(1035);
					recog.base.match_token(DOT,&mut recog.err_handler)?;

					/*InvokeRule functionName*/
					recog.base.set_state(1036);
					recog.functionName()?;

					recog.base.set_state(1037);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					recog.base.set_state(1046);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9) | (1usize << T__19) | (1usize << T__23) | (1usize << T__26) | (1usize << T__27) | (1usize << T__28) | (1usize << T__29) | (1usize << T__30))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (T__32 - 33)) | (1usize << (T__36 - 33)) | (1usize << (T__37 - 33)) | (1usize << (T__38 - 33)) | (1usize << (T__39 - 33)) | (1usize << (T__40 - 33)) | (1usize << (T__41 - 33)) | (1usize << (T__42 - 33)) | (1usize << (T__54 - 33)) | (1usize << (T__61 - 33)) | (1usize << (T__62 - 33)) | (1usize << (T__63 - 33)))) != 0) || ((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (T__64 - 65)) | (1usize << (T__69 - 65)) | (1usize << (T__78 - 65)) | (1usize << (T__91 - 65)) | (1usize << (T__94 - 65)))) != 0) || ((((_la - 98)) & !0x3f) == 0 && ((1usize << (_la - 98)) & ((1usize << (T__97 - 98)) | (1usize << (T__98 - 98)) | (1usize << (T__99 - 98)) | (1usize << (T__100 - 98)) | (1usize << (T__101 - 98)) | (1usize << (T__102 - 98)) | (1usize << (T__103 - 98)) | (1usize << (T__104 - 98)) | (1usize << (T__105 - 98)) | (1usize << (T__106 - 98)) | (1usize << (T__107 - 98)) | (1usize << (T__108 - 98)) | (1usize << (T__109 - 98)) | (1usize << (T__110 - 98)) | (1usize << (T__111 - 98)) | (1usize << (T__112 - 98)) | (1usize << (T__113 - 98)) | (1usize << (INTEGER - 98)) | (1usize << (DECIMAL - 98)) | (1usize << (BOOLEAN - 98)))) != 0) || ((((_la - 130)) & !0x3f) == 0 && ((1usize << (_la - 130)) & ((1usize << (IDENTIFIER - 130)) | (1usize << (STRING - 130)) | (1usize << (LBRACKET - 130)) | (1usize << (LPAREN - 130)) | (1usize << (LBRACE - 130)) | (1usize << (MINUS - 130)))) != 0) {
						{
						/*InvokeRule expression*/
						recog.base.set_state(1038);
						recog.expression_rec(0)?;

						recog.base.set_state(1043);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						while _la==COMMA {
							{
							{
							recog.base.set_state(1039);
							recog.base.match_token(COMMA,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(1040);
							recog.expression_rec(0)?;

							}
							}
							recog.base.set_state(1045);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
						}
						}
					}

					recog.base.set_state(1048);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

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
//------------------- functionName ----------------
pub type FunctionNameContextAll<'input> = FunctionNameContext<'input>;


pub type FunctionNameContext<'input> = BaseParserRuleContext<'input,FunctionNameContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for FunctionNameContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for FunctionNameContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_functionName(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_functionName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for FunctionNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_functionName(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionName }
}
antlr_rust::tid!{FunctionNameContextExt<'a>}

impl<'input> FunctionNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionNameContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<FunctionNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> FunctionNameContextAttrs<'input> for FunctionNameContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionName(&mut self,)
	-> Result<Rc<FunctionNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 178, RULE_functionName);
        let mut _localctx: Rc<FunctionNameContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1052);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__6) | (1usize << T__8) | (1usize << T__19) | (1usize << T__28) | (1usize << T__29) | (1usize << T__30))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (T__32 - 33)) | (1usize << (T__36 - 33)) | (1usize << (T__37 - 33)) | (1usize << (T__38 - 33)) | (1usize << (T__39 - 33)) | (1usize << (T__40 - 33)) | (1usize << (T__41 - 33)) | (1usize << (T__42 - 33)))) != 0) || ((((_la - 98)) & !0x3f) == 0 && ((1usize << (_la - 98)) & ((1usize << (T__97 - 98)) | (1usize << (T__98 - 98)) | (1usize << (T__99 - 98)) | (1usize << (T__100 - 98)) | (1usize << (T__101 - 98)) | (1usize << (T__102 - 98)) | (1usize << (T__103 - 98)) | (1usize << (T__104 - 98)) | (1usize << (T__105 - 98)) | (1usize << (T__106 - 98)) | (1usize << (T__107 - 98)) | (1usize << (T__108 - 98)) | (1usize << (T__109 - 98)) | (1usize << (T__110 - 98)) | (1usize << (T__111 - 98)) | (1usize << (T__112 - 98)) | (1usize << (T__113 - 98)))) != 0) || _la==IDENTIFIER) } {
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
//------------------- listLiteral ----------------
pub type ListLiteralContextAll<'input> = ListLiteralContext<'input>;


pub type ListLiteralContext<'input> = BaseParserRuleContext<'input,ListLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct ListLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for ListLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for ListLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_listLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_listLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for ListLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_listLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for ListLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listLiteral }
}
antlr_rust::tid!{ListLiteralContextExt<'a>}

impl<'input> ListLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ListLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ListLiteralContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<ListLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACKET
/// Returns `None` if there is no child corresponding to token LBRACKET
fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(LBRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACKET
/// Returns `None` if there is no child corresponding to token RBRACKET
fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(RBRACKET, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ListLiteralContextAttrs<'input> for ListLiteralContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn listLiteral(&mut self,)
	-> Result<Rc<ListLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ListLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 180, RULE_listLiteral);
        let mut _localctx: Rc<ListLiteralContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1054);
			recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

			recog.base.set_state(1063);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9) | (1usize << T__19) | (1usize << T__23) | (1usize << T__26) | (1usize << T__27) | (1usize << T__28) | (1usize << T__29) | (1usize << T__30))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (T__32 - 33)) | (1usize << (T__36 - 33)) | (1usize << (T__37 - 33)) | (1usize << (T__38 - 33)) | (1usize << (T__39 - 33)) | (1usize << (T__40 - 33)) | (1usize << (T__41 - 33)) | (1usize << (T__42 - 33)) | (1usize << (T__54 - 33)) | (1usize << (T__61 - 33)) | (1usize << (T__62 - 33)) | (1usize << (T__63 - 33)))) != 0) || ((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (T__64 - 65)) | (1usize << (T__69 - 65)) | (1usize << (T__78 - 65)) | (1usize << (T__91 - 65)) | (1usize << (T__94 - 65)))) != 0) || ((((_la - 98)) & !0x3f) == 0 && ((1usize << (_la - 98)) & ((1usize << (T__97 - 98)) | (1usize << (T__98 - 98)) | (1usize << (T__99 - 98)) | (1usize << (T__100 - 98)) | (1usize << (T__101 - 98)) | (1usize << (T__102 - 98)) | (1usize << (T__103 - 98)) | (1usize << (T__104 - 98)) | (1usize << (T__105 - 98)) | (1usize << (T__106 - 98)) | (1usize << (T__107 - 98)) | (1usize << (T__108 - 98)) | (1usize << (T__109 - 98)) | (1usize << (T__110 - 98)) | (1usize << (T__111 - 98)) | (1usize << (T__112 - 98)) | (1usize << (T__113 - 98)) | (1usize << (INTEGER - 98)) | (1usize << (DECIMAL - 98)) | (1usize << (BOOLEAN - 98)))) != 0) || ((((_la - 130)) & !0x3f) == 0 && ((1usize << (_la - 130)) & ((1usize << (IDENTIFIER - 130)) | (1usize << (STRING - 130)) | (1usize << (LBRACKET - 130)) | (1usize << (LPAREN - 130)) | (1usize << (LBRACE - 130)) | (1usize << (MINUS - 130)))) != 0) {
				{
				/*InvokeRule expression*/
				recog.base.set_state(1055);
				recog.expression_rec(0)?;

				recog.base.set_state(1060);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==COMMA {
					{
					{
					recog.base.set_state(1056);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(1057);
					recog.expression_rec(0)?;

					}
					}
					recog.base.set_state(1062);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(1065);
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
//------------------- fieldPath ----------------
pub type FieldPathContextAll<'input> = FieldPathContext<'input>;


pub type FieldPathContext<'input> = BaseParserRuleContext<'input,FieldPathContextExt<'input>>;

#[derive(Clone)]
pub struct FieldPathContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for FieldPathContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for FieldPathContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fieldPath(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_fieldPath(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for FieldPathContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_fieldPath(self);
	}
}

impl<'input> CustomRuleContext<'input> for FieldPathContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fieldPath }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fieldPath }
}
antlr_rust::tid!{FieldPathContextExt<'a>}

impl<'input> FieldPathContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FieldPathContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FieldPathContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FieldPathContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<FieldPathContextExt<'input>>{

fn fieldOrKeyword_all(&self) ->  Vec<Rc<FieldOrKeywordContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fieldOrKeyword(&self, i: usize) -> Option<Rc<FieldOrKeywordContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
/// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, i)
}

}

impl<'input> FieldPathContextAttrs<'input> for FieldPathContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fieldPath(&mut self,)
	-> Result<Rc<FieldPathContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FieldPathContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 182, RULE_fieldPath);
        let mut _localctx: Rc<FieldPathContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fieldOrKeyword*/
			recog.base.set_state(1067);
			recog.fieldOrKeyword()?;

			recog.base.set_state(1072);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(110,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(1068);
					recog.base.match_token(DOT,&mut recog.err_handler)?;

					/*InvokeRule fieldOrKeyword*/
					recog.base.set_state(1069);
					recog.fieldOrKeyword()?;

					}
					} 
				}
				recog.base.set_state(1074);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(110,&mut recog.base)?;
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
//------------------- fieldOrKeyword ----------------
pub type FieldOrKeywordContextAll<'input> = FieldOrKeywordContext<'input>;


pub type FieldOrKeywordContext<'input> = BaseParserRuleContext<'input,FieldOrKeywordContextExt<'input>>;

#[derive(Clone)]
pub struct FieldOrKeywordContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for FieldOrKeywordContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for FieldOrKeywordContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fieldOrKeyword(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_fieldOrKeyword(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for FieldOrKeywordContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_fieldOrKeyword(self);
	}
}

impl<'input> CustomRuleContext<'input> for FieldOrKeywordContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fieldOrKeyword }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fieldOrKeyword }
}
antlr_rust::tid!{FieldOrKeywordContextExt<'a>}

impl<'input> FieldOrKeywordContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FieldOrKeywordContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FieldOrKeywordContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FieldOrKeywordContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<FieldOrKeywordContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> FieldOrKeywordContextAttrs<'input> for FieldOrKeywordContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fieldOrKeyword(&mut self,)
	-> Result<Rc<FieldOrKeywordContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FieldOrKeywordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 184, RULE_fieldOrKeyword);
        let mut _localctx: Rc<FieldOrKeywordContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1075);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9) | (1usize << T__23) | (1usize << T__26) | (1usize << T__27))) != 0) || ((((_la - 40)) & !0x3f) == 0 && ((1usize << (_la - 40)) & ((1usize << (T__39 - 40)) | (1usize << (T__40 - 40)) | (1usize << (T__41 - 40)) | (1usize << (T__42 - 40)) | (1usize << (T__61 - 40)) | (1usize << (T__62 - 40)) | (1usize << (T__63 - 40)) | (1usize << (T__64 - 40)) | (1usize << (T__69 - 40)))) != 0) || _la==T__78 || _la==IDENTIFIER) } {
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
//------------------- fieldArray ----------------
pub type FieldArrayContextAll<'input> = FieldArrayContext<'input>;


pub type FieldArrayContext<'input> = BaseParserRuleContext<'input,FieldArrayContextExt<'input>>;

#[derive(Clone)]
pub struct FieldArrayContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for FieldArrayContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for FieldArrayContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fieldArray(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_fieldArray(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for FieldArrayContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_fieldArray(self);
	}
}

impl<'input> CustomRuleContext<'input> for FieldArrayContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fieldArray }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fieldArray }
}
antlr_rust::tid!{FieldArrayContextExt<'a>}

impl<'input> FieldArrayContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FieldArrayContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FieldArrayContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FieldArrayContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<FieldArrayContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACKET
/// Returns `None` if there is no child corresponding to token LBRACKET
fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(LBRACKET, 0)
}
fn fieldPath_all(&self) ->  Vec<Rc<FieldPathContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fieldPath(&self, i: usize) -> Option<Rc<FieldPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RBRACKET
/// Returns `None` if there is no child corresponding to token RBRACKET
fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(RBRACKET, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,TransformDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> FieldArrayContextAttrs<'input> for FieldArrayContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fieldArray(&mut self,)
	-> Result<Rc<FieldArrayContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FieldArrayContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 186, RULE_fieldArray);
        let mut _localctx: Rc<FieldArrayContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1077);
			recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

			/*InvokeRule fieldPath*/
			recog.base.set_state(1078);
			recog.fieldPath()?;

			recog.base.set_state(1083);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(1079);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule fieldPath*/
				recog.base.set_state(1080);
				recog.fieldPath()?;

				}
				}
				recog.base.set_state(1085);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1086);
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
//------------------- fieldName ----------------
pub type FieldNameContextAll<'input> = FieldNameContext<'input>;


pub type FieldNameContext<'input> = BaseParserRuleContext<'input,FieldNameContextExt<'input>>;

#[derive(Clone)]
pub struct FieldNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for FieldNameContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for FieldNameContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fieldName(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_fieldName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for FieldNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_fieldName(self);
	}
}

impl<'input> CustomRuleContext<'input> for FieldNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fieldName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fieldName }
}
antlr_rust::tid!{FieldNameContextExt<'a>}

impl<'input> FieldNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FieldNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FieldNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FieldNameContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<FieldNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> FieldNameContextAttrs<'input> for FieldNameContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fieldName(&mut self,)
	-> Result<Rc<FieldNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FieldNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 188, RULE_fieldName);
        let mut _localctx: Rc<FieldNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1088);
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
//------------------- duration ----------------
pub type DurationContextAll<'input> = DurationContext<'input>;


pub type DurationContext<'input> = BaseParserRuleContext<'input,DurationContextExt<'input>>;

#[derive(Clone)]
pub struct DurationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for DurationContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for DurationContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_duration(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_duration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for DurationContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_duration(self);
	}
}

impl<'input> CustomRuleContext<'input> for DurationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_duration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_duration }
}
antlr_rust::tid!{DurationContextExt<'a>}

impl<'input> DurationContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DurationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DurationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DurationContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<DurationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
fn timeUnit(&self) -> Option<Rc<TimeUnitContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DURATION_LITERAL
/// Returns `None` if there is no child corresponding to token DURATION_LITERAL
fn DURATION_LITERAL(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(DURATION_LITERAL, 0)
}

}

impl<'input> DurationContextAttrs<'input> for DurationContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn duration(&mut self,)
	-> Result<Rc<DurationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DurationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 190, RULE_duration);
        let mut _localctx: Rc<DurationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1093);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 INTEGER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1090);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					/*InvokeRule timeUnit*/
					recog.base.set_state(1091);
					recog.timeUnit()?;

					}
				}

			 DURATION_LITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1092);
					recog.base.match_token(DURATION_LITERAL,&mut recog.err_handler)?;

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
//------------------- timeUnit ----------------
pub type TimeUnitContextAll<'input> = TimeUnitContext<'input>;


pub type TimeUnitContext<'input> = BaseParserRuleContext<'input,TimeUnitContextExt<'input>>;

#[derive(Clone)]
pub struct TimeUnitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for TimeUnitContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for TimeUnitContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_timeUnit(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_timeUnit(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for TimeUnitContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_timeUnit(self);
	}
}

impl<'input> CustomRuleContext<'input> for TimeUnitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_timeUnit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_timeUnit }
}
antlr_rust::tid!{TimeUnitContextExt<'a>}

impl<'input> TimeUnitContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TimeUnitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TimeUnitContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TimeUnitContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<TimeUnitContextExt<'input>>{


}

impl<'input> TimeUnitContextAttrs<'input> for TimeUnitContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn timeUnit(&mut self,)
	-> Result<Rc<TimeUnitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TimeUnitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 192, RULE_timeUnit);
        let mut _localctx: Rc<TimeUnitContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1095);
			_la = recog.base.input.la(1);
			if { !(((((_la - 115)) & !0x3f) == 0 && ((1usize << (_la - 115)) & ((1usize << (T__114 - 115)) | (1usize << (T__115 - 115)) | (1usize << (T__116 - 115)) | (1usize << (T__117 - 115)) | (1usize << (T__118 - 115)) | (1usize << (T__119 - 115)) | (1usize << (T__120 - 115)) | (1usize << (T__121 - 115)))) != 0)) } {
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
//------------------- literal ----------------
pub type LiteralContextAll<'input> = LiteralContext<'input>;


pub type LiteralContext<'input> = BaseParserRuleContext<'input,LiteralContextExt<'input>>;

#[derive(Clone)]
pub struct LiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for LiteralContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for LiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_literal(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_literal(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for LiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_literal(self);
	}
}

impl<'input> CustomRuleContext<'input> for LiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literal }
}
antlr_rust::tid!{LiteralContextExt<'a>}

impl<'input> LiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LiteralContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<LiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
fn numberLiteral(&self) -> Option<Rc<NumberLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token BOOLEAN
/// Returns `None` if there is no child corresponding to token BOOLEAN
fn BOOLEAN(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(BOOLEAN, 0)
}

}

impl<'input> LiteralContextAttrs<'input> for LiteralContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn literal(&mut self,)
	-> Result<Rc<LiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 194, RULE_literal);
        let mut _localctx: Rc<LiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1101);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 STRING 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1097);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}

			 INTEGER | DECIMAL | MINUS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule numberLiteral*/
					recog.base.set_state(1098);
					recog.numberLiteral()?;

					}
				}

			 BOOLEAN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(1099);
					recog.base.match_token(BOOLEAN,&mut recog.err_handler)?;

					}
				}

			 T__94 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(1100);
					recog.base.match_token(T__94,&mut recog.err_handler)?;

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
//------------------- numberLiteral ----------------
pub type NumberLiteralContextAll<'input> = NumberLiteralContext<'input>;


pub type NumberLiteralContext<'input> = BaseParserRuleContext<'input,NumberLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct NumberLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TransformDSLParserContext<'input> for NumberLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn TransformDSLListener<'input> + 'a> for NumberLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_numberLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn TransformDSLListener<'input> + 'a)) {
			listener.exit_numberLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn TransformDSLVisitor<'input> + 'a> for NumberLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn TransformDSLVisitor<'input> + 'a)) {
		visitor.visit_numberLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumberLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TransformDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_numberLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_numberLiteral }
}
antlr_rust::tid!{NumberLiteralContextExt<'a>}

impl<'input> NumberLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn TransformDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NumberLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NumberLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NumberLiteralContextAttrs<'input>: TransformDSLParserContext<'input> + BorrowMut<NumberLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
/// Retrieves first TerminalNode corresponding to token DECIMAL
/// Returns `None` if there is no child corresponding to token DECIMAL
fn DECIMAL(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(DECIMAL, 0)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,TransformDSLParserContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}

}

impl<'input> NumberLiteralContextAttrs<'input> for NumberLiteralContext<'input>{}

impl<'input, I, H> TransformDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn numberLiteral(&mut self,)
	-> Result<Rc<NumberLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NumberLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 196, RULE_numberLiteral);
        let mut _localctx: Rc<NumberLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1109);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(114,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1103);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1104);
					recog.base.match_token(DECIMAL,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(1105);
					recog.base.match_token(MINUS,&mut recog.err_handler)?;

					recog.base.set_state(1106);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(1107);
					recog.base.match_token(MINUS,&mut recog.err_handler)?;

					recog.base.set_state(1108);
					recog.base.match_token(DECIMAL,&mut recog.err_handler)?;

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
	\u{a2}\u{45a}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
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
	\x44\x04\x45\x09\x45\x04\x46\x09\x46\x04\x47\x09\x47\x04\x48\x09\x48\x04\
	\x49\x09\x49\x04\x4a\x09\x4a\x04\x4b\x09\x4b\x04\x4c\x09\x4c\x04\x4d\x09\
	\x4d\x04\x4e\x09\x4e\x04\x4f\x09\x4f\x04\x50\x09\x50\x04\x51\x09\x51\x04\
	\x52\x09\x52\x04\x53\x09\x53\x04\x54\x09\x54\x04\x55\x09\x55\x04\x56\x09\
	\x56\x04\x57\x09\x57\x04\x58\x09\x58\x04\x59\x09\x59\x04\x5a\x09\x5a\x04\
	\x5b\x09\x5b\x04\x5c\x09\x5c\x04\x5d\x09\x5d\x04\x5e\x09\x5e\x04\x5f\x09\
	\x5f\x04\x60\x09\x60\x04\x61\x09\x61\x04\x62\x09\x62\x04\x63\x09\x63\x04\
	\x64\x09\x64\x03\x02\x07\x02\u{ca}\x0a\x02\x0c\x02\x0e\x02\u{cd}\x0b\x02\
	\x03\x02\x03\x02\x06\x02\u{d1}\x0a\x02\x0d\x02\x0e\x02\u{d2}\x03\x02\x03\
	\x02\x03\x03\x03\x03\x03\x03\x03\x04\x06\x04\u{db}\x0a\x04\x0d\x04\x0e\x04\
	\u{dc}\x03\x04\x03\x04\x03\x05\x03\x05\x03\x06\x03\x06\x03\x06\x03\x07\x03\
	\x07\x03\x07\x05\x07\u{e9}\x0a\x07\x03\x07\x05\x07\u{ec}\x0a\x07\x03\x07\
	\x05\x07\u{ef}\x0a\x07\x03\x07\x05\x07\u{f2}\x0a\x07\x03\x07\x03\x07\x05\
	\x07\u{f6}\x0a\x07\x03\x07\x05\x07\u{f9}\x0a\x07\x03\x07\x05\x07\u{fc}\x0a\
	\x07\x03\x07\x05\x07\u{ff}\x0a\x07\x03\x07\x03\x07\x05\x07\u{103}\x0a\x07\
	\x03\x07\x03\x07\x05\x07\u{107}\x0a\x07\x03\x07\x05\x07\u{10a}\x0a\x07\x03\
	\x07\x03\x07\x03\x08\x03\x08\x03\x08\x03\x08\x03\x09\x03\x09\x03\x09\x03\
	\x09\x03\x0a\x03\x0a\x03\x0a\x06\x0a\u{119}\x0a\x0a\x0d\x0a\x0e\x0a\u{11a}\
	\x03\x0a\x05\x0a\u{11e}\x0a\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0c\
	\x03\x0c\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0d\x06\x0d\u{12b}\x0a\x0d\
	\x0d\x0d\x0e\x0d\u{12c}\x03\x0d\x05\x0d\u{130}\x0a\x0d\x03\x0e\x03\x0e\x03\
	\x0e\x03\x0e\x05\x0e\u{136}\x0a\x0e\x03\x0f\x03\x0f\x05\x0f\u{13a}\x0a\x0f\
	\x03\x0f\x03\x0f\x05\x0f\u{13e}\x0a\x0f\x03\x0f\x05\x0f\u{141}\x0a\x0f\x03\
	\x10\x03\x10\x03\x10\x03\x10\x03\x11\x03\x11\x03\x12\x05\x12\u{14a}\x0a\
	\x12\x03\x12\x05\x12\u{14d}\x0a\x12\x03\x12\x05\x12\u{150}\x0a\x12\x03\x12\
	\x05\x12\u{153}\x0a\x12\x03\x13\x03\x13\x03\x13\x03\x13\x03\x14\x03\x14\
	\x03\x14\x03\x14\x03\x15\x03\x15\x03\x15\x03\x15\x03\x16\x03\x16\x03\x16\
	\x03\x16\x03\x17\x03\x17\x03\x18\x03\x18\x03\x18\x03\x18\x03\x19\x03\x19\
	\x05\x19\u{16d}\x0a\x19\x03\x19\x05\x19\u{170}\x0a\x19\x03\x19\x05\x19\u{173}\
	\x0a\x19\x03\x19\x03\x19\x03\x19\x05\x19\u{178}\x0a\x19\x03\x1a\x03\x1a\
	\x03\x1a\x03\x1a\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1c\x03\x1c\x03\x1c\
	\x05\x1c\u{185}\x0a\x1c\x03\x1c\x05\x1c\u{188}\x0a\x1c\x03\x1c\x03\x1c\x03\
	\x1c\x05\x1c\u{18d}\x0a\x1c\x03\x1c\x05\x1c\u{190}\x0a\x1c\x03\x1c\x03\x1c\
	\x05\x1c\u{194}\x0a\x1c\x03\x1c\x05\x1c\u{197}\x0a\x1c\x03\x1c\x05\x1c\u{19a}\
	\x0a\x1c\x03\x1c\x05\x1c\u{19d}\x0a\x1c\x03\x1c\x03\x1c\x03\x1d\x03\x1d\
	\x06\x1d\u{1a3}\x0a\x1d\x0d\x1d\x0e\x1d\u{1a4}\x03\x1d\x03\x1d\x03\x1e\x03\
	\x1e\x03\x1e\x03\x1e\x05\x1e\u{1ad}\x0a\x1e\x03\x1e\x03\x1e\x06\x1e\u{1b1}\
	\x0a\x1e\x0d\x1e\x0e\x1e\u{1b2}\x03\x1e\x03\x1e\x05\x1e\u{1b7}\x0a\x1e\x03\
	\x1f\x03\x1f\x03\x1f\x03\x1f\x05\x1f\u{1bd}\x0a\x1f\x03\x20\x03\x20\x03\
	\x20\x03\x20\x05\x20\u{1c3}\x0a\x20\x03\x20\x03\x20\x06\x20\u{1c7}\x0a\x20\
	\x0d\x20\x0e\x20\u{1c8}\x03\x20\x03\x20\x05\x20\u{1cd}\x0a\x20\x03\x21\x03\
	\x21\x03\x21\x03\x21\x05\x21\u{1d3}\x0a\x21\x03\x22\x03\x22\x07\x22\u{1d7}\
	\x0a\x22\x0c\x22\x0e\x22\u{1da}\x0b\x22\x03\x22\x03\x22\x03\x22\x05\x22\
	\u{1df}\x0a\x22\x03\x23\x03\x23\x03\x24\x03\x24\x03\x24\x03\x24\x03\x24\
	\x03\x24\x03\x24\x03\x24\x03\x24\x03\x24\x03\x24\x03\x24\x03\x24\x03\x24\
	\x03\x24\x03\x24\x03\x24\x05\x24\u{1f4}\x0a\x24\x03\x25\x03\x25\x03\x25\
	\x03\x25\x07\x25\u{1fa}\x0a\x25\x0c\x25\x0e\x25\u{1fd}\x0b\x25\x03\x25\x03\
	\x25\x03\x26\x03\x26\x03\x26\x03\x26\x03\x26\x03\x26\x03\x26\x03\x26\x03\
	\x26\x03\x26\x03\x26\x03\x26\x03\x26\x03\x26\x03\x26\x03\x26\x03\x26\x03\
	\x26\x03\x26\x05\x26\u{214}\x0a\x26\x03\x26\x03\x26\x03\x26\x05\x26\u{219}\
	\x0a\x26\x03\x27\x03\x27\x03\x27\x03\x27\x03\x27\x03\x27\x03\x27\x03\x27\
	\x03\x27\x05\x27\u{224}\x0a\x27\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\
	\x03\x28\x03\x28\x03\x28\x05\x28\u{22e}\x0a\x28\x03\x29\x03\x29\x03\x29\
	\x07\x29\u{233}\x0a\x29\x0c\x29\x0e\x29\u{236}\x0b\x29\x03\x29\x03\x29\x03\
	\x29\x07\x29\u{23b}\x0a\x29\x0c\x29\x0e\x29\u{23e}\x0b\x29\x05\x29\u{240}\
	\x0a\x29\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x07\x2a\u{246}\x0a\x2a\x0c\x2a\
	\x0e\x2a\u{249}\x0b\x2a\x03\x2b\x03\x2b\x03\x2b\x03\x2b\x03\x2b\x05\x2b\
	\u{250}\x0a\x2b\x03\x2c\x03\x2c\x06\x2c\u{254}\x0a\x2c\x0d\x2c\x0e\x2c\u{255}\
	\x03\x2c\x03\x2c\x03\x2d\x03\x2d\x05\x2d\u{25c}\x0a\x2d\x03\x2e\x03\x2e\
	\x03\x2e\x03\x2e\x03\x2e\x03\x2e\x03\x2e\x05\x2e\u{265}\x0a\x2e\x03\x2f\
	\x03\x2f\x03\x2f\x03\x2f\x03\x2f\x03\x30\x03\x30\x06\x30\u{26e}\x0a\x30\
	\x0d\x30\x0e\x30\u{26f}\x03\x30\x03\x30\x03\x31\x03\x31\x03\x31\x03\x31\
	\x03\x32\x03\x32\x05\x32\u{27a}\x0a\x32\x03\x32\x06\x32\u{27d}\x0a\x32\x0d\
	\x32\x0e\x32\u{27e}\x03\x32\x03\x32\x05\x32\u{283}\x0a\x32\x03\x33\x03\x33\
	\x03\x34\x03\x34\x03\x34\x03\x34\x03\x34\x03\x34\x03\x34\x03\x34\x03\x34\
	\x05\x34\u{290}\x0a\x34\x03\x35\x03\x35\x05\x35\u{294}\x0a\x35\x03\x35\x06\
	\x35\u{297}\x0a\x35\x0d\x35\x0e\x35\u{298}\x03\x35\x03\x35\x03\x36\x03\x36\
	\x06\x36\u{29f}\x0a\x36\x0d\x36\x0e\x36\u{2a0}\x03\x36\x03\x36\x03\x37\x03\
	\x37\x06\x37\u{2a7}\x0a\x37\x0d\x37\x0e\x37\u{2a8}\x03\x37\x03\x37\x03\x38\
	\x03\x38\x03\x38\x03\x38\x03\x38\x03\x38\x03\x38\x03\x38\x06\x38\u{2b5}\
	\x0a\x38\x0d\x38\x0e\x38\u{2b6}\x03\x38\x03\x38\x03\x38\x03\x38\x03\x38\
	\x03\x38\x03\x38\x05\x38\u{2c0}\x0a\x38\x03\x39\x03\x39\x05\x39\u{2c4}\x0a\
	\x39\x03\x3a\x03\x3a\x03\x3a\x03\x3a\x03\x3a\x03\x3a\x05\x3a\u{2cc}\x0a\
	\x3a\x03\x3a\x03\x3a\x03\x3a\x05\x3a\u{2d1}\x0a\x3a\x03\x3b\x03\x3b\x03\
	\x3c\x03\x3c\x06\x3c\u{2d7}\x0a\x3c\x0d\x3c\x0e\x3c\u{2d8}\x03\x3c\x03\x3c\
	\x03\x3d\x03\x3d\x06\x3d\u{2df}\x0a\x3d\x0d\x3d\x0e\x3d\u{2e0}\x03\x3d\x03\
	\x3d\x03\x3e\x03\x3e\x03\x3e\x03\x3e\x05\x3e\u{2e9}\x0a\x3e\x03\x3f\x03\
	\x3f\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x03\
	\x3f\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x05\x3f\u{2fa}\x0a\x3f\x03\x40\x03\
	\x40\x03\x40\x03\x40\x03\x40\x03\x41\x03\x41\x03\x41\x03\x41\x03\x42\x03\
	\x42\x03\x42\x05\x42\u{308}\x0a\x42\x05\x42\u{30a}\x0a\x42\x03\x43\x03\x43\
	\x03\x43\x03\x43\x03\x44\x03\x44\x03\x44\x05\x44\u{313}\x0a\x44\x03\x45\
	\x03\x45\x03\x46\x03\x46\x03\x47\x03\x47\x06\x47\u{31b}\x0a\x47\x0d\x47\
	\x0e\x47\u{31c}\x03\x47\x03\x47\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\
	\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\
	\x03\x48\x05\x48\u{330}\x0a\x48\x03\x49\x03\x49\x03\x49\x03\x49\x03\x49\
	\x03\x4a\x03\x4a\x06\x4a\u{339}\x0a\x4a\x0d\x4a\x0e\x4a\u{33a}\x03\x4a\x03\
	\x4a\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x05\x4b\u{344}\x0a\x4b\x03\
	\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\
	\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\
	\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\
	\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\
	\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\
	\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x07\x4b\u{379}\x0a\x4b\x0c\
	\x4b\x0e\x4b\u{37c}\x0b\x4b\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\
	\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x05\x4c\u{38b}\
	\x0a\x4c\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x07\x4d\u{391}\x0a\x4d\x0c\x4d\
	\x0e\x4d\u{394}\x0b\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x05\x4d\u{39a}\x0a\
	\x4d\x03\x4e\x03\x4e\x03\x4e\x03\x4e\x03\x4f\x03\x4f\x03\x50\x03\x50\x03\
	\x50\x03\x50\x03\x50\x03\x50\x03\x50\x07\x50\u{3a9}\x0a\x50\x0c\x50\x0e\
	\x50\u{3ac}\x0b\x50\x03\x50\x03\x50\x03\x50\x05\x50\u{3b1}\x0a\x50\x03\x51\
	\x03\x51\x03\x51\x07\x51\u{3b6}\x0a\x51\x0c\x51\x0e\x51\u{3b9}\x0b\x51\x03\
	\x52\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\x07\
	\x52\u{3c4}\x0a\x52\x0c\x52\x0e\x52\u{3c7}\x0b\x52\x03\x52\x03\x52\x03\x52\
	\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\
	\x03\x52\x07\x52\u{3d6}\x0a\x52\x0c\x52\x0e\x52\u{3d9}\x0b\x52\x03\x52\x03\
	\x52\x03\x52\x05\x52\u{3de}\x0a\x52\x03\x53\x03\x53\x03\x53\x03\x53\x03\
	\x53\x03\x53\x07\x53\u{3e6}\x0a\x53\x0c\x53\x0e\x53\u{3e9}\x0b\x53\x03\x54\
	\x03\x54\x03\x54\x06\x54\u{3ee}\x0a\x54\x0d\x54\x0e\x54\u{3ef}\x03\x55\x03\
	\x55\x03\x55\x05\x55\u{3f5}\x0a\x55\x03\x56\x03\x56\x03\x57\x03\x57\x03\
	\x58\x03\x58\x03\x59\x03\x59\x03\x5a\x03\x5a\x03\x5a\x03\x5a\x03\x5a\x07\
	\x5a\u{404}\x0a\x5a\x0c\x5a\x0e\x5a\u{407}\x0b\x5a\x05\x5a\u{409}\x0a\x5a\
	\x03\x5a\x03\x5a\x03\x5a\x03\x5a\x03\x5a\x03\x5a\x03\x5a\x03\x5a\x03\x5a\
	\x07\x5a\u{414}\x0a\x5a\x0c\x5a\x0e\x5a\u{417}\x0b\x5a\x05\x5a\u{419}\x0a\
	\x5a\x03\x5a\x03\x5a\x05\x5a\u{41d}\x0a\x5a\x03\x5b\x03\x5b\x03\x5c\x03\
	\x5c\x03\x5c\x03\x5c\x07\x5c\u{425}\x0a\x5c\x0c\x5c\x0e\x5c\u{428}\x0b\x5c\
	\x05\x5c\u{42a}\x0a\x5c\x03\x5c\x03\x5c\x03\x5d\x03\x5d\x03\x5d\x07\x5d\
	\u{431}\x0a\x5d\x0c\x5d\x0e\x5d\u{434}\x0b\x5d\x03\x5e\x03\x5e\x03\x5f\x03\
	\x5f\x03\x5f\x03\x5f\x07\x5f\u{43c}\x0a\x5f\x0c\x5f\x0e\x5f\u{43f}\x0b\x5f\
	\x03\x5f\x03\x5f\x03\x60\x03\x60\x03\x61\x03\x61\x03\x61\x05\x61\u{448}\
	\x0a\x61\x03\x62\x03\x62\x03\x63\x03\x63\x03\x63\x03\x63\x05\x63\u{450}\
	\x0a\x63\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x05\x64\u{458}\
	\x0a\x64\x03\x64\x02\x03\u{94}\x65\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\
	\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x30\x32\x34\x36\x38\
	\x3a\x3c\x3e\x40\x42\x44\x46\x48\x4a\x4c\x4e\x50\x52\x54\x56\x58\x5a\x5c\
	\x5e\x60\x62\x64\x66\x68\x6a\x6c\x6e\x70\x72\x74\x76\x78\x7a\x7c\x7e\u{80}\
	\u{82}\u{84}\u{86}\u{88}\u{8a}\u{8c}\u{8e}\u{90}\u{92}\u{94}\u{96}\u{98}\
	\u{9a}\u{9c}\u{9e}\u{a0}\u{a2}\u{a4}\u{a6}\u{a8}\u{aa}\u{ac}\u{ae}\u{b0}\
	\u{b2}\u{b4}\u{b6}\u{b8}\u{ba}\u{bc}\u{be}\u{c0}\u{c2}\u{c4}\u{c6}\x02\x13\
	\x06\x02\u{84}\u{85}\u{89}\u{8a}\u{99}\u{99}\u{9b}\u{9b}\x03\x02\x03\x06\
	\x03\x02\x13\x16\x03\x02\x1f\x26\x03\x02\x36\x38\x03\x02\x43\x45\x03\x02\
	\x52\x55\x04\x02\x43\x45\x56\x56\x04\x02\u{83}\u{83}\u{9f}\u{9f}\x0c\x02\
	\x09\x09\x0b\x0c\x1a\x1a\x1d\x1e\x2a\x2d\x40\x43\x48\x48\x51\x51\u{84}\u{84}\
	\u{86}\u{86}\x03\x02\u{98}\u{9c}\x04\x02\x32\x32\u{91}\u{97}\x04\x02\x5d\
	\x5d\x63\x63\x04\x02\x5e\x5e\u{99}\u{99}\x0a\x02\x09\x09\x0b\x0b\x16\x16\
	\x1f\x21\x23\x23\x27\x2d\x64\x74\u{84}\u{84}\x0a\x02\x09\x0c\x1a\x1a\x1d\
	\x1e\x2a\x2d\x40\x43\x48\x48\x51\x51\u{84}\u{84}\x03\x02\x75\x7c\x02\u{496}\
	\x02\u{cb}\x03\x02\x02\x02\x04\u{d6}\x03\x02\x02\x02\x06\u{da}\x03\x02\x02\
	\x02\x08\u{e0}\x03\x02\x02\x02\x0a\u{e2}\x03\x02\x02\x02\x0c\u{e5}\x03\x02\
	\x02\x02\x0e\u{10d}\x03\x02\x02\x02\x10\u{111}\x03\x02\x02\x02\x12\u{115}\
	\x03\x02\x02\x02\x14\u{11f}\x03\x02\x02\x02\x16\u{123}\x03\x02\x02\x02\x18\
	\u{127}\x03\x02\x02\x02\x1a\u{131}\x03\x02\x02\x02\x1c\u{140}\x03\x02\x02\
	\x02\x1e\u{142}\x03\x02\x02\x02\x20\u{146}\x03\x02\x02\x02\x22\u{149}\x03\
	\x02\x02\x02\x24\u{154}\x03\x02\x02\x02\x26\u{158}\x03\x02\x02\x02\x28\u{15c}\
	\x03\x02\x02\x02\x2a\u{160}\x03\x02\x02\x02\x2c\u{164}\x03\x02\x02\x02\x2e\
	\u{166}\x03\x02\x02\x02\x30\u{177}\x03\x02\x02\x02\x32\u{179}\x03\x02\x02\
	\x02\x34\u{17d}\x03\x02\x02\x02\x36\u{181}\x03\x02\x02\x02\x38\u{1a0}\x03\
	\x02\x02\x02\x3a\u{1b6}\x03\x02\x02\x02\x3c\u{1b8}\x03\x02\x02\x02\x3e\u{1cc}\
	\x03\x02\x02\x02\x40\u{1ce}\x03\x02\x02\x02\x42\u{1de}\x03\x02\x02\x02\x44\
	\u{1e0}\x03\x02\x02\x02\x46\u{1f3}\x03\x02\x02\x02\x48\u{1f5}\x03\x02\x02\
	\x02\x4a\u{218}\x03\x02\x02\x02\x4c\u{223}\x03\x02\x02\x02\x4e\u{22d}\x03\
	\x02\x02\x02\x50\u{23f}\x03\x02\x02\x02\x52\u{241}\x03\x02\x02\x02\x54\u{24f}\
	\x03\x02\x02\x02\x56\u{251}\x03\x02\x02\x02\x58\u{25b}\x03\x02\x02\x02\x5a\
	\u{264}\x03\x02\x02\x02\x5c\u{266}\x03\x02\x02\x02\x5e\u{26b}\x03\x02\x02\
	\x02\x60\u{273}\x03\x02\x02\x02\x62\u{277}\x03\x02\x02\x02\x64\u{284}\x03\
	\x02\x02\x02\x66\u{28f}\x03\x02\x02\x02\x68\u{291}\x03\x02\x02\x02\x6a\u{29c}\
	\x03\x02\x02\x02\x6c\u{2a4}\x03\x02\x02\x02\x6e\u{2bf}\x03\x02\x02\x02\x70\
	\u{2c3}\x03\x02\x02\x02\x72\u{2c5}\x03\x02\x02\x02\x74\u{2d2}\x03\x02\x02\
	\x02\x76\u{2d4}\x03\x02\x02\x02\x78\u{2dc}\x03\x02\x02\x02\x7a\u{2e8}\x03\
	\x02\x02\x02\x7c\u{2f9}\x03\x02\x02\x02\x7e\u{2fb}\x03\x02\x02\x02\u{80}\
	\u{300}\x03\x02\x02\x02\u{82}\u{309}\x03\x02\x02\x02\u{84}\u{30b}\x03\x02\
	\x02\x02\u{86}\u{312}\x03\x02\x02\x02\u{88}\u{314}\x03\x02\x02\x02\u{8a}\
	\u{316}\x03\x02\x02\x02\u{8c}\u{318}\x03\x02\x02\x02\u{8e}\u{32f}\x03\x02\
	\x02\x02\u{90}\u{331}\x03\x02\x02\x02\u{92}\u{336}\x03\x02\x02\x02\u{94}\
	\u{343}\x03\x02\x02\x02\u{96}\u{38a}\x03\x02\x02\x02\u{98}\u{399}\x03\x02\
	\x02\x02\u{9a}\u{39b}\x03\x02\x02\x02\u{9c}\u{39f}\x03\x02\x02\x02\u{9e}\
	\u{3b0}\x03\x02\x02\x02\u{a0}\u{3b2}\x03\x02\x02\x02\u{a2}\u{3dd}\x03\x02\
	\x02\x02\u{a4}\u{3df}\x03\x02\x02\x02\u{a6}\u{3ea}\x03\x02\x02\x02\u{a8}\
	\u{3f4}\x03\x02\x02\x02\u{aa}\u{3f6}\x03\x02\x02\x02\u{ac}\u{3f8}\x03\x02\
	\x02\x02\u{ae}\u{3fa}\x03\x02\x02\x02\u{b0}\u{3fc}\x03\x02\x02\x02\u{b2}\
	\u{41c}\x03\x02\x02\x02\u{b4}\u{41e}\x03\x02\x02\x02\u{b6}\u{420}\x03\x02\
	\x02\x02\u{b8}\u{42d}\x03\x02\x02\x02\u{ba}\u{435}\x03\x02\x02\x02\u{bc}\
	\u{437}\x03\x02\x02\x02\u{be}\u{442}\x03\x02\x02\x02\u{c0}\u{447}\x03\x02\
	\x02\x02\u{c2}\u{449}\x03\x02\x02\x02\u{c4}\u{44f}\x03\x02\x02\x02\u{c6}\
	\u{457}\x03\x02\x02\x02\u{c8}\u{ca}\x05\x04\x03\x02\u{c9}\u{c8}\x03\x02\
	\x02\x02\u{ca}\u{cd}\x03\x02\x02\x02\u{cb}\u{c9}\x03\x02\x02\x02\u{cb}\u{cc}\
	\x03\x02\x02\x02\u{cc}\u{d0}\x03\x02\x02\x02\u{cd}\u{cb}\x03\x02\x02\x02\
	\u{ce}\u{d1}\x05\x0c\x07\x02\u{cf}\u{d1}\x05\x36\x1c\x02\u{d0}\u{ce}\x03\
	\x02\x02\x02\u{d0}\u{cf}\x03\x02\x02\x02\u{d1}\u{d2}\x03\x02\x02\x02\u{d2}\
	\u{d0}\x03\x02\x02\x02\u{d2}\u{d3}\x03\x02\x02\x02\u{d3}\u{d4}\x03\x02\x02\
	\x02\u{d4}\u{d5}\x07\x02\x02\x03\u{d5}\x03\x03\x02\x02\x02\u{d6}\u{d7}\x07\
	\u{82}\x02\x02\u{d7}\u{d8}\x05\x06\x04\x02\u{d8}\x05\x03\x02\x02\x02\u{d9}\
	\u{db}\x05\x08\x05\x02\u{da}\u{d9}\x03\x02\x02\x02\u{db}\u{dc}\x03\x02\x02\
	\x02\u{dc}\u{da}\x03\x02\x02\x02\u{dc}\u{dd}\x03\x02\x02\x02\u{dd}\u{de}\
	\x03\x02\x02\x02\u{de}\u{df}\x05\x0a\x06\x02\u{df}\x07\x03\x02\x02\x02\u{e0}\
	\u{e1}\x09\x02\x02\x02\u{e1}\x09\x03\x02\x02\x02\u{e2}\u{e3}\x07\u{8a}\x02\
	\x02\u{e3}\u{e4}\x09\x03\x02\x02\u{e4}\x0b\x03\x02\x02\x02\u{e5}\u{e6}\x07\
	\x04\x02\x02\u{e6}\u{e8}\x05\x20\x11\x02\u{e7}\u{e9}\x05\x22\x12\x02\u{e8}\
	\u{e7}\x03\x02\x02\x02\u{e8}\u{e9}\x03\x02\x02\x02\u{e9}\u{eb}\x03\x02\x02\
	\x02\u{ea}\u{ec}\x05\x2e\x18\x02\u{eb}\u{ea}\x03\x02\x02\x02\u{eb}\u{ec}\
	\x03\x02\x02\x02\u{ec}\u{ee}\x03\x02\x02\x02\u{ed}\u{ef}\x05\x0e\x08\x02\
	\u{ee}\u{ed}\x03\x02\x02\x02\u{ee}\u{ef}\x03\x02\x02\x02\u{ef}\u{f1}\x03\
	\x02\x02\x02\u{f0}\u{f2}\x05\x30\x19\x02\u{f1}\u{f0}\x03\x02\x02\x02\u{f1}\
	\u{f2}\x03\x02\x02\x02\u{f2}\u{f3}\x03\x02\x02\x02\u{f3}\u{f5}\x05\x3a\x1e\
	\x02\u{f4}\u{f6}\x05\x10\x09\x02\u{f5}\u{f4}\x03\x02\x02\x02\u{f5}\u{f6}\
	\x03\x02\x02\x02\u{f6}\u{f8}\x03\x02\x02\x02\u{f7}\u{f9}\x05\x12\x0a\x02\
	\u{f8}\u{f7}\x03\x02\x02\x02\u{f8}\u{f9}\x03\x02\x02\x02\u{f9}\u{fb}\x03\
	\x02\x02\x02\u{fa}\u{fc}\x05\x16\x0c\x02\u{fb}\u{fa}\x03\x02\x02\x02\u{fb}\
	\u{fc}\x03\x02\x02\x02\u{fc}\u{fe}\x03\x02\x02\x02\u{fd}\u{ff}\x05\x18\x0d\
	\x02\u{fe}\u{fd}\x03\x02\x02\x02\u{fe}\u{ff}\x03\x02\x02\x02\u{ff}\u{100}\
	\x03\x02\x02\x02\u{100}\u{102}\x05\x3e\x20\x02\u{101}\u{103}\x05\x6a\x36\
	\x02\u{102}\u{101}\x03\x02\x02\x02\u{102}\u{103}\x03\x02\x02\x02\u{103}\
	\u{104}\x03\x02\x02\x02\u{104}\u{106}\x05\x56\x2c\x02\u{105}\u{107}\x05\
	\x6c\x37\x02\u{106}\u{105}\x03\x02\x02\x02\u{106}\u{107}\x03\x02\x02\x02\
	\u{107}\u{109}\x03\x02\x02\x02\u{108}\u{10a}\x05\x78\x3d\x02\u{109}\u{108}\
	\x03\x02\x02\x02\u{109}\u{10a}\x03\x02\x02\x02\u{10a}\u{10b}\x03\x02\x02\
	\x02\u{10b}\u{10c}\x07\x07\x02\x02\u{10c}\x0d\x03\x02\x02\x02\u{10d}\u{10e}\
	\x07\x08\x02\x02\u{10e}\u{10f}\x07\u{87}\x02\x02\u{10f}\u{110}\x07\u{81}\
	\x02\x02\u{110}\x0f\x03\x02\x02\x02\u{111}\u{112}\x07\x09\x02\x02\u{112}\
	\u{113}\x07\u{87}\x02\x02\u{113}\u{114}\x07\u{84}\x02\x02\u{114}\x11\x03\
	\x02\x02\x02\u{115}\u{116}\x07\x0a\x02\x02\u{116}\u{118}\x07\u{87}\x02\x02\
	\u{117}\u{119}\x05\x14\x0b\x02\u{118}\u{117}\x03\x02\x02\x02\u{119}\u{11a}\
	\x03\x02\x02\x02\u{11a}\u{118}\x03\x02\x02\x02\u{11a}\u{11b}\x03\x02\x02\
	\x02\u{11b}\u{11d}\x03\x02\x02\x02\u{11c}\u{11e}\x07\x07\x02\x02\u{11d}\
	\u{11c}\x03\x02\x02\x02\u{11d}\u{11e}\x03\x02\x02\x02\u{11e}\x13\x03\x02\
	\x02\x02\u{11f}\u{120}\x07\u{84}\x02\x02\u{120}\u{121}\x07\u{87}\x02\x02\
	\u{121}\u{122}\x07\u{84}\x02\x02\u{122}\x15\x03\x02\x02\x02\u{123}\u{124}\
	\x07\x0b\x02\x02\u{124}\u{125}\x07\u{87}\x02\x02\u{125}\u{126}\x07\u{84}\
	\x02\x02\u{126}\x17\x03\x02\x02\x02\u{127}\u{128}\x07\x0c\x02\x02\u{128}\
	\u{12a}\x07\u{87}\x02\x02\u{129}\u{12b}\x05\x1a\x0e\x02\u{12a}\u{129}\x03\
	\x02\x02\x02\u{12b}\u{12c}\x03\x02\x02\x02\u{12c}\u{12a}\x03\x02\x02\x02\
	\u{12c}\u{12d}\x03\x02\x02\x02\u{12d}\u{12f}\x03\x02\x02\x02\u{12e}\u{130}\
	\x07\x07\x02\x02\u{12f}\u{12e}\x03\x02\x02\x02\u{12f}\u{130}\x03\x02\x02\
	\x02\u{130}\x19\x03\x02\x02\x02\u{131}\u{132}\x07\u{84}\x02\x02\u{132}\u{133}\
	\x07\u{87}\x02\x02\u{133}\u{135}\x05\x42\x22\x02\u{134}\u{136}\x05\x1c\x0f\
	\x02\u{135}\u{134}\x03\x02\x02\x02\u{135}\u{136}\x03\x02\x02\x02\u{136}\
	\x1b\x03\x02\x02\x02\u{137}\u{139}\x07\x0d\x02\x02\u{138}\u{13a}\x05\x1e\
	\x10\x02\u{139}\u{138}\x03\x02\x02\x02\u{139}\u{13a}\x03\x02\x02\x02\u{13a}\
	\u{141}\x03\x02\x02\x02\u{13b}\u{13d}\x07\x0e\x02\x02\u{13c}\u{13e}\x05\
	\x1e\x10\x02\u{13d}\u{13c}\x03\x02\x02\x02\u{13d}\u{13e}\x03\x02\x02\x02\
	\u{13e}\u{141}\x03\x02\x02\x02\u{13f}\u{141}\x05\x1e\x10\x02\u{140}\u{137}\
	\x03\x02\x02\x02\u{140}\u{13b}\x03\x02\x02\x02\u{140}\u{13f}\x03\x02\x02\
	\x02\u{141}\x1d\x03\x02\x02\x02\u{142}\u{143}\x07\u{83}\x02\x02\u{143}\u{144}\
	\x07\u{87}\x02\x02\u{144}\u{145}\x05\u{94}\x4b\x02\u{145}\x1f\x03\x02\x02\
	\x02\u{146}\u{147}\x07\u{84}\x02\x02\u{147}\x21\x03\x02\x02\x02\u{148}\u{14a}\
	\x05\x24\x13\x02\u{149}\u{148}\x03\x02\x02\x02\u{149}\u{14a}\x03\x02\x02\
	\x02\u{14a}\u{14c}\x03\x02\x02\x02\u{14b}\u{14d}\x05\x26\x14\x02\u{14c}\
	\u{14b}\x03\x02\x02\x02\u{14c}\u{14d}\x03\x02\x02\x02\u{14d}\u{14f}\x03\
	\x02\x02\x02\u{14e}\u{150}\x05\x28\x15\x02\u{14f}\u{14e}\x03\x02\x02\x02\
	\u{14f}\u{150}\x03\x02\x02\x02\u{150}\u{152}\x03\x02\x02\x02\u{151}\u{153}\
	\x05\x2a\x16\x02\u{152}\u{151}\x03\x02\x02\x02\u{152}\u{153}\x03\x02\x02\
	\x02\u{153}\x23\x03\x02\x02\x02\u{154}\u{155}\x07\x0f\x02\x02\u{155}\u{156}\
	\x07\u{87}\x02\x02\u{156}\u{157}\x07\x7d\x02\x02\u{157}\x25\x03\x02\x02\
	\x02\u{158}\u{159}\x07\x10\x02\x02\u{159}\u{15a}\x07\u{87}\x02\x02\u{15a}\
	\u{15b}\x07\u{86}\x02\x02\u{15b}\x27\x03\x02\x02\x02\u{15c}\u{15d}\x07\x11\
	\x02\x02\u{15d}\u{15e}\x07\u{87}\x02\x02\u{15e}\u{15f}\x07\x7d\x02\x02\u{15f}\
	\x29\x03\x02\x02\x02\u{160}\u{161}\x07\x12\x02\x02\u{161}\u{162}\x07\u{87}\
	\x02\x02\u{162}\u{163}\x05\x2c\x17\x02\u{163}\x2b\x03\x02\x02\x02\u{164}\
	\u{165}\x09\x04\x02\x02\u{165}\x2d\x03\x02\x02\x02\u{166}\u{167}\x07\x17\
	\x02\x02\u{167}\u{168}\x07\u{87}\x02\x02\u{168}\u{169}\x07\u{81}\x02\x02\
	\u{169}\x2f\x03\x02\x02\x02\u{16a}\u{16c}\x07\x18\x02\x02\u{16b}\u{16d}\
	\x05\x32\x1a\x02\u{16c}\u{16b}\x03\x02\x02\x02\u{16c}\u{16d}\x03\x02\x02\
	\x02\u{16d}\u{16f}\x03\x02\x02\x02\u{16e}\u{170}\x05\x34\x1b\x02\u{16f}\
	\u{16e}\x03\x02\x02\x02\u{16f}\u{170}\x03\x02\x02\x02\u{170}\u{172}\x03\
	\x02\x02\x02\u{171}\u{173}\x07\x07\x02\x02\u{172}\u{171}\x03\x02\x02\x02\
	\u{172}\u{173}\x03\x02\x02\x02\u{173}\u{178}\x03\x02\x02\x02\u{174}\u{175}\
	\x07\x18\x02\x02\u{175}\u{176}\x07\u{87}\x02\x02\u{176}\u{178}\x05\u{c0}\
	\x61\x02\u{177}\u{16a}\x03\x02\x02\x02\u{177}\u{174}\x03\x02\x02\x02\u{178}\
	\x31\x03\x02\x02\x02\u{179}\u{17a}\x07\x19\x02\x02\u{17a}\u{17b}\x07\u{87}\
	\x02\x02\u{17b}\u{17c}\x05\u{c0}\x61\x02\u{17c}\x33\x03\x02\x02\x02\u{17d}\
	\u{17e}\x07\x1a\x02\x02\u{17e}\u{17f}\x07\u{87}\x02\x02\u{17f}\u{180}\x05\
	\u{bc}\x5f\x02\u{180}\x35\x03\x02\x02\x02\u{181}\u{182}\x07\x1b\x02\x02\
	\u{182}\u{184}\x05\x20\x11\x02\u{183}\u{185}\x05\x22\x12\x02\u{184}\u{183}\
	\x03\x02\x02\x02\u{184}\u{185}\x03\x02\x02\x02\u{185}\u{187}\x03\x02\x02\
	\x02\u{186}\u{188}\x05\x38\x1d\x02\u{187}\u{186}\x03\x02\x02\x02\u{187}\
	\u{188}\x03\x02\x02\x02\u{188}\u{189}\x03\x02\x02\x02\u{189}\u{18a}\x05\
	\x3a\x1e\x02\u{18a}\u{18c}\x05\x3e\x20\x02\u{18b}\u{18d}\x05\x6a\x36\x02\
	\u{18c}\u{18b}\x03\x02\x02\x02\u{18c}\u{18d}\x03\x02\x02\x02\u{18d}\u{18f}\
	\x03\x02\x02\x02\u{18e}\u{190}\x05\x76\x3c\x02\u{18f}\u{18e}\x03\x02\x02\
	\x02\u{18f}\u{190}\x03\x02\x02\x02\u{190}\u{193}\x03\x02\x02\x02\u{191}\
	\u{194}\x05\x5e\x30\x02\u{192}\u{194}\x05\x62\x32\x02\u{193}\u{191}\x03\
	\x02\x02\x02\u{193}\u{192}\x03\x02\x02\x02\u{194}\u{196}\x03\x02\x02\x02\
	\u{195}\u{197}\x05\x6c\x37\x02\u{196}\u{195}\x03\x02\x02\x02\u{196}\u{197}\
	\x03\x02\x02\x02\u{197}\u{199}\x03\x02\x02\x02\u{198}\u{19a}\x05\u{90}\x49\
	\x02\u{199}\u{198}\x03\x02\x02\x02\u{199}\u{19a}\x03\x02\x02\x02\u{19a}\
	\u{19c}\x03\x02\x02\x02\u{19b}\u{19d}\x05\x78\x3d\x02\u{19c}\u{19b}\x03\
	\x02\x02\x02\u{19c}\u{19d}\x03\x02\x02\x02\u{19d}\u{19e}\x03\x02\x02\x02\
	\u{19e}\u{19f}\x07\x07\x02\x02\u{19f}\x37\x03\x02\x02\x02\u{1a0}\u{1a2}\
	\x07\x1c\x02\x02\u{1a1}\u{1a3}\x07\u{84}\x02\x02\u{1a2}\u{1a1}\x03\x02\x02\
	\x02\u{1a3}\u{1a4}\x03\x02\x02\x02\u{1a4}\u{1a2}\x03\x02\x02\x02\u{1a4}\
	\u{1a5}\x03\x02\x02\x02\u{1a5}\u{1a6}\x03\x02\x02\x02\u{1a6}\u{1a7}\x07\
	\x07\x02\x02\u{1a7}\x39\x03\x02\x02\x02\u{1a8}\u{1a9}\x07\x1d\x02\x02\u{1a9}\
	\u{1aa}\x07\u{87}\x02\x02\u{1aa}\u{1ac}\x05\x42\x22\x02\u{1ab}\u{1ad}\x05\
	\x52\x2a\x02\u{1ac}\u{1ab}\x03\x02\x02\x02\u{1ac}\u{1ad}\x03\x02\x02\x02\
	\u{1ad}\u{1b7}\x03\x02\x02\x02\u{1ae}\u{1b0}\x07\x1d\x02\x02\u{1af}\u{1b1}\
	\x05\x3c\x1f\x02\u{1b0}\u{1af}\x03\x02\x02\x02\u{1b1}\u{1b2}\x03\x02\x02\
	\x02\u{1b2}\u{1b0}\x03\x02\x02\x02\u{1b2}\u{1b3}\x03\x02\x02\x02\u{1b3}\
	\u{1b4}\x03\x02\x02\x02\u{1b4}\u{1b5}\x07\x07\x02\x02\u{1b5}\u{1b7}\x03\
	\x02\x02\x02\u{1b6}\u{1a8}\x03\x02\x02\x02\u{1b6}\u{1ae}\x03\x02\x02\x02\
	\u{1b7}\x3b\x03\x02\x02\x02\u{1b8}\u{1b9}\x05\u{be}\x60\x02\u{1b9}\u{1ba}\
	\x07\u{87}\x02\x02\u{1ba}\u{1bc}\x05\x42\x22\x02\u{1bb}\u{1bd}\x05\x52\x2a\
	\x02\u{1bc}\u{1bb}\x03\x02\x02\x02\u{1bc}\u{1bd}\x03\x02\x02\x02\u{1bd}\
	\x3d\x03\x02\x02\x02\u{1be}\u{1bf}\x07\x1e\x02\x02\u{1bf}\u{1c0}\x07\u{87}\
	\x02\x02\u{1c0}\u{1c2}\x05\x42\x22\x02\u{1c1}\u{1c3}\x05\x52\x2a\x02\u{1c2}\
	\u{1c1}\x03\x02\x02\x02\u{1c2}\u{1c3}\x03\x02\x02\x02\u{1c3}\u{1cd}\x03\
	\x02\x02\x02\u{1c4}\u{1c6}\x07\x1e\x02\x02\u{1c5}\u{1c7}\x05\x40\x21\x02\
	\u{1c6}\u{1c5}\x03\x02\x02\x02\u{1c7}\u{1c8}\x03\x02\x02\x02\u{1c8}\u{1c6}\
	\x03\x02\x02\x02\u{1c8}\u{1c9}\x03\x02\x02\x02\u{1c9}\u{1ca}\x03\x02\x02\
	\x02\u{1ca}\u{1cb}\x07\x07\x02\x02\u{1cb}\u{1cd}\x03\x02\x02\x02\u{1cc}\
	\u{1be}\x03\x02\x02\x02\u{1cc}\u{1c4}\x03\x02\x02\x02\u{1cd}\x3f\x03\x02\
	\x02\x02\u{1ce}\u{1cf}\x05\u{be}\x60\x02\u{1cf}\u{1d0}\x07\u{87}\x02\x02\
	\u{1d0}\u{1d2}\x05\x42\x22\x02\u{1d1}\u{1d3}\x05\x52\x2a\x02\u{1d2}\u{1d1}\
	\x03\x02\x02\x02\u{1d2}\u{1d3}\x03\x02\x02\x02\u{1d3}\x41\x03\x02\x02\x02\
	\u{1d4}\u{1d8}\x05\x44\x23\x02\u{1d5}\u{1d7}\x05\x48\x25\x02\u{1d6}\u{1d5}\
	\x03\x02\x02\x02\u{1d7}\u{1da}\x03\x02\x02\x02\u{1d8}\u{1d6}\x03\x02\x02\
	\x02\u{1d8}\u{1d9}\x03\x02\x02\x02\u{1d9}\u{1df}\x03\x02\x02\x02\u{1da}\
	\u{1d8}\x03\x02\x02\x02\u{1db}\u{1df}\x05\x46\x24\x02\u{1dc}\u{1df}\x07\
	\u{84}\x02\x02\u{1dd}\u{1df}\x07\u{85}\x02\x02\u{1de}\u{1d4}\x03\x02\x02\
	\x02\u{1de}\u{1db}\x03\x02\x02\x02\u{1de}\u{1dc}\x03\x02\x02\x02\u{1de}\
	\u{1dd}\x03\x02\x02\x02\u{1df}\x43\x03\x02\x02\x02\u{1e0}\u{1e1}\x09\x05\
	\x02\x02\u{1e1}\x45\x03\x02\x02\x02\u{1e2}\u{1e3}\x07\x27\x02\x02\u{1e3}\
	\u{1e4}\x07\u{91}\x02\x02\u{1e4}\u{1e5}\x05\x42\x22\x02\u{1e5}\u{1e6}\x07\
	\u{92}\x02\x02\u{1e6}\u{1f4}\x03\x02\x02\x02\u{1e7}\u{1e8}\x07\x28\x02\x02\
	\u{1e8}\u{1e9}\x07\u{91}\x02\x02\u{1e9}\u{1ea}\x05\x42\x22\x02\u{1ea}\u{1eb}\
	\x07\u{92}\x02\x02\u{1eb}\u{1f4}\x03\x02\x02\x02\u{1ec}\u{1ed}\x07\x29\x02\
	\x02\u{1ed}\u{1ee}\x07\u{91}\x02\x02\u{1ee}\u{1ef}\x05\x42\x22\x02\u{1ef}\
	\u{1f0}\x07\u{88}\x02\x02\u{1f0}\u{1f1}\x05\x42\x22\x02\u{1f1}\u{1f2}\x07\
	\u{92}\x02\x02\u{1f2}\u{1f4}\x03\x02\x02\x02\u{1f3}\u{1e2}\x03\x02\x02\x02\
	\u{1f3}\u{1e7}\x03\x02\x02\x02\u{1f3}\u{1ec}\x03\x02\x02\x02\u{1f4}\x47\
	\x03\x02\x02\x02\u{1f5}\u{1f6}\x07\u{8b}\x02\x02\u{1f6}\u{1fb}\x05\x4a\x26\
	\x02\u{1f7}\u{1f8}\x07\u{88}\x02\x02\u{1f8}\u{1fa}\x05\x4a\x26\x02\u{1f9}\
	\u{1f7}\x03\x02\x02\x02\u{1fa}\u{1fd}\x03\x02\x02\x02\u{1fb}\u{1f9}\x03\
	\x02\x02\x02\u{1fb}\u{1fc}\x03\x02\x02\x02\u{1fc}\u{1fe}\x03\x02\x02\x02\
	\u{1fd}\u{1fb}\x03\x02\x02\x02\u{1fe}\u{1ff}\x07\u{8c}\x02\x02\u{1ff}\x49\
	\x03\x02\x02\x02\u{200}\u{201}\x07\x2a\x02\x02\u{201}\u{202}\x07\u{87}\x02\
	\x02\u{202}\u{219}\x05\x4c\x27\x02\u{203}\u{204}\x07\x2b\x02\x02\u{204}\
	\u{205}\x07\u{87}\x02\x02\u{205}\u{219}\x05\x4e\x28\x02\u{206}\u{207}\x07\
	\x2c\x02\x02\u{207}\u{208}\x07\u{87}\x02\x02\u{208}\u{219}\x07\u{86}\x02\
	\x02\u{209}\u{20a}\x07\x2d\x02\x02\u{20a}\u{20b}\x07\u{87}\x02\x02\u{20b}\
	\u{219}\x05\x50\x29\x02\u{20c}\u{20d}\x07\x2e\x02\x02\u{20d}\u{20e}\x07\
	\u{87}\x02\x02\u{20e}\u{213}\x07\x7e\x02\x02\u{20f}\u{210}\x07\u{88}\x02\
	\x02\u{210}\u{211}\x07\x2f\x02\x02\u{211}\u{212}\x07\u{87}\x02\x02\u{212}\
	\u{214}\x07\x7e\x02\x02\u{213}\u{20f}\x03\x02\x02\x02\u{213}\u{214}\x03\
	\x02\x02\x02\u{214}\u{219}\x03\x02\x02\x02\u{215}\u{216}\x07\x2f\x02\x02\
	\u{216}\u{217}\x07\u{87}\x02\x02\u{217}\u{219}\x07\x7e\x02\x02\u{218}\u{200}\
	\x03\x02\x02\x02\u{218}\u{203}\x03\x02\x02\x02\u{218}\u{206}\x03\x02\x02\
	\x02\u{218}\u{209}\x03\x02\x02\x02\u{218}\u{20c}\x03\x02\x02\x02\u{218}\
	\u{215}\x03\x02\x02\x02\u{219}\x4b\x03\x02\x02\x02\u{21a}\u{21b}\x05\u{c6}\
	\x64\x02\u{21b}\u{21c}\x07\u{89}\x02\x02\u{21c}\u{21d}\x05\u{c6}\x64\x02\
	\u{21d}\u{224}\x03\x02\x02\x02\u{21e}\u{21f}\x07\u{89}\x02\x02\u{21f}\u{224}\
	\x05\u{c6}\x64\x02\u{220}\u{221}\x05\u{c6}\x64\x02\u{221}\u{222}\x07\u{89}\
	\x02\x02\u{222}\u{224}\x03\x02\x02\x02\u{223}\u{21a}\x03\x02\x02\x02\u{223}\
	\u{21e}\x03\x02\x02\x02\u{223}\u{220}\x03\x02\x02\x02\u{224}\x4d\x03\x02\
	\x02\x02\u{225}\u{22e}\x07\x7e\x02\x02\u{226}\u{227}\x07\x7e\x02\x02\u{227}\
	\u{228}\x07\u{89}\x02\x02\u{228}\u{22e}\x07\x7e\x02\x02\u{229}\u{22a}\x07\
	\u{89}\x02\x02\u{22a}\u{22e}\x07\x7e\x02\x02\u{22b}\u{22c}\x07\x7e\x02\x02\
	\u{22c}\u{22e}\x07\u{89}\x02\x02\u{22d}\u{225}\x03\x02\x02\x02\u{22d}\u{226}\
	\x03\x02\x02\x02\u{22d}\u{229}\x03\x02\x02\x02\u{22d}\u{22b}\x03\x02\x02\
	\x02\u{22e}\x4f\x03\x02\x02\x02\u{22f}\u{234}\x07\u{84}\x02\x02\u{230}\u{231}\
	\x07\u{88}\x02\x02\u{231}\u{233}\x07\u{84}\x02\x02\u{232}\u{230}\x03\x02\
	\x02\x02\u{233}\u{236}\x03\x02\x02\x02\u{234}\u{232}\x03\x02\x02\x02\u{234}\
	\u{235}\x03\x02\x02\x02\u{235}\u{240}\x03\x02\x02\x02\u{236}\u{234}\x03\
	\x02\x02\x02\u{237}\u{23c}\x07\u{86}\x02\x02\u{238}\u{239}\x07\u{88}\x02\
	\x02\u{239}\u{23b}\x07\u{86}\x02\x02\u{23a}\u{238}\x03\x02\x02\x02\u{23b}\
	\u{23e}\x03\x02\x02\x02\u{23c}\u{23a}\x03\x02\x02\x02\u{23c}\u{23d}\x03\
	\x02\x02\x02\u{23d}\u{240}\x03\x02\x02\x02\u{23e}\u{23c}\x03\x02\x02\x02\
	\u{23f}\u{22f}\x03\x02\x02\x02\u{23f}\u{237}\x03\x02\x02\x02\u{240}\x51\
	\x03\x02\x02\x02\u{241}\u{242}\x07\u{88}\x02\x02\u{242}\u{247}\x05\x54\x2b\
	\x02\u{243}\u{244}\x07\u{88}\x02\x02\u{244}\u{246}\x05\x54\x2b\x02\u{245}\
	\u{243}\x03\x02\x02\x02\u{246}\u{249}\x03\x02\x02\x02\u{247}\u{245}\x03\
	\x02\x02\x02\u{247}\u{248}\x03\x02\x02\x02\u{248}\x53\x03\x02\x02\x02\u{249}\
	\u{247}\x03\x02\x02\x02\u{24a}\u{250}\x07\x30\x02\x02\u{24b}\u{250}\x07\
	\x0d\x02\x02\u{24c}\u{24d}\x07\u{83}\x02\x02\u{24d}\u{24e}\x07\u{87}\x02\
	\x02\u{24e}\u{250}\x05\u{94}\x4b\x02\u{24f}\u{24a}\x03\x02\x02\x02\u{24f}\
	\u{24b}\x03\x02\x02\x02\u{24f}\u{24c}\x03\x02\x02\x02\u{250}\x55\x03\x02\
	\x02\x02\u{251}\u{253}\x07\x31\x02\x02\u{252}\u{254}\x05\x58\x2d\x02\u{253}\
	\u{252}\x03\x02\x02\x02\u{254}\u{255}\x03\x02\x02\x02\u{255}\u{253}\x03\
	\x02\x02\x02\u{255}\u{256}\x03\x02\x02\x02\u{256}\u{257}\x03\x02\x02\x02\
	\u{257}\u{258}\x07\x07\x02\x02\u{258}\x57\x03\x02\x02\x02\u{259}\u{25c}\
	\x05\x5a\x2e\x02\u{25a}\u{25c}\x05\x5c\x2f\x02\u{25b}\u{259}\x03\x02\x02\
	\x02\u{25b}\u{25a}\x03\x02\x02\x02\u{25c}\x59\x03\x02\x02\x02\u{25d}\u{25e}\
	\x05\u{b8}\x5d\x02\u{25e}\u{25f}\x07\x32\x02\x02\u{25f}\u{260}\x05\u{94}\
	\x4b\x02\u{260}\u{265}\x03\x02\x02\x02\u{261}\u{262}\x07\u{84}\x02\x02\u{262}\
	\u{263}\x07\x32\x02\x02\u{263}\u{265}\x05\u{94}\x4b\x02\u{264}\u{25d}\x03\
	\x02\x02\x02\u{264}\u{261}\x03\x02\x02\x02\u{265}\x5b\x03\x02\x02\x02\u{266}\
	\u{267}\x07\x33\x02\x02\u{267}\u{268}\x07\u{84}\x02\x02\u{268}\u{269}\x07\
	\x32\x02\x02\u{269}\u{26a}\x05\u{94}\x4b\x02\u{26a}\x5d\x03\x02\x02\x02\
	\u{26b}\u{26d}\x07\x34\x02\x02\u{26c}\u{26e}\x05\x60\x31\x02\u{26d}\u{26c}\
	\x03\x02\x02\x02\u{26e}\u{26f}\x03\x02\x02\x02\u{26f}\u{26d}\x03\x02\x02\
	\x02\u{26f}\u{270}\x03\x02\x02\x02\u{270}\u{271}\x03\x02\x02\x02\u{271}\
	\u{272}\x07\x07\x02\x02\u{272}\x5f\x03\x02\x02\x02\u{273}\u{274}\x05\u{b8}\
	\x5d\x02\u{274}\u{275}\x07\x32\x02\x02\u{275}\u{276}\x05\u{94}\x4b\x02\u{276}\
	\x61\x03\x02\x02\x02\u{277}\u{279}\x07\x35\x02\x02\u{278}\u{27a}\x05\x64\
	\x33\x02\u{279}\u{278}\x03\x02\x02\x02\u{279}\u{27a}\x03\x02\x02\x02\u{27a}\
	\u{27c}\x03\x02\x02\x02\u{27b}\u{27d}\x05\x66\x34\x02\u{27c}\u{27b}\x03\
	\x02\x02\x02\u{27d}\u{27e}\x03\x02\x02\x02\u{27e}\u{27c}\x03\x02\x02\x02\
	\u{27e}\u{27f}\x03\x02\x02\x02\u{27f}\u{280}\x03\x02\x02\x02\u{280}\u{282}\
	\x07\x07\x02\x02\u{281}\u{283}\x05\x68\x35\x02\u{282}\u{281}\x03\x02\x02\
	\x02\u{282}\u{283}\x03\x02\x02\x02\u{283}\x63\x03\x02\x02\x02\u{284}\u{285}\
	\x09\x06\x02\x02\u{285}\x65\x03\x02\x02\x02\u{286}\u{290}\x07\u{84}\x02\
	\x02\u{287}\u{288}\x07\x39\x02\x02\u{288}\u{289}\x05\u{94}\x4b\x02\u{289}\
	\u{28a}\x07\u{87}\x02\x02\u{28a}\u{28b}\x07\u{84}\x02\x02\u{28b}\u{290}\
	\x03\x02\x02\x02\u{28c}\u{28d}\x07\x3a\x02\x02\u{28d}\u{28e}\x07\u{87}\x02\
	\x02\u{28e}\u{290}\x07\u{84}\x02\x02\u{28f}\u{286}\x03\x02\x02\x02\u{28f}\
	\u{287}\x03\x02\x02\x02\u{28f}\u{28c}\x03\x02\x02\x02\u{290}\x67\x03\x02\
	\x02\x02\u{291}\u{293}\x07\x3b\x02\x02\u{292}\u{294}\x05\x64\x33\x02\u{293}\
	\u{292}\x03\x02\x02\x02\u{293}\u{294}\x03\x02\x02\x02\u{294}\u{296}\x03\
	\x02\x02\x02\u{295}\u{297}\x05\x66\x34\x02\u{296}\u{295}\x03\x02\x02\x02\
	\u{297}\u{298}\x03\x02\x02\x02\u{298}\u{296}\x03\x02\x02\x02\u{298}\u{299}\
	\x03\x02\x02\x02\u{299}\u{29a}\x03\x02\x02\x02\u{29a}\u{29b}\x07\x07\x02\
	\x02\u{29b}\x69\x03\x02\x02\x02\u{29c}\u{29e}\x07\x3c\x02\x02\u{29d}\u{29f}\
	\x05\x6e\x38\x02\u{29e}\u{29d}\x03\x02\x02\x02\u{29f}\u{2a0}\x03\x02\x02\
	\x02\u{2a0}\u{29e}\x03\x02\x02\x02\u{2a0}\u{2a1}\x03\x02\x02\x02\u{2a1}\
	\u{2a2}\x03\x02\x02\x02\u{2a2}\u{2a3}\x07\x07\x02\x02\u{2a3}\x6b\x03\x02\
	\x02\x02\u{2a4}\u{2a6}\x07\x3d\x02\x02\u{2a5}\u{2a7}\x05\x6e\x38\x02\u{2a6}\
	\u{2a5}\x03\x02\x02\x02\u{2a7}\u{2a8}\x03\x02\x02\x02\u{2a8}\u{2a6}\x03\
	\x02\x02\x02\u{2a8}\u{2a9}\x03\x02\x02\x02\u{2a9}\u{2aa}\x03\x02\x02\x02\
	\u{2aa}\u{2ab}\x07\x07\x02\x02\u{2ab}\x6d\x03\x02\x02\x02\u{2ac}\u{2ad}\
	\x05\u{94}\x4b\x02\u{2ad}\u{2ae}\x07\u{87}\x02\x02\u{2ae}\u{2af}\x05\x70\
	\x39\x02\u{2af}\u{2c0}\x03\x02\x02\x02\u{2b0}\u{2b1}\x07\x39\x02\x02\u{2b1}\
	\u{2b2}\x05\u{94}\x4b\x02\u{2b2}\u{2b4}\x07\u{87}\x02\x02\u{2b3}\u{2b5}\
	\x05\x6e\x38\x02\u{2b4}\u{2b3}\x03\x02\x02\x02\u{2b5}\u{2b6}\x03\x02\x02\
	\x02\u{2b6}\u{2b4}\x03\x02\x02\x02\u{2b6}\u{2b7}\x03\x02\x02\x02\u{2b7}\
	\u{2b8}\x03\x02\x02\x02\u{2b8}\u{2b9}\x07\x07\x02\x02\u{2b9}\u{2c0}\x03\
	\x02\x02\x02\u{2ba}\u{2bb}\x07\x3e\x02\x02\u{2bb}\u{2bc}\x05\u{94}\x4b\x02\
	\u{2bc}\u{2bd}\x07\x3f\x02\x02\u{2bd}\u{2be}\x05\x70\x39\x02\u{2be}\u{2c0}\
	\x03\x02\x02\x02\u{2bf}\u{2ac}\x03\x02\x02\x02\u{2bf}\u{2b0}\x03\x02\x02\
	\x02\u{2bf}\u{2ba}\x03\x02\x02\x02\u{2c0}\x6f\x03\x02\x02\x02\u{2c1}\u{2c4}\
	\x07\u{86}\x02\x02\u{2c2}\u{2c4}\x05\x72\x3a\x02\u{2c3}\u{2c1}\x03\x02\x02\
	\x02\u{2c3}\u{2c2}\x03\x02\x02\x02\u{2c4}\x71\x03\x02\x02\x02\u{2c5}\u{2c6}\
	\x07\x40\x02\x02\u{2c6}\u{2c7}\x07\u{87}\x02\x02\u{2c7}\u{2cb}\x07\u{86}\
	\x02\x02\u{2c8}\u{2c9}\x07\x41\x02\x02\u{2c9}\u{2ca}\x07\u{87}\x02\x02\u{2ca}\
	\u{2cc}\x07\u{86}\x02\x02\u{2cb}\u{2c8}\x03\x02\x02\x02\u{2cb}\u{2cc}\x03\
	\x02\x02\x02\u{2cc}\u{2d0}\x03\x02\x02\x02\u{2cd}\u{2ce}\x07\x42\x02\x02\
	\u{2ce}\u{2cf}\x07\u{87}\x02\x02\u{2cf}\u{2d1}\x05\x74\x3b\x02\u{2d0}\u{2cd}\
	\x03\x02\x02\x02\u{2d0}\u{2d1}\x03\x02\x02\x02\u{2d1}\x73\x03\x02\x02\x02\
	\u{2d2}\u{2d3}\x09\x07\x02\x02\u{2d3}\x75\x03\x02\x02\x02\u{2d4}\u{2d6}\
	\x07\x46\x02\x02\u{2d5}\u{2d7}\x05\x6e\x38\x02\u{2d6}\u{2d5}\x03\x02\x02\
	\x02\u{2d7}\u{2d8}\x03\x02\x02\x02\u{2d8}\u{2d6}\x03\x02\x02\x02\u{2d8}\
	\u{2d9}\x03\x02\x02\x02\u{2d9}\u{2da}\x03\x02\x02\x02\u{2da}\u{2db}\x07\
	\x07\x02\x02\u{2db}\x77\x03\x02\x02\x02\u{2dc}\u{2de}\x07\x47\x02\x02\u{2dd}\
	\u{2df}\x05\x7a\x3e\x02\u{2de}\u{2dd}\x03\x02\x02\x02\u{2df}\u{2e0}\x03\
	\x02\x02\x02\u{2e0}\u{2de}\x03\x02\x02\x02\u{2e0}\u{2e1}\x03\x02\x02\x02\
	\u{2e1}\u{2e2}\x03\x02\x02\x02\u{2e2}\u{2e3}\x07\x07\x02\x02\u{2e3}\x79\
	\x03\x02\x02\x02\u{2e4}\u{2e9}\x05\x7c\x3f\x02\u{2e5}\u{2e9}\x05\x7e\x40\
	\x02\u{2e6}\u{2e9}\x05\u{80}\x41\x02\u{2e7}\u{2e9}\x05\u{84}\x43\x02\u{2e8}\
	\u{2e4}\x03\x02\x02\x02\u{2e8}\u{2e5}\x03\x02\x02\x02\u{2e8}\u{2e6}\x03\
	\x02\x02\x02\u{2e8}\u{2e7}\x03\x02\x02\x02\u{2e9}\x7b\x03\x02\x02\x02\u{2ea}\
	\u{2eb}\x07\x48\x02\x02\u{2eb}\u{2ec}\x07\u{87}\x02\x02\u{2ec}\u{2fa}\x05\
	\u{88}\x45\x02\u{2ed}\u{2ee}\x07\u{83}\x02\x02\u{2ee}\u{2ef}\x07\u{87}\x02\
	\x02\u{2ef}\u{2fa}\x05\u{94}\x4b\x02\u{2f0}\u{2f1}\x07\x49\x02\x02\u{2f1}\
	\u{2f2}\x07\u{87}\x02\x02\u{2f2}\u{2fa}\x05\u{8a}\x46\x02\u{2f3}\u{2f4}\
	\x07\x4a\x02\x02\u{2f4}\u{2f5}\x07\u{87}\x02\x02\u{2f5}\u{2fa}\x07\u{84}\
	\x02\x02\u{2f6}\u{2f7}\x07\x4b\x02\x02\u{2f7}\u{2f8}\x07\u{87}\x02\x02\u{2f8}\
	\u{2fa}\x07\u{86}\x02\x02\u{2f9}\u{2ea}\x03\x02\x02\x02\u{2f9}\u{2ed}\x03\
	\x02\x02\x02\u{2f9}\u{2f0}\x03\x02\x02\x02\u{2f9}\u{2f3}\x03\x02\x02\x02\
	\u{2f9}\u{2f6}\x03\x02\x02\x02\u{2fa}\x7d\x03\x02\x02\x02\u{2fb}\u{2fc}\
	\x07\x4c\x02\x02\u{2fc}\u{2fd}\x07\u{8d}\x02\x02\u{2fd}\u{2fe}\x07\u{86}\
	\x02\x02\u{2fe}\u{2ff}\x07\u{8e}\x02\x02\u{2ff}\x7f\x03\x02\x02\x02\u{300}\
	\u{301}\x07\x4d\x02\x02\u{301}\u{302}\x07\x4e\x02\x02\u{302}\u{303}\x05\
	\u{82}\x42\x02\u{303}\u{81}\x03\x02\x02\x02\u{304}\u{30a}\x07\x4f\x02\x02\
	\u{305}\u{307}\x07\x50\x02\x02\u{306}\u{308}\x07\x51\x02\x02\u{307}\u{306}\
	\x03\x02\x02\x02\u{307}\u{308}\x03\x02\x02\x02\u{308}\u{30a}\x03\x02\x02\
	\x02\u{309}\u{304}\x03\x02\x02\x02\u{309}\u{305}\x03\x02\x02\x02\u{30a}\
	\u{83}\x03\x02\x02\x02\u{30b}\u{30c}\x07\x52\x02\x02\u{30c}\u{30d}\x07\x4e\
	\x02\x02\u{30d}\u{30e}\x05\u{86}\x44\x02\u{30e}\u{85}\x03\x02\x02\x02\u{30f}\
	\u{310}\x07\x41\x02\x02\u{310}\u{313}\x07\u{86}\x02\x02\u{311}\u{313}\x07\
	\u{86}\x02\x02\u{312}\u{30f}\x03\x02\x02\x02\u{312}\u{311}\x03\x02\x02\x02\
	\u{313}\u{87}\x03\x02\x02\x02\u{314}\u{315}\x09\x08\x02\x02\u{315}\u{89}\
	\x03\x02\x02\x02\u{316}\u{317}\x09\x09\x02\x02\u{317}\u{8b}\x03\x02\x02\
	\x02\u{318}\u{31a}\x07\x57\x02\x02\u{319}\u{31b}\x05\u{8e}\x48\x02\u{31a}\
	\u{319}\x03\x02\x02\x02\u{31b}\u{31c}\x03\x02\x02\x02\u{31c}\u{31a}\x03\
	\x02\x02\x02\u{31c}\u{31d}\x03\x02\x02\x02\u{31d}\u{31e}\x03\x02\x02\x02\
	\u{31e}\u{31f}\x07\x07\x02\x02\u{31f}\u{8d}\x03\x02\x02\x02\u{320}\u{321}\
	\x07\x52\x02\x02\u{321}\u{322}\x07\u{87}\x02\x02\u{322}\u{330}\x07\u{81}\
	\x02\x02\u{323}\u{324}\x07\x4b\x02\x02\u{324}\u{325}\x07\u{87}\x02\x02\u{325}\
	\u{330}\x07\u{86}\x02\x02\u{326}\u{327}\x07\x58\x02\x02\u{327}\u{328}\x07\
	\u{87}\x02\x02\u{328}\u{330}\x05\u{94}\x4b\x02\u{329}\u{32a}\x07\x4a\x02\
	\x02\u{32a}\u{32b}\x07\u{87}\x02\x02\u{32b}\u{330}\x07\u{84}\x02\x02\u{32c}\
	\u{32d}\x07\x59\x02\x02\u{32d}\u{32e}\x07\u{87}\x02\x02\u{32e}\u{330}\x07\
	\u{81}\x02\x02\u{32f}\u{320}\x03\x02\x02\x02\u{32f}\u{323}\x03\x02\x02\x02\
	\u{32f}\u{326}\x03\x02\x02\x02\u{32f}\u{329}\x03\x02\x02\x02\u{32f}\u{32c}\
	\x03\x02\x02\x02\u{330}\u{8f}\x03\x02\x02\x02\u{331}\u{332}\x07\x5a\x02\
	\x02\u{332}\u{333}\x05\u{bc}\x5f\x02\u{333}\u{334}\x05\u{92}\x4a\x02\u{334}\
	\u{335}\x07\x07\x02\x02\u{335}\u{91}\x03\x02\x02\x02\u{336}\u{338}\x07\x5b\
	\x02\x02\u{337}\u{339}\x05\x5a\x2e\x02\u{338}\u{337}\x03\x02\x02\x02\u{339}\
	\u{33a}\x03\x02\x02\x02\u{33a}\u{338}\x03\x02\x02\x02\u{33a}\u{33b}\x03\
	\x02\x02\x02\u{33b}\u{33c}\x03\x02\x02\x02\u{33c}\u{33d}\x07\x07\x02\x02\
	\u{33d}\u{93}\x03\x02\x02\x02\u{33e}\u{33f}\x08\x4b\x01\x02\u{33f}\u{340}\
	\x05\u{b0}\x59\x02\u{340}\u{341}\x05\u{94}\x4b\x10\u{341}\u{344}\x03\x02\
	\x02\x02\u{342}\u{344}\x05\u{96}\x4c\x02\u{343}\u{33e}\x03\x02\x02\x02\u{343}\
	\u{342}\x03\x02\x02\x02\u{344}\u{37a}\x03\x02\x02\x02\u{345}\u{346}\x0c\
	\x0f\x02\x02\u{346}\u{347}\x05\u{aa}\x56\x02\u{347}\u{348}\x05\u{94}\x4b\
	\x10\u{348}\u{379}\x03\x02\x02\x02\u{349}\u{34a}\x0c\x0e\x02\x02\u{34a}\
	\u{34b}\x05\u{ac}\x57\x02\u{34b}\u{34c}\x05\u{94}\x4b\x0f\u{34c}\u{379}\
	\x03\x02\x02\x02\u{34d}\u{34e}\x0c\x0d\x02\x02\u{34e}\u{34f}\x05\u{ae}\x58\
	\x02\u{34f}\u{350}\x05\u{94}\x4b\x0e\u{350}\u{379}\x03\x02\x02\x02\u{351}\
	\u{352}\x0c\x0c\x02\x02\u{352}\u{353}\x09\x0a\x02\x02\u{353}\u{379}\x05\
	\u{94}\x4b\x0d\u{354}\u{355}\x0c\x0b\x02\x02\u{355}\u{356}\x07\x5c\x02\x02\
	\u{356}\u{357}\x05\u{94}\x4b\x02\u{357}\u{358}\x07\x5d\x02\x02\u{358}\u{359}\
	\x05\u{94}\x4b\x0c\u{359}\u{379}\x03\x02\x02\x02\u{35a}\u{35b}\x0c\x0a\x02\
	\x02\u{35b}\u{35c}\x07\x5e\x02\x02\u{35c}\u{35d}\x07\x5c\x02\x02\u{35d}\
	\u{35e}\x05\u{94}\x4b\x02\u{35e}\u{35f}\x07\x5d\x02\x02\u{35f}\u{360}\x05\
	\u{94}\x4b\x0b\u{360}\u{379}\x03\x02\x02\x02\u{361}\u{362}\x0c\x09\x02\x02\
	\u{362}\u{363}\x07\x5f\x02\x02\u{363}\u{379}\x05\u{b6}\x5c\x02\u{364}\u{365}\
	\x0c\x08\x02\x02\u{365}\u{366}\x07\x5f\x02\x02\u{366}\u{367}\x07\u{8b}\x02\
	\x02\u{367}\u{368}\x05\u{a0}\x51\x02\u{368}\u{369}\x07\u{8c}\x02\x02\u{369}\
	\u{379}\x03\x02\x02\x02\u{36a}\u{36b}\x0c\x07\x02\x02\u{36b}\u{36c}\x07\
	\x5e\x02\x02\u{36c}\u{36d}\x07\x5f\x02\x02\u{36d}\u{379}\x05\u{b6}\x5c\x02\
	\u{36e}\u{36f}\x0c\x06\x02\x02\u{36f}\u{370}\x07\x60\x02\x02\u{370}\u{379}\
	\x07\x61\x02\x02\u{371}\u{372}\x0c\x05\x02\x02\u{372}\u{373}\x07\x60\x02\
	\x02\u{373}\u{374}\x07\x5e\x02\x02\u{374}\u{379}\x07\x61\x02\x02\u{375}\
	\u{376}\x0c\x04\x02\x02\u{376}\u{377}\x07\x62\x02\x02\u{377}\u{379}\x07\
	\u{86}\x02\x02\u{378}\u{345}\x03\x02\x02\x02\u{378}\u{349}\x03\x02\x02\x02\
	\u{378}\u{34d}\x03\x02\x02\x02\u{378}\u{351}\x03\x02\x02\x02\u{378}\u{354}\
	\x03\x02\x02\x02\u{378}\u{35a}\x03\x02\x02\x02\u{378}\u{361}\x03\x02\x02\
	\x02\u{378}\u{364}\x03\x02\x02\x02\u{378}\u{36a}\x03\x02\x02\x02\u{378}\
	\u{36e}\x03\x02\x02\x02\u{378}\u{371}\x03\x02\x02\x02\u{378}\u{375}\x03\
	\x02\x02\x02\u{379}\u{37c}\x03\x02\x02\x02\u{37a}\u{378}\x03\x02\x02\x02\
	\u{37a}\u{37b}\x03\x02\x02\x02\u{37b}\u{95}\x03\x02\x02\x02\u{37c}\u{37a}\
	\x03\x02\x02\x02\u{37d}\u{38b}\x05\u{c4}\x63\x02\u{37e}\u{38b}\x05\u{b8}\
	\x5d\x02\u{37f}\u{38b}\x05\u{b2}\x5a\x02\u{380}\u{381}\x07\u{8d}\x02\x02\
	\u{381}\u{382}\x05\u{94}\x4b\x02\u{382}\u{383}\x07\u{8e}\x02\x02\u{383}\
	\u{38b}\x03\x02\x02\x02\u{384}\u{38b}\x05\u{a2}\x52\x02\u{385}\u{38b}\x05\
	\u{a4}\x53\x02\u{386}\u{38b}\x05\u{a6}\x54\x02\u{387}\u{38b}\x05\u{98}\x4d\
	\x02\u{388}\u{38b}\x05\u{9e}\x50\x02\u{389}\u{38b}\x05\u{b6}\x5c\x02\u{38a}\
	\u{37d}\x03\x02\x02\x02\u{38a}\u{37e}\x03\x02\x02\x02\u{38a}\u{37f}\x03\
	\x02\x02\x02\u{38a}\u{380}\x03\x02\x02\x02\u{38a}\u{384}\x03\x02\x02\x02\
	\u{38a}\u{385}\x03\x02\x02\x02\u{38a}\u{386}\x03\x02\x02\x02\u{38a}\u{387}\
	\x03\x02\x02\x02\u{38a}\u{388}\x03\x02\x02\x02\u{38a}\u{389}\x03\x02\x02\
	\x02\u{38b}\u{97}\x03\x02\x02\x02\u{38c}\u{38d}\x07\u{8f}\x02\x02\u{38d}\
	\u{392}\x05\u{9a}\x4e\x02\u{38e}\u{38f}\x07\u{88}\x02\x02\u{38f}\u{391}\
	\x05\u{9a}\x4e\x02\u{390}\u{38e}\x03\x02\x02\x02\u{391}\u{394}\x03\x02\x02\
	\x02\u{392}\u{390}\x03\x02\x02\x02\u{392}\u{393}\x03\x02\x02\x02\u{393}\
	\u{395}\x03\x02\x02\x02\u{394}\u{392}\x03\x02\x02\x02\u{395}\u{396}\x07\
	\u{90}\x02\x02\u{396}\u{39a}\x03\x02\x02\x02\u{397}\u{398}\x07\u{8f}\x02\
	\x02\u{398}\u{39a}\x07\u{90}\x02\x02\u{399}\u{38c}\x03\x02\x02\x02\u{399}\
	\u{397}\x03\x02\x02\x02\u{39a}\u{99}\x03\x02\x02\x02\u{39b}\u{39c}\x05\u{9c}\
	\x4f\x02\u{39c}\u{39d}\x07\u{87}\x02\x02\u{39d}\u{39e}\x05\u{94}\x4b\x02\
	\u{39e}\u{9b}\x03\x02\x02\x02\u{39f}\u{3a0}\x09\x0b\x02\x02\u{3a0}\u{9d}\
	\x03\x02\x02\x02\u{3a1}\u{3a2}\x07\u{84}\x02\x02\u{3a2}\u{3a3}\x07\u{9d}\
	\x02\x02\u{3a3}\u{3b1}\x05\u{94}\x4b\x02\u{3a4}\u{3a5}\x07\u{8d}\x02\x02\
	\u{3a5}\u{3aa}\x07\u{84}\x02\x02\u{3a6}\u{3a7}\x07\u{88}\x02\x02\u{3a7}\
	\u{3a9}\x07\u{84}\x02\x02\u{3a8}\u{3a6}\x03\x02\x02\x02\u{3a9}\u{3ac}\x03\
	\x02\x02\x02\u{3aa}\u{3a8}\x03\x02\x02\x02\u{3aa}\u{3ab}\x03\x02\x02\x02\
	\u{3ab}\u{3ad}\x03\x02\x02\x02\u{3ac}\u{3aa}\x03\x02\x02\x02\u{3ad}\u{3ae}\
	\x07\u{8e}\x02\x02\u{3ae}\u{3af}\x07\u{9d}\x02\x02\u{3af}\u{3b1}\x05\u{94}\
	\x4b\x02\u{3b0}\u{3a1}\x03\x02\x02\x02\u{3b0}\u{3a4}\x03\x02\x02\x02\u{3b1}\
	\u{9f}\x03\x02\x02\x02\u{3b2}\u{3b7}\x05\u{94}\x4b\x02\u{3b3}\u{3b4}\x07\
	\u{88}\x02\x02\u{3b4}\u{3b6}\x05\u{94}\x4b\x02\u{3b5}\u{3b3}\x03\x02\x02\
	\x02\u{3b6}\u{3b9}\x03\x02\x02\x02\u{3b7}\u{3b5}\x03\x02\x02\x02\u{3b7}\
	\u{3b8}\x03\x02\x02\x02\u{3b8}\u{a1}\x03\x02\x02\x02\u{3b9}\u{3b7}\x03\x02\
	\x02\x02\u{3ba}\u{3bb}\x07\x39\x02\x02\u{3bb}\u{3bc}\x05\u{94}\x4b\x02\u{3bc}\
	\u{3bd}\x07\u{87}\x02\x02\u{3bd}\u{3c5}\x05\u{94}\x4b\x02\u{3be}\u{3bf}\
	\x07\x39\x02\x02\u{3bf}\u{3c0}\x05\u{94}\x4b\x02\u{3c0}\u{3c1}\x07\u{87}\
	\x02\x02\u{3c1}\u{3c2}\x05\u{94}\x4b\x02\u{3c2}\u{3c4}\x03\x02\x02\x02\u{3c3}\
	\u{3be}\x03\x02\x02\x02\u{3c4}\u{3c7}\x03\x02\x02\x02\u{3c5}\u{3c3}\x03\
	\x02\x02\x02\u{3c5}\u{3c6}\x03\x02\x02\x02\u{3c6}\u{3c8}\x03\x02\x02\x02\
	\u{3c7}\u{3c5}\x03\x02\x02\x02\u{3c8}\u{3c9}\x07\x3a\x02\x02\u{3c9}\u{3ca}\
	\x07\u{87}\x02\x02\u{3ca}\u{3cb}\x05\u{94}\x4b\x02\u{3cb}\u{3de}\x03\x02\
	\x02\x02\u{3cc}\u{3cd}\x07\x39\x02\x02\u{3cd}\u{3ce}\x05\u{94}\x4b\x02\u{3ce}\
	\u{3cf}\x07\x3b\x02\x02\u{3cf}\u{3d7}\x05\u{94}\x4b\x02\u{3d0}\u{3d1}\x07\
	\x39\x02\x02\u{3d1}\u{3d2}\x05\u{94}\x4b\x02\u{3d2}\u{3d3}\x07\x3b\x02\x02\
	\u{3d3}\u{3d4}\x05\u{94}\x4b\x02\u{3d4}\u{3d6}\x03\x02\x02\x02\u{3d5}\u{3d0}\
	\x03\x02\x02\x02\u{3d6}\u{3d9}\x03\x02\x02\x02\u{3d7}\u{3d5}\x03\x02\x02\
	\x02\u{3d7}\u{3d8}\x03\x02\x02\x02\u{3d8}\u{3da}\x03\x02\x02\x02\u{3d9}\
	\u{3d7}\x03\x02\x02\x02\u{3da}\u{3db}\x07\x3a\x02\x02\u{3db}\u{3dc}\x05\
	\u{94}\x4b\x02\u{3dc}\u{3de}\x03\x02\x02\x02\u{3dd}\u{3ba}\x03\x02\x02\x02\
	\u{3dd}\u{3cc}\x03\x02\x02\x02\u{3de}\u{a3}\x03\x02\x02\x02\u{3df}\u{3e0}\
	\x05\u{b8}\x5d\x02\u{3e0}\u{3e1}\x07\u{8b}\x02\x02\u{3e1}\u{3e2}\x05\u{94}\
	\x4b\x02\u{3e2}\u{3e7}\x07\u{8c}\x02\x02\u{3e3}\u{3e4}\x07\u{8a}\x02\x02\
	\u{3e4}\u{3e6}\x05\u{ba}\x5e\x02\u{3e5}\u{3e3}\x03\x02\x02\x02\u{3e6}\u{3e9}\
	\x03\x02\x02\x02\u{3e7}\u{3e5}\x03\x02\x02\x02\u{3e7}\u{3e8}\x03\x02\x02\
	\x02\u{3e8}\u{a5}\x03\x02\x02\x02\u{3e9}\u{3e7}\x03\x02\x02\x02\u{3ea}\u{3ed}\
	\x05\u{b8}\x5d\x02\u{3eb}\u{3ec}\x07\u{9e}\x02\x02\u{3ec}\u{3ee}\x05\u{ba}\
	\x5e\x02\u{3ed}\u{3eb}\x03\x02\x02\x02\u{3ee}\u{3ef}\x03\x02\x02\x02\u{3ef}\
	\u{3ed}\x03\x02\x02\x02\u{3ef}\u{3f0}\x03\x02\x02\x02\u{3f0}\u{a7}\x03\x02\
	\x02\x02\u{3f1}\u{3f5}\x05\u{aa}\x56\x02\u{3f2}\u{3f5}\x05\u{ac}\x57\x02\
	\u{3f3}\u{3f5}\x05\u{ae}\x58\x02\u{3f4}\u{3f1}\x03\x02\x02\x02\u{3f4}\u{3f2}\
	\x03\x02\x02\x02\u{3f4}\u{3f3}\x03\x02\x02\x02\u{3f5}\u{a9}\x03\x02\x02\
	\x02\u{3f6}\u{3f7}\x09\x0c\x02\x02\u{3f7}\u{ab}\x03\x02\x02\x02\u{3f8}\u{3f9}\
	\x09\x0d\x02\x02\u{3f9}\u{ad}\x03\x02\x02\x02\u{3fa}\u{3fb}\x09\x0e\x02\
	\x02\u{3fb}\u{af}\x03\x02\x02\x02\u{3fc}\u{3fd}\x09\x0f\x02\x02\u{3fd}\u{b1}\
	\x03\x02\x02\x02\u{3fe}\u{3ff}\x05\u{b4}\x5b\x02\u{3ff}\u{408}\x07\u{8d}\
	\x02\x02\u{400}\u{405}\x05\u{94}\x4b\x02\u{401}\u{402}\x07\u{88}\x02\x02\
	\u{402}\u{404}\x05\u{94}\x4b\x02\u{403}\u{401}\x03\x02\x02\x02\u{404}\u{407}\
	\x03\x02\x02\x02\u{405}\u{403}\x03\x02\x02\x02\u{405}\u{406}\x03\x02\x02\
	\x02\u{406}\u{409}\x03\x02\x02\x02\u{407}\u{405}\x03\x02\x02\x02\u{408}\
	\u{400}\x03\x02\x02\x02\u{408}\u{409}\x03\x02\x02\x02\u{409}\u{40a}\x03\
	\x02\x02\x02\u{40a}\u{40b}\x07\u{8e}\x02\x02\u{40b}\u{41d}\x03\x02\x02\x02\
	\u{40c}\u{40d}\x05\u{b8}\x5d\x02\u{40d}\u{40e}\x07\u{8a}\x02\x02\u{40e}\
	\u{40f}\x05\u{b4}\x5b\x02\u{40f}\u{418}\x07\u{8d}\x02\x02\u{410}\u{415}\
	\x05\u{94}\x4b\x02\u{411}\u{412}\x07\u{88}\x02\x02\u{412}\u{414}\x05\u{94}\
	\x4b\x02\u{413}\u{411}\x03\x02\x02\x02\u{414}\u{417}\x03\x02\x02\x02\u{415}\
	\u{413}\x03\x02\x02\x02\u{415}\u{416}\x03\x02\x02\x02\u{416}\u{419}\x03\
	\x02\x02\x02\u{417}\u{415}\x03\x02\x02\x02\u{418}\u{410}\x03\x02\x02\x02\
	\u{418}\u{419}\x03\x02\x02\x02\u{419}\u{41a}\x03\x02\x02\x02\u{41a}\u{41b}\
	\x07\u{8e}\x02\x02\u{41b}\u{41d}\x03\x02\x02\x02\u{41c}\u{3fe}\x03\x02\x02\
	\x02\u{41c}\u{40c}\x03\x02\x02\x02\u{41d}\u{b3}\x03\x02\x02\x02\u{41e}\u{41f}\
	\x09\x10\x02\x02\u{41f}\u{b5}\x03\x02\x02\x02\u{420}\u{429}\x07\u{8b}\x02\
	\x02\u{421}\u{426}\x05\u{94}\x4b\x02\u{422}\u{423}\x07\u{88}\x02\x02\u{423}\
	\u{425}\x05\u{94}\x4b\x02\u{424}\u{422}\x03\x02\x02\x02\u{425}\u{428}\x03\
	\x02\x02\x02\u{426}\u{424}\x03\x02\x02\x02\u{426}\u{427}\x03\x02\x02\x02\
	\u{427}\u{42a}\x03\x02\x02\x02\u{428}\u{426}\x03\x02\x02\x02\u{429}\u{421}\
	\x03\x02\x02\x02\u{429}\u{42a}\x03\x02\x02\x02\u{42a}\u{42b}\x03\x02\x02\
	\x02\u{42b}\u{42c}\x07\u{8c}\x02\x02\u{42c}\u{b7}\x03\x02\x02\x02\u{42d}\
	\u{432}\x05\u{ba}\x5e\x02\u{42e}\u{42f}\x07\u{8a}\x02\x02\u{42f}\u{431}\
	\x05\u{ba}\x5e\x02\u{430}\u{42e}\x03\x02\x02\x02\u{431}\u{434}\x03\x02\x02\
	\x02\u{432}\u{430}\x03\x02\x02\x02\u{432}\u{433}\x03\x02\x02\x02\u{433}\
	\u{b9}\x03\x02\x02\x02\u{434}\u{432}\x03\x02\x02\x02\u{435}\u{436}\x09\x11\
	\x02\x02\u{436}\u{bb}\x03\x02\x02\x02\u{437}\u{438}\x07\u{8b}\x02\x02\u{438}\
	\u{43d}\x05\u{b8}\x5d\x02\u{439}\u{43a}\x07\u{88}\x02\x02\u{43a}\u{43c}\
	\x05\u{b8}\x5d\x02\u{43b}\u{439}\x03\x02\x02\x02\u{43c}\u{43f}\x03\x02\x02\
	\x02\u{43d}\u{43b}\x03\x02\x02\x02\u{43d}\u{43e}\x03\x02\x02\x02\u{43e}\
	\u{440}\x03\x02\x02\x02\u{43f}\u{43d}\x03\x02\x02\x02\u{440}\u{441}\x07\
	\u{8c}\x02\x02\u{441}\u{bd}\x03\x02\x02\x02\u{442}\u{443}\x07\u{84}\x02\
	\x02\u{443}\u{bf}\x03\x02\x02\x02\u{444}\u{445}\x07\x7e\x02\x02\u{445}\u{448}\
	\x05\u{c2}\x62\x02\u{446}\u{448}\x07\u{80}\x02\x02\u{447}\u{444}\x03\x02\
	\x02\x02\u{447}\u{446}\x03\x02\x02\x02\u{448}\u{c1}\x03\x02\x02\x02\u{449}\
	\u{44a}\x09\x12\x02\x02\u{44a}\u{c3}\x03\x02\x02\x02\u{44b}\u{450}\x07\u{86}\
	\x02\x02\u{44c}\u{450}\x05\u{c6}\x64\x02\u{44d}\u{450}\x07\u{81}\x02\x02\
	\u{44e}\u{450}\x07\x61\x02\x02\u{44f}\u{44b}\x03\x02\x02\x02\u{44f}\u{44c}\
	\x03\x02\x02\x02\u{44f}\u{44d}\x03\x02\x02\x02\u{44f}\u{44e}\x03\x02\x02\
	\x02\u{450}\u{c5}\x03\x02\x02\x02\u{451}\u{458}\x07\x7e\x02\x02\u{452}\u{458}\
	\x07\x7f\x02\x02\u{453}\u{454}\x07\u{99}\x02\x02\u{454}\u{458}\x07\x7e\x02\
	\x02\u{455}\u{456}\x07\u{99}\x02\x02\u{456}\u{458}\x07\x7f\x02\x02\u{457}\
	\u{451}\x03\x02\x02\x02\u{457}\u{452}\x03\x02\x02\x02\u{457}\u{453}\x03\
	\x02\x02\x02\u{457}\u{455}\x03\x02\x02\x02\u{458}\u{c7}\x03\x02\x02\x02\
	\x75\u{cb}\u{d0}\u{d2}\u{dc}\u{e8}\u{eb}\u{ee}\u{f1}\u{f5}\u{f8}\u{fb}\u{fe}\
	\u{102}\u{106}\u{109}\u{11a}\u{11d}\u{12c}\u{12f}\u{135}\u{139}\u{13d}\u{140}\
	\u{149}\u{14c}\u{14f}\u{152}\u{16c}\u{16f}\u{172}\u{177}\u{184}\u{187}\u{18c}\
	\u{18f}\u{193}\u{196}\u{199}\u{19c}\u{1a4}\u{1ac}\u{1b2}\u{1b6}\u{1bc}\u{1c2}\
	\u{1c8}\u{1cc}\u{1d2}\u{1d8}\u{1de}\u{1f3}\u{1fb}\u{213}\u{218}\u{223}\u{22d}\
	\u{234}\u{23c}\u{23f}\u{247}\u{24f}\u{255}\u{25b}\u{264}\u{26f}\u{279}\u{27e}\
	\u{282}\u{28f}\u{293}\u{298}\u{2a0}\u{2a8}\u{2b6}\u{2bf}\u{2c3}\u{2cb}\u{2d0}\
	\u{2d8}\u{2e0}\u{2e8}\u{2f9}\u{307}\u{309}\u{312}\u{31c}\u{32f}\u{33a}\u{343}\
	\u{378}\u{37a}\u{38a}\u{392}\u{399}\u{3aa}\u{3b0}\u{3b7}\u{3c5}\u{3d7}\u{3dd}\
	\u{3e7}\u{3ef}\u{3f4}\u{405}\u{408}\u{415}\u{418}\u{41c}\u{426}\u{429}\u{432}\
	\u{43d}\u{447}\u{44f}\u{457}";

