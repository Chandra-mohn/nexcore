// Generated from /Users/chandramohn/workspace/nexcore/grammar/nexflow/RulesDSL.g4 by ANTLR 4.8
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
use super::rulesdsllistener::*;
use super::rulesdslvisitor::*;

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
		pub const IMPORT:isize=5; 
		pub const DECISION_TABLE:isize=6; 
		pub const RULE:isize=7; 
		pub const END:isize=8; 
		pub const GIVEN:isize=9; 
		pub const DECIDE:isize=10; 
		pub const RETURN:isize=11; 
		pub const EXECUTE:isize=12; 
		pub const HIT_POLICY:isize=13; 
		pub const FIRST_MATCH:isize=14; 
		pub const SINGLE_HIT:isize=15; 
		pub const MULTI_HIT:isize=16; 
		pub const COLLECT_ALL:isize=17; 
		pub const IF:isize=18; 
		pub const THEN:isize=19; 
		pub const ELSEIF:isize=20; 
		pub const ELSE:isize=21; 
		pub const ENDIF:isize=22; 
		pub const SET:isize=23; 
		pub const LET:isize=24; 
		pub const WHEN:isize=25; 
		pub const OTHERWISE:isize=26; 
		pub const POST_CALCULATE:isize=27; 
		pub const AGGREGATE:isize=28; 
		pub const AND:isize=29; 
		pub const OR:isize=30; 
		pub const NOT:isize=31; 
		pub const IN:isize=32; 
		pub const IS:isize=33; 
		pub const NULL:isize=34; 
		pub const IS_NULL:isize=35; 
		pub const IS_NOT_NULL:isize=36; 
		pub const MATCHES:isize=37; 
		pub const CONTAINS:isize=38; 
		pub const STARTS_WITH:isize=39; 
		pub const ENDS_WITH:isize=40; 
		pub const LOOKUP:isize=41; 
		pub const EMIT:isize=42; 
		pub const TO:isize=43; 
		pub const DEFAULT:isize=44; 
		pub const AS_OF:isize=45; 
		pub const SERVICES:isize=46; 
		pub const SYNC:isize=47; 
		pub const ASYNC:isize=48; 
		pub const CACHED:isize=49; 
		pub const TIMEOUT:isize=50; 
		pub const FALLBACK:isize=51; 
		pub const RETRY:isize=52; 
		pub const ACTIONS:isize=53; 
		pub const STATE:isize=54; 
		pub const AUDIT:isize=55; 
		pub const CALL:isize=56; 
		pub const ANY:isize=57; 
		pub const ALL:isize=58; 
		pub const NONE:isize=59; 
		pub const SUM:isize=60; 
		pub const COUNT:isize=61; 
		pub const AVG:isize=62; 
		pub const MAX_FN:isize=63; 
		pub const MIN_FN:isize=64; 
		pub const FILTER:isize=65; 
		pub const FIND:isize=66; 
		pub const DISTINCT:isize=67; 
		pub const MS:isize=68; 
		pub const S:isize=69; 
		pub const M:isize=70; 
		pub const H:isize=71; 
		pub const TEXT_TYPE:isize=72; 
		pub const NUMBER_TYPE:isize=73; 
		pub const BOOLEAN_TYPE:isize=74; 
		pub const DATE_TYPE:isize=75; 
		pub const TIMESTAMP_TYPE:isize=76; 
		pub const MONEY_TYPE:isize=77; 
		pub const PERCENTAGE_TYPE:isize=78; 
		pub const BIZDATE_TYPE:isize=79; 
		pub const MARKER:isize=80; 
		pub const FIRED:isize=81; 
		pub const PENDING:isize=82; 
		pub const BETWEEN_MARKERS:isize=83; 
		pub const DESCRIPTION:isize=84; 
		pub const PRIORITY:isize=85; 
		pub const VERSION:isize=86; 
		pub const YES:isize=87; 
		pub const MULTI:isize=88; 
		pub const EQ:isize=89; 
		pub const NE:isize=90; 
		pub const LT:isize=91; 
		pub const GT:isize=92; 
		pub const LE:isize=93; 
		pub const GE:isize=94; 
		pub const PLUS:isize=95; 
		pub const MINUS:isize=96; 
		pub const STAR:isize=97; 
		pub const SLASH:isize=98; 
		pub const PERCENT:isize=99; 
		pub const MOD:isize=100; 
		pub const ARROW:isize=101; 
		pub const PIPE:isize=102; 
		pub const COLON:isize=103; 
		pub const COMMA:isize=104; 
		pub const DOTDOT:isize=105; 
		pub const DOT:isize=106; 
		pub const LPAREN:isize=107; 
		pub const RPAREN:isize=108; 
		pub const LBRACKET:isize=109; 
		pub const RBRACKET:isize=110; 
		pub const LBRACE:isize=111; 
		pub const RBRACE:isize=112; 
		pub const SEP:isize=113; 
		pub const BOOLEAN:isize=114; 
		pub const VERSION_NUMBER:isize=115; 
		pub const INTEGER:isize=116; 
		pub const DECIMAL:isize=117; 
		pub const MONEY_LITERAL:isize=118; 
		pub const PERCENTAGE_LITERAL:isize=119; 
		pub const DQUOTED_STRING:isize=120; 
		pub const SQUOTED_STRING:isize=121; 
		pub const STRING:isize=122; 
		pub const IDENTIFIER:isize=123; 
		pub const LINE_COMMENT:isize=124; 
		pub const LINE_COMMENT_INLINE:isize=125; 
		pub const BLOCK_COMMENT:isize=126; 
		pub const WS:isize=127;
	pub const RULE_program:usize = 0; 
	pub const RULE_importStatement:usize = 1; 
	pub const RULE_importPath:usize = 2; 
	pub const RULE_importPathSegment:usize = 3; 
	pub const RULE_importFileExtension:usize = 4; 
	pub const RULE_servicesBlock:usize = 5; 
	pub const RULE_serviceDecl:usize = 6; 
	pub const RULE_serviceName:usize = 7; 
	pub const RULE_serviceClassName:usize = 8; 
	pub const RULE_serviceMethodName:usize = 9; 
	pub const RULE_serviceType:usize = 10; 
	pub const RULE_serviceParamList:usize = 11; 
	pub const RULE_serviceParam:usize = 12; 
	pub const RULE_serviceReturnType:usize = 13; 
	pub const RULE_serviceOptions:usize = 14; 
	pub const RULE_serviceOption:usize = 15; 
	pub const RULE_duration:usize = 16; 
	pub const RULE_durationUnit:usize = 17; 
	pub const RULE_actionsBlock:usize = 18; 
	pub const RULE_actionDecl:usize = 19; 
	pub const RULE_actionDeclName:usize = 20; 
	pub const RULE_actionParamList:usize = 21; 
	pub const RULE_actionParam:usize = 22; 
	pub const RULE_actionTarget:usize = 23; 
	pub const RULE_emitTarget:usize = 24; 
	pub const RULE_stateTarget:usize = 25; 
	pub const RULE_stateOperation:usize = 26; 
	pub const RULE_stateOperationArg:usize = 27; 
	pub const RULE_auditTarget:usize = 28; 
	pub const RULE_callTarget:usize = 29; 
	pub const RULE_decisionTableDef:usize = 30; 
	pub const RULE_versionDecl:usize = 31; 
	pub const RULE_tableName:usize = 32; 
	pub const RULE_hitPolicyDecl:usize = 33; 
	pub const RULE_hitPolicyType:usize = 34; 
	pub const RULE_descriptionDecl:usize = 35; 
	pub const RULE_stringLiteral:usize = 36; 
	pub const RULE_givenBlock:usize = 37; 
	pub const RULE_inputParam:usize = 38; 
	pub const RULE_paramName:usize = 39; 
	pub const RULE_paramType:usize = 40; 
	pub const RULE_baseType:usize = 41; 
	pub const RULE_inlineComment:usize = 42; 
	pub const RULE_decideBlock:usize = 43; 
	pub const RULE_tableMatrix:usize = 44; 
	pub const RULE_tableHeader:usize = 45; 
	pub const RULE_priorityHeader:usize = 46; 
	pub const RULE_columnHeader:usize = 47; 
	pub const RULE_columnName:usize = 48; 
	pub const RULE_tableSeparator:usize = 49; 
	pub const RULE_tableRow:usize = 50; 
	pub const RULE_priorityCell:usize = 51; 
	pub const RULE_cell:usize = 52; 
	pub const RULE_cellContent:usize = 53; 
	pub const RULE_condition:usize = 54; 
	pub const RULE_exactMatch:usize = 55; 
	pub const RULE_rangeCondition:usize = 56; 
	pub const RULE_setCondition:usize = 57; 
	pub const RULE_patternCondition:usize = 58; 
	pub const RULE_nullCondition:usize = 59; 
	pub const RULE_comparisonCondition:usize = 60; 
	pub const RULE_expressionCondition:usize = 61; 
	pub const RULE_markerStateCondition:usize = 62; 
	pub const RULE_action:usize = 63; 
	pub const RULE_assignAction:usize = 64; 
	pub const RULE_calculateAction:usize = 65; 
	pub const RULE_lookupAction:usize = 66; 
	pub const RULE_callAction:usize = 67; 
	pub const RULE_actionArg:usize = 68; 
	pub const RULE_emitAction:usize = 69; 
	pub const RULE_returnSpec:usize = 70; 
	pub const RULE_returnParam:usize = 71; 
	pub const RULE_executeSpec:usize = 72; 
	pub const RULE_executeType:usize = 73; 
	pub const RULE_hybridSpec:usize = 74; 
	pub const RULE_postCalculateBlock:usize = 75; 
	pub const RULE_postCalculateStatement:usize = 76; 
	pub const RULE_assignmentStatement:usize = 77; 
	pub const RULE_aggregateBlock:usize = 78; 
	pub const RULE_aggregateStatement:usize = 79; 
	pub const RULE_whenExpression:usize = 80; 
	pub const RULE_proceduralRuleDef:usize = 81; 
	pub const RULE_ruleName:usize = 82; 
	pub const RULE_blockItem:usize = 83; 
	pub const RULE_setStatement:usize = 84; 
	pub const RULE_letStatement:usize = 85; 
	pub const RULE_ruleStep:usize = 86; 
	pub const RULE_block:usize = 87; 
	pub const RULE_actionSequence:usize = 88; 
	pub const RULE_actionCall:usize = 89; 
	pub const RULE_parameterList:usize = 90; 
	pub const RULE_parameter:usize = 91; 
	pub const RULE_returnStatement:usize = 92; 
	pub const RULE_booleanExpr:usize = 93; 
	pub const RULE_booleanTerm:usize = 94; 
	pub const RULE_booleanFactor:usize = 95; 
	pub const RULE_comparisonExpr:usize = 96; 
	pub const RULE_comparisonOp:usize = 97; 
	pub const RULE_valueExpr:usize = 98; 
	pub const RULE_term:usize = 99; 
	pub const RULE_factor:usize = 100; 
	pub const RULE_atom:usize = 101; 
	pub const RULE_collectionExpr:usize = 102; 
	pub const RULE_predicateFunction:usize = 103; 
	pub const RULE_aggregateFunction:usize = 104; 
	pub const RULE_transformFunction:usize = 105; 
	pub const RULE_collectionPredicate:usize = 106; 
	pub const RULE_collectionPredicateOr:usize = 107; 
	pub const RULE_collectionPredicateAnd:usize = 108; 
	pub const RULE_collectionPredicateAtom:usize = 109; 
	pub const RULE_lambdaExpression:usize = 110; 
	pub const RULE_arithmeticExpr:usize = 111; 
	pub const RULE_functionCall:usize = 112; 
	pub const RULE_fieldPath:usize = 113; 
	pub const RULE_attributeIdentifier:usize = 114; 
	pub const RULE_valueList:usize = 115; 
	pub const RULE_listLiteral:usize = 116; 
	pub const RULE_objectLiteral:usize = 117; 
	pub const RULE_objectField:usize = 118; 
	pub const RULE_objectFieldName:usize = 119; 
	pub const RULE_literal:usize = 120; 
	pub const RULE_numberLiteral:usize = 121;
	pub const ruleNames: [&'static str; 122] =  [
		"program", "importStatement", "importPath", "importPathSegment", "importFileExtension", 
		"servicesBlock", "serviceDecl", "serviceName", "serviceClassName", "serviceMethodName", 
		"serviceType", "serviceParamList", "serviceParam", "serviceReturnType", 
		"serviceOptions", "serviceOption", "duration", "durationUnit", "actionsBlock", 
		"actionDecl", "actionDeclName", "actionParamList", "actionParam", "actionTarget", 
		"emitTarget", "stateTarget", "stateOperation", "stateOperationArg", "auditTarget", 
		"callTarget", "decisionTableDef", "versionDecl", "tableName", "hitPolicyDecl", 
		"hitPolicyType", "descriptionDecl", "stringLiteral", "givenBlock", "inputParam", 
		"paramName", "paramType", "baseType", "inlineComment", "decideBlock", 
		"tableMatrix", "tableHeader", "priorityHeader", "columnHeader", "columnName", 
		"tableSeparator", "tableRow", "priorityCell", "cell", "cellContent", "condition", 
		"exactMatch", "rangeCondition", "setCondition", "patternCondition", "nullCondition", 
		"comparisonCondition", "expressionCondition", "markerStateCondition", 
		"action", "assignAction", "calculateAction", "lookupAction", "callAction", 
		"actionArg", "emitAction", "returnSpec", "returnParam", "executeSpec", 
		"executeType", "hybridSpec", "postCalculateBlock", "postCalculateStatement", 
		"assignmentStatement", "aggregateBlock", "aggregateStatement", "whenExpression", 
		"proceduralRuleDef", "ruleName", "blockItem", "setStatement", "letStatement", 
		"ruleStep", "block", "actionSequence", "actionCall", "parameterList", 
		"parameter", "returnStatement", "booleanExpr", "booleanTerm", "booleanFactor", 
		"comparisonExpr", "comparisonOp", "valueExpr", "term", "factor", "atom", 
		"collectionExpr", "predicateFunction", "aggregateFunction", "transformFunction", 
		"collectionPredicate", "collectionPredicateOr", "collectionPredicateAnd", 
		"collectionPredicateAtom", "lambdaExpression", "arithmeticExpr", "functionCall", 
		"fieldPath", "attributeIdentifier", "valueList", "listLiteral", "objectLiteral", 
		"objectField", "objectFieldName", "literal", "numberLiteral"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;113] = [
		None, Some("'schema'"), Some("'transform'"), Some("'flow'"), Some("'rules'"), 
		Some("'import'"), Some("'decision_table'"), Some("'rule'"), Some("'end'"), 
		Some("'given'"), Some("'decide'"), Some("'return'"), Some("'execute'"), 
		Some("'hit_policy'"), Some("'first_match'"), Some("'single_hit'"), Some("'multi_hit'"), 
		Some("'collect_all'"), Some("'if'"), Some("'then'"), Some("'elseif'"), 
		Some("'else'"), Some("'endif'"), Some("'set'"), Some("'let'"), Some("'when'"), 
		Some("'otherwise'"), Some("'post_calculate'"), Some("'aggregate'"), Some("'and'"), 
		Some("'or'"), Some("'not'"), Some("'in'"), Some("'is'"), Some("'null'"), 
		Some("'is_null'"), Some("'is_not_null'"), Some("'matches'"), Some("'contains'"), 
		Some("'starts_with'"), Some("'ends_with'"), Some("'lookup'"), Some("'emit'"), 
		Some("'to'"), Some("'default'"), Some("'as_of'"), Some("'services'"), 
		Some("'sync'"), Some("'async'"), Some("'cached'"), Some("'timeout'"), 
		Some("'fallback'"), Some("'retry'"), Some("'actions'"), Some("'state'"), 
		Some("'audit'"), Some("'call'"), Some("'any'"), Some("'all'"), Some("'none'"), 
		Some("'sum'"), Some("'count'"), Some("'avg'"), Some("'max'"), Some("'min'"), 
		Some("'filter'"), Some("'find'"), Some("'distinct'"), Some("'ms'"), Some("'s'"), 
		Some("'m'"), Some("'h'"), Some("'text'"), Some("'number'"), Some("'boolean'"), 
		Some("'date'"), Some("'timestamp'"), Some("'money'"), Some("'percentage'"), 
		Some("'bizdate'"), Some("'marker'"), Some("'fired'"), Some("'pending'"), 
		Some("'between'"), Some("'description'"), Some("'priority'"), Some("'version'"), 
		Some("'yes'"), Some("'multi'"), None, Some("'!='"), Some("'<'"), Some("'>'"), 
		Some("'<='"), Some("'>='"), Some("'+'"), Some("'-'"), Some("'*'"), Some("'/'"), 
		Some("'%'"), Some("'mod'"), Some("'->'"), Some("'|'"), Some("':'"), Some("','"), 
		Some("'..'"), Some("'.'"), Some("'('"), Some("')'"), Some("'['"), Some("']'"), 
		Some("'{'"), Some("'}'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;128]  = [
		None, None, None, None, None, Some("IMPORT"), Some("DECISION_TABLE"), 
		Some("RULE"), Some("END"), Some("GIVEN"), Some("DECIDE"), Some("RETURN"), 
		Some("EXECUTE"), Some("HIT_POLICY"), Some("FIRST_MATCH"), Some("SINGLE_HIT"), 
		Some("MULTI_HIT"), Some("COLLECT_ALL"), Some("IF"), Some("THEN"), Some("ELSEIF"), 
		Some("ELSE"), Some("ENDIF"), Some("SET"), Some("LET"), Some("WHEN"), Some("OTHERWISE"), 
		Some("POST_CALCULATE"), Some("AGGREGATE"), Some("AND"), Some("OR"), Some("NOT"), 
		Some("IN"), Some("IS"), Some("NULL"), Some("IS_NULL"), Some("IS_NOT_NULL"), 
		Some("MATCHES"), Some("CONTAINS"), Some("STARTS_WITH"), Some("ENDS_WITH"), 
		Some("LOOKUP"), Some("EMIT"), Some("TO"), Some("DEFAULT"), Some("AS_OF"), 
		Some("SERVICES"), Some("SYNC"), Some("ASYNC"), Some("CACHED"), Some("TIMEOUT"), 
		Some("FALLBACK"), Some("RETRY"), Some("ACTIONS"), Some("STATE"), Some("AUDIT"), 
		Some("CALL"), Some("ANY"), Some("ALL"), Some("NONE"), Some("SUM"), Some("COUNT"), 
		Some("AVG"), Some("MAX_FN"), Some("MIN_FN"), Some("FILTER"), Some("FIND"), 
		Some("DISTINCT"), Some("MS"), Some("S"), Some("M"), Some("H"), Some("TEXT_TYPE"), 
		Some("NUMBER_TYPE"), Some("BOOLEAN_TYPE"), Some("DATE_TYPE"), Some("TIMESTAMP_TYPE"), 
		Some("MONEY_TYPE"), Some("PERCENTAGE_TYPE"), Some("BIZDATE_TYPE"), Some("MARKER"), 
		Some("FIRED"), Some("PENDING"), Some("BETWEEN_MARKERS"), Some("DESCRIPTION"), 
		Some("PRIORITY"), Some("VERSION"), Some("YES"), Some("MULTI"), Some("EQ"), 
		Some("NE"), Some("LT"), Some("GT"), Some("LE"), Some("GE"), Some("PLUS"), 
		Some("MINUS"), Some("STAR"), Some("SLASH"), Some("PERCENT"), Some("MOD"), 
		Some("ARROW"), Some("PIPE"), Some("COLON"), Some("COMMA"), Some("DOTDOT"), 
		Some("DOT"), Some("LPAREN"), Some("RPAREN"), Some("LBRACKET"), Some("RBRACKET"), 
		Some("LBRACE"), Some("RBRACE"), Some("SEP"), Some("BOOLEAN"), Some("VERSION_NUMBER"), 
		Some("INTEGER"), Some("DECIMAL"), Some("MONEY_LITERAL"), Some("PERCENTAGE_LITERAL"), 
		Some("DQUOTED_STRING"), Some("SQUOTED_STRING"), Some("STRING"), Some("IDENTIFIER"), 
		Some("LINE_COMMENT"), Some("LINE_COMMENT_INLINE"), Some("BLOCK_COMMENT"), 
		Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,RulesDSLParserExt<'input>, I, RulesDSLParserContextType , dyn RulesDSLListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type RulesDSLTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, RulesDSLParserContextType , dyn RulesDSLListener<'input> + 'a>;

/// Parser for RulesDSL grammar
pub struct RulesDSLParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> RulesDSLParser<'input, I, H>
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
				RulesDSLParserExt{
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

impl<'input, I> RulesDSLParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> RulesDSLParser<'input, I, DefaultErrorStrategy<'input,RulesDSLParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for RulesDSLParser
pub trait RulesDSLParserContext<'input>:
	for<'x> Listenable<dyn RulesDSLListener<'input> + 'x > + 
	for<'x> Visitable<dyn RulesDSLVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=RulesDSLParserContextType>
{}

antlr_rust::coerce_from!{ 'input : RulesDSLParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn RulesDSLParserContext<'input> + 'input
where
    T: RulesDSLVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn RulesDSLVisitor<'input> + 'x))
    }
}

impl<'input> RulesDSLParserContext<'input> for TerminalNode<'input,RulesDSLParserContextType> {}
impl<'input> RulesDSLParserContext<'input> for ErrorNode<'input,RulesDSLParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn RulesDSLParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn RulesDSLListener<'input> + 'input }

pub struct RulesDSLParserContextType;
antlr_rust::tid!{RulesDSLParserContextType}

impl<'input> ParserNodeType<'input> for RulesDSLParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn RulesDSLParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct RulesDSLParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> RulesDSLParserExt<'input>{
}
antlr_rust::tid! { RulesDSLParserExt<'a> }

impl<'input> TokenAware<'input> for RulesDSLParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for RulesDSLParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for RulesDSLParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "RulesDSL.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- program ----------------
pub type ProgramContextAll<'input> = ProgramContext<'input>;


pub type ProgramContext<'input> = BaseParserRuleContext<'input,ProgramContextExt<'input>>;

#[derive(Clone)]
pub struct ProgramContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ProgramContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ProgramContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_program(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_program(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ProgramContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_program(self);
	}
}

impl<'input> CustomRuleContext<'input> for ProgramContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_program }
	//fn type_rule_index() -> usize where Self: Sized { RULE_program }
}
antlr_rust::tid!{ProgramContextExt<'a>}

impl<'input> ProgramContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProgramContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProgramContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ProgramContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ProgramContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn importStatement_all(&self) ->  Vec<Rc<ImportStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn importStatement(&self, i: usize) -> Option<Rc<ImportStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn servicesBlock(&self) -> Option<Rc<ServicesBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn actionsBlock(&self) -> Option<Rc<ActionsBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn decisionTableDef_all(&self) ->  Vec<Rc<DecisionTableDefContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn decisionTableDef(&self, i: usize) -> Option<Rc<DecisionTableDefContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn proceduralRuleDef_all(&self) ->  Vec<Rc<ProceduralRuleDefContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn proceduralRuleDef(&self, i: usize) -> Option<Rc<ProceduralRuleDefContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ProgramContextAttrs<'input> for ProgramContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
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
			recog.base.set_state(247);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==IMPORT {
				{
				{
				/*InvokeRule importStatement*/
				recog.base.set_state(244);
				recog.importStatement()?;

				}
				}
				recog.base.set_state(249);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(251);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==SERVICES {
				{
				/*InvokeRule servicesBlock*/
				recog.base.set_state(250);
				recog.servicesBlock()?;

				}
			}

			recog.base.set_state(254);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ACTIONS {
				{
				/*InvokeRule actionsBlock*/
				recog.base.set_state(253);
				recog.actionsBlock()?;

				}
			}

			recog.base.set_state(258); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				recog.base.set_state(258);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.base.input.la(1) {
				 DECISION_TABLE 
					=> {
						{
						/*InvokeRule decisionTableDef*/
						recog.base.set_state(256);
						recog.decisionTableDef()?;

						}
					}

				 RULE 
					=> {
						{
						/*InvokeRule proceduralRuleDef*/
						recog.base.set_state(257);
						recog.proceduralRuleDef()?;

						}
					}

					_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				}
				recog.base.set_state(260); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==DECISION_TABLE || _la==RULE) {break}
			}
			recog.base.set_state(262);
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

impl<'input> RulesDSLParserContext<'input> for ImportStatementContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ImportStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_importStatement(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_importStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ImportStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_importStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importStatement }
}
antlr_rust::tid!{ImportStatementContextExt<'a>}

impl<'input> ImportStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportStatementContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ImportStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IMPORT
/// Returns `None` if there is no child corresponding to token IMPORT
fn IMPORT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IMPORT, 0)
}
fn importPath(&self) -> Option<Rc<ImportPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ImportStatementContextAttrs<'input> for ImportStatementContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
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
			recog.base.set_state(264);
			recog.base.match_token(IMPORT,&mut recog.err_handler)?;

			/*InvokeRule importPath*/
			recog.base.set_state(265);
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

impl<'input> RulesDSLParserContext<'input> for ImportPathContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ImportPathContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_importPath(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_importPath(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ImportPathContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_importPath(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportPathContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importPath }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importPath }
}
antlr_rust::tid!{ImportPathContextExt<'a>}

impl<'input> ImportPathContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportPathContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportPathContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportPathContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ImportPathContextExt<'input>>{

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

impl<'input, I, H> RulesDSLParser<'input, I, H>
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
			recog.base.set_state(268); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					{
					/*InvokeRule importPathSegment*/
					recog.base.set_state(267);
					recog.importPathSegment()?;

					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(270); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(5,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
			}
			/*InvokeRule importFileExtension*/
			recog.base.set_state(272);
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

impl<'input> RulesDSLParserContext<'input> for ImportPathSegmentContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ImportPathSegmentContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_importPathSegment(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_importPathSegment(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ImportPathSegmentContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_importPathSegment(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportPathSegmentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importPathSegment }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importPathSegment }
}
antlr_rust::tid!{ImportPathSegmentContextExt<'a>}

impl<'input> ImportPathSegmentContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportPathSegmentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportPathSegmentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportPathSegmentContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ImportPathSegmentContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DOTDOT
/// Returns `None` if there is no child corresponding to token DOTDOT
fn DOTDOT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DOTDOT, 0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}
/// Retrieves first TerminalNode corresponding to token SLASH
/// Returns `None` if there is no child corresponding to token SLASH
fn SLASH(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(SLASH, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}

}

impl<'input> ImportPathSegmentContextAttrs<'input> for ImportPathSegmentContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
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
			recog.base.set_state(274);
			_la = recog.base.input.la(1);
			if { !(((((_la - 96)) & !0x3f) == 0 && ((1usize << (_la - 96)) & ((1usize << (MINUS - 96)) | (1usize << (SLASH - 96)) | (1usize << (DOTDOT - 96)) | (1usize << (DOT - 96)) | (1usize << (IDENTIFIER - 96)))) != 0)) } {
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

impl<'input> RulesDSLParserContext<'input> for ImportFileExtensionContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ImportFileExtensionContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_importFileExtension(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_importFileExtension(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ImportFileExtensionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_importFileExtension(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportFileExtensionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importFileExtension }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importFileExtension }
}
antlr_rust::tid!{ImportFileExtensionContextExt<'a>}

impl<'input> ImportFileExtensionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportFileExtensionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportFileExtensionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportFileExtensionContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ImportFileExtensionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}

}

impl<'input> ImportFileExtensionContextAttrs<'input> for ImportFileExtensionContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
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
			recog.base.set_state(276);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			recog.base.set_state(277);
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
//------------------- servicesBlock ----------------
pub type ServicesBlockContextAll<'input> = ServicesBlockContext<'input>;


pub type ServicesBlockContext<'input> = BaseParserRuleContext<'input,ServicesBlockContextExt<'input>>;

#[derive(Clone)]
pub struct ServicesBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ServicesBlockContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ServicesBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_servicesBlock(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_servicesBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ServicesBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_servicesBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for ServicesBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_servicesBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_servicesBlock }
}
antlr_rust::tid!{ServicesBlockContextExt<'a>}

impl<'input> ServicesBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ServicesBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ServicesBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ServicesBlockContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ServicesBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SERVICES
/// Returns `None` if there is no child corresponding to token SERVICES
fn SERVICES(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(SERVICES, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn serviceDecl_all(&self) ->  Vec<Rc<ServiceDeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn serviceDecl(&self, i: usize) -> Option<Rc<ServiceDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ServicesBlockContextAttrs<'input> for ServicesBlockContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn servicesBlock(&mut self,)
	-> Result<Rc<ServicesBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ServicesBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_servicesBlock);
        let mut _localctx: Rc<ServicesBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(279);
			recog.base.match_token(SERVICES,&mut recog.err_handler)?;

			recog.base.set_state(280);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(282); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule serviceDecl*/
				recog.base.set_state(281);
				recog.serviceDecl()?;

				}
				}
				recog.base.set_state(284); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==IDENTIFIER) {break}
			}
			recog.base.set_state(286);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

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
//------------------- serviceDecl ----------------
pub type ServiceDeclContextAll<'input> = ServiceDeclContext<'input>;


pub type ServiceDeclContext<'input> = BaseParserRuleContext<'input,ServiceDeclContextExt<'input>>;

#[derive(Clone)]
pub struct ServiceDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ServiceDeclContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ServiceDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_serviceDecl(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_serviceDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ServiceDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_serviceDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for ServiceDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_serviceDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_serviceDecl }
}
antlr_rust::tid!{ServiceDeclContextExt<'a>}

impl<'input> ServiceDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ServiceDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ServiceDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ServiceDeclContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ServiceDeclContextExt<'input>>{

fn serviceName(&self) -> Option<Rc<ServiceNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn serviceType(&self) -> Option<Rc<ServiceTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn serviceClassName(&self) -> Option<Rc<ServiceClassNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}
fn serviceMethodName(&self) -> Option<Rc<ServiceMethodNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token ARROW
/// Returns `None` if there is no child corresponding to token ARROW
fn ARROW(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(ARROW, 0)
}
fn serviceReturnType(&self) -> Option<Rc<ServiceReturnTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn serviceParamList(&self) -> Option<Rc<ServiceParamListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn serviceOptions(&self) -> Option<Rc<ServiceOptionsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ServiceDeclContextAttrs<'input> for ServiceDeclContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn serviceDecl(&mut self,)
	-> Result<Rc<ServiceDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ServiceDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_serviceDecl);
        let mut _localctx: Rc<ServiceDeclContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule serviceName*/
			recog.base.set_state(288);
			recog.serviceName()?;

			recog.base.set_state(289);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule serviceType*/
			recog.base.set_state(290);
			recog.serviceType()?;

			/*InvokeRule serviceClassName*/
			recog.base.set_state(291);
			recog.serviceClassName()?;

			recog.base.set_state(292);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			/*InvokeRule serviceMethodName*/
			recog.base.set_state(293);
			recog.serviceMethodName()?;

			recog.base.set_state(294);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			recog.base.set_state(296);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==IDENTIFIER {
				{
				/*InvokeRule serviceParamList*/
				recog.base.set_state(295);
				recog.serviceParamList()?;

				}
			}

			recog.base.set_state(298);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			recog.base.set_state(299);
			recog.base.match_token(ARROW,&mut recog.err_handler)?;

			/*InvokeRule serviceReturnType*/
			recog.base.set_state(300);
			recog.serviceReturnType()?;

			recog.base.set_state(302);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 50)) & !0x3f) == 0 && ((1usize << (_la - 50)) & ((1usize << (TIMEOUT - 50)) | (1usize << (FALLBACK - 50)) | (1usize << (RETRY - 50)))) != 0) {
				{
				/*InvokeRule serviceOptions*/
				recog.base.set_state(301);
				recog.serviceOptions()?;

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
//------------------- serviceName ----------------
pub type ServiceNameContextAll<'input> = ServiceNameContext<'input>;


pub type ServiceNameContext<'input> = BaseParserRuleContext<'input,ServiceNameContextExt<'input>>;

#[derive(Clone)]
pub struct ServiceNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ServiceNameContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ServiceNameContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_serviceName(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_serviceName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ServiceNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_serviceName(self);
	}
}

impl<'input> CustomRuleContext<'input> for ServiceNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_serviceName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_serviceName }
}
antlr_rust::tid!{ServiceNameContextExt<'a>}

impl<'input> ServiceNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ServiceNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ServiceNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ServiceNameContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ServiceNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> ServiceNameContextAttrs<'input> for ServiceNameContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn serviceName(&mut self,)
	-> Result<Rc<ServiceNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ServiceNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_serviceName);
        let mut _localctx: Rc<ServiceNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(304);
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
//------------------- serviceClassName ----------------
pub type ServiceClassNameContextAll<'input> = ServiceClassNameContext<'input>;


pub type ServiceClassNameContext<'input> = BaseParserRuleContext<'input,ServiceClassNameContextExt<'input>>;

#[derive(Clone)]
pub struct ServiceClassNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ServiceClassNameContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ServiceClassNameContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_serviceClassName(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_serviceClassName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ServiceClassNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_serviceClassName(self);
	}
}

impl<'input> CustomRuleContext<'input> for ServiceClassNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_serviceClassName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_serviceClassName }
}
antlr_rust::tid!{ServiceClassNameContextExt<'a>}

impl<'input> ServiceClassNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ServiceClassNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ServiceClassNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ServiceClassNameContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ServiceClassNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> ServiceClassNameContextAttrs<'input> for ServiceClassNameContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn serviceClassName(&mut self,)
	-> Result<Rc<ServiceClassNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ServiceClassNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_serviceClassName);
        let mut _localctx: Rc<ServiceClassNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
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
//------------------- serviceMethodName ----------------
pub type ServiceMethodNameContextAll<'input> = ServiceMethodNameContext<'input>;


pub type ServiceMethodNameContext<'input> = BaseParserRuleContext<'input,ServiceMethodNameContextExt<'input>>;

#[derive(Clone)]
pub struct ServiceMethodNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ServiceMethodNameContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ServiceMethodNameContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_serviceMethodName(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_serviceMethodName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ServiceMethodNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_serviceMethodName(self);
	}
}

impl<'input> CustomRuleContext<'input> for ServiceMethodNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_serviceMethodName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_serviceMethodName }
}
antlr_rust::tid!{ServiceMethodNameContextExt<'a>}

impl<'input> ServiceMethodNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ServiceMethodNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ServiceMethodNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ServiceMethodNameContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ServiceMethodNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token LOOKUP
/// Returns `None` if there is no child corresponding to token LOOKUP
fn LOOKUP(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LOOKUP, 0)
}
/// Retrieves first TerminalNode corresponding to token EMIT
/// Returns `None` if there is no child corresponding to token EMIT
fn EMIT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(EMIT, 0)
}
/// Retrieves first TerminalNode corresponding to token MATCHES
/// Returns `None` if there is no child corresponding to token MATCHES
fn MATCHES(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(MATCHES, 0)
}
/// Retrieves first TerminalNode corresponding to token CONTAINS
/// Returns `None` if there is no child corresponding to token CONTAINS
fn CONTAINS(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(CONTAINS, 0)
}

}

impl<'input> ServiceMethodNameContextAttrs<'input> for ServiceMethodNameContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn serviceMethodName(&mut self,)
	-> Result<Rc<ServiceMethodNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ServiceMethodNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_serviceMethodName);
        let mut _localctx: Rc<ServiceMethodNameContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(308);
			_la = recog.base.input.la(1);
			if { !(((((_la - 37)) & !0x3f) == 0 && ((1usize << (_la - 37)) & ((1usize << (MATCHES - 37)) | (1usize << (CONTAINS - 37)) | (1usize << (LOOKUP - 37)) | (1usize << (EMIT - 37)))) != 0) || _la==IDENTIFIER) } {
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
//------------------- serviceType ----------------
pub type ServiceTypeContextAll<'input> = ServiceTypeContext<'input>;


pub type ServiceTypeContext<'input> = BaseParserRuleContext<'input,ServiceTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ServiceTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ServiceTypeContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ServiceTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_serviceType(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_serviceType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ServiceTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_serviceType(self);
	}
}

impl<'input> CustomRuleContext<'input> for ServiceTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_serviceType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_serviceType }
}
antlr_rust::tid!{ServiceTypeContextExt<'a>}

impl<'input> ServiceTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ServiceTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ServiceTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ServiceTypeContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ServiceTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SYNC
/// Returns `None` if there is no child corresponding to token SYNC
fn SYNC(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(SYNC, 0)
}
/// Retrieves first TerminalNode corresponding to token ASYNC
/// Returns `None` if there is no child corresponding to token ASYNC
fn ASYNC(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(ASYNC, 0)
}
/// Retrieves first TerminalNode corresponding to token CACHED
/// Returns `None` if there is no child corresponding to token CACHED
fn CACHED(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(CACHED, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn duration(&self) -> Option<Rc<DurationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> ServiceTypeContextAttrs<'input> for ServiceTypeContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn serviceType(&mut self,)
	-> Result<Rc<ServiceTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ServiceTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_serviceType);
        let mut _localctx: Rc<ServiceTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(317);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 SYNC 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(310);
					recog.base.match_token(SYNC,&mut recog.err_handler)?;

					}
				}

			 ASYNC 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(311);
					recog.base.match_token(ASYNC,&mut recog.err_handler)?;

					}
				}

			 CACHED 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(312);
					recog.base.match_token(CACHED,&mut recog.err_handler)?;

					recog.base.set_state(313);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule duration*/
					recog.base.set_state(314);
					recog.duration()?;

					recog.base.set_state(315);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

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
//------------------- serviceParamList ----------------
pub type ServiceParamListContextAll<'input> = ServiceParamListContext<'input>;


pub type ServiceParamListContext<'input> = BaseParserRuleContext<'input,ServiceParamListContextExt<'input>>;

#[derive(Clone)]
pub struct ServiceParamListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ServiceParamListContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ServiceParamListContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_serviceParamList(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_serviceParamList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ServiceParamListContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_serviceParamList(self);
	}
}

impl<'input> CustomRuleContext<'input> for ServiceParamListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_serviceParamList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_serviceParamList }
}
antlr_rust::tid!{ServiceParamListContextExt<'a>}

impl<'input> ServiceParamListContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ServiceParamListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ServiceParamListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ServiceParamListContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ServiceParamListContextExt<'input>>{

fn serviceParam_all(&self) ->  Vec<Rc<ServiceParamContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn serviceParam(&self, i: usize) -> Option<Rc<ServiceParamContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ServiceParamListContextAttrs<'input> for ServiceParamListContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn serviceParamList(&mut self,)
	-> Result<Rc<ServiceParamListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ServiceParamListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_serviceParamList);
        let mut _localctx: Rc<ServiceParamListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule serviceParam*/
			recog.base.set_state(319);
			recog.serviceParam()?;

			recog.base.set_state(324);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(320);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule serviceParam*/
				recog.base.set_state(321);
				recog.serviceParam()?;

				}
				}
				recog.base.set_state(326);
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
//------------------- serviceParam ----------------
pub type ServiceParamContextAll<'input> = ServiceParamContext<'input>;


pub type ServiceParamContext<'input> = BaseParserRuleContext<'input,ServiceParamContextExt<'input>>;

#[derive(Clone)]
pub struct ServiceParamContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ServiceParamContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ServiceParamContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_serviceParam(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_serviceParam(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ServiceParamContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_serviceParam(self);
	}
}

impl<'input> CustomRuleContext<'input> for ServiceParamContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_serviceParam }
	//fn type_rule_index() -> usize where Self: Sized { RULE_serviceParam }
}
antlr_rust::tid!{ServiceParamContextExt<'a>}

impl<'input> ServiceParamContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ServiceParamContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ServiceParamContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ServiceParamContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ServiceParamContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn paramType(&self) -> Option<Rc<ParamTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ServiceParamContextAttrs<'input> for ServiceParamContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn serviceParam(&mut self,)
	-> Result<Rc<ServiceParamContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ServiceParamContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_serviceParam);
        let mut _localctx: Rc<ServiceParamContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(327);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(328);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule paramType*/
			recog.base.set_state(329);
			recog.paramType()?;

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
//------------------- serviceReturnType ----------------
pub type ServiceReturnTypeContextAll<'input> = ServiceReturnTypeContext<'input>;


pub type ServiceReturnTypeContext<'input> = BaseParserRuleContext<'input,ServiceReturnTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ServiceReturnTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ServiceReturnTypeContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ServiceReturnTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_serviceReturnType(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_serviceReturnType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ServiceReturnTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_serviceReturnType(self);
	}
}

impl<'input> CustomRuleContext<'input> for ServiceReturnTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_serviceReturnType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_serviceReturnType }
}
antlr_rust::tid!{ServiceReturnTypeContextExt<'a>}

impl<'input> ServiceReturnTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ServiceReturnTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ServiceReturnTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ServiceReturnTypeContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ServiceReturnTypeContextExt<'input>>{

fn paramType(&self) -> Option<Rc<ParamTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ServiceReturnTypeContextAttrs<'input> for ServiceReturnTypeContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn serviceReturnType(&mut self,)
	-> Result<Rc<ServiceReturnTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ServiceReturnTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_serviceReturnType);
        let mut _localctx: Rc<ServiceReturnTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule paramType*/
			recog.base.set_state(331);
			recog.paramType()?;

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
//------------------- serviceOptions ----------------
pub type ServiceOptionsContextAll<'input> = ServiceOptionsContext<'input>;


pub type ServiceOptionsContext<'input> = BaseParserRuleContext<'input,ServiceOptionsContextExt<'input>>;

#[derive(Clone)]
pub struct ServiceOptionsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ServiceOptionsContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ServiceOptionsContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_serviceOptions(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_serviceOptions(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ServiceOptionsContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_serviceOptions(self);
	}
}

impl<'input> CustomRuleContext<'input> for ServiceOptionsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_serviceOptions }
	//fn type_rule_index() -> usize where Self: Sized { RULE_serviceOptions }
}
antlr_rust::tid!{ServiceOptionsContextExt<'a>}

impl<'input> ServiceOptionsContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ServiceOptionsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ServiceOptionsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ServiceOptionsContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ServiceOptionsContextExt<'input>>{

fn serviceOption_all(&self) ->  Vec<Rc<ServiceOptionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn serviceOption(&self, i: usize) -> Option<Rc<ServiceOptionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ServiceOptionsContextAttrs<'input> for ServiceOptionsContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn serviceOptions(&mut self,)
	-> Result<Rc<ServiceOptionsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ServiceOptionsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_serviceOptions);
        let mut _localctx: Rc<ServiceOptionsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(334); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule serviceOption*/
				recog.base.set_state(333);
				recog.serviceOption()?;

				}
				}
				recog.base.set_state(336); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(((((_la - 50)) & !0x3f) == 0 && ((1usize << (_la - 50)) & ((1usize << (TIMEOUT - 50)) | (1usize << (FALLBACK - 50)) | (1usize << (RETRY - 50)))) != 0)) {break}
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
//------------------- serviceOption ----------------
pub type ServiceOptionContextAll<'input> = ServiceOptionContext<'input>;


pub type ServiceOptionContext<'input> = BaseParserRuleContext<'input,ServiceOptionContextExt<'input>>;

#[derive(Clone)]
pub struct ServiceOptionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ServiceOptionContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ServiceOptionContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_serviceOption(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_serviceOption(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ServiceOptionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_serviceOption(self);
	}
}

impl<'input> CustomRuleContext<'input> for ServiceOptionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_serviceOption }
	//fn type_rule_index() -> usize where Self: Sized { RULE_serviceOption }
}
antlr_rust::tid!{ServiceOptionContextExt<'a>}

impl<'input> ServiceOptionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ServiceOptionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ServiceOptionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ServiceOptionContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ServiceOptionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TIMEOUT
/// Returns `None` if there is no child corresponding to token TIMEOUT
fn TIMEOUT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(TIMEOUT, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn duration(&self) -> Option<Rc<DurationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token FALLBACK
/// Returns `None` if there is no child corresponding to token FALLBACK
fn FALLBACK(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(FALLBACK, 0)
}
fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RETRY
/// Returns `None` if there is no child corresponding to token RETRY
fn RETRY(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RETRY, 0)
}
/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}

}

impl<'input> ServiceOptionContextAttrs<'input> for ServiceOptionContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn serviceOption(&mut self,)
	-> Result<Rc<ServiceOptionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ServiceOptionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_serviceOption);
        let mut _localctx: Rc<ServiceOptionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(347);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 TIMEOUT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(338);
					recog.base.match_token(TIMEOUT,&mut recog.err_handler)?;

					recog.base.set_state(339);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule duration*/
					recog.base.set_state(340);
					recog.duration()?;

					}
				}

			 FALLBACK 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(341);
					recog.base.match_token(FALLBACK,&mut recog.err_handler)?;

					recog.base.set_state(342);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule literal*/
					recog.base.set_state(343);
					recog.literal()?;

					}
				}

			 RETRY 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(344);
					recog.base.match_token(RETRY,&mut recog.err_handler)?;

					recog.base.set_state(345);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(346);
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
//------------------- duration ----------------
pub type DurationContextAll<'input> = DurationContext<'input>;


pub type DurationContext<'input> = BaseParserRuleContext<'input,DurationContextExt<'input>>;

#[derive(Clone)]
pub struct DurationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for DurationContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for DurationContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_duration(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_duration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for DurationContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_duration(self);
	}
}

impl<'input> CustomRuleContext<'input> for DurationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_duration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_duration }
}
antlr_rust::tid!{DurationContextExt<'a>}

impl<'input> DurationContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DurationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DurationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DurationContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<DurationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
fn durationUnit(&self) -> Option<Rc<DurationUnitContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DurationContextAttrs<'input> for DurationContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn duration(&mut self,)
	-> Result<Rc<DurationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DurationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_duration);
        let mut _localctx: Rc<DurationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(349);
			recog.base.match_token(INTEGER,&mut recog.err_handler)?;

			/*InvokeRule durationUnit*/
			recog.base.set_state(350);
			recog.durationUnit()?;

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
//------------------- durationUnit ----------------
pub type DurationUnitContextAll<'input> = DurationUnitContext<'input>;


pub type DurationUnitContext<'input> = BaseParserRuleContext<'input,DurationUnitContextExt<'input>>;

#[derive(Clone)]
pub struct DurationUnitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for DurationUnitContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for DurationUnitContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_durationUnit(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_durationUnit(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for DurationUnitContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_durationUnit(self);
	}
}

impl<'input> CustomRuleContext<'input> for DurationUnitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_durationUnit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_durationUnit }
}
antlr_rust::tid!{DurationUnitContextExt<'a>}

impl<'input> DurationUnitContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DurationUnitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DurationUnitContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DurationUnitContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<DurationUnitContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token MS
/// Returns `None` if there is no child corresponding to token MS
fn MS(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(MS, 0)
}
/// Retrieves first TerminalNode corresponding to token S
/// Returns `None` if there is no child corresponding to token S
fn S(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(S, 0)
}
/// Retrieves first TerminalNode corresponding to token M
/// Returns `None` if there is no child corresponding to token M
fn M(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(M, 0)
}
/// Retrieves first TerminalNode corresponding to token H
/// Returns `None` if there is no child corresponding to token H
fn H(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(H, 0)
}

}

impl<'input> DurationUnitContextAttrs<'input> for DurationUnitContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn durationUnit(&mut self,)
	-> Result<Rc<DurationUnitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DurationUnitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_durationUnit);
        let mut _localctx: Rc<DurationUnitContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(352);
			_la = recog.base.input.la(1);
			if { !(((((_la - 68)) & !0x3f) == 0 && ((1usize << (_la - 68)) & ((1usize << (MS - 68)) | (1usize << (S - 68)) | (1usize << (M - 68)) | (1usize << (H - 68)))) != 0)) } {
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
//------------------- actionsBlock ----------------
pub type ActionsBlockContextAll<'input> = ActionsBlockContext<'input>;


pub type ActionsBlockContext<'input> = BaseParserRuleContext<'input,ActionsBlockContextExt<'input>>;

#[derive(Clone)]
pub struct ActionsBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ActionsBlockContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ActionsBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_actionsBlock(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_actionsBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ActionsBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_actionsBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for ActionsBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_actionsBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_actionsBlock }
}
antlr_rust::tid!{ActionsBlockContextExt<'a>}

impl<'input> ActionsBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ActionsBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ActionsBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ActionsBlockContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ActionsBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ACTIONS
/// Returns `None` if there is no child corresponding to token ACTIONS
fn ACTIONS(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(ACTIONS, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn actionDecl_all(&self) ->  Vec<Rc<ActionDeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn actionDecl(&self, i: usize) -> Option<Rc<ActionDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ActionsBlockContextAttrs<'input> for ActionsBlockContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn actionsBlock(&mut self,)
	-> Result<Rc<ActionsBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ActionsBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_actionsBlock);
        let mut _localctx: Rc<ActionsBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(354);
			recog.base.match_token(ACTIONS,&mut recog.err_handler)?;

			recog.base.set_state(355);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(357); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule actionDecl*/
				recog.base.set_state(356);
				recog.actionDecl()?;

				}
				}
				recog.base.set_state(359); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==IDENTIFIER) {break}
			}
			recog.base.set_state(361);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

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
//------------------- actionDecl ----------------
pub type ActionDeclContextAll<'input> = ActionDeclContext<'input>;


pub type ActionDeclContext<'input> = BaseParserRuleContext<'input,ActionDeclContextExt<'input>>;

#[derive(Clone)]
pub struct ActionDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ActionDeclContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ActionDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_actionDecl(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_actionDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ActionDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_actionDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for ActionDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_actionDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_actionDecl }
}
antlr_rust::tid!{ActionDeclContextExt<'a>}

impl<'input> ActionDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ActionDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ActionDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ActionDeclContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ActionDeclContextExt<'input>>{

fn actionDeclName(&self) -> Option<Rc<ActionDeclNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token ARROW
/// Returns `None` if there is no child corresponding to token ARROW
fn ARROW(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(ARROW, 0)
}
fn actionTarget(&self) -> Option<Rc<ActionTargetContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn actionParamList(&self) -> Option<Rc<ActionParamListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ActionDeclContextAttrs<'input> for ActionDeclContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn actionDecl(&mut self,)
	-> Result<Rc<ActionDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ActionDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_actionDecl);
        let mut _localctx: Rc<ActionDeclContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule actionDeclName*/
			recog.base.set_state(363);
			recog.actionDeclName()?;

			recog.base.set_state(364);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			recog.base.set_state(366);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==IDENTIFIER {
				{
				/*InvokeRule actionParamList*/
				recog.base.set_state(365);
				recog.actionParamList()?;

				}
			}

			recog.base.set_state(368);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			recog.base.set_state(369);
			recog.base.match_token(ARROW,&mut recog.err_handler)?;

			/*InvokeRule actionTarget*/
			recog.base.set_state(370);
			recog.actionTarget()?;

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
//------------------- actionDeclName ----------------
pub type ActionDeclNameContextAll<'input> = ActionDeclNameContext<'input>;


pub type ActionDeclNameContext<'input> = BaseParserRuleContext<'input,ActionDeclNameContextExt<'input>>;

#[derive(Clone)]
pub struct ActionDeclNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ActionDeclNameContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ActionDeclNameContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_actionDeclName(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_actionDeclName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ActionDeclNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_actionDeclName(self);
	}
}

impl<'input> CustomRuleContext<'input> for ActionDeclNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_actionDeclName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_actionDeclName }
}
antlr_rust::tid!{ActionDeclNameContextExt<'a>}

impl<'input> ActionDeclNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ActionDeclNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ActionDeclNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ActionDeclNameContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ActionDeclNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> ActionDeclNameContextAttrs<'input> for ActionDeclNameContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn actionDeclName(&mut self,)
	-> Result<Rc<ActionDeclNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ActionDeclNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_actionDeclName);
        let mut _localctx: Rc<ActionDeclNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(372);
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
//------------------- actionParamList ----------------
pub type ActionParamListContextAll<'input> = ActionParamListContext<'input>;


pub type ActionParamListContext<'input> = BaseParserRuleContext<'input,ActionParamListContextExt<'input>>;

#[derive(Clone)]
pub struct ActionParamListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ActionParamListContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ActionParamListContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_actionParamList(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_actionParamList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ActionParamListContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_actionParamList(self);
	}
}

impl<'input> CustomRuleContext<'input> for ActionParamListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_actionParamList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_actionParamList }
}
antlr_rust::tid!{ActionParamListContextExt<'a>}

impl<'input> ActionParamListContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ActionParamListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ActionParamListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ActionParamListContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ActionParamListContextExt<'input>>{

fn actionParam_all(&self) ->  Vec<Rc<ActionParamContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn actionParam(&self, i: usize) -> Option<Rc<ActionParamContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ActionParamListContextAttrs<'input> for ActionParamListContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn actionParamList(&mut self,)
	-> Result<Rc<ActionParamListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ActionParamListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_actionParamList);
        let mut _localctx: Rc<ActionParamListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule actionParam*/
			recog.base.set_state(374);
			recog.actionParam()?;

			recog.base.set_state(379);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(375);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule actionParam*/
				recog.base.set_state(376);
				recog.actionParam()?;

				}
				}
				recog.base.set_state(381);
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
//------------------- actionParam ----------------
pub type ActionParamContextAll<'input> = ActionParamContext<'input>;


pub type ActionParamContext<'input> = BaseParserRuleContext<'input,ActionParamContextExt<'input>>;

#[derive(Clone)]
pub struct ActionParamContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ActionParamContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ActionParamContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_actionParam(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_actionParam(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ActionParamContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_actionParam(self);
	}
}

impl<'input> CustomRuleContext<'input> for ActionParamContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_actionParam }
	//fn type_rule_index() -> usize where Self: Sized { RULE_actionParam }
}
antlr_rust::tid!{ActionParamContextExt<'a>}

impl<'input> ActionParamContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ActionParamContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ActionParamContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ActionParamContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ActionParamContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn paramType(&self) -> Option<Rc<ParamTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ActionParamContextAttrs<'input> for ActionParamContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn actionParam(&mut self,)
	-> Result<Rc<ActionParamContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ActionParamContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_actionParam);
        let mut _localctx: Rc<ActionParamContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(382);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(383);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule paramType*/
			recog.base.set_state(384);
			recog.paramType()?;

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
//------------------- actionTarget ----------------
pub type ActionTargetContextAll<'input> = ActionTargetContext<'input>;


pub type ActionTargetContext<'input> = BaseParserRuleContext<'input,ActionTargetContextExt<'input>>;

#[derive(Clone)]
pub struct ActionTargetContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ActionTargetContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ActionTargetContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_actionTarget(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_actionTarget(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ActionTargetContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_actionTarget(self);
	}
}

impl<'input> CustomRuleContext<'input> for ActionTargetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_actionTarget }
	//fn type_rule_index() -> usize where Self: Sized { RULE_actionTarget }
}
antlr_rust::tid!{ActionTargetContextExt<'a>}

impl<'input> ActionTargetContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ActionTargetContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ActionTargetContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ActionTargetContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ActionTargetContextExt<'input>>{

fn emitTarget(&self) -> Option<Rc<EmitTargetContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stateTarget(&self) -> Option<Rc<StateTargetContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn auditTarget(&self) -> Option<Rc<AuditTargetContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn callTarget(&self) -> Option<Rc<CallTargetContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ActionTargetContextAttrs<'input> for ActionTargetContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn actionTarget(&mut self,)
	-> Result<Rc<ActionTargetContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ActionTargetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_actionTarget);
        let mut _localctx: Rc<ActionTargetContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(390);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 EMIT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule emitTarget*/
					recog.base.set_state(386);
					recog.emitTarget()?;

					}
				}

			 STATE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule stateTarget*/
					recog.base.set_state(387);
					recog.stateTarget()?;

					}
				}

			 AUDIT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule auditTarget*/
					recog.base.set_state(388);
					recog.auditTarget()?;

					}
				}

			 CALL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule callTarget*/
					recog.base.set_state(389);
					recog.callTarget()?;

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
//------------------- emitTarget ----------------
pub type EmitTargetContextAll<'input> = EmitTargetContext<'input>;


pub type EmitTargetContext<'input> = BaseParserRuleContext<'input,EmitTargetContextExt<'input>>;

#[derive(Clone)]
pub struct EmitTargetContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for EmitTargetContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for EmitTargetContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_emitTarget(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_emitTarget(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for EmitTargetContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_emitTarget(self);
	}
}

impl<'input> CustomRuleContext<'input> for EmitTargetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_emitTarget }
	//fn type_rule_index() -> usize where Self: Sized { RULE_emitTarget }
}
antlr_rust::tid!{EmitTargetContextExt<'a>}

impl<'input> EmitTargetContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EmitTargetContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EmitTargetContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EmitTargetContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<EmitTargetContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EMIT
/// Returns `None` if there is no child corresponding to token EMIT
fn EMIT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(EMIT, 0)
}
/// Retrieves first TerminalNode corresponding to token TO
/// Returns `None` if there is no child corresponding to token TO
fn TO(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(TO, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> EmitTargetContextAttrs<'input> for EmitTargetContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn emitTarget(&mut self,)
	-> Result<Rc<EmitTargetContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EmitTargetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_emitTarget);
        let mut _localctx: Rc<EmitTargetContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(392);
			recog.base.match_token(EMIT,&mut recog.err_handler)?;

			recog.base.set_state(393);
			recog.base.match_token(TO,&mut recog.err_handler)?;

			recog.base.set_state(394);
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
//------------------- stateTarget ----------------
pub type StateTargetContextAll<'input> = StateTargetContext<'input>;


pub type StateTargetContext<'input> = BaseParserRuleContext<'input,StateTargetContextExt<'input>>;

#[derive(Clone)]
pub struct StateTargetContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for StateTargetContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for StateTargetContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stateTarget(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_stateTarget(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for StateTargetContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_stateTarget(self);
	}
}

impl<'input> CustomRuleContext<'input> for StateTargetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stateTarget }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stateTarget }
}
antlr_rust::tid!{StateTargetContextExt<'a>}

impl<'input> StateTargetContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StateTargetContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StateTargetContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StateTargetContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<StateTargetContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STATE
/// Returns `None` if there is no child corresponding to token STATE
fn STATE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(STATE, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}
fn stateOperation(&self) -> Option<Rc<StateOperationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StateTargetContextAttrs<'input> for StateTargetContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stateTarget(&mut self,)
	-> Result<Rc<StateTargetContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StateTargetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_stateTarget);
        let mut _localctx: Rc<StateTargetContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(396);
			recog.base.match_token(STATE,&mut recog.err_handler)?;

			recog.base.set_state(397);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(398);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			/*InvokeRule stateOperation*/
			recog.base.set_state(399);
			recog.stateOperation()?;

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
//------------------- stateOperation ----------------
pub type StateOperationContextAll<'input> = StateOperationContext<'input>;


pub type StateOperationContext<'input> = BaseParserRuleContext<'input,StateOperationContextExt<'input>>;

#[derive(Clone)]
pub struct StateOperationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for StateOperationContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for StateOperationContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stateOperation(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_stateOperation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for StateOperationContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_stateOperation(self);
	}
}

impl<'input> CustomRuleContext<'input> for StateOperationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stateOperation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stateOperation }
}
antlr_rust::tid!{StateOperationContextExt<'a>}

impl<'input> StateOperationContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StateOperationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StateOperationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StateOperationContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<StateOperationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn stateOperationArg(&self) -> Option<Rc<StateOperationArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> StateOperationContextAttrs<'input> for StateOperationContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stateOperation(&mut self,)
	-> Result<Rc<StateOperationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StateOperationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_stateOperation);
        let mut _localctx: Rc<StateOperationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(407);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(17,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(401);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(402);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(403);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule stateOperationArg*/
					recog.base.set_state(404);
					recog.stateOperationArg()?;

					recog.base.set_state(405);
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
//------------------- stateOperationArg ----------------
pub type StateOperationArgContextAll<'input> = StateOperationArgContext<'input>;


pub type StateOperationArgContext<'input> = BaseParserRuleContext<'input,StateOperationArgContextExt<'input>>;

#[derive(Clone)]
pub struct StateOperationArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for StateOperationArgContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for StateOperationArgContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stateOperationArg(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_stateOperationArg(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for StateOperationArgContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_stateOperationArg(self);
	}
}

impl<'input> CustomRuleContext<'input> for StateOperationArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stateOperationArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stateOperationArg }
}
antlr_rust::tid!{StateOperationArgContextExt<'a>}

impl<'input> StateOperationArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StateOperationArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StateOperationArgContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StateOperationArgContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<StateOperationArgContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token DQUOTED_STRING
/// Returns `None` if there is no child corresponding to token DQUOTED_STRING
fn DQUOTED_STRING(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DQUOTED_STRING, 0)
}

}

impl<'input> StateOperationArgContextAttrs<'input> for StateOperationArgContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stateOperationArg(&mut self,)
	-> Result<Rc<StateOperationArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StateOperationArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_stateOperationArg);
        let mut _localctx: Rc<StateOperationArgContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(409);
			_la = recog.base.input.la(1);
			if { !(_la==DQUOTED_STRING || _la==IDENTIFIER) } {
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
//------------------- auditTarget ----------------
pub type AuditTargetContextAll<'input> = AuditTargetContext<'input>;


pub type AuditTargetContext<'input> = BaseParserRuleContext<'input,AuditTargetContextExt<'input>>;

#[derive(Clone)]
pub struct AuditTargetContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for AuditTargetContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for AuditTargetContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_auditTarget(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_auditTarget(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for AuditTargetContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_auditTarget(self);
	}
}

impl<'input> CustomRuleContext<'input> for AuditTargetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_auditTarget }
	//fn type_rule_index() -> usize where Self: Sized { RULE_auditTarget }
}
antlr_rust::tid!{AuditTargetContextExt<'a>}

impl<'input> AuditTargetContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AuditTargetContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AuditTargetContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AuditTargetContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<AuditTargetContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token AUDIT
/// Returns `None` if there is no child corresponding to token AUDIT
fn AUDIT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(AUDIT, 0)
}

}

impl<'input> AuditTargetContextAttrs<'input> for AuditTargetContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn auditTarget(&mut self,)
	-> Result<Rc<AuditTargetContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AuditTargetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_auditTarget);
        let mut _localctx: Rc<AuditTargetContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(411);
			recog.base.match_token(AUDIT,&mut recog.err_handler)?;

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
//------------------- callTarget ----------------
pub type CallTargetContextAll<'input> = CallTargetContext<'input>;


pub type CallTargetContext<'input> = BaseParserRuleContext<'input,CallTargetContextExt<'input>>;

#[derive(Clone)]
pub struct CallTargetContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for CallTargetContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for CallTargetContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_callTarget(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_callTarget(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for CallTargetContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_callTarget(self);
	}
}

impl<'input> CustomRuleContext<'input> for CallTargetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_callTarget }
	//fn type_rule_index() -> usize where Self: Sized { RULE_callTarget }
}
antlr_rust::tid!{CallTargetContextExt<'a>}

impl<'input> CallTargetContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CallTargetContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CallTargetContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CallTargetContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<CallTargetContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CALL
/// Returns `None` if there is no child corresponding to token CALL
fn CALL(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(CALL, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}

}

impl<'input> CallTargetContextAttrs<'input> for CallTargetContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn callTarget(&mut self,)
	-> Result<Rc<CallTargetContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CallTargetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_callTarget);
        let mut _localctx: Rc<CallTargetContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(413);
			recog.base.match_token(CALL,&mut recog.err_handler)?;

			recog.base.set_state(414);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(415);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			recog.base.set_state(416);
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
//------------------- decisionTableDef ----------------
pub type DecisionTableDefContextAll<'input> = DecisionTableDefContext<'input>;


pub type DecisionTableDefContext<'input> = BaseParserRuleContext<'input,DecisionTableDefContextExt<'input>>;

#[derive(Clone)]
pub struct DecisionTableDefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for DecisionTableDefContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for DecisionTableDefContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_decisionTableDef(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_decisionTableDef(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for DecisionTableDefContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_decisionTableDef(self);
	}
}

impl<'input> CustomRuleContext<'input> for DecisionTableDefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_decisionTableDef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_decisionTableDef }
}
antlr_rust::tid!{DecisionTableDefContextExt<'a>}

impl<'input> DecisionTableDefContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DecisionTableDefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DecisionTableDefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DecisionTableDefContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<DecisionTableDefContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DECISION_TABLE
/// Returns `None` if there is no child corresponding to token DECISION_TABLE
fn DECISION_TABLE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DECISION_TABLE, 0)
}
fn tableName(&self) -> Option<Rc<TableNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn givenBlock(&self) -> Option<Rc<GivenBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn decideBlock(&self) -> Option<Rc<DecideBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
fn hitPolicyDecl(&self) -> Option<Rc<HitPolicyDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn descriptionDecl(&self) -> Option<Rc<DescriptionDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn versionDecl(&self) -> Option<Rc<VersionDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn returnSpec(&self) -> Option<Rc<ReturnSpecContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn executeSpec(&self) -> Option<Rc<ExecuteSpecContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn hybridSpec(&self) -> Option<Rc<HybridSpecContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn postCalculateBlock(&self) -> Option<Rc<PostCalculateBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn aggregateBlock(&self) -> Option<Rc<AggregateBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DecisionTableDefContextAttrs<'input> for DecisionTableDefContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn decisionTableDef(&mut self,)
	-> Result<Rc<DecisionTableDefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DecisionTableDefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_decisionTableDef);
        let mut _localctx: Rc<DecisionTableDefContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(418);
			recog.base.match_token(DECISION_TABLE,&mut recog.err_handler)?;

			/*InvokeRule tableName*/
			recog.base.set_state(419);
			recog.tableName()?;

			recog.base.set_state(421);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==HIT_POLICY {
				{
				/*InvokeRule hitPolicyDecl*/
				recog.base.set_state(420);
				recog.hitPolicyDecl()?;

				}
			}

			recog.base.set_state(424);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==DESCRIPTION {
				{
				/*InvokeRule descriptionDecl*/
				recog.base.set_state(423);
				recog.descriptionDecl()?;

				}
			}

			recog.base.set_state(427);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==VERSION {
				{
				/*InvokeRule versionDecl*/
				recog.base.set_state(426);
				recog.versionDecl()?;

				}
			}

			/*InvokeRule givenBlock*/
			recog.base.set_state(429);
			recog.givenBlock()?;

			/*InvokeRule decideBlock*/
			recog.base.set_state(430);
			recog.decideBlock()?;

			recog.base.set_state(434);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(21,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule returnSpec*/
					recog.base.set_state(431);
					recog.returnSpec()?;

					}
				}

				x if x == 2=>{
					{
					/*InvokeRule executeSpec*/
					recog.base.set_state(432);
					recog.executeSpec()?;

					}
				}

				x if x == 3=>{
					{
					/*InvokeRule hybridSpec*/
					recog.base.set_state(433);
					recog.hybridSpec()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(437);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==POST_CALCULATE {
				{
				/*InvokeRule postCalculateBlock*/
				recog.base.set_state(436);
				recog.postCalculateBlock()?;

				}
			}

			recog.base.set_state(440);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==AGGREGATE {
				{
				/*InvokeRule aggregateBlock*/
				recog.base.set_state(439);
				recog.aggregateBlock()?;

				}
			}

			recog.base.set_state(442);
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
//------------------- versionDecl ----------------
pub type VersionDeclContextAll<'input> = VersionDeclContext<'input>;


pub type VersionDeclContext<'input> = BaseParserRuleContext<'input,VersionDeclContextExt<'input>>;

#[derive(Clone)]
pub struct VersionDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for VersionDeclContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for VersionDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_versionDecl(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_versionDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for VersionDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_versionDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for VersionDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_versionDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_versionDecl }
}
antlr_rust::tid!{VersionDeclContextExt<'a>}

impl<'input> VersionDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VersionDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VersionDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VersionDeclContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<VersionDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token VERSION
/// Returns `None` if there is no child corresponding to token VERSION
fn VERSION(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(VERSION, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token VERSION_NUMBER
/// Returns `None` if there is no child corresponding to token VERSION_NUMBER
fn VERSION_NUMBER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(VERSION_NUMBER, 0)
}

}

impl<'input> VersionDeclContextAttrs<'input> for VersionDeclContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn versionDecl(&mut self,)
	-> Result<Rc<VersionDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VersionDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_versionDecl);
        let mut _localctx: Rc<VersionDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(444);
			recog.base.match_token(VERSION,&mut recog.err_handler)?;

			recog.base.set_state(445);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(446);
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
//------------------- tableName ----------------
pub type TableNameContextAll<'input> = TableNameContext<'input>;


pub type TableNameContext<'input> = BaseParserRuleContext<'input,TableNameContextExt<'input>>;

#[derive(Clone)]
pub struct TableNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for TableNameContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for TableNameContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tableName(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_tableName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for TableNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_tableName(self);
	}
}

impl<'input> CustomRuleContext<'input> for TableNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tableName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tableName }
}
antlr_rust::tid!{TableNameContextExt<'a>}

impl<'input> TableNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TableNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TableNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TableNameContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<TableNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> TableNameContextAttrs<'input> for TableNameContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tableName(&mut self,)
	-> Result<Rc<TableNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TableNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_tableName);
        let mut _localctx: Rc<TableNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(448);
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
//------------------- hitPolicyDecl ----------------
pub type HitPolicyDeclContextAll<'input> = HitPolicyDeclContext<'input>;


pub type HitPolicyDeclContext<'input> = BaseParserRuleContext<'input,HitPolicyDeclContextExt<'input>>;

#[derive(Clone)]
pub struct HitPolicyDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for HitPolicyDeclContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for HitPolicyDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_hitPolicyDecl(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_hitPolicyDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for HitPolicyDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_hitPolicyDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for HitPolicyDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hitPolicyDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hitPolicyDecl }
}
antlr_rust::tid!{HitPolicyDeclContextExt<'a>}

impl<'input> HitPolicyDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<HitPolicyDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HitPolicyDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait HitPolicyDeclContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<HitPolicyDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token HIT_POLICY
/// Returns `None` if there is no child corresponding to token HIT_POLICY
fn HIT_POLICY(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(HIT_POLICY, 0)
}
fn hitPolicyType(&self) -> Option<Rc<HitPolicyTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> HitPolicyDeclContextAttrs<'input> for HitPolicyDeclContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn hitPolicyDecl(&mut self,)
	-> Result<Rc<HitPolicyDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HitPolicyDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_hitPolicyDecl);
        let mut _localctx: Rc<HitPolicyDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(450);
			recog.base.match_token(HIT_POLICY,&mut recog.err_handler)?;

			/*InvokeRule hitPolicyType*/
			recog.base.set_state(451);
			recog.hitPolicyType()?;

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
//------------------- hitPolicyType ----------------
pub type HitPolicyTypeContextAll<'input> = HitPolicyTypeContext<'input>;


pub type HitPolicyTypeContext<'input> = BaseParserRuleContext<'input,HitPolicyTypeContextExt<'input>>;

#[derive(Clone)]
pub struct HitPolicyTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for HitPolicyTypeContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for HitPolicyTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_hitPolicyType(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_hitPolicyType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for HitPolicyTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_hitPolicyType(self);
	}
}

impl<'input> CustomRuleContext<'input> for HitPolicyTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hitPolicyType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hitPolicyType }
}
antlr_rust::tid!{HitPolicyTypeContextExt<'a>}

impl<'input> HitPolicyTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<HitPolicyTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HitPolicyTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait HitPolicyTypeContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<HitPolicyTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FIRST_MATCH
/// Returns `None` if there is no child corresponding to token FIRST_MATCH
fn FIRST_MATCH(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(FIRST_MATCH, 0)
}
/// Retrieves first TerminalNode corresponding to token SINGLE_HIT
/// Returns `None` if there is no child corresponding to token SINGLE_HIT
fn SINGLE_HIT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(SINGLE_HIT, 0)
}
/// Retrieves first TerminalNode corresponding to token MULTI_HIT
/// Returns `None` if there is no child corresponding to token MULTI_HIT
fn MULTI_HIT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(MULTI_HIT, 0)
}
/// Retrieves first TerminalNode corresponding to token COLLECT_ALL
/// Returns `None` if there is no child corresponding to token COLLECT_ALL
fn COLLECT_ALL(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COLLECT_ALL, 0)
}

}

impl<'input> HitPolicyTypeContextAttrs<'input> for HitPolicyTypeContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn hitPolicyType(&mut self,)
	-> Result<Rc<HitPolicyTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HitPolicyTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_hitPolicyType);
        let mut _localctx: Rc<HitPolicyTypeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(453);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << FIRST_MATCH) | (1usize << SINGLE_HIT) | (1usize << MULTI_HIT) | (1usize << COLLECT_ALL))) != 0)) } {
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
//------------------- descriptionDecl ----------------
pub type DescriptionDeclContextAll<'input> = DescriptionDeclContext<'input>;


pub type DescriptionDeclContext<'input> = BaseParserRuleContext<'input,DescriptionDeclContextExt<'input>>;

#[derive(Clone)]
pub struct DescriptionDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for DescriptionDeclContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for DescriptionDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_descriptionDecl(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_descriptionDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for DescriptionDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_descriptionDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for DescriptionDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_descriptionDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_descriptionDecl }
}
antlr_rust::tid!{DescriptionDeclContextExt<'a>}

impl<'input> DescriptionDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DescriptionDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DescriptionDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DescriptionDeclContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<DescriptionDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DESCRIPTION
/// Returns `None` if there is no child corresponding to token DESCRIPTION
fn DESCRIPTION(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DESCRIPTION, 0)
}
fn stringLiteral(&self) -> Option<Rc<StringLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DescriptionDeclContextAttrs<'input> for DescriptionDeclContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn descriptionDecl(&mut self,)
	-> Result<Rc<DescriptionDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DescriptionDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_descriptionDecl);
        let mut _localctx: Rc<DescriptionDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(455);
			recog.base.match_token(DESCRIPTION,&mut recog.err_handler)?;

			/*InvokeRule stringLiteral*/
			recog.base.set_state(456);
			recog.stringLiteral()?;

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
//------------------- stringLiteral ----------------
pub type StringLiteralContextAll<'input> = StringLiteralContext<'input>;


pub type StringLiteralContext<'input> = BaseParserRuleContext<'input,StringLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct StringLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for StringLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for StringLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stringLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_stringLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for StringLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_stringLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stringLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stringLiteral }
}
antlr_rust::tid!{StringLiteralContextExt<'a>}

impl<'input> StringLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StringLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StringLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StringLiteralContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<StringLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DQUOTED_STRING
/// Returns `None` if there is no child corresponding to token DQUOTED_STRING
fn DQUOTED_STRING(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DQUOTED_STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token SQUOTED_STRING
/// Returns `None` if there is no child corresponding to token SQUOTED_STRING
fn SQUOTED_STRING(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(SQUOTED_STRING, 0)
}

}

impl<'input> StringLiteralContextAttrs<'input> for StringLiteralContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stringLiteral(&mut self,)
	-> Result<Rc<StringLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StringLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_stringLiteral);
        let mut _localctx: Rc<StringLiteralContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(458);
			_la = recog.base.input.la(1);
			if { !(_la==DQUOTED_STRING || _la==SQUOTED_STRING) } {
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
//------------------- givenBlock ----------------
pub type GivenBlockContextAll<'input> = GivenBlockContext<'input>;


pub type GivenBlockContext<'input> = BaseParserRuleContext<'input,GivenBlockContextExt<'input>>;

#[derive(Clone)]
pub struct GivenBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for GivenBlockContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for GivenBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_givenBlock(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_givenBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for GivenBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_givenBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for GivenBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_givenBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_givenBlock }
}
antlr_rust::tid!{GivenBlockContextExt<'a>}

impl<'input> GivenBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<GivenBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GivenBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait GivenBlockContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<GivenBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token GIVEN
/// Returns `None` if there is no child corresponding to token GIVEN
fn GIVEN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(GIVEN, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn inputParam_all(&self) ->  Vec<Rc<InputParamContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn inputParam(&self, i: usize) -> Option<Rc<InputParamContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> GivenBlockContextAttrs<'input> for GivenBlockContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn givenBlock(&mut self,)
	-> Result<Rc<GivenBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GivenBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_givenBlock);
        let mut _localctx: Rc<GivenBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(460);
			recog.base.match_token(GIVEN,&mut recog.err_handler)?;

			recog.base.set_state(461);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(463); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule inputParam*/
				recog.base.set_state(462);
				recog.inputParam()?;

				}
				}
				recog.base.set_state(465); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(((((_la - 72)) & !0x3f) == 0 && ((1usize << (_la - 72)) & ((1usize << (TEXT_TYPE - 72)) | (1usize << (DESCRIPTION - 72)) | (1usize << (PRIORITY - 72)) | (1usize << (VERSION - 72)))) != 0) || _la==IDENTIFIER) {break}
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
//------------------- inputParam ----------------
pub type InputParamContextAll<'input> = InputParamContext<'input>;


pub type InputParamContext<'input> = BaseParserRuleContext<'input,InputParamContextExt<'input>>;

#[derive(Clone)]
pub struct InputParamContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for InputParamContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for InputParamContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_inputParam(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_inputParam(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for InputParamContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_inputParam(self);
	}
}

impl<'input> CustomRuleContext<'input> for InputParamContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inputParam }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inputParam }
}
antlr_rust::tid!{InputParamContextExt<'a>}

impl<'input> InputParamContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InputParamContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InputParamContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InputParamContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<InputParamContextExt<'input>>{

fn paramName(&self) -> Option<Rc<ParamNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn paramType(&self) -> Option<Rc<ParamTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn inlineComment(&self) -> Option<Rc<InlineCommentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> InputParamContextAttrs<'input> for InputParamContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inputParam(&mut self,)
	-> Result<Rc<InputParamContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InputParamContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_inputParam);
        let mut _localctx: Rc<InputParamContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule paramName*/
			recog.base.set_state(467);
			recog.paramName()?;

			recog.base.set_state(468);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule paramType*/
			recog.base.set_state(469);
			recog.paramType()?;

			recog.base.set_state(471);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LINE_COMMENT_INLINE {
				{
				/*InvokeRule inlineComment*/
				recog.base.set_state(470);
				recog.inlineComment()?;

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
//------------------- paramName ----------------
pub type ParamNameContextAll<'input> = ParamNameContext<'input>;


pub type ParamNameContext<'input> = BaseParserRuleContext<'input,ParamNameContextExt<'input>>;

#[derive(Clone)]
pub struct ParamNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ParamNameContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ParamNameContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_paramName(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_paramName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ParamNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_paramName(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParamNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_paramName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_paramName }
}
antlr_rust::tid!{ParamNameContextExt<'a>}

impl<'input> ParamNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParamNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParamNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParamNameContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ParamNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token PRIORITY
/// Returns `None` if there is no child corresponding to token PRIORITY
fn PRIORITY(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(PRIORITY, 0)
}
/// Retrieves first TerminalNode corresponding to token DESCRIPTION
/// Returns `None` if there is no child corresponding to token DESCRIPTION
fn DESCRIPTION(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DESCRIPTION, 0)
}
/// Retrieves first TerminalNode corresponding to token TEXT_TYPE
/// Returns `None` if there is no child corresponding to token TEXT_TYPE
fn TEXT_TYPE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(TEXT_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token VERSION
/// Returns `None` if there is no child corresponding to token VERSION
fn VERSION(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(VERSION, 0)
}

}

impl<'input> ParamNameContextAttrs<'input> for ParamNameContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn paramName(&mut self,)
	-> Result<Rc<ParamNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParamNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_paramName);
        let mut _localctx: Rc<ParamNameContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(473);
			_la = recog.base.input.la(1);
			if { !(((((_la - 72)) & !0x3f) == 0 && ((1usize << (_la - 72)) & ((1usize << (TEXT_TYPE - 72)) | (1usize << (DESCRIPTION - 72)) | (1usize << (PRIORITY - 72)) | (1usize << (VERSION - 72)))) != 0) || _la==IDENTIFIER) } {
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
//------------------- paramType ----------------
pub type ParamTypeContextAll<'input> = ParamTypeContext<'input>;


pub type ParamTypeContext<'input> = BaseParserRuleContext<'input,ParamTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ParamTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ParamTypeContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ParamTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_paramType(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_paramType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ParamTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_paramType(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParamTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_paramType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_paramType }
}
antlr_rust::tid!{ParamTypeContextExt<'a>}

impl<'input> ParamTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParamTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParamTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParamTypeContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ParamTypeContextExt<'input>>{

fn baseType(&self) -> Option<Rc<BaseTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token MONEY_TYPE
/// Returns `None` if there is no child corresponding to token MONEY_TYPE
fn MONEY_TYPE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(MONEY_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token PERCENTAGE_TYPE
/// Returns `None` if there is no child corresponding to token PERCENTAGE_TYPE
fn PERCENTAGE_TYPE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(PERCENTAGE_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> ParamTypeContextAttrs<'input> for ParamTypeContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn paramType(&mut self,)
	-> Result<Rc<ParamTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParamTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_paramType);
        let mut _localctx: Rc<ParamTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(479);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 TEXT_TYPE | NUMBER_TYPE | BOOLEAN_TYPE | DATE_TYPE | TIMESTAMP_TYPE |
			 BIZDATE_TYPE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule baseType*/
					recog.base.set_state(475);
					recog.baseType()?;

					}
				}

			 MONEY_TYPE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(476);
					recog.base.match_token(MONEY_TYPE,&mut recog.err_handler)?;

					}
				}

			 PERCENTAGE_TYPE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(477);
					recog.base.match_token(PERCENTAGE_TYPE,&mut recog.err_handler)?;

					}
				}

			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(478);
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
//------------------- baseType ----------------
pub type BaseTypeContextAll<'input> = BaseTypeContext<'input>;


pub type BaseTypeContext<'input> = BaseParserRuleContext<'input,BaseTypeContextExt<'input>>;

#[derive(Clone)]
pub struct BaseTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for BaseTypeContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for BaseTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_baseType(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_baseType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for BaseTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_baseType(self);
	}
}

impl<'input> CustomRuleContext<'input> for BaseTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_baseType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_baseType }
}
antlr_rust::tid!{BaseTypeContextExt<'a>}

impl<'input> BaseTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BaseTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BaseTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BaseTypeContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<BaseTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TEXT_TYPE
/// Returns `None` if there is no child corresponding to token TEXT_TYPE
fn TEXT_TYPE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(TEXT_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMBER_TYPE
/// Returns `None` if there is no child corresponding to token NUMBER_TYPE
fn NUMBER_TYPE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(NUMBER_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token BOOLEAN_TYPE
/// Returns `None` if there is no child corresponding to token BOOLEAN_TYPE
fn BOOLEAN_TYPE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(BOOLEAN_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token DATE_TYPE
/// Returns `None` if there is no child corresponding to token DATE_TYPE
fn DATE_TYPE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DATE_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token TIMESTAMP_TYPE
/// Returns `None` if there is no child corresponding to token TIMESTAMP_TYPE
fn TIMESTAMP_TYPE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(TIMESTAMP_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token BIZDATE_TYPE
/// Returns `None` if there is no child corresponding to token BIZDATE_TYPE
fn BIZDATE_TYPE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(BIZDATE_TYPE, 0)
}

}

impl<'input> BaseTypeContextAttrs<'input> for BaseTypeContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn baseType(&mut self,)
	-> Result<Rc<BaseTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BaseTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_baseType);
        let mut _localctx: Rc<BaseTypeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(481);
			_la = recog.base.input.la(1);
			if { !(((((_la - 72)) & !0x3f) == 0 && ((1usize << (_la - 72)) & ((1usize << (TEXT_TYPE - 72)) | (1usize << (NUMBER_TYPE - 72)) | (1usize << (BOOLEAN_TYPE - 72)) | (1usize << (DATE_TYPE - 72)) | (1usize << (TIMESTAMP_TYPE - 72)) | (1usize << (BIZDATE_TYPE - 72)))) != 0)) } {
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
//------------------- inlineComment ----------------
pub type InlineCommentContextAll<'input> = InlineCommentContext<'input>;


pub type InlineCommentContext<'input> = BaseParserRuleContext<'input,InlineCommentContextExt<'input>>;

#[derive(Clone)]
pub struct InlineCommentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for InlineCommentContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for InlineCommentContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_inlineComment(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_inlineComment(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for InlineCommentContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_inlineComment(self);
	}
}

impl<'input> CustomRuleContext<'input> for InlineCommentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineComment }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineComment }
}
antlr_rust::tid!{InlineCommentContextExt<'a>}

impl<'input> InlineCommentContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InlineCommentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InlineCommentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InlineCommentContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<InlineCommentContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LINE_COMMENT_INLINE
/// Returns `None` if there is no child corresponding to token LINE_COMMENT_INLINE
fn LINE_COMMENT_INLINE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LINE_COMMENT_INLINE, 0)
}

}

impl<'input> InlineCommentContextAttrs<'input> for InlineCommentContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inlineComment(&mut self,)
	-> Result<Rc<InlineCommentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InlineCommentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_inlineComment);
        let mut _localctx: Rc<InlineCommentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(483);
			recog.base.match_token(LINE_COMMENT_INLINE,&mut recog.err_handler)?;

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
//------------------- decideBlock ----------------
pub type DecideBlockContextAll<'input> = DecideBlockContext<'input>;


pub type DecideBlockContext<'input> = BaseParserRuleContext<'input,DecideBlockContextExt<'input>>;

#[derive(Clone)]
pub struct DecideBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for DecideBlockContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for DecideBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_decideBlock(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_decideBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for DecideBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_decideBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for DecideBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_decideBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_decideBlock }
}
antlr_rust::tid!{DecideBlockContextExt<'a>}

impl<'input> DecideBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DecideBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DecideBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DecideBlockContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<DecideBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DECIDE
/// Returns `None` if there is no child corresponding to token DECIDE
fn DECIDE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DECIDE, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn tableMatrix(&self) -> Option<Rc<TableMatrixContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DecideBlockContextAttrs<'input> for DecideBlockContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn decideBlock(&mut self,)
	-> Result<Rc<DecideBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DecideBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_decideBlock);
        let mut _localctx: Rc<DecideBlockContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(485);
			recog.base.match_token(DECIDE,&mut recog.err_handler)?;

			recog.base.set_state(486);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule tableMatrix*/
			recog.base.set_state(487);
			recog.tableMatrix()?;

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
//------------------- tableMatrix ----------------
pub type TableMatrixContextAll<'input> = TableMatrixContext<'input>;


pub type TableMatrixContext<'input> = BaseParserRuleContext<'input,TableMatrixContextExt<'input>>;

#[derive(Clone)]
pub struct TableMatrixContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for TableMatrixContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for TableMatrixContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tableMatrix(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_tableMatrix(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for TableMatrixContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_tableMatrix(self);
	}
}

impl<'input> CustomRuleContext<'input> for TableMatrixContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tableMatrix }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tableMatrix }
}
antlr_rust::tid!{TableMatrixContextExt<'a>}

impl<'input> TableMatrixContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TableMatrixContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TableMatrixContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TableMatrixContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<TableMatrixContextExt<'input>>{

fn tableHeader(&self) -> Option<Rc<TableHeaderContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tableSeparator(&self) -> Option<Rc<TableSeparatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tableRow_all(&self) ->  Vec<Rc<TableRowContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn tableRow(&self, i: usize) -> Option<Rc<TableRowContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TableMatrixContextAttrs<'input> for TableMatrixContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tableMatrix(&mut self,)
	-> Result<Rc<TableMatrixContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TableMatrixContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_tableMatrix);
        let mut _localctx: Rc<TableMatrixContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule tableHeader*/
			recog.base.set_state(489);
			recog.tableHeader()?;

			/*InvokeRule tableSeparator*/
			recog.base.set_state(490);
			recog.tableSeparator()?;

			recog.base.set_state(492); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule tableRow*/
				recog.base.set_state(491);
				recog.tableRow()?;

				}
				}
				recog.base.set_state(494); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==PIPE) {break}
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
//------------------- tableHeader ----------------
pub type TableHeaderContextAll<'input> = TableHeaderContext<'input>;


pub type TableHeaderContext<'input> = BaseParserRuleContext<'input,TableHeaderContextExt<'input>>;

#[derive(Clone)]
pub struct TableHeaderContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for TableHeaderContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for TableHeaderContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tableHeader(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_tableHeader(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for TableHeaderContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_tableHeader(self);
	}
}

impl<'input> CustomRuleContext<'input> for TableHeaderContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tableHeader }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tableHeader }
}
antlr_rust::tid!{TableHeaderContextExt<'a>}

impl<'input> TableHeaderContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TableHeaderContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TableHeaderContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TableHeaderContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<TableHeaderContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PIPE
/// Returns `None` if there is no child corresponding to token PIPE
fn PIPE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(PIPE, 0)
}
fn priorityHeader(&self) -> Option<Rc<PriorityHeaderContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn columnHeader_all(&self) ->  Vec<Rc<ColumnHeaderContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn columnHeader(&self, i: usize) -> Option<Rc<ColumnHeaderContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TableHeaderContextAttrs<'input> for TableHeaderContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tableHeader(&mut self,)
	-> Result<Rc<TableHeaderContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TableHeaderContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_tableHeader);
        let mut _localctx: Rc<TableHeaderContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(496);
			recog.base.match_token(PIPE,&mut recog.err_handler)?;

			recog.base.set_state(498);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(28,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule priorityHeader*/
					recog.base.set_state(497);
					recog.priorityHeader()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(501); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule columnHeader*/
				recog.base.set_state(500);
				recog.columnHeader()?;

				}
				}
				recog.base.set_state(503); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==RETURN || _la==DEFAULT || _la==TEXT_TYPE || ((((_la - 84)) & !0x3f) == 0 && ((1usize << (_la - 84)) & ((1usize << (DESCRIPTION - 84)) | (1usize << (PRIORITY - 84)) | (1usize << (VERSION - 84)))) != 0) || _la==IDENTIFIER) {break}
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
//------------------- priorityHeader ----------------
pub type PriorityHeaderContextAll<'input> = PriorityHeaderContext<'input>;


pub type PriorityHeaderContext<'input> = BaseParserRuleContext<'input,PriorityHeaderContextExt<'input>>;

#[derive(Clone)]
pub struct PriorityHeaderContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for PriorityHeaderContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for PriorityHeaderContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_priorityHeader(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_priorityHeader(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for PriorityHeaderContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_priorityHeader(self);
	}
}

impl<'input> CustomRuleContext<'input> for PriorityHeaderContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_priorityHeader }
	//fn type_rule_index() -> usize where Self: Sized { RULE_priorityHeader }
}
antlr_rust::tid!{PriorityHeaderContextExt<'a>}

impl<'input> PriorityHeaderContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PriorityHeaderContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PriorityHeaderContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PriorityHeaderContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<PriorityHeaderContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PRIORITY
/// Returns `None` if there is no child corresponding to token PRIORITY
fn PRIORITY(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(PRIORITY, 0)
}
/// Retrieves first TerminalNode corresponding to token PIPE
/// Returns `None` if there is no child corresponding to token PIPE
fn PIPE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(PIPE, 0)
}

}

impl<'input> PriorityHeaderContextAttrs<'input> for PriorityHeaderContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn priorityHeader(&mut self,)
	-> Result<Rc<PriorityHeaderContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PriorityHeaderContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_priorityHeader);
        let mut _localctx: Rc<PriorityHeaderContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(505);
			recog.base.match_token(PRIORITY,&mut recog.err_handler)?;

			recog.base.set_state(506);
			recog.base.match_token(PIPE,&mut recog.err_handler)?;

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
//------------------- columnHeader ----------------
pub type ColumnHeaderContextAll<'input> = ColumnHeaderContext<'input>;


pub type ColumnHeaderContext<'input> = BaseParserRuleContext<'input,ColumnHeaderContextExt<'input>>;

#[derive(Clone)]
pub struct ColumnHeaderContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ColumnHeaderContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ColumnHeaderContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_columnHeader(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_columnHeader(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ColumnHeaderContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_columnHeader(self);
	}
}

impl<'input> CustomRuleContext<'input> for ColumnHeaderContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_columnHeader }
	//fn type_rule_index() -> usize where Self: Sized { RULE_columnHeader }
}
antlr_rust::tid!{ColumnHeaderContextExt<'a>}

impl<'input> ColumnHeaderContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ColumnHeaderContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ColumnHeaderContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ColumnHeaderContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ColumnHeaderContextExt<'input>>{

fn columnName(&self) -> Option<Rc<ColumnNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token PIPE
/// Returns `None` if there is no child corresponding to token PIPE
fn PIPE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(PIPE, 0)
}

}

impl<'input> ColumnHeaderContextAttrs<'input> for ColumnHeaderContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn columnHeader(&mut self,)
	-> Result<Rc<ColumnHeaderContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ColumnHeaderContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_columnHeader);
        let mut _localctx: Rc<ColumnHeaderContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule columnName*/
			recog.base.set_state(508);
			recog.columnName()?;

			recog.base.set_state(509);
			recog.base.match_token(PIPE,&mut recog.err_handler)?;

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
//------------------- columnName ----------------
pub type ColumnNameContextAll<'input> = ColumnNameContext<'input>;


pub type ColumnNameContext<'input> = BaseParserRuleContext<'input,ColumnNameContextExt<'input>>;

#[derive(Clone)]
pub struct ColumnNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ColumnNameContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ColumnNameContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_columnName(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_columnName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ColumnNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_columnName(self);
	}
}

impl<'input> CustomRuleContext<'input> for ColumnNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_columnName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_columnName }
}
antlr_rust::tid!{ColumnNameContextExt<'a>}

impl<'input> ColumnNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ColumnNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ColumnNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ColumnNameContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ColumnNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token PRIORITY
/// Returns `None` if there is no child corresponding to token PRIORITY
fn PRIORITY(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(PRIORITY, 0)
}
/// Retrieves first TerminalNode corresponding to token DESCRIPTION
/// Returns `None` if there is no child corresponding to token DESCRIPTION
fn DESCRIPTION(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DESCRIPTION, 0)
}
/// Retrieves first TerminalNode corresponding to token TEXT_TYPE
/// Returns `None` if there is no child corresponding to token TEXT_TYPE
fn TEXT_TYPE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(TEXT_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token VERSION
/// Returns `None` if there is no child corresponding to token VERSION
fn VERSION(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(VERSION, 0)
}
/// Retrieves first TerminalNode corresponding to token DEFAULT
/// Returns `None` if there is no child corresponding to token DEFAULT
fn DEFAULT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DEFAULT, 0)
}
/// Retrieves first TerminalNode corresponding to token RETURN
/// Returns `None` if there is no child corresponding to token RETURN
fn RETURN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RETURN, 0)
}

}

impl<'input> ColumnNameContextAttrs<'input> for ColumnNameContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn columnName(&mut self,)
	-> Result<Rc<ColumnNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ColumnNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 96, RULE_columnName);
        let mut _localctx: Rc<ColumnNameContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(511);
			_la = recog.base.input.la(1);
			if { !(_la==RETURN || _la==DEFAULT || _la==TEXT_TYPE || ((((_la - 84)) & !0x3f) == 0 && ((1usize << (_la - 84)) & ((1usize << (DESCRIPTION - 84)) | (1usize << (PRIORITY - 84)) | (1usize << (VERSION - 84)))) != 0) || _la==IDENTIFIER) } {
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
//------------------- tableSeparator ----------------
pub type TableSeparatorContextAll<'input> = TableSeparatorContext<'input>;


pub type TableSeparatorContext<'input> = BaseParserRuleContext<'input,TableSeparatorContextExt<'input>>;

#[derive(Clone)]
pub struct TableSeparatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for TableSeparatorContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for TableSeparatorContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tableSeparator(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_tableSeparator(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for TableSeparatorContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_tableSeparator(self);
	}
}

impl<'input> CustomRuleContext<'input> for TableSeparatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tableSeparator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tableSeparator }
}
antlr_rust::tid!{TableSeparatorContextExt<'a>}

impl<'input> TableSeparatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TableSeparatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TableSeparatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TableSeparatorContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<TableSeparatorContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token PIPE in current rule
fn PIPE_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token PIPE, starting from 0.
/// Returns `None` if number of children corresponding to token PIPE is less or equal than `i`.
fn PIPE(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(PIPE, i)
}
/// Retrieves first TerminalNode corresponding to token SEP
/// Returns `None` if there is no child corresponding to token SEP
fn SEP(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(SEP, 0)
}

}

impl<'input> TableSeparatorContextAttrs<'input> for TableSeparatorContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tableSeparator(&mut self,)
	-> Result<Rc<TableSeparatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TableSeparatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 98, RULE_tableSeparator);
        let mut _localctx: Rc<TableSeparatorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(513);
			recog.base.match_token(PIPE,&mut recog.err_handler)?;

			recog.base.set_state(514);
			recog.base.match_token(SEP,&mut recog.err_handler)?;

			recog.base.set_state(515);
			recog.base.match_token(PIPE,&mut recog.err_handler)?;

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
//------------------- tableRow ----------------
pub type TableRowContextAll<'input> = TableRowContext<'input>;


pub type TableRowContext<'input> = BaseParserRuleContext<'input,TableRowContextExt<'input>>;

#[derive(Clone)]
pub struct TableRowContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for TableRowContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for TableRowContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tableRow(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_tableRow(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for TableRowContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_tableRow(self);
	}
}

impl<'input> CustomRuleContext<'input> for TableRowContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tableRow }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tableRow }
}
antlr_rust::tid!{TableRowContextExt<'a>}

impl<'input> TableRowContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TableRowContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TableRowContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TableRowContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<TableRowContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PIPE
/// Returns `None` if there is no child corresponding to token PIPE
fn PIPE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(PIPE, 0)
}
fn priorityCell(&self) -> Option<Rc<PriorityCellContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn cell_all(&self) ->  Vec<Rc<CellContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn cell(&self, i: usize) -> Option<Rc<CellContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TableRowContextAttrs<'input> for TableRowContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tableRow(&mut self,)
	-> Result<Rc<TableRowContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TableRowContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 100, RULE_tableRow);
        let mut _localctx: Rc<TableRowContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(517);
			recog.base.match_token(PIPE,&mut recog.err_handler)?;

			recog.base.set_state(519);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(30,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule priorityCell*/
					recog.base.set_state(518);
					recog.priorityCell()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(522); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule cell*/
				recog.base.set_state(521);
				recog.cell()?;

				}
				}
				recog.base.set_state(524); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(((((_la - 31)) & !0x3f) == 0 && ((1usize << (_la - 31)) & ((1usize << (NOT - 31)) | (1usize << (IN - 31)) | (1usize << (IS - 31)) | (1usize << (NULL - 31)) | (1usize << (IS_NULL - 31)) | (1usize << (IS_NOT_NULL - 31)) | (1usize << (MATCHES - 31)) | (1usize << (CONTAINS - 31)) | (1usize << (STARTS_WITH - 31)) | (1usize << (ENDS_WITH - 31)) | (1usize << (LOOKUP - 31)) | (1usize << (EMIT - 31)) | (1usize << (ANY - 31)) | (1usize << (ALL - 31)) | (1usize << (NONE - 31)) | (1usize << (SUM - 31)) | (1usize << (COUNT - 31)) | (1usize << (AVG - 31)))) != 0) || ((((_la - 63)) & !0x3f) == 0 && ((1usize << (_la - 63)) & ((1usize << (MAX_FN - 63)) | (1usize << (MIN_FN - 63)) | (1usize << (FILTER - 63)) | (1usize << (FIND - 63)) | (1usize << (DISTINCT - 63)) | (1usize << (MARKER - 63)) | (1usize << (BETWEEN_MARKERS - 63)) | (1usize << (EQ - 63)) | (1usize << (NE - 63)) | (1usize << (LT - 63)) | (1usize << (GT - 63)) | (1usize << (LE - 63)) | (1usize << (GE - 63)))) != 0) || ((((_la - 96)) & !0x3f) == 0 && ((1usize << (_la - 96)) & ((1usize << (MINUS - 96)) | (1usize << (STAR - 96)) | (1usize << (LPAREN - 96)) | (1usize << (LBRACKET - 96)) | (1usize << (LBRACE - 96)) | (1usize << (BOOLEAN - 96)) | (1usize << (INTEGER - 96)) | (1usize << (DECIMAL - 96)) | (1usize << (MONEY_LITERAL - 96)) | (1usize << (PERCENTAGE_LITERAL - 96)) | (1usize << (DQUOTED_STRING - 96)) | (1usize << (SQUOTED_STRING - 96)) | (1usize << (IDENTIFIER - 96)))) != 0)) {break}
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
//------------------- priorityCell ----------------
pub type PriorityCellContextAll<'input> = PriorityCellContext<'input>;


pub type PriorityCellContext<'input> = BaseParserRuleContext<'input,PriorityCellContextExt<'input>>;

#[derive(Clone)]
pub struct PriorityCellContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for PriorityCellContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for PriorityCellContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_priorityCell(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_priorityCell(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for PriorityCellContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_priorityCell(self);
	}
}

impl<'input> CustomRuleContext<'input> for PriorityCellContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_priorityCell }
	//fn type_rule_index() -> usize where Self: Sized { RULE_priorityCell }
}
antlr_rust::tid!{PriorityCellContextExt<'a>}

impl<'input> PriorityCellContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PriorityCellContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PriorityCellContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PriorityCellContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<PriorityCellContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
/// Retrieves first TerminalNode corresponding to token PIPE
/// Returns `None` if there is no child corresponding to token PIPE
fn PIPE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(PIPE, 0)
}

}

impl<'input> PriorityCellContextAttrs<'input> for PriorityCellContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn priorityCell(&mut self,)
	-> Result<Rc<PriorityCellContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PriorityCellContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 102, RULE_priorityCell);
        let mut _localctx: Rc<PriorityCellContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(526);
			recog.base.match_token(INTEGER,&mut recog.err_handler)?;

			recog.base.set_state(527);
			recog.base.match_token(PIPE,&mut recog.err_handler)?;

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
//------------------- cell ----------------
pub type CellContextAll<'input> = CellContext<'input>;


pub type CellContext<'input> = BaseParserRuleContext<'input,CellContextExt<'input>>;

#[derive(Clone)]
pub struct CellContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for CellContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for CellContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_cell(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_cell(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for CellContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_cell(self);
	}
}

impl<'input> CustomRuleContext<'input> for CellContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cell }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cell }
}
antlr_rust::tid!{CellContextExt<'a>}

impl<'input> CellContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CellContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CellContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CellContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<CellContextExt<'input>>{

fn cellContent(&self) -> Option<Rc<CellContentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token PIPE
/// Returns `None` if there is no child corresponding to token PIPE
fn PIPE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(PIPE, 0)
}

}

impl<'input> CellContextAttrs<'input> for CellContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn cell(&mut self,)
	-> Result<Rc<CellContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CellContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 104, RULE_cell);
        let mut _localctx: Rc<CellContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule cellContent*/
			recog.base.set_state(529);
			recog.cellContent()?;

			recog.base.set_state(530);
			recog.base.match_token(PIPE,&mut recog.err_handler)?;

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
//------------------- cellContent ----------------
pub type CellContentContextAll<'input> = CellContentContext<'input>;


pub type CellContentContext<'input> = BaseParserRuleContext<'input,CellContentContextExt<'input>>;

#[derive(Clone)]
pub struct CellContentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for CellContentContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for CellContentContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_cellContent(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_cellContent(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for CellContentContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_cellContent(self);
	}
}

impl<'input> CustomRuleContext<'input> for CellContentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cellContent }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cellContent }
}
antlr_rust::tid!{CellContentContextExt<'a>}

impl<'input> CellContentContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CellContentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CellContentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CellContentContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<CellContentContextExt<'input>>{

fn condition(&self) -> Option<Rc<ConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn action(&self) -> Option<Rc<ActionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CellContentContextAttrs<'input> for CellContentContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn cellContent(&mut self,)
	-> Result<Rc<CellContentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CellContentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 106, RULE_cellContent);
        let mut _localctx: Rc<CellContentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(534);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(32,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule condition*/
					recog.base.set_state(532);
					recog.condition()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule action*/
					recog.base.set_state(533);
					recog.action()?;

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
//------------------- condition ----------------
pub type ConditionContextAll<'input> = ConditionContext<'input>;


pub type ConditionContext<'input> = BaseParserRuleContext<'input,ConditionContextExt<'input>>;

#[derive(Clone)]
pub struct ConditionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ConditionContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ConditionContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_condition(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_condition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ConditionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_condition(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConditionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_condition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_condition }
}
antlr_rust::tid!{ConditionContextExt<'a>}

impl<'input> ConditionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConditionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConditionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConditionContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ConditionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STAR
/// Returns `None` if there is no child corresponding to token STAR
fn STAR(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(STAR, 0)
}
fn exactMatch(&self) -> Option<Rc<ExactMatchContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn rangeCondition(&self) -> Option<Rc<RangeConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn setCondition(&self) -> Option<Rc<SetConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn patternCondition(&self) -> Option<Rc<PatternConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn nullCondition(&self) -> Option<Rc<NullConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn comparisonCondition(&self) -> Option<Rc<ComparisonConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expressionCondition(&self) -> Option<Rc<ExpressionConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn markerStateCondition(&self) -> Option<Rc<MarkerStateConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ConditionContextAttrs<'input> for ConditionContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn condition(&mut self,)
	-> Result<Rc<ConditionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 108, RULE_condition);
        let mut _localctx: Rc<ConditionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(545);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(33,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(536);
					recog.base.match_token(STAR,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule exactMatch*/
					recog.base.set_state(537);
					recog.exactMatch()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule rangeCondition*/
					recog.base.set_state(538);
					recog.rangeCondition()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule setCondition*/
					recog.base.set_state(539);
					recog.setCondition()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule patternCondition*/
					recog.base.set_state(540);
					recog.patternCondition()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule nullCondition*/
					recog.base.set_state(541);
					recog.nullCondition()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule comparisonCondition*/
					recog.base.set_state(542);
					recog.comparisonCondition()?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule expressionCondition*/
					recog.base.set_state(543);
					recog.expressionCondition()?;

					}
				}
			,
				9 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					/*InvokeRule markerStateCondition*/
					recog.base.set_state(544);
					recog.markerStateCondition()?;

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
//------------------- exactMatch ----------------
pub type ExactMatchContextAll<'input> = ExactMatchContext<'input>;


pub type ExactMatchContext<'input> = BaseParserRuleContext<'input,ExactMatchContextExt<'input>>;

#[derive(Clone)]
pub struct ExactMatchContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ExactMatchContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ExactMatchContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_exactMatch(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_exactMatch(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ExactMatchContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_exactMatch(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExactMatchContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exactMatch }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exactMatch }
}
antlr_rust::tid!{ExactMatchContextExt<'a>}

impl<'input> ExactMatchContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExactMatchContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExactMatchContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExactMatchContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ExactMatchContextExt<'input>>{

fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExactMatchContextAttrs<'input> for ExactMatchContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn exactMatch(&mut self,)
	-> Result<Rc<ExactMatchContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExactMatchContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 110, RULE_exactMatch);
        let mut _localctx: Rc<ExactMatchContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule literal*/
			recog.base.set_state(547);
			recog.literal()?;

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
//------------------- rangeCondition ----------------
pub type RangeConditionContextAll<'input> = RangeConditionContext<'input>;


pub type RangeConditionContext<'input> = BaseParserRuleContext<'input,RangeConditionContextExt<'input>>;

#[derive(Clone)]
pub struct RangeConditionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for RangeConditionContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for RangeConditionContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_rangeCondition(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_rangeCondition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for RangeConditionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_rangeCondition(self);
	}
}

impl<'input> CustomRuleContext<'input> for RangeConditionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_rangeCondition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_rangeCondition }
}
antlr_rust::tid!{RangeConditionContextExt<'a>}

impl<'input> RangeConditionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RangeConditionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RangeConditionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RangeConditionContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<RangeConditionContextExt<'input>>{

fn numberLiteral_all(&self) ->  Vec<Rc<NumberLiteralContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn numberLiteral(&self, i: usize) -> Option<Rc<NumberLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token TO
/// Returns `None` if there is no child corresponding to token TO
fn TO(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(TO, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token MONEY_LITERAL in current rule
fn MONEY_LITERAL_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token MONEY_LITERAL, starting from 0.
/// Returns `None` if number of children corresponding to token MONEY_LITERAL is less or equal than `i`.
fn MONEY_LITERAL(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(MONEY_LITERAL, i)
}

}

impl<'input> RangeConditionContextAttrs<'input> for RangeConditionContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn rangeCondition(&mut self,)
	-> Result<Rc<RangeConditionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RangeConditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 112, RULE_rangeCondition);
        let mut _localctx: Rc<RangeConditionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(556);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 MINUS | INTEGER | DECIMAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule numberLiteral*/
					recog.base.set_state(549);
					recog.numberLiteral()?;

					recog.base.set_state(550);
					recog.base.match_token(TO,&mut recog.err_handler)?;

					/*InvokeRule numberLiteral*/
					recog.base.set_state(551);
					recog.numberLiteral()?;

					}
				}

			 MONEY_LITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(553);
					recog.base.match_token(MONEY_LITERAL,&mut recog.err_handler)?;

					recog.base.set_state(554);
					recog.base.match_token(TO,&mut recog.err_handler)?;

					recog.base.set_state(555);
					recog.base.match_token(MONEY_LITERAL,&mut recog.err_handler)?;

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
//------------------- setCondition ----------------
pub type SetConditionContextAll<'input> = SetConditionContext<'input>;


pub type SetConditionContext<'input> = BaseParserRuleContext<'input,SetConditionContextExt<'input>>;

#[derive(Clone)]
pub struct SetConditionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for SetConditionContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for SetConditionContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_setCondition(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_setCondition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for SetConditionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_setCondition(self);
	}
}

impl<'input> CustomRuleContext<'input> for SetConditionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_setCondition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_setCondition }
}
antlr_rust::tid!{SetConditionContextExt<'a>}

impl<'input> SetConditionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SetConditionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SetConditionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SetConditionContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<SetConditionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IN, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn valueList(&self) -> Option<Rc<ValueListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token NOT
/// Returns `None` if there is no child corresponding to token NOT
fn NOT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(NOT, 0)
}

}

impl<'input> SetConditionContextAttrs<'input> for SetConditionContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn setCondition(&mut self,)
	-> Result<Rc<SetConditionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SetConditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 114, RULE_setCondition);
        let mut _localctx: Rc<SetConditionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(569);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(558);
					recog.base.match_token(IN,&mut recog.err_handler)?;

					recog.base.set_state(559);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule valueList*/
					recog.base.set_state(560);
					recog.valueList()?;

					recog.base.set_state(561);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}

			 NOT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(563);
					recog.base.match_token(NOT,&mut recog.err_handler)?;

					recog.base.set_state(564);
					recog.base.match_token(IN,&mut recog.err_handler)?;

					recog.base.set_state(565);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule valueList*/
					recog.base.set_state(566);
					recog.valueList()?;

					recog.base.set_state(567);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

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
//------------------- patternCondition ----------------
pub type PatternConditionContextAll<'input> = PatternConditionContext<'input>;


pub type PatternConditionContext<'input> = BaseParserRuleContext<'input,PatternConditionContextExt<'input>>;

#[derive(Clone)]
pub struct PatternConditionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for PatternConditionContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for PatternConditionContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_patternCondition(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_patternCondition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for PatternConditionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_patternCondition(self);
	}
}

impl<'input> CustomRuleContext<'input> for PatternConditionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternCondition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternCondition }
}
antlr_rust::tid!{PatternConditionContextExt<'a>}

impl<'input> PatternConditionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PatternConditionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PatternConditionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PatternConditionContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<PatternConditionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token MATCHES
/// Returns `None` if there is no child corresponding to token MATCHES
fn MATCHES(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(MATCHES, 0)
}
fn stringLiteral(&self) -> Option<Rc<StringLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ENDS_WITH
/// Returns `None` if there is no child corresponding to token ENDS_WITH
fn ENDS_WITH(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(ENDS_WITH, 0)
}
/// Retrieves first TerminalNode corresponding to token STARTS_WITH
/// Returns `None` if there is no child corresponding to token STARTS_WITH
fn STARTS_WITH(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(STARTS_WITH, 0)
}
/// Retrieves first TerminalNode corresponding to token CONTAINS
/// Returns `None` if there is no child corresponding to token CONTAINS
fn CONTAINS(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(CONTAINS, 0)
}

}

impl<'input> PatternConditionContextAttrs<'input> for PatternConditionContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn patternCondition(&mut self,)
	-> Result<Rc<PatternConditionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternConditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_patternCondition);
        let mut _localctx: Rc<PatternConditionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(579);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 MATCHES 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(571);
					recog.base.match_token(MATCHES,&mut recog.err_handler)?;

					/*InvokeRule stringLiteral*/
					recog.base.set_state(572);
					recog.stringLiteral()?;

					}
				}

			 ENDS_WITH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(573);
					recog.base.match_token(ENDS_WITH,&mut recog.err_handler)?;

					/*InvokeRule stringLiteral*/
					recog.base.set_state(574);
					recog.stringLiteral()?;

					}
				}

			 STARTS_WITH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(575);
					recog.base.match_token(STARTS_WITH,&mut recog.err_handler)?;

					/*InvokeRule stringLiteral*/
					recog.base.set_state(576);
					recog.stringLiteral()?;

					}
				}

			 CONTAINS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(577);
					recog.base.match_token(CONTAINS,&mut recog.err_handler)?;

					/*InvokeRule stringLiteral*/
					recog.base.set_state(578);
					recog.stringLiteral()?;

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
//------------------- nullCondition ----------------
pub type NullConditionContextAll<'input> = NullConditionContext<'input>;


pub type NullConditionContext<'input> = BaseParserRuleContext<'input,NullConditionContextExt<'input>>;

#[derive(Clone)]
pub struct NullConditionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for NullConditionContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for NullConditionContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_nullCondition(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_nullCondition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for NullConditionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_nullCondition(self);
	}
}

impl<'input> CustomRuleContext<'input> for NullConditionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nullCondition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nullCondition }
}
antlr_rust::tid!{NullConditionContextExt<'a>}

impl<'input> NullConditionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NullConditionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NullConditionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NullConditionContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<NullConditionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IS
/// Returns `None` if there is no child corresponding to token IS
fn IS(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IS, 0)
}
/// Retrieves first TerminalNode corresponding to token NULL
/// Returns `None` if there is no child corresponding to token NULL
fn NULL(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(NULL, 0)
}
/// Retrieves first TerminalNode corresponding to token NOT
/// Returns `None` if there is no child corresponding to token NOT
fn NOT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(NOT, 0)
}
/// Retrieves first TerminalNode corresponding to token IS_NULL
/// Returns `None` if there is no child corresponding to token IS_NULL
fn IS_NULL(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IS_NULL, 0)
}
/// Retrieves first TerminalNode corresponding to token IS_NOT_NULL
/// Returns `None` if there is no child corresponding to token IS_NOT_NULL
fn IS_NOT_NULL(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IS_NOT_NULL, 0)
}

}

impl<'input> NullConditionContextAttrs<'input> for NullConditionContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn nullCondition(&mut self,)
	-> Result<Rc<NullConditionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NullConditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_nullCondition);
        let mut _localctx: Rc<NullConditionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(588);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(37,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(581);
					recog.base.match_token(IS,&mut recog.err_handler)?;

					recog.base.set_state(582);
					recog.base.match_token(NULL,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(583);
					recog.base.match_token(IS,&mut recog.err_handler)?;

					recog.base.set_state(584);
					recog.base.match_token(NOT,&mut recog.err_handler)?;

					recog.base.set_state(585);
					recog.base.match_token(NULL,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(586);
					recog.base.match_token(IS_NULL,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(587);
					recog.base.match_token(IS_NOT_NULL,&mut recog.err_handler)?;

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
//------------------- comparisonCondition ----------------
pub type ComparisonConditionContextAll<'input> = ComparisonConditionContext<'input>;


pub type ComparisonConditionContext<'input> = BaseParserRuleContext<'input,ComparisonConditionContextExt<'input>>;

#[derive(Clone)]
pub struct ComparisonConditionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ComparisonConditionContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ComparisonConditionContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_comparisonCondition(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_comparisonCondition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ComparisonConditionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_comparisonCondition(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComparisonConditionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comparisonCondition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comparisonCondition }
}
antlr_rust::tid!{ComparisonConditionContextExt<'a>}

impl<'input> ComparisonConditionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComparisonConditionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComparisonConditionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComparisonConditionContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ComparisonConditionContextExt<'input>>{

fn comparisonOp(&self) -> Option<Rc<ComparisonOpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn valueExpr(&self) -> Option<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ComparisonConditionContextAttrs<'input> for ComparisonConditionContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn comparisonCondition(&mut self,)
	-> Result<Rc<ComparisonConditionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComparisonConditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 120, RULE_comparisonCondition);
        let mut _localctx: Rc<ComparisonConditionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule comparisonOp*/
			recog.base.set_state(590);
			recog.comparisonOp()?;

			/*InvokeRule valueExpr*/
			recog.base.set_state(591);
			recog.valueExpr()?;

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
//------------------- expressionCondition ----------------
pub type ExpressionConditionContextAll<'input> = ExpressionConditionContext<'input>;


pub type ExpressionConditionContext<'input> = BaseParserRuleContext<'input,ExpressionConditionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionConditionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ExpressionConditionContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ExpressionConditionContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expressionCondition(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_expressionCondition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ExpressionConditionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_expressionCondition(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionConditionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expressionCondition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expressionCondition }
}
antlr_rust::tid!{ExpressionConditionContextExt<'a>}

impl<'input> ExpressionConditionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionConditionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionConditionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionConditionContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ExpressionConditionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn booleanExpr(&self) -> Option<Rc<BooleanExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> ExpressionConditionContextAttrs<'input> for ExpressionConditionContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expressionCondition(&mut self,)
	-> Result<Rc<ExpressionConditionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionConditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_expressionCondition);
        let mut _localctx: Rc<ExpressionConditionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(593);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule booleanExpr*/
			recog.base.set_state(594);
			recog.booleanExpr()?;

			recog.base.set_state(595);
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
//------------------- markerStateCondition ----------------
pub type MarkerStateConditionContextAll<'input> = MarkerStateConditionContext<'input>;


pub type MarkerStateConditionContext<'input> = BaseParserRuleContext<'input,MarkerStateConditionContextExt<'input>>;

#[derive(Clone)]
pub struct MarkerStateConditionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for MarkerStateConditionContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for MarkerStateConditionContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_markerStateCondition(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_markerStateCondition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for MarkerStateConditionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_markerStateCondition(self);
	}
}

impl<'input> CustomRuleContext<'input> for MarkerStateConditionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_markerStateCondition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_markerStateCondition }
}
antlr_rust::tid!{MarkerStateConditionContextExt<'a>}

impl<'input> MarkerStateConditionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MarkerStateConditionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MarkerStateConditionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MarkerStateConditionContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<MarkerStateConditionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token MARKER
/// Returns `None` if there is no child corresponding to token MARKER
fn MARKER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(MARKER, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves first TerminalNode corresponding to token FIRED
/// Returns `None` if there is no child corresponding to token FIRED
fn FIRED(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(FIRED, 0)
}
/// Retrieves first TerminalNode corresponding to token PENDING
/// Returns `None` if there is no child corresponding to token PENDING
fn PENDING(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(PENDING, 0)
}
/// Retrieves first TerminalNode corresponding to token BETWEEN_MARKERS
/// Returns `None` if there is no child corresponding to token BETWEEN_MARKERS
fn BETWEEN_MARKERS(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(BETWEEN_MARKERS, 0)
}
/// Retrieves first TerminalNode corresponding to token AND
/// Returns `None` if there is no child corresponding to token AND
fn AND(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(AND, 0)
}

}

impl<'input> MarkerStateConditionContextAttrs<'input> for MarkerStateConditionContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn markerStateCondition(&mut self,)
	-> Result<Rc<MarkerStateConditionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MarkerStateConditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 124, RULE_markerStateCondition);
        let mut _localctx: Rc<MarkerStateConditionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(607);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(38,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(597);
					recog.base.match_token(MARKER,&mut recog.err_handler)?;

					recog.base.set_state(598);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(599);
					recog.base.match_token(FIRED,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(600);
					recog.base.match_token(MARKER,&mut recog.err_handler)?;

					recog.base.set_state(601);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(602);
					recog.base.match_token(PENDING,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(603);
					recog.base.match_token(BETWEEN_MARKERS,&mut recog.err_handler)?;

					recog.base.set_state(604);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(605);
					recog.base.match_token(AND,&mut recog.err_handler)?;

					recog.base.set_state(606);
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
//------------------- action ----------------
pub type ActionContextAll<'input> = ActionContext<'input>;


pub type ActionContext<'input> = BaseParserRuleContext<'input,ActionContextExt<'input>>;

#[derive(Clone)]
pub struct ActionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ActionContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ActionContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_action(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_action(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ActionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_action(self);
	}
}

impl<'input> CustomRuleContext<'input> for ActionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_action }
	//fn type_rule_index() -> usize where Self: Sized { RULE_action }
}
antlr_rust::tid!{ActionContextExt<'a>}

impl<'input> ActionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ActionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ActionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ActionContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ActionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STAR
/// Returns `None` if there is no child corresponding to token STAR
fn STAR(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(STAR, 0)
}
fn assignAction(&self) -> Option<Rc<AssignActionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn calculateAction(&self) -> Option<Rc<CalculateActionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lookupAction(&self) -> Option<Rc<LookupActionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn callAction(&self) -> Option<Rc<CallActionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn emitAction(&self) -> Option<Rc<EmitActionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ActionContextAttrs<'input> for ActionContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn action(&mut self,)
	-> Result<Rc<ActionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ActionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_action);
        let mut _localctx: Rc<ActionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(615);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(39,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(609);
					recog.base.match_token(STAR,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule assignAction*/
					recog.base.set_state(610);
					recog.assignAction()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule calculateAction*/
					recog.base.set_state(611);
					recog.calculateAction()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule lookupAction*/
					recog.base.set_state(612);
					recog.lookupAction()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule callAction*/
					recog.base.set_state(613);
					recog.callAction()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule emitAction*/
					recog.base.set_state(614);
					recog.emitAction()?;

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
//------------------- assignAction ----------------
pub type AssignActionContextAll<'input> = AssignActionContext<'input>;


pub type AssignActionContext<'input> = BaseParserRuleContext<'input,AssignActionContextExt<'input>>;

#[derive(Clone)]
pub struct AssignActionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for AssignActionContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for AssignActionContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_assignAction(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_assignAction(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for AssignActionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_assignAction(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssignActionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assignAction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assignAction }
}
antlr_rust::tid!{AssignActionContextExt<'a>}

impl<'input> AssignActionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssignActionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssignActionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssignActionContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<AssignActionContextExt<'input>>{

fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AssignActionContextAttrs<'input> for AssignActionContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assignAction(&mut self,)
	-> Result<Rc<AssignActionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssignActionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 128, RULE_assignAction);
        let mut _localctx: Rc<AssignActionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule literal*/
			recog.base.set_state(617);
			recog.literal()?;

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
//------------------- calculateAction ----------------
pub type CalculateActionContextAll<'input> = CalculateActionContext<'input>;


pub type CalculateActionContext<'input> = BaseParserRuleContext<'input,CalculateActionContextExt<'input>>;

#[derive(Clone)]
pub struct CalculateActionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for CalculateActionContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for CalculateActionContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_calculateAction(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_calculateAction(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for CalculateActionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_calculateAction(self);
	}
}

impl<'input> CustomRuleContext<'input> for CalculateActionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_calculateAction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_calculateAction }
}
antlr_rust::tid!{CalculateActionContextExt<'a>}

impl<'input> CalculateActionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CalculateActionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CalculateActionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CalculateActionContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<CalculateActionContextExt<'input>>{

fn arithmeticExpr(&self) -> Option<Rc<ArithmeticExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CalculateActionContextAttrs<'input> for CalculateActionContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn calculateAction(&mut self,)
	-> Result<Rc<CalculateActionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CalculateActionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 130, RULE_calculateAction);
        let mut _localctx: Rc<CalculateActionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule arithmeticExpr*/
			recog.base.set_state(619);
			recog.arithmeticExpr()?;

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
//------------------- lookupAction ----------------
pub type LookupActionContextAll<'input> = LookupActionContext<'input>;


pub type LookupActionContext<'input> = BaseParserRuleContext<'input,LookupActionContextExt<'input>>;

#[derive(Clone)]
pub struct LookupActionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for LookupActionContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for LookupActionContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lookupAction(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_lookupAction(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for LookupActionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_lookupAction(self);
	}
}

impl<'input> CustomRuleContext<'input> for LookupActionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lookupAction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lookupAction }
}
antlr_rust::tid!{LookupActionContextExt<'a>}

impl<'input> LookupActionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LookupActionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LookupActionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LookupActionContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<LookupActionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LOOKUP
/// Returns `None` if there is no child corresponding to token LOOKUP
fn LOOKUP(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LOOKUP, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}
fn valueExpr_all(&self) ->  Vec<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn valueExpr(&self, i: usize) -> Option<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token DEFAULT
/// Returns `None` if there is no child corresponding to token DEFAULT
fn DEFAULT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DEFAULT, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token AS_OF
/// Returns `None` if there is no child corresponding to token AS_OF
fn AS_OF(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(AS_OF, 0)
}

}

impl<'input> LookupActionContextAttrs<'input> for LookupActionContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lookupAction(&mut self,)
	-> Result<Rc<LookupActionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LookupActionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 132, RULE_lookupAction);
        let mut _localctx: Rc<LookupActionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			recog.base.set_state(647);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(42,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(621);
					recog.base.match_token(LOOKUP,&mut recog.err_handler)?;

					recog.base.set_state(622);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					recog.base.set_state(623);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(628);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(40,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(624);
							recog.base.match_token(COMMA,&mut recog.err_handler)?;

							/*InvokeRule valueExpr*/
							recog.base.set_state(625);
							recog.valueExpr()?;

							}
							} 
						}
						recog.base.set_state(630);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(40,&mut recog.base)?;
					}
					recog.base.set_state(635);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==COMMA {
						{
						recog.base.set_state(631);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						recog.base.set_state(632);
						recog.base.match_token(DEFAULT,&mut recog.err_handler)?;

						recog.base.set_state(633);
						recog.base.match_token(COLON,&mut recog.err_handler)?;

						/*InvokeRule valueExpr*/
						recog.base.set_state(634);
						recog.valueExpr()?;

						}
					}

					recog.base.set_state(637);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(638);
					recog.base.match_token(LOOKUP,&mut recog.err_handler)?;

					recog.base.set_state(639);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					recog.base.set_state(640);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(641);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					recog.base.set_state(642);
					recog.base.match_token(AS_OF,&mut recog.err_handler)?;

					recog.base.set_state(643);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule valueExpr*/
					recog.base.set_state(644);
					recog.valueExpr()?;

					recog.base.set_state(645);
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
//------------------- callAction ----------------
pub type CallActionContextAll<'input> = CallActionContext<'input>;


pub type CallActionContext<'input> = BaseParserRuleContext<'input,CallActionContextExt<'input>>;

#[derive(Clone)]
pub struct CallActionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for CallActionContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for CallActionContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_callAction(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_callAction(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for CallActionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_callAction(self);
	}
}

impl<'input> CustomRuleContext<'input> for CallActionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_callAction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_callAction }
}
antlr_rust::tid!{CallActionContextExt<'a>}

impl<'input> CallActionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CallActionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CallActionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CallActionContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<CallActionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn actionArg_all(&self) ->  Vec<Rc<ActionArgContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn actionArg(&self, i: usize) -> Option<Rc<ActionArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> CallActionContextAttrs<'input> for CallActionContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn callAction(&mut self,)
	-> Result<Rc<CallActionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CallActionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 134, RULE_callAction);
        let mut _localctx: Rc<CallActionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(649);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(650);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			recog.base.set_state(659);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 34)) & !0x3f) == 0 && ((1usize << (_la - 34)) & ((1usize << (NULL - 34)) | (1usize << (ANY - 34)) | (1usize << (ALL - 34)) | (1usize << (NONE - 34)) | (1usize << (SUM - 34)) | (1usize << (COUNT - 34)) | (1usize << (AVG - 34)) | (1usize << (MAX_FN - 34)) | (1usize << (MIN_FN - 34)) | (1usize << (FILTER - 34)))) != 0) || ((((_la - 66)) & !0x3f) == 0 && ((1usize << (_la - 66)) & ((1usize << (FIND - 66)) | (1usize << (DISTINCT - 66)) | (1usize << (MINUS - 66)))) != 0) || ((((_la - 107)) & !0x3f) == 0 && ((1usize << (_la - 107)) & ((1usize << (LPAREN - 107)) | (1usize << (LBRACKET - 107)) | (1usize << (LBRACE - 107)) | (1usize << (BOOLEAN - 107)) | (1usize << (INTEGER - 107)) | (1usize << (DECIMAL - 107)) | (1usize << (MONEY_LITERAL - 107)) | (1usize << (PERCENTAGE_LITERAL - 107)) | (1usize << (DQUOTED_STRING - 107)) | (1usize << (SQUOTED_STRING - 107)) | (1usize << (IDENTIFIER - 107)))) != 0) {
				{
				/*InvokeRule actionArg*/
				recog.base.set_state(651);
				recog.actionArg()?;

				recog.base.set_state(656);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==COMMA {
					{
					{
					recog.base.set_state(652);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule actionArg*/
					recog.base.set_state(653);
					recog.actionArg()?;

					}
					}
					recog.base.set_state(658);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(661);
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
//------------------- actionArg ----------------
pub type ActionArgContextAll<'input> = ActionArgContext<'input>;


pub type ActionArgContext<'input> = BaseParserRuleContext<'input,ActionArgContextExt<'input>>;

#[derive(Clone)]
pub struct ActionArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ActionArgContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ActionArgContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_actionArg(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_actionArg(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ActionArgContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_actionArg(self);
	}
}

impl<'input> CustomRuleContext<'input> for ActionArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_actionArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_actionArg }
}
antlr_rust::tid!{ActionArgContextExt<'a>}

impl<'input> ActionArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ActionArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ActionArgContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ActionArgContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ActionArgContextExt<'input>>{

fn valueExpr(&self) -> Option<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}

}

impl<'input> ActionArgContextAttrs<'input> for ActionArgContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn actionArg(&mut self,)
	-> Result<Rc<ActionArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ActionArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 136, RULE_actionArg);
        let mut _localctx: Rc<ActionArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(667);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(45,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule valueExpr*/
					recog.base.set_state(663);
					recog.valueExpr()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(664);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(665);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule valueExpr*/
					recog.base.set_state(666);
					recog.valueExpr()?;

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
//------------------- emitAction ----------------
pub type EmitActionContextAll<'input> = EmitActionContext<'input>;


pub type EmitActionContext<'input> = BaseParserRuleContext<'input,EmitActionContextExt<'input>>;

#[derive(Clone)]
pub struct EmitActionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for EmitActionContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for EmitActionContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_emitAction(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_emitAction(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for EmitActionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_emitAction(self);
	}
}

impl<'input> CustomRuleContext<'input> for EmitActionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_emitAction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_emitAction }
}
antlr_rust::tid!{EmitActionContextExt<'a>}

impl<'input> EmitActionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EmitActionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EmitActionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EmitActionContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<EmitActionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EMIT
/// Returns `None` if there is no child corresponding to token EMIT
fn EMIT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(EMIT, 0)
}
/// Retrieves first TerminalNode corresponding to token TO
/// Returns `None` if there is no child corresponding to token TO
fn TO(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(TO, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> EmitActionContextAttrs<'input> for EmitActionContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn emitAction(&mut self,)
	-> Result<Rc<EmitActionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EmitActionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 138, RULE_emitAction);
        let mut _localctx: Rc<EmitActionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(669);
			recog.base.match_token(EMIT,&mut recog.err_handler)?;

			recog.base.set_state(670);
			recog.base.match_token(TO,&mut recog.err_handler)?;

			recog.base.set_state(671);
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
//------------------- returnSpec ----------------
pub type ReturnSpecContextAll<'input> = ReturnSpecContext<'input>;


pub type ReturnSpecContext<'input> = BaseParserRuleContext<'input,ReturnSpecContextExt<'input>>;

#[derive(Clone)]
pub struct ReturnSpecContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ReturnSpecContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ReturnSpecContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_returnSpec(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_returnSpec(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ReturnSpecContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_returnSpec(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReturnSpecContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_returnSpec }
	//fn type_rule_index() -> usize where Self: Sized { RULE_returnSpec }
}
antlr_rust::tid!{ReturnSpecContextExt<'a>}

impl<'input> ReturnSpecContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReturnSpecContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReturnSpecContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReturnSpecContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ReturnSpecContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token RETURN
/// Returns `None` if there is no child corresponding to token RETURN
fn RETURN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RETURN, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn returnParam_all(&self) ->  Vec<Rc<ReturnParamContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn returnParam(&self, i: usize) -> Option<Rc<ReturnParamContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ReturnSpecContextAttrs<'input> for ReturnSpecContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn returnSpec(&mut self,)
	-> Result<Rc<ReturnSpecContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReturnSpecContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 140, RULE_returnSpec);
        let mut _localctx: Rc<ReturnSpecContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(673);
			recog.base.match_token(RETURN,&mut recog.err_handler)?;

			recog.base.set_state(674);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(676); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule returnParam*/
				recog.base.set_state(675);
				recog.returnParam()?;

				}
				}
				recog.base.set_state(678); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(((((_la - 72)) & !0x3f) == 0 && ((1usize << (_la - 72)) & ((1usize << (TEXT_TYPE - 72)) | (1usize << (DESCRIPTION - 72)) | (1usize << (PRIORITY - 72)) | (1usize << (VERSION - 72)))) != 0) || _la==IDENTIFIER) {break}
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
//------------------- returnParam ----------------
pub type ReturnParamContextAll<'input> = ReturnParamContext<'input>;


pub type ReturnParamContext<'input> = BaseParserRuleContext<'input,ReturnParamContextExt<'input>>;

#[derive(Clone)]
pub struct ReturnParamContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ReturnParamContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ReturnParamContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_returnParam(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_returnParam(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ReturnParamContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_returnParam(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReturnParamContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_returnParam }
	//fn type_rule_index() -> usize where Self: Sized { RULE_returnParam }
}
antlr_rust::tid!{ReturnParamContextExt<'a>}

impl<'input> ReturnParamContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReturnParamContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReturnParamContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReturnParamContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ReturnParamContextExt<'input>>{

fn paramName(&self) -> Option<Rc<ParamNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn paramType(&self) -> Option<Rc<ParamTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ReturnParamContextAttrs<'input> for ReturnParamContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn returnParam(&mut self,)
	-> Result<Rc<ReturnParamContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReturnParamContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 142, RULE_returnParam);
        let mut _localctx: Rc<ReturnParamContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule paramName*/
			recog.base.set_state(680);
			recog.paramName()?;

			recog.base.set_state(681);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule paramType*/
			recog.base.set_state(682);
			recog.paramType()?;

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
//------------------- executeSpec ----------------
pub type ExecuteSpecContextAll<'input> = ExecuteSpecContext<'input>;


pub type ExecuteSpecContext<'input> = BaseParserRuleContext<'input,ExecuteSpecContextExt<'input>>;

#[derive(Clone)]
pub struct ExecuteSpecContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ExecuteSpecContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ExecuteSpecContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_executeSpec(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_executeSpec(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ExecuteSpecContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_executeSpec(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExecuteSpecContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_executeSpec }
	//fn type_rule_index() -> usize where Self: Sized { RULE_executeSpec }
}
antlr_rust::tid!{ExecuteSpecContextExt<'a>}

impl<'input> ExecuteSpecContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExecuteSpecContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExecuteSpecContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExecuteSpecContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ExecuteSpecContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EXECUTE
/// Returns `None` if there is no child corresponding to token EXECUTE
fn EXECUTE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(EXECUTE, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn executeType(&self) -> Option<Rc<ExecuteTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExecuteSpecContextAttrs<'input> for ExecuteSpecContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn executeSpec(&mut self,)
	-> Result<Rc<ExecuteSpecContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExecuteSpecContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 144, RULE_executeSpec);
        let mut _localctx: Rc<ExecuteSpecContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(684);
			recog.base.match_token(EXECUTE,&mut recog.err_handler)?;

			recog.base.set_state(685);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule executeType*/
			recog.base.set_state(686);
			recog.executeType()?;

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
//------------------- executeType ----------------
pub type ExecuteTypeContextAll<'input> = ExecuteTypeContext<'input>;


pub type ExecuteTypeContext<'input> = BaseParserRuleContext<'input,ExecuteTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ExecuteTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ExecuteTypeContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ExecuteTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_executeType(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_executeType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ExecuteTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_executeType(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExecuteTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_executeType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_executeType }
}
antlr_rust::tid!{ExecuteTypeContextExt<'a>}

impl<'input> ExecuteTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExecuteTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExecuteTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExecuteTypeContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ExecuteTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token YES
/// Returns `None` if there is no child corresponding to token YES
fn YES(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(YES, 0)
}
/// Retrieves first TerminalNode corresponding to token MULTI
/// Returns `None` if there is no child corresponding to token MULTI
fn MULTI(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(MULTI, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> ExecuteTypeContextAttrs<'input> for ExecuteTypeContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn executeType(&mut self,)
	-> Result<Rc<ExecuteTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExecuteTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 146, RULE_executeType);
        let mut _localctx: Rc<ExecuteTypeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(688);
			_la = recog.base.input.la(1);
			if { !(_la==YES || _la==MULTI || _la==IDENTIFIER) } {
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
//------------------- hybridSpec ----------------
pub type HybridSpecContextAll<'input> = HybridSpecContext<'input>;


pub type HybridSpecContext<'input> = BaseParserRuleContext<'input,HybridSpecContextExt<'input>>;

#[derive(Clone)]
pub struct HybridSpecContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for HybridSpecContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for HybridSpecContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_hybridSpec(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_hybridSpec(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for HybridSpecContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_hybridSpec(self);
	}
}

impl<'input> CustomRuleContext<'input> for HybridSpecContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hybridSpec }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hybridSpec }
}
antlr_rust::tid!{HybridSpecContextExt<'a>}

impl<'input> HybridSpecContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<HybridSpecContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HybridSpecContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait HybridSpecContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<HybridSpecContextExt<'input>>{

fn returnSpec(&self) -> Option<Rc<ReturnSpecContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn executeSpec(&self) -> Option<Rc<ExecuteSpecContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> HybridSpecContextAttrs<'input> for HybridSpecContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn hybridSpec(&mut self,)
	-> Result<Rc<HybridSpecContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HybridSpecContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 148, RULE_hybridSpec);
        let mut _localctx: Rc<HybridSpecContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule returnSpec*/
			recog.base.set_state(690);
			recog.returnSpec()?;

			/*InvokeRule executeSpec*/
			recog.base.set_state(691);
			recog.executeSpec()?;

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
//------------------- postCalculateBlock ----------------
pub type PostCalculateBlockContextAll<'input> = PostCalculateBlockContext<'input>;


pub type PostCalculateBlockContext<'input> = BaseParserRuleContext<'input,PostCalculateBlockContextExt<'input>>;

#[derive(Clone)]
pub struct PostCalculateBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for PostCalculateBlockContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for PostCalculateBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_postCalculateBlock(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_postCalculateBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for PostCalculateBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_postCalculateBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for PostCalculateBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_postCalculateBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_postCalculateBlock }
}
antlr_rust::tid!{PostCalculateBlockContextExt<'a>}

impl<'input> PostCalculateBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PostCalculateBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PostCalculateBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PostCalculateBlockContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<PostCalculateBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token POST_CALCULATE
/// Returns `None` if there is no child corresponding to token POST_CALCULATE
fn POST_CALCULATE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(POST_CALCULATE, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn postCalculateStatement_all(&self) ->  Vec<Rc<PostCalculateStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn postCalculateStatement(&self, i: usize) -> Option<Rc<PostCalculateStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> PostCalculateBlockContextAttrs<'input> for PostCalculateBlockContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn postCalculateBlock(&mut self,)
	-> Result<Rc<PostCalculateBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PostCalculateBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 150, RULE_postCalculateBlock);
        let mut _localctx: Rc<PostCalculateBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(693);
			recog.base.match_token(POST_CALCULATE,&mut recog.err_handler)?;

			recog.base.set_state(694);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(696); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule postCalculateStatement*/
				recog.base.set_state(695);
				recog.postCalculateStatement()?;

				}
				}
				recog.base.set_state(698); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==LET || _la==IDENTIFIER) {break}
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
//------------------- postCalculateStatement ----------------
pub type PostCalculateStatementContextAll<'input> = PostCalculateStatementContext<'input>;


pub type PostCalculateStatementContext<'input> = BaseParserRuleContext<'input,PostCalculateStatementContextExt<'input>>;

#[derive(Clone)]
pub struct PostCalculateStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for PostCalculateStatementContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for PostCalculateStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_postCalculateStatement(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_postCalculateStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for PostCalculateStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_postCalculateStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for PostCalculateStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_postCalculateStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_postCalculateStatement }
}
antlr_rust::tid!{PostCalculateStatementContextExt<'a>}

impl<'input> PostCalculateStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PostCalculateStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PostCalculateStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PostCalculateStatementContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<PostCalculateStatementContextExt<'input>>{

fn letStatement(&self) -> Option<Rc<LetStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assignmentStatement(&self) -> Option<Rc<AssignmentStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PostCalculateStatementContextAttrs<'input> for PostCalculateStatementContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn postCalculateStatement(&mut self,)
	-> Result<Rc<PostCalculateStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PostCalculateStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 152, RULE_postCalculateStatement);
        let mut _localctx: Rc<PostCalculateStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(702);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LET 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule letStatement*/
					recog.base.set_state(700);
					recog.letStatement()?;

					}
				}

			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule assignmentStatement*/
					recog.base.set_state(701);
					recog.assignmentStatement()?;

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
//------------------- assignmentStatement ----------------
pub type AssignmentStatementContextAll<'input> = AssignmentStatementContext<'input>;


pub type AssignmentStatementContext<'input> = BaseParserRuleContext<'input,AssignmentStatementContextExt<'input>>;

#[derive(Clone)]
pub struct AssignmentStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for AssignmentStatementContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for AssignmentStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_assignmentStatement(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_assignmentStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for AssignmentStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_assignmentStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssignmentStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assignmentStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assignmentStatement }
}
antlr_rust::tid!{AssignmentStatementContextExt<'a>}

impl<'input> AssignmentStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssignmentStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssignmentStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssignmentStatementContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<AssignmentStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token EQ
/// Returns `None` if there is no child corresponding to token EQ
fn EQ(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(EQ, 0)
}
fn valueExpr(&self) -> Option<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn whenExpression(&self) -> Option<Rc<WhenExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AssignmentStatementContextAttrs<'input> for AssignmentStatementContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assignmentStatement(&mut self,)
	-> Result<Rc<AssignmentStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssignmentStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 154, RULE_assignmentStatement);
        let mut _localctx: Rc<AssignmentStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(710);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(49,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(704);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(705);
					recog.base.match_token(EQ,&mut recog.err_handler)?;

					/*InvokeRule valueExpr*/
					recog.base.set_state(706);
					recog.valueExpr()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(707);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(708);
					recog.base.match_token(EQ,&mut recog.err_handler)?;

					/*InvokeRule whenExpression*/
					recog.base.set_state(709);
					recog.whenExpression()?;

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
//------------------- aggregateBlock ----------------
pub type AggregateBlockContextAll<'input> = AggregateBlockContext<'input>;


pub type AggregateBlockContext<'input> = BaseParserRuleContext<'input,AggregateBlockContextExt<'input>>;

#[derive(Clone)]
pub struct AggregateBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for AggregateBlockContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for AggregateBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_aggregateBlock(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_aggregateBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for AggregateBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_aggregateBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for AggregateBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_aggregateBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_aggregateBlock }
}
antlr_rust::tid!{AggregateBlockContextExt<'a>}

impl<'input> AggregateBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AggregateBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AggregateBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AggregateBlockContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<AggregateBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token AGGREGATE
/// Returns `None` if there is no child corresponding to token AGGREGATE
fn AGGREGATE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(AGGREGATE, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn aggregateStatement_all(&self) ->  Vec<Rc<AggregateStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn aggregateStatement(&self, i: usize) -> Option<Rc<AggregateStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> AggregateBlockContextAttrs<'input> for AggregateBlockContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn aggregateBlock(&mut self,)
	-> Result<Rc<AggregateBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AggregateBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 156, RULE_aggregateBlock);
        let mut _localctx: Rc<AggregateBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(712);
			recog.base.match_token(AGGREGATE,&mut recog.err_handler)?;

			recog.base.set_state(713);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(715); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule aggregateStatement*/
				recog.base.set_state(714);
				recog.aggregateStatement()?;

				}
				}
				recog.base.set_state(717); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==IDENTIFIER) {break}
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
//------------------- aggregateStatement ----------------
pub type AggregateStatementContextAll<'input> = AggregateStatementContext<'input>;


pub type AggregateStatementContext<'input> = BaseParserRuleContext<'input,AggregateStatementContextExt<'input>>;

#[derive(Clone)]
pub struct AggregateStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for AggregateStatementContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for AggregateStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_aggregateStatement(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_aggregateStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for AggregateStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_aggregateStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for AggregateStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_aggregateStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_aggregateStatement }
}
antlr_rust::tid!{AggregateStatementContextExt<'a>}

impl<'input> AggregateStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AggregateStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AggregateStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AggregateStatementContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<AggregateStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token EQ
/// Returns `None` if there is no child corresponding to token EQ
fn EQ(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(EQ, 0)
}
fn valueExpr(&self) -> Option<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AggregateStatementContextAttrs<'input> for AggregateStatementContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn aggregateStatement(&mut self,)
	-> Result<Rc<AggregateStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AggregateStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 158, RULE_aggregateStatement);
        let mut _localctx: Rc<AggregateStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(719);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(720);
			recog.base.match_token(EQ,&mut recog.err_handler)?;

			/*InvokeRule valueExpr*/
			recog.base.set_state(721);
			recog.valueExpr()?;

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

impl<'input> RulesDSLParserContext<'input> for WhenExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for WhenExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_whenExpression(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_whenExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for WhenExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_whenExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for WhenExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_whenExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_whenExpression }
}
antlr_rust::tid!{WhenExpressionContextExt<'a>}

impl<'input> WhenExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<WhenExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,WhenExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait WhenExpressionContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<WhenExpressionContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token WHEN in current rule
fn WHEN_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token WHEN, starting from 0.
/// Returns `None` if number of children corresponding to token WHEN is less or equal than `i`.
fn WHEN(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(WHEN, i)
}
fn booleanExpr_all(&self) ->  Vec<Rc<BooleanExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn booleanExpr(&self, i: usize) -> Option<Rc<BooleanExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token THEN in current rule
fn THEN_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token THEN, starting from 0.
/// Returns `None` if number of children corresponding to token THEN is less or equal than `i`.
fn THEN(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(THEN, i)
}
fn valueExpr_all(&self) ->  Vec<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn valueExpr(&self, i: usize) -> Option<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token OTHERWISE
/// Returns `None` if there is no child corresponding to token OTHERWISE
fn OTHERWISE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(OTHERWISE, 0)
}

}

impl<'input> WhenExpressionContextAttrs<'input> for WhenExpressionContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
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

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(723);
			recog.base.match_token(WHEN,&mut recog.err_handler)?;

			/*InvokeRule booleanExpr*/
			recog.base.set_state(724);
			recog.booleanExpr()?;

			recog.base.set_state(725);
			recog.base.match_token(THEN,&mut recog.err_handler)?;

			/*InvokeRule valueExpr*/
			recog.base.set_state(726);
			recog.valueExpr()?;

			recog.base.set_state(734);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==WHEN {
				{
				{
				recog.base.set_state(727);
				recog.base.match_token(WHEN,&mut recog.err_handler)?;

				/*InvokeRule booleanExpr*/
				recog.base.set_state(728);
				recog.booleanExpr()?;

				recog.base.set_state(729);
				recog.base.match_token(THEN,&mut recog.err_handler)?;

				/*InvokeRule valueExpr*/
				recog.base.set_state(730);
				recog.valueExpr()?;

				}
				}
				recog.base.set_state(736);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(737);
			recog.base.match_token(OTHERWISE,&mut recog.err_handler)?;

			/*InvokeRule valueExpr*/
			recog.base.set_state(738);
			recog.valueExpr()?;

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
//------------------- proceduralRuleDef ----------------
pub type ProceduralRuleDefContextAll<'input> = ProceduralRuleDefContext<'input>;


pub type ProceduralRuleDefContext<'input> = BaseParserRuleContext<'input,ProceduralRuleDefContextExt<'input>>;

#[derive(Clone)]
pub struct ProceduralRuleDefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ProceduralRuleDefContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ProceduralRuleDefContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_proceduralRuleDef(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_proceduralRuleDef(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ProceduralRuleDefContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_proceduralRuleDef(self);
	}
}

impl<'input> CustomRuleContext<'input> for ProceduralRuleDefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_proceduralRuleDef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_proceduralRuleDef }
}
antlr_rust::tid!{ProceduralRuleDefContextExt<'a>}

impl<'input> ProceduralRuleDefContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProceduralRuleDefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProceduralRuleDefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ProceduralRuleDefContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ProceduralRuleDefContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token RULE
/// Returns `None` if there is no child corresponding to token RULE
fn RULE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RULE, 0)
}
fn ruleName(&self) -> Option<Rc<RuleNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
fn descriptionDecl(&self) -> Option<Rc<DescriptionDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn blockItem_all(&self) ->  Vec<Rc<BlockItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn blockItem(&self, i: usize) -> Option<Rc<BlockItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ProceduralRuleDefContextAttrs<'input> for ProceduralRuleDefContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn proceduralRuleDef(&mut self,)
	-> Result<Rc<ProceduralRuleDefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProceduralRuleDefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 162, RULE_proceduralRuleDef);
        let mut _localctx: Rc<ProceduralRuleDefContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(740);
			recog.base.match_token(RULE,&mut recog.err_handler)?;

			/*InvokeRule ruleName*/
			recog.base.set_state(741);
			recog.ruleName()?;

			recog.base.set_state(742);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(744);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==DESCRIPTION {
				{
				/*InvokeRule descriptionDecl*/
				recog.base.set_state(743);
				recog.descriptionDecl()?;

				}
			}

			recog.base.set_state(747); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule blockItem*/
				recog.base.set_state(746);
				recog.blockItem()?;

				}
				}
				recog.base.set_state(749); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << RETURN) | (1usize << IF) | (1usize << SET) | (1usize << LET))) != 0) || _la==DQUOTED_STRING || _la==IDENTIFIER) {break}
			}
			recog.base.set_state(751);
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
//------------------- ruleName ----------------
pub type RuleNameContextAll<'input> = RuleNameContext<'input>;


pub type RuleNameContext<'input> = BaseParserRuleContext<'input,RuleNameContextExt<'input>>;

#[derive(Clone)]
pub struct RuleNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for RuleNameContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for RuleNameContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_ruleName(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_ruleName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for RuleNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_ruleName(self);
	}
}

impl<'input> CustomRuleContext<'input> for RuleNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ruleName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ruleName }
}
antlr_rust::tid!{RuleNameContextExt<'a>}

impl<'input> RuleNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RuleNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RuleNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RuleNameContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<RuleNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token DQUOTED_STRING
/// Returns `None` if there is no child corresponding to token DQUOTED_STRING
fn DQUOTED_STRING(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DQUOTED_STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token SQUOTED_STRING
/// Returns `None` if there is no child corresponding to token SQUOTED_STRING
fn SQUOTED_STRING(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(SQUOTED_STRING, 0)
}

}

impl<'input> RuleNameContextAttrs<'input> for RuleNameContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn ruleName(&mut self,)
	-> Result<Rc<RuleNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RuleNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 164, RULE_ruleName);
        let mut _localctx: Rc<RuleNameContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(753);
			_la = recog.base.input.la(1);
			if { !(((((_la - 120)) & !0x3f) == 0 && ((1usize << (_la - 120)) & ((1usize << (DQUOTED_STRING - 120)) | (1usize << (SQUOTED_STRING - 120)) | (1usize << (IDENTIFIER - 120)))) != 0)) } {
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
//------------------- blockItem ----------------
pub type BlockItemContextAll<'input> = BlockItemContext<'input>;


pub type BlockItemContext<'input> = BaseParserRuleContext<'input,BlockItemContextExt<'input>>;

#[derive(Clone)]
pub struct BlockItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for BlockItemContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for BlockItemContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_blockItem(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_blockItem(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for BlockItemContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_blockItem(self);
	}
}

impl<'input> CustomRuleContext<'input> for BlockItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_blockItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_blockItem }
}
antlr_rust::tid!{BlockItemContextExt<'a>}

impl<'input> BlockItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BlockItemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BlockItemContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BlockItemContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<BlockItemContextExt<'input>>{

fn ruleStep(&self) -> Option<Rc<RuleStepContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn setStatement(&self) -> Option<Rc<SetStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn letStatement(&self) -> Option<Rc<LetStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn actionSequence(&self) -> Option<Rc<ActionSequenceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn returnStatement(&self) -> Option<Rc<ReturnStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> BlockItemContextAttrs<'input> for BlockItemContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn blockItem(&mut self,)
	-> Result<Rc<BlockItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BlockItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 166, RULE_blockItem);
        let mut _localctx: Rc<BlockItemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(760);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IF 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule ruleStep*/
					recog.base.set_state(755);
					recog.ruleStep()?;

					}
				}

			 SET 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule setStatement*/
					recog.base.set_state(756);
					recog.setStatement()?;

					}
				}

			 LET 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule letStatement*/
					recog.base.set_state(757);
					recog.letStatement()?;

					}
				}

			 DQUOTED_STRING | IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule actionSequence*/
					recog.base.set_state(758);
					recog.actionSequence()?;

					}
				}

			 RETURN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule returnStatement*/
					recog.base.set_state(759);
					recog.returnStatement()?;

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
//------------------- setStatement ----------------
pub type SetStatementContextAll<'input> = SetStatementContext<'input>;


pub type SetStatementContext<'input> = BaseParserRuleContext<'input,SetStatementContextExt<'input>>;

#[derive(Clone)]
pub struct SetStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for SetStatementContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for SetStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_setStatement(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_setStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for SetStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_setStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for SetStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_setStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_setStatement }
}
antlr_rust::tid!{SetStatementContextExt<'a>}

impl<'input> SetStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SetStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SetStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SetStatementContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<SetStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SET
/// Returns `None` if there is no child corresponding to token SET
fn SET(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(SET, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token EQ
/// Returns `None` if there is no child corresponding to token EQ
fn EQ(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(EQ, 0)
}
fn valueExpr(&self) -> Option<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SetStatementContextAttrs<'input> for SetStatementContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn setStatement(&mut self,)
	-> Result<Rc<SetStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SetStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 168, RULE_setStatement);
        let mut _localctx: Rc<SetStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(762);
			recog.base.match_token(SET,&mut recog.err_handler)?;

			recog.base.set_state(763);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(764);
			recog.base.match_token(EQ,&mut recog.err_handler)?;

			/*InvokeRule valueExpr*/
			recog.base.set_state(765);
			recog.valueExpr()?;

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
//------------------- letStatement ----------------
pub type LetStatementContextAll<'input> = LetStatementContext<'input>;


pub type LetStatementContext<'input> = BaseParserRuleContext<'input,LetStatementContextExt<'input>>;

#[derive(Clone)]
pub struct LetStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for LetStatementContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for LetStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_letStatement(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_letStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for LetStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_letStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for LetStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_letStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_letStatement }
}
antlr_rust::tid!{LetStatementContextExt<'a>}

impl<'input> LetStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LetStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LetStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LetStatementContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<LetStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LET
/// Returns `None` if there is no child corresponding to token LET
fn LET(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LET, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token EQ
/// Returns `None` if there is no child corresponding to token EQ
fn EQ(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(EQ, 0)
}
fn valueExpr(&self) -> Option<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> LetStatementContextAttrs<'input> for LetStatementContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn letStatement(&mut self,)
	-> Result<Rc<LetStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LetStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 170, RULE_letStatement);
        let mut _localctx: Rc<LetStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(767);
			recog.base.match_token(LET,&mut recog.err_handler)?;

			recog.base.set_state(768);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(769);
			recog.base.match_token(EQ,&mut recog.err_handler)?;

			/*InvokeRule valueExpr*/
			recog.base.set_state(770);
			recog.valueExpr()?;

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
//------------------- ruleStep ----------------
pub type RuleStepContextAll<'input> = RuleStepContext<'input>;


pub type RuleStepContext<'input> = BaseParserRuleContext<'input,RuleStepContextExt<'input>>;

#[derive(Clone)]
pub struct RuleStepContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for RuleStepContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for RuleStepContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_ruleStep(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_ruleStep(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for RuleStepContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_ruleStep(self);
	}
}

impl<'input> CustomRuleContext<'input> for RuleStepContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ruleStep }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ruleStep }
}
antlr_rust::tid!{RuleStepContextExt<'a>}

impl<'input> RuleStepContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RuleStepContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RuleStepContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RuleStepContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<RuleStepContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IF
/// Returns `None` if there is no child corresponding to token IF
fn IF(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IF, 0)
}
fn booleanExpr_all(&self) ->  Vec<Rc<BooleanExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn booleanExpr(&self, i: usize) -> Option<Rc<BooleanExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token THEN in current rule
fn THEN_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token THEN, starting from 0.
/// Returns `None` if number of children corresponding to token THEN is less or equal than `i`.
fn THEN(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(THEN, i)
}
fn block_all(&self) ->  Vec<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn block(&self, i: usize) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token ENDIF
/// Returns `None` if there is no child corresponding to token ENDIF
fn ENDIF(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(ENDIF, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token ELSEIF in current rule
fn ELSEIF_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token ELSEIF, starting from 0.
/// Returns `None` if number of children corresponding to token ELSEIF is less or equal than `i`.
fn ELSEIF(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(ELSEIF, i)
}
/// Retrieves first TerminalNode corresponding to token ELSE
/// Returns `None` if there is no child corresponding to token ELSE
fn ELSE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(ELSE, 0)
}

}

impl<'input> RuleStepContextAttrs<'input> for RuleStepContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn ruleStep(&mut self,)
	-> Result<Rc<RuleStepContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RuleStepContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 172, RULE_ruleStep);
        let mut _localctx: Rc<RuleStepContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(772);
			recog.base.match_token(IF,&mut recog.err_handler)?;

			/*InvokeRule booleanExpr*/
			recog.base.set_state(773);
			recog.booleanExpr()?;

			recog.base.set_state(774);
			recog.base.match_token(THEN,&mut recog.err_handler)?;

			/*InvokeRule block*/
			recog.base.set_state(775);
			recog.block()?;

			recog.base.set_state(783);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==ELSEIF {
				{
				{
				recog.base.set_state(776);
				recog.base.match_token(ELSEIF,&mut recog.err_handler)?;

				/*InvokeRule booleanExpr*/
				recog.base.set_state(777);
				recog.booleanExpr()?;

				recog.base.set_state(778);
				recog.base.match_token(THEN,&mut recog.err_handler)?;

				/*InvokeRule block*/
				recog.base.set_state(779);
				recog.block()?;

				}
				}
				recog.base.set_state(785);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(788);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ELSE {
				{
				recog.base.set_state(786);
				recog.base.match_token(ELSE,&mut recog.err_handler)?;

				/*InvokeRule block*/
				recog.base.set_state(787);
				recog.block()?;

				}
			}

			recog.base.set_state(790);
			recog.base.match_token(ENDIF,&mut recog.err_handler)?;

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
//------------------- block ----------------
pub type BlockContextAll<'input> = BlockContext<'input>;


pub type BlockContext<'input> = BaseParserRuleContext<'input,BlockContextExt<'input>>;

#[derive(Clone)]
pub struct BlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for BlockContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for BlockContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_block(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_block(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for BlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_block(self);
	}
}

impl<'input> CustomRuleContext<'input> for BlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_block }
	//fn type_rule_index() -> usize where Self: Sized { RULE_block }
}
antlr_rust::tid!{BlockContextExt<'a>}

impl<'input> BlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BlockContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<BlockContextExt<'input>>{

fn blockItem_all(&self) ->  Vec<Rc<BlockItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn blockItem(&self, i: usize) -> Option<Rc<BlockItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> BlockContextAttrs<'input> for BlockContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn block(&mut self,)
	-> Result<Rc<BlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 174, RULE_block);
        let mut _localctx: Rc<BlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(793); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule blockItem*/
				recog.base.set_state(792);
				recog.blockItem()?;

				}
				}
				recog.base.set_state(795); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << RETURN) | (1usize << IF) | (1usize << SET) | (1usize << LET))) != 0) || _la==DQUOTED_STRING || _la==IDENTIFIER) {break}
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
//------------------- actionSequence ----------------
pub type ActionSequenceContextAll<'input> = ActionSequenceContext<'input>;


pub type ActionSequenceContext<'input> = BaseParserRuleContext<'input,ActionSequenceContextExt<'input>>;

#[derive(Clone)]
pub struct ActionSequenceContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ActionSequenceContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ActionSequenceContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_actionSequence(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_actionSequence(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ActionSequenceContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_actionSequence(self);
	}
}

impl<'input> CustomRuleContext<'input> for ActionSequenceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_actionSequence }
	//fn type_rule_index() -> usize where Self: Sized { RULE_actionSequence }
}
antlr_rust::tid!{ActionSequenceContextExt<'a>}

impl<'input> ActionSequenceContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ActionSequenceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ActionSequenceContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ActionSequenceContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ActionSequenceContextExt<'input>>{

fn actionCall_all(&self) ->  Vec<Rc<ActionCallContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn actionCall(&self, i: usize) -> Option<Rc<ActionCallContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ActionSequenceContextAttrs<'input> for ActionSequenceContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn actionSequence(&mut self,)
	-> Result<Rc<ActionSequenceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ActionSequenceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 176, RULE_actionSequence);
        let mut _localctx: Rc<ActionSequenceContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule actionCall*/
			recog.base.set_state(797);
			recog.actionCall()?;

			recog.base.set_state(802);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(798);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule actionCall*/
				recog.base.set_state(799);
				recog.actionCall()?;

				}
				}
				recog.base.set_state(804);
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
//------------------- actionCall ----------------
pub type ActionCallContextAll<'input> = ActionCallContext<'input>;


pub type ActionCallContext<'input> = BaseParserRuleContext<'input,ActionCallContextExt<'input>>;

#[derive(Clone)]
pub struct ActionCallContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ActionCallContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ActionCallContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_actionCall(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_actionCall(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ActionCallContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_actionCall(self);
	}
}

impl<'input> CustomRuleContext<'input> for ActionCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_actionCall }
	//fn type_rule_index() -> usize where Self: Sized { RULE_actionCall }
}
antlr_rust::tid!{ActionCallContextExt<'a>}

impl<'input> ActionCallContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ActionCallContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ActionCallContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ActionCallContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ActionCallContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn parameterList(&self) -> Option<Rc<ParameterListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DQUOTED_STRING
/// Returns `None` if there is no child corresponding to token DQUOTED_STRING
fn DQUOTED_STRING(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DQUOTED_STRING, 0)
}

}

impl<'input> ActionCallContextAttrs<'input> for ActionCallContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn actionCall(&mut self,)
	-> Result<Rc<ActionCallContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ActionCallContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 178, RULE_actionCall);
        let mut _localctx: Rc<ActionCallContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(821);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(805);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(811);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LPAREN {
						{
						recog.base.set_state(806);
						recog.base.match_token(LPAREN,&mut recog.err_handler)?;

						recog.base.set_state(808);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if ((((_la - 34)) & !0x3f) == 0 && ((1usize << (_la - 34)) & ((1usize << (NULL - 34)) | (1usize << (ANY - 34)) | (1usize << (ALL - 34)) | (1usize << (NONE - 34)) | (1usize << (SUM - 34)) | (1usize << (COUNT - 34)) | (1usize << (AVG - 34)) | (1usize << (MAX_FN - 34)) | (1usize << (MIN_FN - 34)) | (1usize << (FILTER - 34)))) != 0) || ((((_la - 66)) & !0x3f) == 0 && ((1usize << (_la - 66)) & ((1usize << (FIND - 66)) | (1usize << (DISTINCT - 66)) | (1usize << (MINUS - 66)))) != 0) || ((((_la - 107)) & !0x3f) == 0 && ((1usize << (_la - 107)) & ((1usize << (LPAREN - 107)) | (1usize << (LBRACKET - 107)) | (1usize << (LBRACE - 107)) | (1usize << (BOOLEAN - 107)) | (1usize << (INTEGER - 107)) | (1usize << (DECIMAL - 107)) | (1usize << (MONEY_LITERAL - 107)) | (1usize << (PERCENTAGE_LITERAL - 107)) | (1usize << (DQUOTED_STRING - 107)) | (1usize << (SQUOTED_STRING - 107)) | (1usize << (IDENTIFIER - 107)))) != 0) {
							{
							/*InvokeRule parameterList*/
							recog.base.set_state(807);
							recog.parameterList()?;

							}
						}

						recog.base.set_state(810);
						recog.base.match_token(RPAREN,&mut recog.err_handler)?;

						}
					}

					}
				}

			 DQUOTED_STRING 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(813);
					recog.base.match_token(DQUOTED_STRING,&mut recog.err_handler)?;

					recog.base.set_state(819);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LPAREN {
						{
						recog.base.set_state(814);
						recog.base.match_token(LPAREN,&mut recog.err_handler)?;

						recog.base.set_state(816);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if ((((_la - 34)) & !0x3f) == 0 && ((1usize << (_la - 34)) & ((1usize << (NULL - 34)) | (1usize << (ANY - 34)) | (1usize << (ALL - 34)) | (1usize << (NONE - 34)) | (1usize << (SUM - 34)) | (1usize << (COUNT - 34)) | (1usize << (AVG - 34)) | (1usize << (MAX_FN - 34)) | (1usize << (MIN_FN - 34)) | (1usize << (FILTER - 34)))) != 0) || ((((_la - 66)) & !0x3f) == 0 && ((1usize << (_la - 66)) & ((1usize << (FIND - 66)) | (1usize << (DISTINCT - 66)) | (1usize << (MINUS - 66)))) != 0) || ((((_la - 107)) & !0x3f) == 0 && ((1usize << (_la - 107)) & ((1usize << (LPAREN - 107)) | (1usize << (LBRACKET - 107)) | (1usize << (LBRACE - 107)) | (1usize << (BOOLEAN - 107)) | (1usize << (INTEGER - 107)) | (1usize << (DECIMAL - 107)) | (1usize << (MONEY_LITERAL - 107)) | (1usize << (PERCENTAGE_LITERAL - 107)) | (1usize << (DQUOTED_STRING - 107)) | (1usize << (SQUOTED_STRING - 107)) | (1usize << (IDENTIFIER - 107)))) != 0) {
							{
							/*InvokeRule parameterList*/
							recog.base.set_state(815);
							recog.parameterList()?;

							}
						}

						recog.base.set_state(818);
						recog.base.match_token(RPAREN,&mut recog.err_handler)?;

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
//------------------- parameterList ----------------
pub type ParameterListContextAll<'input> = ParameterListContext<'input>;


pub type ParameterListContext<'input> = BaseParserRuleContext<'input,ParameterListContextExt<'input>>;

#[derive(Clone)]
pub struct ParameterListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ParameterListContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ParameterListContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_parameterList(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_parameterList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ParameterListContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_parameterList(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParameterListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterList }
}
antlr_rust::tid!{ParameterListContextExt<'a>}

impl<'input> ParameterListContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParameterListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParameterListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParameterListContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ParameterListContextExt<'input>>{

fn parameter_all(&self) ->  Vec<Rc<ParameterContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn parameter(&self, i: usize) -> Option<Rc<ParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ParameterListContextAttrs<'input> for ParameterListContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parameterList(&mut self,)
	-> Result<Rc<ParameterListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 180, RULE_parameterList);
        let mut _localctx: Rc<ParameterListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule parameter*/
			recog.base.set_state(823);
			recog.parameter()?;

			recog.base.set_state(828);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(824);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule parameter*/
				recog.base.set_state(825);
				recog.parameter()?;

				}
				}
				recog.base.set_state(830);
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
//------------------- parameter ----------------
pub type ParameterContextAll<'input> = ParameterContext<'input>;


pub type ParameterContext<'input> = BaseParserRuleContext<'input,ParameterContextExt<'input>>;

#[derive(Clone)]
pub struct ParameterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ParameterContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ParameterContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_parameter(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_parameter(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ParameterContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_parameter(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameter }
}
antlr_rust::tid!{ParameterContextExt<'a>}

impl<'input> ParameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParameterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParameterContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParameterContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ParameterContextExt<'input>>{

fn valueExpr(&self) -> Option<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ParameterContextAttrs<'input> for ParameterContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parameter(&mut self,)
	-> Result<Rc<ParameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 182, RULE_parameter);
        let mut _localctx: Rc<ParameterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule valueExpr*/
			recog.base.set_state(831);
			recog.valueExpr()?;

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
//------------------- returnStatement ----------------
pub type ReturnStatementContextAll<'input> = ReturnStatementContext<'input>;


pub type ReturnStatementContext<'input> = BaseParserRuleContext<'input,ReturnStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ReturnStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ReturnStatementContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ReturnStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_returnStatement(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_returnStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ReturnStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_returnStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReturnStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_returnStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_returnStatement }
}
antlr_rust::tid!{ReturnStatementContextExt<'a>}

impl<'input> ReturnStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReturnStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReturnStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReturnStatementContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ReturnStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token RETURN
/// Returns `None` if there is no child corresponding to token RETURN
fn RETURN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RETURN, 0)
}

}

impl<'input> ReturnStatementContextAttrs<'input> for ReturnStatementContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn returnStatement(&mut self,)
	-> Result<Rc<ReturnStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReturnStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 184, RULE_returnStatement);
        let mut _localctx: Rc<ReturnStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(833);
			recog.base.match_token(RETURN,&mut recog.err_handler)?;

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
//------------------- booleanExpr ----------------
pub type BooleanExprContextAll<'input> = BooleanExprContext<'input>;


pub type BooleanExprContext<'input> = BaseParserRuleContext<'input,BooleanExprContextExt<'input>>;

#[derive(Clone)]
pub struct BooleanExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for BooleanExprContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for BooleanExprContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_booleanExpr(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_booleanExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for BooleanExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_booleanExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for BooleanExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_booleanExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_booleanExpr }
}
antlr_rust::tid!{BooleanExprContextExt<'a>}

impl<'input> BooleanExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BooleanExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BooleanExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BooleanExprContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<BooleanExprContextExt<'input>>{

fn booleanTerm_all(&self) ->  Vec<Rc<BooleanTermContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn booleanTerm(&self, i: usize) -> Option<Rc<BooleanTermContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token AND in current rule
fn AND_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token AND, starting from 0.
/// Returns `None` if number of children corresponding to token AND is less or equal than `i`.
fn AND(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(AND, i)
}
/// Retrieves all `TerminalNode`s corresponding to token OR in current rule
fn OR_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token OR, starting from 0.
/// Returns `None` if number of children corresponding to token OR is less or equal than `i`.
fn OR(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(OR, i)
}

}

impl<'input> BooleanExprContextAttrs<'input> for BooleanExprContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn booleanExpr(&mut self,)
	-> Result<Rc<BooleanExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BooleanExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 186, RULE_booleanExpr);
        let mut _localctx: Rc<BooleanExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule booleanTerm*/
			recog.base.set_state(835);
			recog.booleanTerm()?;

			recog.base.set_state(840);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==AND || _la==OR {
				{
				{
				recog.base.set_state(836);
				_la = recog.base.input.la(1);
				if { !(_la==AND || _la==OR) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				/*InvokeRule booleanTerm*/
				recog.base.set_state(837);
				recog.booleanTerm()?;

				}
				}
				recog.base.set_state(842);
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
//------------------- booleanTerm ----------------
pub type BooleanTermContextAll<'input> = BooleanTermContext<'input>;


pub type BooleanTermContext<'input> = BaseParserRuleContext<'input,BooleanTermContextExt<'input>>;

#[derive(Clone)]
pub struct BooleanTermContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for BooleanTermContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for BooleanTermContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_booleanTerm(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_booleanTerm(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for BooleanTermContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_booleanTerm(self);
	}
}

impl<'input> CustomRuleContext<'input> for BooleanTermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_booleanTerm }
	//fn type_rule_index() -> usize where Self: Sized { RULE_booleanTerm }
}
antlr_rust::tid!{BooleanTermContextExt<'a>}

impl<'input> BooleanTermContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BooleanTermContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BooleanTermContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BooleanTermContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<BooleanTermContextExt<'input>>{

fn booleanFactor(&self) -> Option<Rc<BooleanFactorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token NOT
/// Returns `None` if there is no child corresponding to token NOT
fn NOT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(NOT, 0)
}

}

impl<'input> BooleanTermContextAttrs<'input> for BooleanTermContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn booleanTerm(&mut self,)
	-> Result<Rc<BooleanTermContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BooleanTermContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 188, RULE_booleanTerm);
        let mut _localctx: Rc<BooleanTermContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(844);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==NOT {
				{
				recog.base.set_state(843);
				recog.base.match_token(NOT,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule booleanFactor*/
			recog.base.set_state(846);
			recog.booleanFactor()?;

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
//------------------- booleanFactor ----------------
pub type BooleanFactorContextAll<'input> = BooleanFactorContext<'input>;


pub type BooleanFactorContext<'input> = BaseParserRuleContext<'input,BooleanFactorContextExt<'input>>;

#[derive(Clone)]
pub struct BooleanFactorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for BooleanFactorContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for BooleanFactorContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_booleanFactor(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_booleanFactor(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for BooleanFactorContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_booleanFactor(self);
	}
}

impl<'input> CustomRuleContext<'input> for BooleanFactorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_booleanFactor }
	//fn type_rule_index() -> usize where Self: Sized { RULE_booleanFactor }
}
antlr_rust::tid!{BooleanFactorContextExt<'a>}

impl<'input> BooleanFactorContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BooleanFactorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BooleanFactorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BooleanFactorContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<BooleanFactorContextExt<'input>>{

fn comparisonExpr(&self) -> Option<Rc<ComparisonExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn booleanExpr(&self) -> Option<Rc<BooleanExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn functionCall(&self) -> Option<Rc<FunctionCallContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> BooleanFactorContextAttrs<'input> for BooleanFactorContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn booleanFactor(&mut self,)
	-> Result<Rc<BooleanFactorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BooleanFactorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 190, RULE_booleanFactor);
        let mut _localctx: Rc<BooleanFactorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(854);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(67,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule comparisonExpr*/
					recog.base.set_state(848);
					recog.comparisonExpr()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(849);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule booleanExpr*/
					recog.base.set_state(850);
					recog.booleanExpr()?;

					recog.base.set_state(851);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule functionCall*/
					recog.base.set_state(853);
					recog.functionCall()?;

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
//------------------- comparisonExpr ----------------
pub type ComparisonExprContextAll<'input> = ComparisonExprContext<'input>;


pub type ComparisonExprContext<'input> = BaseParserRuleContext<'input,ComparisonExprContextExt<'input>>;

#[derive(Clone)]
pub struct ComparisonExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ComparisonExprContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ComparisonExprContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_comparisonExpr(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_comparisonExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ComparisonExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_comparisonExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComparisonExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comparisonExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comparisonExpr }
}
antlr_rust::tid!{ComparisonExprContextExt<'a>}

impl<'input> ComparisonExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComparisonExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComparisonExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComparisonExprContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ComparisonExprContextExt<'input>>{

fn valueExpr_all(&self) ->  Vec<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn valueExpr(&self, i: usize) -> Option<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn comparisonOp(&self) -> Option<Rc<ComparisonOpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IN, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn valueList(&self) -> Option<Rc<ValueListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token NOT
/// Returns `None` if there is no child corresponding to token NOT
fn NOT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(NOT, 0)
}
fn listLiteral(&self) -> Option<Rc<ListLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldPath(&self) -> Option<Rc<FieldPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IS
/// Returns `None` if there is no child corresponding to token IS
fn IS(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IS, 0)
}
/// Retrieves first TerminalNode corresponding to token NULL
/// Returns `None` if there is no child corresponding to token NULL
fn NULL(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(NULL, 0)
}
/// Retrieves first TerminalNode corresponding to token IS_NULL
/// Returns `None` if there is no child corresponding to token IS_NULL
fn IS_NULL(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IS_NULL, 0)
}
/// Retrieves first TerminalNode corresponding to token IS_NOT_NULL
/// Returns `None` if there is no child corresponding to token IS_NOT_NULL
fn IS_NOT_NULL(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IS_NOT_NULL, 0)
}

}

impl<'input> ComparisonExprContextAttrs<'input> for ComparisonExprContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn comparisonExpr(&mut self,)
	-> Result<Rc<ComparisonExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComparisonExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 192, RULE_comparisonExpr);
        let mut _localctx: Rc<ComparisonExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(906);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(68,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule valueExpr*/
					recog.base.set_state(856);
					recog.valueExpr()?;

					/*InvokeRule comparisonOp*/
					recog.base.set_state(857);
					recog.comparisonOp()?;

					/*InvokeRule valueExpr*/
					recog.base.set_state(858);
					recog.valueExpr()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule valueExpr*/
					recog.base.set_state(860);
					recog.valueExpr()?;

					recog.base.set_state(861);
					recog.base.match_token(IN,&mut recog.err_handler)?;

					recog.base.set_state(862);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule valueList*/
					recog.base.set_state(863);
					recog.valueList()?;

					recog.base.set_state(864);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule valueExpr*/
					recog.base.set_state(866);
					recog.valueExpr()?;

					recog.base.set_state(867);
					recog.base.match_token(NOT,&mut recog.err_handler)?;

					recog.base.set_state(868);
					recog.base.match_token(IN,&mut recog.err_handler)?;

					recog.base.set_state(869);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule valueList*/
					recog.base.set_state(870);
					recog.valueList()?;

					recog.base.set_state(871);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule valueExpr*/
					recog.base.set_state(873);
					recog.valueExpr()?;

					recog.base.set_state(874);
					recog.base.match_token(IN,&mut recog.err_handler)?;

					/*InvokeRule listLiteral*/
					recog.base.set_state(875);
					recog.listLiteral()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule valueExpr*/
					recog.base.set_state(877);
					recog.valueExpr()?;

					recog.base.set_state(878);
					recog.base.match_token(NOT,&mut recog.err_handler)?;

					recog.base.set_state(879);
					recog.base.match_token(IN,&mut recog.err_handler)?;

					/*InvokeRule listLiteral*/
					recog.base.set_state(880);
					recog.listLiteral()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule valueExpr*/
					recog.base.set_state(882);
					recog.valueExpr()?;

					recog.base.set_state(883);
					recog.base.match_token(IN,&mut recog.err_handler)?;

					/*InvokeRule fieldPath*/
					recog.base.set_state(884);
					recog.fieldPath()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule valueExpr*/
					recog.base.set_state(886);
					recog.valueExpr()?;

					recog.base.set_state(887);
					recog.base.match_token(NOT,&mut recog.err_handler)?;

					recog.base.set_state(888);
					recog.base.match_token(IN,&mut recog.err_handler)?;

					/*InvokeRule fieldPath*/
					recog.base.set_state(889);
					recog.fieldPath()?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule valueExpr*/
					recog.base.set_state(891);
					recog.valueExpr()?;

					recog.base.set_state(892);
					recog.base.match_token(IS,&mut recog.err_handler)?;

					recog.base.set_state(893);
					recog.base.match_token(NULL,&mut recog.err_handler)?;

					}
				}
			,
				9 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					/*InvokeRule valueExpr*/
					recog.base.set_state(895);
					recog.valueExpr()?;

					recog.base.set_state(896);
					recog.base.match_token(IS,&mut recog.err_handler)?;

					recog.base.set_state(897);
					recog.base.match_token(NOT,&mut recog.err_handler)?;

					recog.base.set_state(898);
					recog.base.match_token(NULL,&mut recog.err_handler)?;

					}
				}
			,
				10 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 10);
					recog.base.enter_outer_alt(None, 10);
					{
					/*InvokeRule valueExpr*/
					recog.base.set_state(900);
					recog.valueExpr()?;

					recog.base.set_state(901);
					recog.base.match_token(IS_NULL,&mut recog.err_handler)?;

					}
				}
			,
				11 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 11);
					recog.base.enter_outer_alt(None, 11);
					{
					/*InvokeRule valueExpr*/
					recog.base.set_state(903);
					recog.valueExpr()?;

					recog.base.set_state(904);
					recog.base.match_token(IS_NOT_NULL,&mut recog.err_handler)?;

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
//------------------- comparisonOp ----------------
pub type ComparisonOpContextAll<'input> = ComparisonOpContext<'input>;


pub type ComparisonOpContext<'input> = BaseParserRuleContext<'input,ComparisonOpContextExt<'input>>;

#[derive(Clone)]
pub struct ComparisonOpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ComparisonOpContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ComparisonOpContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_comparisonOp(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_comparisonOp(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ComparisonOpContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_comparisonOp(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComparisonOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comparisonOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comparisonOp }
}
antlr_rust::tid!{ComparisonOpContextExt<'a>}

impl<'input> ComparisonOpContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComparisonOpContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComparisonOpContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComparisonOpContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ComparisonOpContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EQ
/// Returns `None` if there is no child corresponding to token EQ
fn EQ(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(EQ, 0)
}
/// Retrieves first TerminalNode corresponding to token NE
/// Returns `None` if there is no child corresponding to token NE
fn NE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(NE, 0)
}
/// Retrieves first TerminalNode corresponding to token LT
/// Returns `None` if there is no child corresponding to token LT
fn LT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LT, 0)
}
/// Retrieves first TerminalNode corresponding to token GT
/// Returns `None` if there is no child corresponding to token GT
fn GT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(GT, 0)
}
/// Retrieves first TerminalNode corresponding to token LE
/// Returns `None` if there is no child corresponding to token LE
fn LE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LE, 0)
}
/// Retrieves first TerminalNode corresponding to token GE
/// Returns `None` if there is no child corresponding to token GE
fn GE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(GE, 0)
}

}

impl<'input> ComparisonOpContextAttrs<'input> for ComparisonOpContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn comparisonOp(&mut self,)
	-> Result<Rc<ComparisonOpContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComparisonOpContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 194, RULE_comparisonOp);
        let mut _localctx: Rc<ComparisonOpContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(908);
			_la = recog.base.input.la(1);
			if { !(((((_la - 89)) & !0x3f) == 0 && ((1usize << (_la - 89)) & ((1usize << (EQ - 89)) | (1usize << (NE - 89)) | (1usize << (LT - 89)) | (1usize << (GT - 89)) | (1usize << (LE - 89)) | (1usize << (GE - 89)))) != 0)) } {
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
//------------------- valueExpr ----------------
pub type ValueExprContextAll<'input> = ValueExprContext<'input>;


pub type ValueExprContext<'input> = BaseParserRuleContext<'input,ValueExprContextExt<'input>>;

#[derive(Clone)]
pub struct ValueExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ValueExprContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ValueExprContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_valueExpr(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_valueExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ValueExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_valueExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_valueExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_valueExpr }
}
antlr_rust::tid!{ValueExprContextExt<'a>}

impl<'input> ValueExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValueExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValueExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ValueExprContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ValueExprContextExt<'input>>{

fn term_all(&self) ->  Vec<Rc<TermContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn term(&self, i: usize) -> Option<Rc<TermContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token PLUS in current rule
fn PLUS_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token PLUS, starting from 0.
/// Returns `None` if number of children corresponding to token PLUS is less or equal than `i`.
fn PLUS(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(PLUS, i)
}
/// Retrieves all `TerminalNode`s corresponding to token MINUS in current rule
fn MINUS_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token MINUS, starting from 0.
/// Returns `None` if number of children corresponding to token MINUS is less or equal than `i`.
fn MINUS(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(MINUS, i)
}

}

impl<'input> ValueExprContextAttrs<'input> for ValueExprContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn valueExpr(&mut self,)
	-> Result<Rc<ValueExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValueExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 196, RULE_valueExpr);
        let mut _localctx: Rc<ValueExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule term*/
			recog.base.set_state(910);
			recog.term()?;

			recog.base.set_state(915);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(69,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(911);
					_la = recog.base.input.la(1);
					if { !(_la==PLUS || _la==MINUS) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule term*/
					recog.base.set_state(912);
					recog.term()?;

					}
					} 
				}
				recog.base.set_state(917);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(69,&mut recog.base)?;
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
//------------------- term ----------------
pub type TermContextAll<'input> = TermContext<'input>;


pub type TermContext<'input> = BaseParserRuleContext<'input,TermContextExt<'input>>;

#[derive(Clone)]
pub struct TermContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for TermContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for TermContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_term(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_term(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for TermContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for TermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_term }
}
antlr_rust::tid!{TermContextExt<'a>}

impl<'input> TermContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TermContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TermContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TermContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<TermContextExt<'input>>{

fn factor_all(&self) ->  Vec<Rc<FactorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn factor(&self, i: usize) -> Option<Rc<FactorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token STAR in current rule
fn STAR_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token STAR, starting from 0.
/// Returns `None` if number of children corresponding to token STAR is less or equal than `i`.
fn STAR(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(STAR, i)
}
/// Retrieves all `TerminalNode`s corresponding to token SLASH in current rule
fn SLASH_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SLASH, starting from 0.
/// Returns `None` if number of children corresponding to token SLASH is less or equal than `i`.
fn SLASH(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(SLASH, i)
}
/// Retrieves all `TerminalNode`s corresponding to token PERCENT in current rule
fn PERCENT_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token PERCENT, starting from 0.
/// Returns `None` if number of children corresponding to token PERCENT is less or equal than `i`.
fn PERCENT(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(PERCENT, i)
}
/// Retrieves all `TerminalNode`s corresponding to token MOD in current rule
fn MOD_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token MOD, starting from 0.
/// Returns `None` if number of children corresponding to token MOD is less or equal than `i`.
fn MOD(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(MOD, i)
}

}

impl<'input> TermContextAttrs<'input> for TermContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn term(&mut self,)
	-> Result<Rc<TermContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TermContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 198, RULE_term);
        let mut _localctx: Rc<TermContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule factor*/
			recog.base.set_state(918);
			recog.factor()?;

			recog.base.set_state(923);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(70,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(919);
					_la = recog.base.input.la(1);
					if { !(((((_la - 97)) & !0x3f) == 0 && ((1usize << (_la - 97)) & ((1usize << (STAR - 97)) | (1usize << (SLASH - 97)) | (1usize << (PERCENT - 97)) | (1usize << (MOD - 97)))) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule factor*/
					recog.base.set_state(920);
					recog.factor()?;

					}
					} 
				}
				recog.base.set_state(925);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(70,&mut recog.base)?;
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
//------------------- factor ----------------
pub type FactorContextAll<'input> = FactorContext<'input>;


pub type FactorContext<'input> = BaseParserRuleContext<'input,FactorContextExt<'input>>;

#[derive(Clone)]
pub struct FactorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for FactorContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for FactorContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_factor(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_factor(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for FactorContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_factor(self);
	}
}

impl<'input> CustomRuleContext<'input> for FactorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_factor }
	//fn type_rule_index() -> usize where Self: Sized { RULE_factor }
}
antlr_rust::tid!{FactorContextExt<'a>}

impl<'input> FactorContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FactorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FactorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FactorContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<FactorContextExt<'input>>{

fn atom(&self) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}

}

impl<'input> FactorContextAttrs<'input> for FactorContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn factor(&mut self,)
	-> Result<Rc<FactorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FactorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 200, RULE_factor);
        let mut _localctx: Rc<FactorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(927);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(71,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(926);
					recog.base.match_token(MINUS,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			/*InvokeRule atom*/
			recog.base.set_state(929);
			recog.atom()?;

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
//------------------- atom ----------------
pub type AtomContextAll<'input> = AtomContext<'input>;


pub type AtomContext<'input> = BaseParserRuleContext<'input,AtomContextExt<'input>>;

#[derive(Clone)]
pub struct AtomContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for AtomContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for AtomContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_atom(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_atom(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for AtomContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_atom(self);
	}
}

impl<'input> CustomRuleContext<'input> for AtomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atom }
}
antlr_rust::tid!{AtomContextExt<'a>}

impl<'input> AtomContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AtomContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AtomContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AtomContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<AtomContextExt<'input>>{

fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldPath(&self) -> Option<Rc<FieldPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn collectionExpr(&self) -> Option<Rc<CollectionExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functionCall(&self) -> Option<Rc<FunctionCallContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn listLiteral(&self) -> Option<Rc<ListLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn objectLiteral(&self) -> Option<Rc<ObjectLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lambdaExpression(&self) -> Option<Rc<LambdaExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn valueExpr(&self) -> Option<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> AtomContextAttrs<'input> for AtomContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn atom(&mut self,)
	-> Result<Rc<AtomContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AtomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 202, RULE_atom);
        let mut _localctx: Rc<AtomContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(942);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(72,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule literal*/
					recog.base.set_state(931);
					recog.literal()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule fieldPath*/
					recog.base.set_state(932);
					recog.fieldPath()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule collectionExpr*/
					recog.base.set_state(933);
					recog.collectionExpr()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule functionCall*/
					recog.base.set_state(934);
					recog.functionCall()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule listLiteral*/
					recog.base.set_state(935);
					recog.listLiteral()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule objectLiteral*/
					recog.base.set_state(936);
					recog.objectLiteral()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule lambdaExpression*/
					recog.base.set_state(937);
					recog.lambdaExpression()?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					recog.base.set_state(938);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule valueExpr*/
					recog.base.set_state(939);
					recog.valueExpr()?;

					recog.base.set_state(940);
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
//------------------- collectionExpr ----------------
pub type CollectionExprContextAll<'input> = CollectionExprContext<'input>;


pub type CollectionExprContext<'input> = BaseParserRuleContext<'input,CollectionExprContextExt<'input>>;

#[derive(Clone)]
pub struct CollectionExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for CollectionExprContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for CollectionExprContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_collectionExpr(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_collectionExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for CollectionExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_collectionExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for CollectionExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_collectionExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_collectionExpr }
}
antlr_rust::tid!{CollectionExprContextExt<'a>}

impl<'input> CollectionExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CollectionExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CollectionExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CollectionExprContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<CollectionExprContextExt<'input>>{

fn predicateFunction(&self) -> Option<Rc<PredicateFunctionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn valueExpr(&self) -> Option<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}
fn collectionPredicate(&self) -> Option<Rc<CollectionPredicateContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn aggregateFunction(&self) -> Option<Rc<AggregateFunctionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldPath(&self) -> Option<Rc<FieldPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn transformFunction(&self) -> Option<Rc<TransformFunctionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CollectionExprContextAttrs<'input> for CollectionExprContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn collectionExpr(&mut self,)
	-> Result<Rc<CollectionExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CollectionExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 204, RULE_collectionExpr);
        let mut _localctx: Rc<CollectionExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(967);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 ANY | ALL | NONE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule predicateFunction*/
					recog.base.set_state(944);
					recog.predicateFunction()?;

					recog.base.set_state(945);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule valueExpr*/
					recog.base.set_state(946);
					recog.valueExpr()?;

					recog.base.set_state(947);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule collectionPredicate*/
					recog.base.set_state(948);
					recog.collectionPredicate()?;

					recog.base.set_state(949);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}

			 SUM | COUNT | AVG | MAX_FN | MIN_FN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule aggregateFunction*/
					recog.base.set_state(951);
					recog.aggregateFunction()?;

					recog.base.set_state(952);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule valueExpr*/
					recog.base.set_state(953);
					recog.valueExpr()?;

					recog.base.set_state(956);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==COMMA {
						{
						recog.base.set_state(954);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						/*InvokeRule fieldPath*/
						recog.base.set_state(955);
						recog.fieldPath()?;

						}
					}

					recog.base.set_state(958);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}

			 FILTER | FIND | DISTINCT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule transformFunction*/
					recog.base.set_state(960);
					recog.transformFunction()?;

					recog.base.set_state(961);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule valueExpr*/
					recog.base.set_state(962);
					recog.valueExpr()?;

					recog.base.set_state(963);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule collectionPredicate*/
					recog.base.set_state(964);
					recog.collectionPredicate()?;

					recog.base.set_state(965);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

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
//------------------- predicateFunction ----------------
pub type PredicateFunctionContextAll<'input> = PredicateFunctionContext<'input>;


pub type PredicateFunctionContext<'input> = BaseParserRuleContext<'input,PredicateFunctionContextExt<'input>>;

#[derive(Clone)]
pub struct PredicateFunctionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for PredicateFunctionContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for PredicateFunctionContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_predicateFunction(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_predicateFunction(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for PredicateFunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_predicateFunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for PredicateFunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_predicateFunction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_predicateFunction }
}
antlr_rust::tid!{PredicateFunctionContextExt<'a>}

impl<'input> PredicateFunctionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PredicateFunctionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PredicateFunctionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PredicateFunctionContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<PredicateFunctionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ANY
/// Returns `None` if there is no child corresponding to token ANY
fn ANY(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(ANY, 0)
}
/// Retrieves first TerminalNode corresponding to token ALL
/// Returns `None` if there is no child corresponding to token ALL
fn ALL(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(ALL, 0)
}
/// Retrieves first TerminalNode corresponding to token NONE
/// Returns `None` if there is no child corresponding to token NONE
fn NONE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(NONE, 0)
}

}

impl<'input> PredicateFunctionContextAttrs<'input> for PredicateFunctionContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn predicateFunction(&mut self,)
	-> Result<Rc<PredicateFunctionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PredicateFunctionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 206, RULE_predicateFunction);
        let mut _localctx: Rc<PredicateFunctionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(969);
			_la = recog.base.input.la(1);
			if { !(((((_la - 57)) & !0x3f) == 0 && ((1usize << (_la - 57)) & ((1usize << (ANY - 57)) | (1usize << (ALL - 57)) | (1usize << (NONE - 57)))) != 0)) } {
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
//------------------- aggregateFunction ----------------
pub type AggregateFunctionContextAll<'input> = AggregateFunctionContext<'input>;


pub type AggregateFunctionContext<'input> = BaseParserRuleContext<'input,AggregateFunctionContextExt<'input>>;

#[derive(Clone)]
pub struct AggregateFunctionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for AggregateFunctionContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for AggregateFunctionContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_aggregateFunction(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_aggregateFunction(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for AggregateFunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_aggregateFunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for AggregateFunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_aggregateFunction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_aggregateFunction }
}
antlr_rust::tid!{AggregateFunctionContextExt<'a>}

impl<'input> AggregateFunctionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AggregateFunctionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AggregateFunctionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AggregateFunctionContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<AggregateFunctionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SUM
/// Returns `None` if there is no child corresponding to token SUM
fn SUM(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(SUM, 0)
}
/// Retrieves first TerminalNode corresponding to token COUNT
/// Returns `None` if there is no child corresponding to token COUNT
fn COUNT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COUNT, 0)
}
/// Retrieves first TerminalNode corresponding to token AVG
/// Returns `None` if there is no child corresponding to token AVG
fn AVG(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(AVG, 0)
}
/// Retrieves first TerminalNode corresponding to token MAX_FN
/// Returns `None` if there is no child corresponding to token MAX_FN
fn MAX_FN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(MAX_FN, 0)
}
/// Retrieves first TerminalNode corresponding to token MIN_FN
/// Returns `None` if there is no child corresponding to token MIN_FN
fn MIN_FN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(MIN_FN, 0)
}

}

impl<'input> AggregateFunctionContextAttrs<'input> for AggregateFunctionContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn aggregateFunction(&mut self,)
	-> Result<Rc<AggregateFunctionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AggregateFunctionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 208, RULE_aggregateFunction);
        let mut _localctx: Rc<AggregateFunctionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(971);
			_la = recog.base.input.la(1);
			if { !(((((_la - 60)) & !0x3f) == 0 && ((1usize << (_la - 60)) & ((1usize << (SUM - 60)) | (1usize << (COUNT - 60)) | (1usize << (AVG - 60)) | (1usize << (MAX_FN - 60)) | (1usize << (MIN_FN - 60)))) != 0)) } {
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
//------------------- transformFunction ----------------
pub type TransformFunctionContextAll<'input> = TransformFunctionContext<'input>;


pub type TransformFunctionContext<'input> = BaseParserRuleContext<'input,TransformFunctionContextExt<'input>>;

#[derive(Clone)]
pub struct TransformFunctionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for TransformFunctionContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for TransformFunctionContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_transformFunction(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_transformFunction(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for TransformFunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_transformFunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for TransformFunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_transformFunction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_transformFunction }
}
antlr_rust::tid!{TransformFunctionContextExt<'a>}

impl<'input> TransformFunctionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TransformFunctionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TransformFunctionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TransformFunctionContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<TransformFunctionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FILTER
/// Returns `None` if there is no child corresponding to token FILTER
fn FILTER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(FILTER, 0)
}
/// Retrieves first TerminalNode corresponding to token FIND
/// Returns `None` if there is no child corresponding to token FIND
fn FIND(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(FIND, 0)
}
/// Retrieves first TerminalNode corresponding to token DISTINCT
/// Returns `None` if there is no child corresponding to token DISTINCT
fn DISTINCT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DISTINCT, 0)
}

}

impl<'input> TransformFunctionContextAttrs<'input> for TransformFunctionContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn transformFunction(&mut self,)
	-> Result<Rc<TransformFunctionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TransformFunctionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 210, RULE_transformFunction);
        let mut _localctx: Rc<TransformFunctionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(973);
			_la = recog.base.input.la(1);
			if { !(((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (FILTER - 65)) | (1usize << (FIND - 65)) | (1usize << (DISTINCT - 65)))) != 0)) } {
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
//------------------- collectionPredicate ----------------
pub type CollectionPredicateContextAll<'input> = CollectionPredicateContext<'input>;


pub type CollectionPredicateContext<'input> = BaseParserRuleContext<'input,CollectionPredicateContextExt<'input>>;

#[derive(Clone)]
pub struct CollectionPredicateContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for CollectionPredicateContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for CollectionPredicateContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_collectionPredicate(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_collectionPredicate(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for CollectionPredicateContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_collectionPredicate(self);
	}
}

impl<'input> CustomRuleContext<'input> for CollectionPredicateContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_collectionPredicate }
	//fn type_rule_index() -> usize where Self: Sized { RULE_collectionPredicate }
}
antlr_rust::tid!{CollectionPredicateContextExt<'a>}

impl<'input> CollectionPredicateContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CollectionPredicateContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CollectionPredicateContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CollectionPredicateContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<CollectionPredicateContextExt<'input>>{

fn lambdaExpression(&self) -> Option<Rc<LambdaExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn collectionPredicateOr(&self) -> Option<Rc<CollectionPredicateOrContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CollectionPredicateContextAttrs<'input> for CollectionPredicateContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn collectionPredicate(&mut self,)
	-> Result<Rc<CollectionPredicateContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CollectionPredicateContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 212, RULE_collectionPredicate);
        let mut _localctx: Rc<CollectionPredicateContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(977);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(75,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule lambdaExpression*/
					recog.base.set_state(975);
					recog.lambdaExpression()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule collectionPredicateOr*/
					recog.base.set_state(976);
					recog.collectionPredicateOr()?;

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
//------------------- collectionPredicateOr ----------------
pub type CollectionPredicateOrContextAll<'input> = CollectionPredicateOrContext<'input>;


pub type CollectionPredicateOrContext<'input> = BaseParserRuleContext<'input,CollectionPredicateOrContextExt<'input>>;

#[derive(Clone)]
pub struct CollectionPredicateOrContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for CollectionPredicateOrContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for CollectionPredicateOrContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_collectionPredicateOr(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_collectionPredicateOr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for CollectionPredicateOrContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_collectionPredicateOr(self);
	}
}

impl<'input> CustomRuleContext<'input> for CollectionPredicateOrContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_collectionPredicateOr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_collectionPredicateOr }
}
antlr_rust::tid!{CollectionPredicateOrContextExt<'a>}

impl<'input> CollectionPredicateOrContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CollectionPredicateOrContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CollectionPredicateOrContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CollectionPredicateOrContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<CollectionPredicateOrContextExt<'input>>{

fn collectionPredicateAnd_all(&self) ->  Vec<Rc<CollectionPredicateAndContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn collectionPredicateAnd(&self, i: usize) -> Option<Rc<CollectionPredicateAndContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token OR in current rule
fn OR_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token OR, starting from 0.
/// Returns `None` if number of children corresponding to token OR is less or equal than `i`.
fn OR(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(OR, i)
}

}

impl<'input> CollectionPredicateOrContextAttrs<'input> for CollectionPredicateOrContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn collectionPredicateOr(&mut self,)
	-> Result<Rc<CollectionPredicateOrContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CollectionPredicateOrContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 214, RULE_collectionPredicateOr);
        let mut _localctx: Rc<CollectionPredicateOrContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule collectionPredicateAnd*/
			recog.base.set_state(979);
			recog.collectionPredicateAnd()?;

			recog.base.set_state(984);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==OR {
				{
				{
				recog.base.set_state(980);
				recog.base.match_token(OR,&mut recog.err_handler)?;

				/*InvokeRule collectionPredicateAnd*/
				recog.base.set_state(981);
				recog.collectionPredicateAnd()?;

				}
				}
				recog.base.set_state(986);
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
//------------------- collectionPredicateAnd ----------------
pub type CollectionPredicateAndContextAll<'input> = CollectionPredicateAndContext<'input>;


pub type CollectionPredicateAndContext<'input> = BaseParserRuleContext<'input,CollectionPredicateAndContextExt<'input>>;

#[derive(Clone)]
pub struct CollectionPredicateAndContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for CollectionPredicateAndContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for CollectionPredicateAndContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_collectionPredicateAnd(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_collectionPredicateAnd(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for CollectionPredicateAndContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_collectionPredicateAnd(self);
	}
}

impl<'input> CustomRuleContext<'input> for CollectionPredicateAndContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_collectionPredicateAnd }
	//fn type_rule_index() -> usize where Self: Sized { RULE_collectionPredicateAnd }
}
antlr_rust::tid!{CollectionPredicateAndContextExt<'a>}

impl<'input> CollectionPredicateAndContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CollectionPredicateAndContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CollectionPredicateAndContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CollectionPredicateAndContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<CollectionPredicateAndContextExt<'input>>{

fn collectionPredicateAtom_all(&self) ->  Vec<Rc<CollectionPredicateAtomContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn collectionPredicateAtom(&self, i: usize) -> Option<Rc<CollectionPredicateAtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token AND in current rule
fn AND_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token AND, starting from 0.
/// Returns `None` if number of children corresponding to token AND is less or equal than `i`.
fn AND(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(AND, i)
}

}

impl<'input> CollectionPredicateAndContextAttrs<'input> for CollectionPredicateAndContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn collectionPredicateAnd(&mut self,)
	-> Result<Rc<CollectionPredicateAndContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CollectionPredicateAndContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 216, RULE_collectionPredicateAnd);
        let mut _localctx: Rc<CollectionPredicateAndContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule collectionPredicateAtom*/
			recog.base.set_state(987);
			recog.collectionPredicateAtom()?;

			recog.base.set_state(992);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==AND {
				{
				{
				recog.base.set_state(988);
				recog.base.match_token(AND,&mut recog.err_handler)?;

				/*InvokeRule collectionPredicateAtom*/
				recog.base.set_state(989);
				recog.collectionPredicateAtom()?;

				}
				}
				recog.base.set_state(994);
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
//------------------- collectionPredicateAtom ----------------
pub type CollectionPredicateAtomContextAll<'input> = CollectionPredicateAtomContext<'input>;


pub type CollectionPredicateAtomContext<'input> = BaseParserRuleContext<'input,CollectionPredicateAtomContextExt<'input>>;

#[derive(Clone)]
pub struct CollectionPredicateAtomContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for CollectionPredicateAtomContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for CollectionPredicateAtomContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_collectionPredicateAtom(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_collectionPredicateAtom(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for CollectionPredicateAtomContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_collectionPredicateAtom(self);
	}
}

impl<'input> CustomRuleContext<'input> for CollectionPredicateAtomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_collectionPredicateAtom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_collectionPredicateAtom }
}
antlr_rust::tid!{CollectionPredicateAtomContextExt<'a>}

impl<'input> CollectionPredicateAtomContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CollectionPredicateAtomContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CollectionPredicateAtomContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CollectionPredicateAtomContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<CollectionPredicateAtomContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token NOT
/// Returns `None` if there is no child corresponding to token NOT
fn NOT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(NOT, 0)
}
fn collectionPredicateAtom(&self) -> Option<Rc<CollectionPredicateAtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldPath(&self) -> Option<Rc<FieldPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn comparisonOp(&self) -> Option<Rc<ComparisonOpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn valueExpr(&self) -> Option<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IN, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn valueList(&self) -> Option<Rc<ValueListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token IS
/// Returns `None` if there is no child corresponding to token IS
fn IS(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IS, 0)
}
/// Retrieves first TerminalNode corresponding to token NULL
/// Returns `None` if there is no child corresponding to token NULL
fn NULL(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(NULL, 0)
}
fn collectionPredicateOr(&self) -> Option<Rc<CollectionPredicateOrContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CollectionPredicateAtomContextAttrs<'input> for CollectionPredicateAtomContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn collectionPredicateAtom(&mut self,)
	-> Result<Rc<CollectionPredicateAtomContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CollectionPredicateAtomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 218, RULE_collectionPredicateAtom);
        let mut _localctx: Rc<CollectionPredicateAtomContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1027);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(78,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(995);
					recog.base.match_token(NOT,&mut recog.err_handler)?;

					/*InvokeRule collectionPredicateAtom*/
					recog.base.set_state(996);
					recog.collectionPredicateAtom()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule fieldPath*/
					recog.base.set_state(997);
					recog.fieldPath()?;

					/*InvokeRule comparisonOp*/
					recog.base.set_state(998);
					recog.comparisonOp()?;

					/*InvokeRule valueExpr*/
					recog.base.set_state(999);
					recog.valueExpr()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule fieldPath*/
					recog.base.set_state(1001);
					recog.fieldPath()?;

					recog.base.set_state(1002);
					recog.base.match_token(IN,&mut recog.err_handler)?;

					recog.base.set_state(1003);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule valueList*/
					recog.base.set_state(1004);
					recog.valueList()?;

					recog.base.set_state(1005);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule fieldPath*/
					recog.base.set_state(1007);
					recog.fieldPath()?;

					recog.base.set_state(1008);
					recog.base.match_token(NOT,&mut recog.err_handler)?;

					recog.base.set_state(1009);
					recog.base.match_token(IN,&mut recog.err_handler)?;

					recog.base.set_state(1010);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule valueList*/
					recog.base.set_state(1011);
					recog.valueList()?;

					recog.base.set_state(1012);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule fieldPath*/
					recog.base.set_state(1014);
					recog.fieldPath()?;

					recog.base.set_state(1015);
					recog.base.match_token(IS,&mut recog.err_handler)?;

					recog.base.set_state(1016);
					recog.base.match_token(NULL,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule fieldPath*/
					recog.base.set_state(1018);
					recog.fieldPath()?;

					recog.base.set_state(1019);
					recog.base.match_token(IS,&mut recog.err_handler)?;

					recog.base.set_state(1020);
					recog.base.match_token(NOT,&mut recog.err_handler)?;

					recog.base.set_state(1021);
					recog.base.match_token(NULL,&mut recog.err_handler)?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					recog.base.set_state(1023);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule collectionPredicateOr*/
					recog.base.set_state(1024);
					recog.collectionPredicateOr()?;

					recog.base.set_state(1025);
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
//------------------- lambdaExpression ----------------
pub type LambdaExpressionContextAll<'input> = LambdaExpressionContext<'input>;


pub type LambdaExpressionContext<'input> = BaseParserRuleContext<'input,LambdaExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct LambdaExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for LambdaExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for LambdaExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lambdaExpression(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_lambdaExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for LambdaExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_lambdaExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for LambdaExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lambdaExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lambdaExpression }
}
antlr_rust::tid!{LambdaExpressionContextExt<'a>}

impl<'input> LambdaExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LambdaExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LambdaExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LambdaExpressionContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<LambdaExpressionContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves first TerminalNode corresponding to token ARROW
/// Returns `None` if there is no child corresponding to token ARROW
fn ARROW(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(ARROW, 0)
}
fn valueExpr(&self) -> Option<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> LambdaExpressionContextAttrs<'input> for LambdaExpressionContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lambdaExpression(&mut self,)
	-> Result<Rc<LambdaExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LambdaExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 220, RULE_lambdaExpression);
        let mut _localctx: Rc<LambdaExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1044);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1029);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(1030);
					recog.base.match_token(ARROW,&mut recog.err_handler)?;

					/*InvokeRule valueExpr*/
					recog.base.set_state(1031);
					recog.valueExpr()?;

					}
				}

			 LPAREN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1032);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					recog.base.set_state(1033);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(1038);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==COMMA {
						{
						{
						recog.base.set_state(1034);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						recog.base.set_state(1035);
						recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(1040);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(1041);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					recog.base.set_state(1042);
					recog.base.match_token(ARROW,&mut recog.err_handler)?;

					/*InvokeRule valueExpr*/
					recog.base.set_state(1043);
					recog.valueExpr()?;

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
//------------------- arithmeticExpr ----------------
pub type ArithmeticExprContextAll<'input> = ArithmeticExprContext<'input>;


pub type ArithmeticExprContext<'input> = BaseParserRuleContext<'input,ArithmeticExprContextExt<'input>>;

#[derive(Clone)]
pub struct ArithmeticExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ArithmeticExprContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ArithmeticExprContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_arithmeticExpr(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_arithmeticExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ArithmeticExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_arithmeticExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArithmeticExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arithmeticExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arithmeticExpr }
}
antlr_rust::tid!{ArithmeticExprContextExt<'a>}

impl<'input> ArithmeticExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ArithmeticExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArithmeticExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ArithmeticExprContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ArithmeticExprContextExt<'input>>{

fn valueExpr(&self) -> Option<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ArithmeticExprContextAttrs<'input> for ArithmeticExprContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn arithmeticExpr(&mut self,)
	-> Result<Rc<ArithmeticExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArithmeticExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 222, RULE_arithmeticExpr);
        let mut _localctx: Rc<ArithmeticExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule valueExpr*/
			recog.base.set_state(1046);
			recog.valueExpr()?;

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

impl<'input> RulesDSLParserContext<'input> for FunctionCallContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for FunctionCallContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_functionCall(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_functionCall(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for FunctionCallContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_functionCall(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionCall }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionCall }
}
antlr_rust::tid!{FunctionCallContextExt<'a>}

impl<'input> FunctionCallContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionCallContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionCallContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionCallContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<FunctionCallContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn valueExpr_all(&self) ->  Vec<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn valueExpr(&self, i: usize) -> Option<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> FunctionCallContextAttrs<'input> for FunctionCallContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionCall(&mut self,)
	-> Result<Rc<FunctionCallContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionCallContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 224, RULE_functionCall);
        let mut _localctx: Rc<FunctionCallContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1048);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(1049);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			recog.base.set_state(1058);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 34)) & !0x3f) == 0 && ((1usize << (_la - 34)) & ((1usize << (NULL - 34)) | (1usize << (ANY - 34)) | (1usize << (ALL - 34)) | (1usize << (NONE - 34)) | (1usize << (SUM - 34)) | (1usize << (COUNT - 34)) | (1usize << (AVG - 34)) | (1usize << (MAX_FN - 34)) | (1usize << (MIN_FN - 34)) | (1usize << (FILTER - 34)))) != 0) || ((((_la - 66)) & !0x3f) == 0 && ((1usize << (_la - 66)) & ((1usize << (FIND - 66)) | (1usize << (DISTINCT - 66)) | (1usize << (MINUS - 66)))) != 0) || ((((_la - 107)) & !0x3f) == 0 && ((1usize << (_la - 107)) & ((1usize << (LPAREN - 107)) | (1usize << (LBRACKET - 107)) | (1usize << (LBRACE - 107)) | (1usize << (BOOLEAN - 107)) | (1usize << (INTEGER - 107)) | (1usize << (DECIMAL - 107)) | (1usize << (MONEY_LITERAL - 107)) | (1usize << (PERCENTAGE_LITERAL - 107)) | (1usize << (DQUOTED_STRING - 107)) | (1usize << (SQUOTED_STRING - 107)) | (1usize << (IDENTIFIER - 107)))) != 0) {
				{
				/*InvokeRule valueExpr*/
				recog.base.set_state(1050);
				recog.valueExpr()?;

				recog.base.set_state(1055);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==COMMA {
					{
					{
					recog.base.set_state(1051);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule valueExpr*/
					recog.base.set_state(1052);
					recog.valueExpr()?;

					}
					}
					recog.base.set_state(1057);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(1060);
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
//------------------- fieldPath ----------------
pub type FieldPathContextAll<'input> = FieldPathContext<'input>;


pub type FieldPathContext<'input> = BaseParserRuleContext<'input,FieldPathContextExt<'input>>;

#[derive(Clone)]
pub struct FieldPathContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for FieldPathContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for FieldPathContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fieldPath(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_fieldPath(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for FieldPathContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_fieldPath(self);
	}
}

impl<'input> CustomRuleContext<'input> for FieldPathContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fieldPath }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fieldPath }
}
antlr_rust::tid!{FieldPathContextExt<'a>}

impl<'input> FieldPathContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FieldPathContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FieldPathContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FieldPathContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<FieldPathContextExt<'input>>{

fn attributeIdentifier_all(&self) ->  Vec<Rc<AttributeIdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attributeIdentifier(&self, i: usize) -> Option<Rc<AttributeIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
/// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, i)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACKET
/// Returns `None` if there is no child corresponding to token LBRACKET
fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LBRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACKET
/// Returns `None` if there is no child corresponding to token RBRACKET
fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RBRACKET, 0)
}

}

impl<'input> FieldPathContextAttrs<'input> for FieldPathContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fieldPath(&mut self,)
	-> Result<Rc<FieldPathContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FieldPathContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 226, RULE_fieldPath);
        let mut _localctx: Rc<FieldPathContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1081);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(85,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule attributeIdentifier*/
					recog.base.set_state(1062);
					recog.attributeIdentifier()?;

					recog.base.set_state(1067);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==DOT {
						{
						{
						recog.base.set_state(1063);
						recog.base.match_token(DOT,&mut recog.err_handler)?;

						/*InvokeRule attributeIdentifier*/
						recog.base.set_state(1064);
						recog.attributeIdentifier()?;

						}
						}
						recog.base.set_state(1069);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1070);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(1071);
					recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

					recog.base.set_state(1072);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					recog.base.set_state(1073);
					recog.base.match_token(RBRACKET,&mut recog.err_handler)?;

					recog.base.set_state(1078);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==DOT {
						{
						{
						recog.base.set_state(1074);
						recog.base.match_token(DOT,&mut recog.err_handler)?;

						/*InvokeRule attributeIdentifier*/
						recog.base.set_state(1075);
						recog.attributeIdentifier()?;

						}
						}
						recog.base.set_state(1080);
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
//------------------- attributeIdentifier ----------------
pub type AttributeIdentifierContextAll<'input> = AttributeIdentifierContext<'input>;


pub type AttributeIdentifierContext<'input> = BaseParserRuleContext<'input,AttributeIdentifierContextExt<'input>>;

#[derive(Clone)]
pub struct AttributeIdentifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for AttributeIdentifierContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for AttributeIdentifierContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_attributeIdentifier(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_attributeIdentifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for AttributeIdentifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_attributeIdentifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for AttributeIdentifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_attributeIdentifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_attributeIdentifier }
}
antlr_rust::tid!{AttributeIdentifierContextExt<'a>}

impl<'input> AttributeIdentifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AttributeIdentifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AttributeIdentifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AttributeIdentifierContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<AttributeIdentifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token DQUOTED_STRING
/// Returns `None` if there is no child corresponding to token DQUOTED_STRING
fn DQUOTED_STRING(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DQUOTED_STRING, 0)
}

}

impl<'input> AttributeIdentifierContextAttrs<'input> for AttributeIdentifierContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn attributeIdentifier(&mut self,)
	-> Result<Rc<AttributeIdentifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AttributeIdentifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 228, RULE_attributeIdentifier);
        let mut _localctx: Rc<AttributeIdentifierContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1083);
			_la = recog.base.input.la(1);
			if { !(_la==DQUOTED_STRING || _la==IDENTIFIER) } {
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
//------------------- valueList ----------------
pub type ValueListContextAll<'input> = ValueListContext<'input>;


pub type ValueListContext<'input> = BaseParserRuleContext<'input,ValueListContextExt<'input>>;

#[derive(Clone)]
pub struct ValueListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ValueListContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ValueListContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_valueList(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_valueList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ValueListContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_valueList(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_valueList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_valueList }
}
antlr_rust::tid!{ValueListContextExt<'a>}

impl<'input> ValueListContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValueListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValueListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ValueListContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ValueListContextExt<'input>>{

fn valueExpr_all(&self) ->  Vec<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn valueExpr(&self, i: usize) -> Option<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ValueListContextAttrs<'input> for ValueListContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn valueList(&mut self,)
	-> Result<Rc<ValueListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValueListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 230, RULE_valueList);
        let mut _localctx: Rc<ValueListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule valueExpr*/
			recog.base.set_state(1085);
			recog.valueExpr()?;

			recog.base.set_state(1090);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(1086);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule valueExpr*/
				recog.base.set_state(1087);
				recog.valueExpr()?;

				}
				}
				recog.base.set_state(1092);
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
//------------------- listLiteral ----------------
pub type ListLiteralContextAll<'input> = ListLiteralContext<'input>;


pub type ListLiteralContext<'input> = BaseParserRuleContext<'input,ListLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct ListLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ListLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ListLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_listLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_listLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ListLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_listLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for ListLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listLiteral }
}
antlr_rust::tid!{ListLiteralContextExt<'a>}

impl<'input> ListLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ListLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ListLiteralContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ListLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACKET
/// Returns `None` if there is no child corresponding to token LBRACKET
fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LBRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACKET
/// Returns `None` if there is no child corresponding to token RBRACKET
fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RBRACKET, 0)
}
fn valueExpr_all(&self) ->  Vec<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn valueExpr(&self, i: usize) -> Option<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ListLiteralContextAttrs<'input> for ListLiteralContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn listLiteral(&mut self,)
	-> Result<Rc<ListLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ListLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 232, RULE_listLiteral);
        let mut _localctx: Rc<ListLiteralContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1093);
			recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

			recog.base.set_state(1102);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 34)) & !0x3f) == 0 && ((1usize << (_la - 34)) & ((1usize << (NULL - 34)) | (1usize << (ANY - 34)) | (1usize << (ALL - 34)) | (1usize << (NONE - 34)) | (1usize << (SUM - 34)) | (1usize << (COUNT - 34)) | (1usize << (AVG - 34)) | (1usize << (MAX_FN - 34)) | (1usize << (MIN_FN - 34)) | (1usize << (FILTER - 34)))) != 0) || ((((_la - 66)) & !0x3f) == 0 && ((1usize << (_la - 66)) & ((1usize << (FIND - 66)) | (1usize << (DISTINCT - 66)) | (1usize << (MINUS - 66)))) != 0) || ((((_la - 107)) & !0x3f) == 0 && ((1usize << (_la - 107)) & ((1usize << (LPAREN - 107)) | (1usize << (LBRACKET - 107)) | (1usize << (LBRACE - 107)) | (1usize << (BOOLEAN - 107)) | (1usize << (INTEGER - 107)) | (1usize << (DECIMAL - 107)) | (1usize << (MONEY_LITERAL - 107)) | (1usize << (PERCENTAGE_LITERAL - 107)) | (1usize << (DQUOTED_STRING - 107)) | (1usize << (SQUOTED_STRING - 107)) | (1usize << (IDENTIFIER - 107)))) != 0) {
				{
				/*InvokeRule valueExpr*/
				recog.base.set_state(1094);
				recog.valueExpr()?;

				recog.base.set_state(1099);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==COMMA {
					{
					{
					recog.base.set_state(1095);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule valueExpr*/
					recog.base.set_state(1096);
					recog.valueExpr()?;

					}
					}
					recog.base.set_state(1101);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(1104);
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
//------------------- objectLiteral ----------------
pub type ObjectLiteralContextAll<'input> = ObjectLiteralContext<'input>;


pub type ObjectLiteralContext<'input> = BaseParserRuleContext<'input,ObjectLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct ObjectLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> RulesDSLParserContext<'input> for ObjectLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ObjectLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_objectLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_objectLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ObjectLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_objectLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for ObjectLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_objectLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_objectLiteral }
}
antlr_rust::tid!{ObjectLiteralContextExt<'a>}

impl<'input> ObjectLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ObjectLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ObjectLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ObjectLiteralContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ObjectLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn objectField_all(&self) ->  Vec<Rc<ObjectFieldContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn objectField(&self, i: usize) -> Option<Rc<ObjectFieldContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,RulesDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ObjectLiteralContextAttrs<'input> for ObjectLiteralContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn objectLiteral(&mut self,)
	-> Result<Rc<ObjectLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ObjectLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 234, RULE_objectLiteral);
        let mut _localctx: Rc<ObjectLiteralContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1106);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(1115);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==RETURN || _la==DEFAULT || _la==TEXT_TYPE || _la==DESCRIPTION || _la==PRIORITY || _la==DQUOTED_STRING || _la==IDENTIFIER {
				{
				/*InvokeRule objectField*/
				recog.base.set_state(1107);
				recog.objectField()?;

				recog.base.set_state(1112);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==COMMA {
					{
					{
					recog.base.set_state(1108);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule objectField*/
					recog.base.set_state(1109);
					recog.objectField()?;

					}
					}
					recog.base.set_state(1114);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(1117);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

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

impl<'input> RulesDSLParserContext<'input> for ObjectFieldContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ObjectFieldContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_objectField(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_objectField(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ObjectFieldContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_objectField(self);
	}
}

impl<'input> CustomRuleContext<'input> for ObjectFieldContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_objectField }
	//fn type_rule_index() -> usize where Self: Sized { RULE_objectField }
}
antlr_rust::tid!{ObjectFieldContextExt<'a>}

impl<'input> ObjectFieldContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ObjectFieldContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ObjectFieldContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ObjectFieldContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ObjectFieldContextExt<'input>>{

fn objectFieldName(&self) -> Option<Rc<ObjectFieldNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn valueExpr(&self) -> Option<Rc<ValueExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ObjectFieldContextAttrs<'input> for ObjectFieldContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn objectField(&mut self,)
	-> Result<Rc<ObjectFieldContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ObjectFieldContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 236, RULE_objectField);
        let mut _localctx: Rc<ObjectFieldContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule objectFieldName*/
			recog.base.set_state(1119);
			recog.objectFieldName()?;

			recog.base.set_state(1120);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule valueExpr*/
			recog.base.set_state(1121);
			recog.valueExpr()?;

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

impl<'input> RulesDSLParserContext<'input> for ObjectFieldNameContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for ObjectFieldNameContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_objectFieldName(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_objectFieldName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for ObjectFieldNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_objectFieldName(self);
	}
}

impl<'input> CustomRuleContext<'input> for ObjectFieldNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_objectFieldName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_objectFieldName }
}
antlr_rust::tid!{ObjectFieldNameContextExt<'a>}

impl<'input> ObjectFieldNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ObjectFieldNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ObjectFieldNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ObjectFieldNameContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<ObjectFieldNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token DQUOTED_STRING
/// Returns `None` if there is no child corresponding to token DQUOTED_STRING
fn DQUOTED_STRING(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DQUOTED_STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token TEXT_TYPE
/// Returns `None` if there is no child corresponding to token TEXT_TYPE
fn TEXT_TYPE(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(TEXT_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token DESCRIPTION
/// Returns `None` if there is no child corresponding to token DESCRIPTION
fn DESCRIPTION(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DESCRIPTION, 0)
}
/// Retrieves first TerminalNode corresponding to token PRIORITY
/// Returns `None` if there is no child corresponding to token PRIORITY
fn PRIORITY(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(PRIORITY, 0)
}
/// Retrieves first TerminalNode corresponding to token RETURN
/// Returns `None` if there is no child corresponding to token RETURN
fn RETURN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(RETURN, 0)
}
/// Retrieves first TerminalNode corresponding to token DEFAULT
/// Returns `None` if there is no child corresponding to token DEFAULT
fn DEFAULT(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DEFAULT, 0)
}

}

impl<'input> ObjectFieldNameContextAttrs<'input> for ObjectFieldNameContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn objectFieldName(&mut self,)
	-> Result<Rc<ObjectFieldNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ObjectFieldNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 238, RULE_objectFieldName);
        let mut _localctx: Rc<ObjectFieldNameContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1123);
			_la = recog.base.input.la(1);
			if { !(_la==RETURN || _la==DEFAULT || _la==TEXT_TYPE || _la==DESCRIPTION || _la==PRIORITY || _la==DQUOTED_STRING || _la==IDENTIFIER) } {
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

impl<'input> RulesDSLParserContext<'input> for LiteralContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for LiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_literal(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_literal(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for LiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_literal(self);
	}
}

impl<'input> CustomRuleContext<'input> for LiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literal }
}
antlr_rust::tid!{LiteralContextExt<'a>}

impl<'input> LiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LiteralContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<LiteralContextExt<'input>>{

fn stringLiteral(&self) -> Option<Rc<StringLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn numberLiteral(&self) -> Option<Rc<NumberLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token MONEY_LITERAL
/// Returns `None` if there is no child corresponding to token MONEY_LITERAL
fn MONEY_LITERAL(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(MONEY_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token PERCENTAGE_LITERAL
/// Returns `None` if there is no child corresponding to token PERCENTAGE_LITERAL
fn PERCENTAGE_LITERAL(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(PERCENTAGE_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token BOOLEAN
/// Returns `None` if there is no child corresponding to token BOOLEAN
fn BOOLEAN(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(BOOLEAN, 0)
}
/// Retrieves first TerminalNode corresponding to token NULL
/// Returns `None` if there is no child corresponding to token NULL
fn NULL(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(NULL, 0)
}

}

impl<'input> LiteralContextAttrs<'input> for LiteralContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn literal(&mut self,)
	-> Result<Rc<LiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 240, RULE_literal);
        let mut _localctx: Rc<LiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1131);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 DQUOTED_STRING | SQUOTED_STRING 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule stringLiteral*/
					recog.base.set_state(1125);
					recog.stringLiteral()?;

					}
				}

			 MINUS | INTEGER | DECIMAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule numberLiteral*/
					recog.base.set_state(1126);
					recog.numberLiteral()?;

					}
				}

			 MONEY_LITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(1127);
					recog.base.match_token(MONEY_LITERAL,&mut recog.err_handler)?;

					}
				}

			 PERCENTAGE_LITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(1128);
					recog.base.match_token(PERCENTAGE_LITERAL,&mut recog.err_handler)?;

					}
				}

			 BOOLEAN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(1129);
					recog.base.match_token(BOOLEAN,&mut recog.err_handler)?;

					}
				}

			 NULL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(1130);
					recog.base.match_token(NULL,&mut recog.err_handler)?;

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

impl<'input> RulesDSLParserContext<'input> for NumberLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn RulesDSLListener<'input> + 'a> for NumberLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_numberLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn RulesDSLListener<'input> + 'a)) {
			listener.exit_numberLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn RulesDSLVisitor<'input> + 'a> for NumberLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn RulesDSLVisitor<'input> + 'a)) {
		visitor.visit_numberLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumberLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = RulesDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_numberLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_numberLiteral }
}
antlr_rust::tid!{NumberLiteralContextExt<'a>}

impl<'input> NumberLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn RulesDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NumberLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NumberLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NumberLiteralContextAttrs<'input>: RulesDSLParserContext<'input> + BorrowMut<NumberLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
/// Retrieves first TerminalNode corresponding to token DECIMAL
/// Returns `None` if there is no child corresponding to token DECIMAL
fn DECIMAL(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(DECIMAL, 0)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,RulesDSLParserContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}

}

impl<'input> NumberLiteralContextAttrs<'input> for NumberLiteralContext<'input>{}

impl<'input, I, H> RulesDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn numberLiteral(&mut self,)
	-> Result<Rc<NumberLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NumberLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 242, RULE_numberLiteral);
        let mut _localctx: Rc<NumberLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1139);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(92,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1133);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1134);
					recog.base.match_token(DECIMAL,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(1135);
					recog.base.match_token(MINUS,&mut recog.err_handler)?;

					recog.base.set_state(1136);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(1137);
					recog.base.match_token(MINUS,&mut recog.err_handler)?;

					recog.base.set_state(1138);
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
	\u{81}\u{478}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
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
	\x64\x09\x64\x04\x65\x09\x65\x04\x66\x09\x66\x04\x67\x09\x67\x04\x68\x09\
	\x68\x04\x69\x09\x69\x04\x6a\x09\x6a\x04\x6b\x09\x6b\x04\x6c\x09\x6c\x04\
	\x6d\x09\x6d\x04\x6e\x09\x6e\x04\x6f\x09\x6f\x04\x70\x09\x70\x04\x71\x09\
	\x71\x04\x72\x09\x72\x04\x73\x09\x73\x04\x74\x09\x74\x04\x75\x09\x75\x04\
	\x76\x09\x76\x04\x77\x09\x77\x04\x78\x09\x78\x04\x79\x09\x79\x04\x7a\x09\
	\x7a\x04\x7b\x09\x7b\x03\x02\x07\x02\u{f8}\x0a\x02\x0c\x02\x0e\x02\u{fb}\
	\x0b\x02\x03\x02\x05\x02\u{fe}\x0a\x02\x03\x02\x05\x02\u{101}\x0a\x02\x03\
	\x02\x03\x02\x06\x02\u{105}\x0a\x02\x0d\x02\x0e\x02\u{106}\x03\x02\x03\x02\
	\x03\x03\x03\x03\x03\x03\x03\x04\x06\x04\u{10f}\x0a\x04\x0d\x04\x0e\x04\
	\u{110}\x03\x04\x03\x04\x03\x05\x03\x05\x03\x06\x03\x06\x03\x06\x03\x07\
	\x03\x07\x03\x07\x06\x07\u{11d}\x0a\x07\x0d\x07\x0e\x07\u{11e}\x03\x07\x03\
	\x07\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x05\
	\x08\u{12b}\x0a\x08\x03\x08\x03\x08\x03\x08\x03\x08\x05\x08\u{131}\x0a\x08\
	\x03\x09\x03\x09\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0c\
	\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x05\x0c\u{140}\x0a\x0c\x03\x0d\x03\x0d\
	\x03\x0d\x07\x0d\u{145}\x0a\x0d\x0c\x0d\x0e\x0d\u{148}\x0b\x0d\x03\x0e\x03\
	\x0e\x03\x0e\x03\x0e\x03\x0f\x03\x0f\x03\x10\x06\x10\u{151}\x0a\x10\x0d\
	\x10\x0e\x10\u{152}\x03\x11\x03\x11\x03\x11\x03\x11\x03\x11\x03\x11\x03\
	\x11\x03\x11\x03\x11\x05\x11\u{15e}\x0a\x11\x03\x12\x03\x12\x03\x12\x03\
	\x13\x03\x13\x03\x14\x03\x14\x03\x14\x06\x14\u{168}\x0a\x14\x0d\x14\x0e\
	\x14\u{169}\x03\x14\x03\x14\x03\x15\x03\x15\x03\x15\x05\x15\u{171}\x0a\x15\
	\x03\x15\x03\x15\x03\x15\x03\x15\x03\x16\x03\x16\x03\x17\x03\x17\x03\x17\
	\x07\x17\u{17c}\x0a\x17\x0c\x17\x0e\x17\u{17f}\x0b\x17\x03\x18\x03\x18\x03\
	\x18\x03\x18\x03\x19\x03\x19\x03\x19\x03\x19\x05\x19\u{189}\x0a\x19\x03\
	\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\
	\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x05\x1c\u{19a}\x0a\x1c\x03\
	\x1d\x03\x1d\x03\x1e\x03\x1e\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\
	\x20\x03\x20\x03\x20\x05\x20\u{1a8}\x0a\x20\x03\x20\x05\x20\u{1ab}\x0a\x20\
	\x03\x20\x05\x20\u{1ae}\x0a\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\
	\x05\x20\u{1b5}\x0a\x20\x03\x20\x05\x20\u{1b8}\x0a\x20\x03\x20\x05\x20\u{1bb}\
	\x0a\x20\x03\x20\x03\x20\x03\x21\x03\x21\x03\x21\x03\x21\x03\x22\x03\x22\
	\x03\x23\x03\x23\x03\x23\x03\x24\x03\x24\x03\x25\x03\x25\x03\x25\x03\x26\
	\x03\x26\x03\x27\x03\x27\x03\x27\x06\x27\u{1d2}\x0a\x27\x0d\x27\x0e\x27\
	\u{1d3}\x03\x28\x03\x28\x03\x28\x03\x28\x05\x28\u{1da}\x0a\x28\x03\x29\x03\
	\x29\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x05\x2a\u{1e2}\x0a\x2a\x03\x2b\x03\
	\x2b\x03\x2c\x03\x2c\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2e\x03\x2e\x03\
	\x2e\x06\x2e\u{1ef}\x0a\x2e\x0d\x2e\x0e\x2e\u{1f0}\x03\x2f\x03\x2f\x05\x2f\
	\u{1f5}\x0a\x2f\x03\x2f\x06\x2f\u{1f8}\x0a\x2f\x0d\x2f\x0e\x2f\u{1f9}\x03\
	\x30\x03\x30\x03\x30\x03\x31\x03\x31\x03\x31\x03\x32\x03\x32\x03\x33\x03\
	\x33\x03\x33\x03\x33\x03\x34\x03\x34\x05\x34\u{20a}\x0a\x34\x03\x34\x06\
	\x34\u{20d}\x0a\x34\x0d\x34\x0e\x34\u{20e}\x03\x35\x03\x35\x03\x35\x03\x36\
	\x03\x36\x03\x36\x03\x37\x03\x37\x05\x37\u{219}\x0a\x37\x03\x38\x03\x38\
	\x03\x38\x03\x38\x03\x38\x03\x38\x03\x38\x03\x38\x03\x38\x05\x38\u{224}\
	\x0a\x38\x03\x39\x03\x39\x03\x3a\x03\x3a\x03\x3a\x03\x3a\x03\x3a\x03\x3a\
	\x03\x3a\x05\x3a\u{22f}\x0a\x3a\x03\x3b\x03\x3b\x03\x3b\x03\x3b\x03\x3b\
	\x03\x3b\x03\x3b\x03\x3b\x03\x3b\x03\x3b\x03\x3b\x05\x3b\u{23c}\x0a\x3b\
	\x03\x3c\x03\x3c\x03\x3c\x03\x3c\x03\x3c\x03\x3c\x03\x3c\x03\x3c\x05\x3c\
	\u{246}\x0a\x3c\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\
	\x05\x3d\u{24f}\x0a\x3d\x03\x3e\x03\x3e\x03\x3e\x03\x3f\x03\x3f\x03\x3f\
	\x03\x3f\x03\x40\x03\x40\x03\x40\x03\x40\x03\x40\x03\x40\x03\x40\x03\x40\
	\x03\x40\x03\x40\x05\x40\u{262}\x0a\x40\x03\x41\x03\x41\x03\x41\x03\x41\
	\x03\x41\x03\x41\x05\x41\u{26a}\x0a\x41\x03\x42\x03\x42\x03\x43\x03\x43\
	\x03\x44\x03\x44\x03\x44\x03\x44\x03\x44\x07\x44\u{275}\x0a\x44\x0c\x44\
	\x0e\x44\u{278}\x0b\x44\x03\x44\x03\x44\x03\x44\x03\x44\x05\x44\u{27e}\x0a\
	\x44\x03\x44\x03\x44\x03\x44\x03\x44\x03\x44\x03\x44\x03\x44\x03\x44\x03\
	\x44\x03\x44\x05\x44\u{28a}\x0a\x44\x03\x45\x03\x45\x03\x45\x03\x45\x03\
	\x45\x07\x45\u{291}\x0a\x45\x0c\x45\x0e\x45\u{294}\x0b\x45\x05\x45\u{296}\
	\x0a\x45\x03\x45\x03\x45\x03\x46\x03\x46\x03\x46\x03\x46\x05\x46\u{29e}\
	\x0a\x46\x03\x47\x03\x47\x03\x47\x03\x47\x03\x48\x03\x48\x03\x48\x06\x48\
	\u{2a7}\x0a\x48\x0d\x48\x0e\x48\u{2a8}\x03\x49\x03\x49\x03\x49\x03\x49\x03\
	\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4b\x03\x4b\x03\x4c\x03\x4c\x03\x4c\x03\
	\x4d\x03\x4d\x03\x4d\x06\x4d\u{2bb}\x0a\x4d\x0d\x4d\x0e\x4d\u{2bc}\x03\x4e\
	\x03\x4e\x05\x4e\u{2c1}\x0a\x4e\x03\x4f\x03\x4f\x03\x4f\x03\x4f\x03\x4f\
	\x03\x4f\x05\x4f\u{2c9}\x0a\x4f\x03\x50\x03\x50\x03\x50\x06\x50\u{2ce}\x0a\
	\x50\x0d\x50\x0e\x50\u{2cf}\x03\x51\x03\x51\x03\x51\x03\x51\x03\x52\x03\
	\x52\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\x07\x52\u{2df}\
	\x0a\x52\x0c\x52\x0e\x52\u{2e2}\x0b\x52\x03\x52\x03\x52\x03\x52\x03\x53\
	\x03\x53\x03\x53\x03\x53\x05\x53\u{2eb}\x0a\x53\x03\x53\x06\x53\u{2ee}\x0a\
	\x53\x0d\x53\x0e\x53\u{2ef}\x03\x53\x03\x53\x03\x54\x03\x54\x03\x55\x03\
	\x55\x03\x55\x03\x55\x03\x55\x05\x55\u{2fb}\x0a\x55\x03\x56\x03\x56\x03\
	\x56\x03\x56\x03\x56\x03\x57\x03\x57\x03\x57\x03\x57\x03\x57\x03\x58\x03\
	\x58\x03\x58\x03\x58\x03\x58\x03\x58\x03\x58\x03\x58\x03\x58\x07\x58\u{310}\
	\x0a\x58\x0c\x58\x0e\x58\u{313}\x0b\x58\x03\x58\x03\x58\x05\x58\u{317}\x0a\
	\x58\x03\x58\x03\x58\x03\x59\x06\x59\u{31c}\x0a\x59\x0d\x59\x0e\x59\u{31d}\
	\x03\x5a\x03\x5a\x03\x5a\x07\x5a\u{323}\x0a\x5a\x0c\x5a\x0e\x5a\u{326}\x0b\
	\x5a\x03\x5b\x03\x5b\x03\x5b\x05\x5b\u{32b}\x0a\x5b\x03\x5b\x05\x5b\u{32e}\
	\x0a\x5b\x03\x5b\x03\x5b\x03\x5b\x05\x5b\u{333}\x0a\x5b\x03\x5b\x05\x5b\
	\u{336}\x0a\x5b\x05\x5b\u{338}\x0a\x5b\x03\x5c\x03\x5c\x03\x5c\x07\x5c\u{33d}\
	\x0a\x5c\x0c\x5c\x0e\x5c\u{340}\x0b\x5c\x03\x5d\x03\x5d\x03\x5e\x03\x5e\
	\x03\x5f\x03\x5f\x03\x5f\x07\x5f\u{349}\x0a\x5f\x0c\x5f\x0e\x5f\u{34c}\x0b\
	\x5f\x03\x60\x05\x60\u{34f}\x0a\x60\x03\x60\x03\x60\x03\x61\x03\x61\x03\
	\x61\x03\x61\x03\x61\x03\x61\x05\x61\u{359}\x0a\x61\x03\x62\x03\x62\x03\
	\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\
	\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\
	\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\
	\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\
	\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\
	\x62\x03\x62\x03\x62\x05\x62\u{38d}\x0a\x62\x03\x63\x03\x63\x03\x64\x03\
	\x64\x03\x64\x07\x64\u{394}\x0a\x64\x0c\x64\x0e\x64\u{397}\x0b\x64\x03\x65\
	\x03\x65\x03\x65\x07\x65\u{39c}\x0a\x65\x0c\x65\x0e\x65\u{39f}\x0b\x65\x03\
	\x66\x05\x66\u{3a2}\x0a\x66\x03\x66\x03\x66\x03\x67\x03\x67\x03\x67\x03\
	\x67\x03\x67\x03\x67\x03\x67\x03\x67\x03\x67\x03\x67\x03\x67\x05\x67\u{3b1}\
	\x0a\x67\x03\x68\x03\x68\x03\x68\x03\x68\x03\x68\x03\x68\x03\x68\x03\x68\
	\x03\x68\x03\x68\x03\x68\x03\x68\x05\x68\u{3bf}\x0a\x68\x03\x68\x03\x68\
	\x03\x68\x03\x68\x03\x68\x03\x68\x03\x68\x03\x68\x03\x68\x05\x68\u{3ca}\
	\x0a\x68\x03\x69\x03\x69\x03\x6a\x03\x6a\x03\x6b\x03\x6b\x03\x6c\x03\x6c\
	\x05\x6c\u{3d4}\x0a\x6c\x03\x6d\x03\x6d\x03\x6d\x07\x6d\u{3d9}\x0a\x6d\x0c\
	\x6d\x0e\x6d\u{3dc}\x0b\x6d\x03\x6e\x03\x6e\x03\x6e\x07\x6e\u{3e1}\x0a\x6e\
	\x0c\x6e\x0e\x6e\u{3e4}\x0b\x6e\x03\x6f\x03\x6f\x03\x6f\x03\x6f\x03\x6f\
	\x03\x6f\x03\x6f\x03\x6f\x03\x6f\x03\x6f\x03\x6f\x03\x6f\x03\x6f\x03\x6f\
	\x03\x6f\x03\x6f\x03\x6f\x03\x6f\x03\x6f\x03\x6f\x03\x6f\x03\x6f\x03\x6f\
	\x03\x6f\x03\x6f\x03\x6f\x03\x6f\x03\x6f\x03\x6f\x03\x6f\x03\x6f\x03\x6f\
	\x05\x6f\u{406}\x0a\x6f\x03\x70\x03\x70\x03\x70\x03\x70\x03\x70\x03\x70\
	\x03\x70\x07\x70\u{40f}\x0a\x70\x0c\x70\x0e\x70\u{412}\x0b\x70\x03\x70\x03\
	\x70\x03\x70\x05\x70\u{417}\x0a\x70\x03\x71\x03\x71\x03\x72\x03\x72\x03\
	\x72\x03\x72\x03\x72\x07\x72\u{420}\x0a\x72\x0c\x72\x0e\x72\u{423}\x0b\x72\
	\x05\x72\u{425}\x0a\x72\x03\x72\x03\x72\x03\x73\x03\x73\x03\x73\x07\x73\
	\u{42c}\x0a\x73\x0c\x73\x0e\x73\u{42f}\x0b\x73\x03\x73\x03\x73\x03\x73\x03\
	\x73\x03\x73\x03\x73\x07\x73\u{437}\x0a\x73\x0c\x73\x0e\x73\u{43a}\x0b\x73\
	\x05\x73\u{43c}\x0a\x73\x03\x74\x03\x74\x03\x75\x03\x75\x03\x75\x07\x75\
	\u{443}\x0a\x75\x0c\x75\x0e\x75\u{446}\x0b\x75\x03\x76\x03\x76\x03\x76\x03\
	\x76\x07\x76\u{44c}\x0a\x76\x0c\x76\x0e\x76\u{44f}\x0b\x76\x05\x76\u{451}\
	\x0a\x76\x03\x76\x03\x76\x03\x77\x03\x77\x03\x77\x03\x77\x07\x77\u{459}\
	\x0a\x77\x0c\x77\x0e\x77\u{45c}\x0b\x77\x05\x77\u{45e}\x0a\x77\x03\x77\x03\
	\x77\x03\x78\x03\x78\x03\x78\x03\x78\x03\x79\x03\x79\x03\x7a\x03\x7a\x03\
	\x7a\x03\x7a\x03\x7a\x03\x7a\x05\x7a\u{46e}\x0a\x7a\x03\x7b\x03\x7b\x03\
	\x7b\x03\x7b\x03\x7b\x03\x7b\x05\x7b\u{476}\x0a\x7b\x03\x7b\x02\x02\x7c\
	\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\
	\x26\x28\x2a\x2c\x2e\x30\x32\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\x48\
	\x4a\x4c\x4e\x50\x52\x54\x56\x58\x5a\x5c\x5e\x60\x62\x64\x66\x68\x6a\x6c\
	\x6e\x70\x72\x74\x76\x78\x7a\x7c\x7e\u{80}\u{82}\u{84}\u{86}\u{88}\u{8a}\
	\u{8c}\u{8e}\u{90}\u{92}\u{94}\u{96}\u{98}\u{9a}\u{9c}\u{9e}\u{a0}\u{a2}\
	\u{a4}\u{a6}\u{a8}\u{aa}\u{ac}\u{ae}\u{b0}\u{b2}\u{b4}\u{b6}\u{b8}\u{ba}\
	\u{bc}\u{be}\u{c0}\u{c2}\u{c4}\u{c6}\u{c8}\u{ca}\u{cc}\u{ce}\u{d0}\u{d2}\
	\u{d4}\u{d6}\u{d8}\u{da}\u{dc}\u{de}\u{e0}\u{e2}\u{e4}\u{e6}\u{e8}\u{ea}\
	\u{ec}\u{ee}\u{f0}\u{f2}\u{f4}\x02\x16\x06\x02\x62\x62\x64\x64\x6b\x6c\x7d\
	\x7d\x03\x02\x03\x06\x05\x02\x27\x28\x2b\x2c\x7d\x7d\x03\x02\x46\x49\x04\
	\x02\x7a\x7a\x7d\x7d\x03\x02\x10\x13\x03\x02\x7a\x7b\x05\x02\x4a\x4a\x56\
	\x58\x7d\x7d\x04\x02\x4a\x4e\x51\x51\x07\x02\x0d\x0d\x2e\x2e\x4a\x4a\x56\
	\x58\x7d\x7d\x04\x02\x59\x5a\x7d\x7d\x04\x02\x7a\x7b\x7d\x7d\x03\x02\x1f\
	\x20\x03\x02\x5b\x60\x03\x02\x61\x62\x03\x02\x63\x66\x03\x02\x3b\x3d\x03\
	\x02\x3e\x42\x03\x02\x43\x45\x08\x02\x0d\x0d\x2e\x2e\x4a\x4a\x56\x57\x7a\
	\x7a\x7d\x7d\x02\u{491}\x02\u{f9}\x03\x02\x02\x02\x04\u{10a}\x03\x02\x02\
	\x02\x06\u{10e}\x03\x02\x02\x02\x08\u{114}\x03\x02\x02\x02\x0a\u{116}\x03\
	\x02\x02\x02\x0c\u{119}\x03\x02\x02\x02\x0e\u{122}\x03\x02\x02\x02\x10\u{132}\
	\x03\x02\x02\x02\x12\u{134}\x03\x02\x02\x02\x14\u{136}\x03\x02\x02\x02\x16\
	\u{13f}\x03\x02\x02\x02\x18\u{141}\x03\x02\x02\x02\x1a\u{149}\x03\x02\x02\
	\x02\x1c\u{14d}\x03\x02\x02\x02\x1e\u{150}\x03\x02\x02\x02\x20\u{15d}\x03\
	\x02\x02\x02\x22\u{15f}\x03\x02\x02\x02\x24\u{162}\x03\x02\x02\x02\x26\u{164}\
	\x03\x02\x02\x02\x28\u{16d}\x03\x02\x02\x02\x2a\u{176}\x03\x02\x02\x02\x2c\
	\u{178}\x03\x02\x02\x02\x2e\u{180}\x03\x02\x02\x02\x30\u{188}\x03\x02\x02\
	\x02\x32\u{18a}\x03\x02\x02\x02\x34\u{18e}\x03\x02\x02\x02\x36\u{199}\x03\
	\x02\x02\x02\x38\u{19b}\x03\x02\x02\x02\x3a\u{19d}\x03\x02\x02\x02\x3c\u{19f}\
	\x03\x02\x02\x02\x3e\u{1a4}\x03\x02\x02\x02\x40\u{1be}\x03\x02\x02\x02\x42\
	\u{1c2}\x03\x02\x02\x02\x44\u{1c4}\x03\x02\x02\x02\x46\u{1c7}\x03\x02\x02\
	\x02\x48\u{1c9}\x03\x02\x02\x02\x4a\u{1cc}\x03\x02\x02\x02\x4c\u{1ce}\x03\
	\x02\x02\x02\x4e\u{1d5}\x03\x02\x02\x02\x50\u{1db}\x03\x02\x02\x02\x52\u{1e1}\
	\x03\x02\x02\x02\x54\u{1e3}\x03\x02\x02\x02\x56\u{1e5}\x03\x02\x02\x02\x58\
	\u{1e7}\x03\x02\x02\x02\x5a\u{1eb}\x03\x02\x02\x02\x5c\u{1f2}\x03\x02\x02\
	\x02\x5e\u{1fb}\x03\x02\x02\x02\x60\u{1fe}\x03\x02\x02\x02\x62\u{201}\x03\
	\x02\x02\x02\x64\u{203}\x03\x02\x02\x02\x66\u{207}\x03\x02\x02\x02\x68\u{210}\
	\x03\x02\x02\x02\x6a\u{213}\x03\x02\x02\x02\x6c\u{218}\x03\x02\x02\x02\x6e\
	\u{223}\x03\x02\x02\x02\x70\u{225}\x03\x02\x02\x02\x72\u{22e}\x03\x02\x02\
	\x02\x74\u{23b}\x03\x02\x02\x02\x76\u{245}\x03\x02\x02\x02\x78\u{24e}\x03\
	\x02\x02\x02\x7a\u{250}\x03\x02\x02\x02\x7c\u{253}\x03\x02\x02\x02\x7e\u{261}\
	\x03\x02\x02\x02\u{80}\u{269}\x03\x02\x02\x02\u{82}\u{26b}\x03\x02\x02\x02\
	\u{84}\u{26d}\x03\x02\x02\x02\u{86}\u{289}\x03\x02\x02\x02\u{88}\u{28b}\
	\x03\x02\x02\x02\u{8a}\u{29d}\x03\x02\x02\x02\u{8c}\u{29f}\x03\x02\x02\x02\
	\u{8e}\u{2a3}\x03\x02\x02\x02\u{90}\u{2aa}\x03\x02\x02\x02\u{92}\u{2ae}\
	\x03\x02\x02\x02\u{94}\u{2b2}\x03\x02\x02\x02\u{96}\u{2b4}\x03\x02\x02\x02\
	\u{98}\u{2b7}\x03\x02\x02\x02\u{9a}\u{2c0}\x03\x02\x02\x02\u{9c}\u{2c8}\
	\x03\x02\x02\x02\u{9e}\u{2ca}\x03\x02\x02\x02\u{a0}\u{2d1}\x03\x02\x02\x02\
	\u{a2}\u{2d5}\x03\x02\x02\x02\u{a4}\u{2e6}\x03\x02\x02\x02\u{a6}\u{2f3}\
	\x03\x02\x02\x02\u{a8}\u{2fa}\x03\x02\x02\x02\u{aa}\u{2fc}\x03\x02\x02\x02\
	\u{ac}\u{301}\x03\x02\x02\x02\u{ae}\u{306}\x03\x02\x02\x02\u{b0}\u{31b}\
	\x03\x02\x02\x02\u{b2}\u{31f}\x03\x02\x02\x02\u{b4}\u{337}\x03\x02\x02\x02\
	\u{b6}\u{339}\x03\x02\x02\x02\u{b8}\u{341}\x03\x02\x02\x02\u{ba}\u{343}\
	\x03\x02\x02\x02\u{bc}\u{345}\x03\x02\x02\x02\u{be}\u{34e}\x03\x02\x02\x02\
	\u{c0}\u{358}\x03\x02\x02\x02\u{c2}\u{38c}\x03\x02\x02\x02\u{c4}\u{38e}\
	\x03\x02\x02\x02\u{c6}\u{390}\x03\x02\x02\x02\u{c8}\u{398}\x03\x02\x02\x02\
	\u{ca}\u{3a1}\x03\x02\x02\x02\u{cc}\u{3b0}\x03\x02\x02\x02\u{ce}\u{3c9}\
	\x03\x02\x02\x02\u{d0}\u{3cb}\x03\x02\x02\x02\u{d2}\u{3cd}\x03\x02\x02\x02\
	\u{d4}\u{3cf}\x03\x02\x02\x02\u{d6}\u{3d3}\x03\x02\x02\x02\u{d8}\u{3d5}\
	\x03\x02\x02\x02\u{da}\u{3dd}\x03\x02\x02\x02\u{dc}\u{405}\x03\x02\x02\x02\
	\u{de}\u{416}\x03\x02\x02\x02\u{e0}\u{418}\x03\x02\x02\x02\u{e2}\u{41a}\
	\x03\x02\x02\x02\u{e4}\u{43b}\x03\x02\x02\x02\u{e6}\u{43d}\x03\x02\x02\x02\
	\u{e8}\u{43f}\x03\x02\x02\x02\u{ea}\u{447}\x03\x02\x02\x02\u{ec}\u{454}\
	\x03\x02\x02\x02\u{ee}\u{461}\x03\x02\x02\x02\u{f0}\u{465}\x03\x02\x02\x02\
	\u{f2}\u{46d}\x03\x02\x02\x02\u{f4}\u{475}\x03\x02\x02\x02\u{f6}\u{f8}\x05\
	\x04\x03\x02\u{f7}\u{f6}\x03\x02\x02\x02\u{f8}\u{fb}\x03\x02\x02\x02\u{f9}\
	\u{f7}\x03\x02\x02\x02\u{f9}\u{fa}\x03\x02\x02\x02\u{fa}\u{fd}\x03\x02\x02\
	\x02\u{fb}\u{f9}\x03\x02\x02\x02\u{fc}\u{fe}\x05\x0c\x07\x02\u{fd}\u{fc}\
	\x03\x02\x02\x02\u{fd}\u{fe}\x03\x02\x02\x02\u{fe}\u{100}\x03\x02\x02\x02\
	\u{ff}\u{101}\x05\x26\x14\x02\u{100}\u{ff}\x03\x02\x02\x02\u{100}\u{101}\
	\x03\x02\x02\x02\u{101}\u{104}\x03\x02\x02\x02\u{102}\u{105}\x05\x3e\x20\
	\x02\u{103}\u{105}\x05\u{a4}\x53\x02\u{104}\u{102}\x03\x02\x02\x02\u{104}\
	\u{103}\x03\x02\x02\x02\u{105}\u{106}\x03\x02\x02\x02\u{106}\u{104}\x03\
	\x02\x02\x02\u{106}\u{107}\x03\x02\x02\x02\u{107}\u{108}\x03\x02\x02\x02\
	\u{108}\u{109}\x07\x02\x02\x03\u{109}\x03\x03\x02\x02\x02\u{10a}\u{10b}\
	\x07\x07\x02\x02\u{10b}\u{10c}\x05\x06\x04\x02\u{10c}\x05\x03\x02\x02\x02\
	\u{10d}\u{10f}\x05\x08\x05\x02\u{10e}\u{10d}\x03\x02\x02\x02\u{10f}\u{110}\
	\x03\x02\x02\x02\u{110}\u{10e}\x03\x02\x02\x02\u{110}\u{111}\x03\x02\x02\
	\x02\u{111}\u{112}\x03\x02\x02\x02\u{112}\u{113}\x05\x0a\x06\x02\u{113}\
	\x07\x03\x02\x02\x02\u{114}\u{115}\x09\x02\x02\x02\u{115}\x09\x03\x02\x02\
	\x02\u{116}\u{117}\x07\x6c\x02\x02\u{117}\u{118}\x09\x03\x02\x02\u{118}\
	\x0b\x03\x02\x02\x02\u{119}\u{11a}\x07\x30\x02\x02\u{11a}\u{11c}\x07\x71\
	\x02\x02\u{11b}\u{11d}\x05\x0e\x08\x02\u{11c}\u{11b}\x03\x02\x02\x02\u{11d}\
	\u{11e}\x03\x02\x02\x02\u{11e}\u{11c}\x03\x02\x02\x02\u{11e}\u{11f}\x03\
	\x02\x02\x02\u{11f}\u{120}\x03\x02\x02\x02\u{120}\u{121}\x07\x72\x02\x02\
	\u{121}\x0d\x03\x02\x02\x02\u{122}\u{123}\x05\x10\x09\x02\u{123}\u{124}\
	\x07\x69\x02\x02\u{124}\u{125}\x05\x16\x0c\x02\u{125}\u{126}\x05\x12\x0a\
	\x02\u{126}\u{127}\x07\x6c\x02\x02\u{127}\u{128}\x05\x14\x0b\x02\u{128}\
	\u{12a}\x07\x6d\x02\x02\u{129}\u{12b}\x05\x18\x0d\x02\u{12a}\u{129}\x03\
	\x02\x02\x02\u{12a}\u{12b}\x03\x02\x02\x02\u{12b}\u{12c}\x03\x02\x02\x02\
	\u{12c}\u{12d}\x07\x6e\x02\x02\u{12d}\u{12e}\x07\x67\x02\x02\u{12e}\u{130}\
	\x05\x1c\x0f\x02\u{12f}\u{131}\x05\x1e\x10\x02\u{130}\u{12f}\x03\x02\x02\
	\x02\u{130}\u{131}\x03\x02\x02\x02\u{131}\x0f\x03\x02\x02\x02\u{132}\u{133}\
	\x07\x7d\x02\x02\u{133}\x11\x03\x02\x02\x02\u{134}\u{135}\x07\x7d\x02\x02\
	\u{135}\x13\x03\x02\x02\x02\u{136}\u{137}\x09\x04\x02\x02\u{137}\x15\x03\
	\x02\x02\x02\u{138}\u{140}\x07\x31\x02\x02\u{139}\u{140}\x07\x32\x02\x02\
	\u{13a}\u{13b}\x07\x33\x02\x02\u{13b}\u{13c}\x07\x6d\x02\x02\u{13c}\u{13d}\
	\x05\x22\x12\x02\u{13d}\u{13e}\x07\x6e\x02\x02\u{13e}\u{140}\x03\x02\x02\
	\x02\u{13f}\u{138}\x03\x02\x02\x02\u{13f}\u{139}\x03\x02\x02\x02\u{13f}\
	\u{13a}\x03\x02\x02\x02\u{140}\x17\x03\x02\x02\x02\u{141}\u{146}\x05\x1a\
	\x0e\x02\u{142}\u{143}\x07\x6a\x02\x02\u{143}\u{145}\x05\x1a\x0e\x02\u{144}\
	\u{142}\x03\x02\x02\x02\u{145}\u{148}\x03\x02\x02\x02\u{146}\u{144}\x03\
	\x02\x02\x02\u{146}\u{147}\x03\x02\x02\x02\u{147}\x19\x03\x02\x02\x02\u{148}\
	\u{146}\x03\x02\x02\x02\u{149}\u{14a}\x07\x7d\x02\x02\u{14a}\u{14b}\x07\
	\x69\x02\x02\u{14b}\u{14c}\x05\x52\x2a\x02\u{14c}\x1b\x03\x02\x02\x02\u{14d}\
	\u{14e}\x05\x52\x2a\x02\u{14e}\x1d\x03\x02\x02\x02\u{14f}\u{151}\x05\x20\
	\x11\x02\u{150}\u{14f}\x03\x02\x02\x02\u{151}\u{152}\x03\x02\x02\x02\u{152}\
	\u{150}\x03\x02\x02\x02\u{152}\u{153}\x03\x02\x02\x02\u{153}\x1f\x03\x02\
	\x02\x02\u{154}\u{155}\x07\x34\x02\x02\u{155}\u{156}\x07\x69\x02\x02\u{156}\
	\u{15e}\x05\x22\x12\x02\u{157}\u{158}\x07\x35\x02\x02\u{158}\u{159}\x07\
	\x69\x02\x02\u{159}\u{15e}\x05\u{f2}\x7a\x02\u{15a}\u{15b}\x07\x36\x02\x02\
	\u{15b}\u{15c}\x07\x69\x02\x02\u{15c}\u{15e}\x07\x76\x02\x02\u{15d}\u{154}\
	\x03\x02\x02\x02\u{15d}\u{157}\x03\x02\x02\x02\u{15d}\u{15a}\x03\x02\x02\
	\x02\u{15e}\x21\x03\x02\x02\x02\u{15f}\u{160}\x07\x76\x02\x02\u{160}\u{161}\
	\x05\x24\x13\x02\u{161}\x23\x03\x02\x02\x02\u{162}\u{163}\x09\x05\x02\x02\
	\u{163}\x25\x03\x02\x02\x02\u{164}\u{165}\x07\x37\x02\x02\u{165}\u{167}\
	\x07\x71\x02\x02\u{166}\u{168}\x05\x28\x15\x02\u{167}\u{166}\x03\x02\x02\
	\x02\u{168}\u{169}\x03\x02\x02\x02\u{169}\u{167}\x03\x02\x02\x02\u{169}\
	\u{16a}\x03\x02\x02\x02\u{16a}\u{16b}\x03\x02\x02\x02\u{16b}\u{16c}\x07\
	\x72\x02\x02\u{16c}\x27\x03\x02\x02\x02\u{16d}\u{16e}\x05\x2a\x16\x02\u{16e}\
	\u{170}\x07\x6d\x02\x02\u{16f}\u{171}\x05\x2c\x17\x02\u{170}\u{16f}\x03\
	\x02\x02\x02\u{170}\u{171}\x03\x02\x02\x02\u{171}\u{172}\x03\x02\x02\x02\
	\u{172}\u{173}\x07\x6e\x02\x02\u{173}\u{174}\x07\x67\x02\x02\u{174}\u{175}\
	\x05\x30\x19\x02\u{175}\x29\x03\x02\x02\x02\u{176}\u{177}\x07\x7d\x02\x02\
	\u{177}\x2b\x03\x02\x02\x02\u{178}\u{17d}\x05\x2e\x18\x02\u{179}\u{17a}\
	\x07\x6a\x02\x02\u{17a}\u{17c}\x05\x2e\x18\x02\u{17b}\u{179}\x03\x02\x02\
	\x02\u{17c}\u{17f}\x03\x02\x02\x02\u{17d}\u{17b}\x03\x02\x02\x02\u{17d}\
	\u{17e}\x03\x02\x02\x02\u{17e}\x2d\x03\x02\x02\x02\u{17f}\u{17d}\x03\x02\
	\x02\x02\u{180}\u{181}\x07\x7d\x02\x02\u{181}\u{182}\x07\x69\x02\x02\u{182}\
	\u{183}\x05\x52\x2a\x02\u{183}\x2f\x03\x02\x02\x02\u{184}\u{189}\x05\x32\
	\x1a\x02\u{185}\u{189}\x05\x34\x1b\x02\u{186}\u{189}\x05\x3a\x1e\x02\u{187}\
	\u{189}\x05\x3c\x1f\x02\u{188}\u{184}\x03\x02\x02\x02\u{188}\u{185}\x03\
	\x02\x02\x02\u{188}\u{186}\x03\x02\x02\x02\u{188}\u{187}\x03\x02\x02\x02\
	\u{189}\x31\x03\x02\x02\x02\u{18a}\u{18b}\x07\x2c\x02\x02\u{18b}\u{18c}\
	\x07\x2d\x02\x02\u{18c}\u{18d}\x07\x7d\x02\x02\u{18d}\x33\x03\x02\x02\x02\
	\u{18e}\u{18f}\x07\x38\x02\x02\u{18f}\u{190}\x07\x7d\x02\x02\u{190}\u{191}\
	\x07\x6c\x02\x02\u{191}\u{192}\x05\x36\x1c\x02\u{192}\x35\x03\x02\x02\x02\
	\u{193}\u{19a}\x07\x7d\x02\x02\u{194}\u{195}\x07\x7d\x02\x02\u{195}\u{196}\
	\x07\x6d\x02\x02\u{196}\u{197}\x05\x38\x1d\x02\u{197}\u{198}\x07\x6e\x02\
	\x02\u{198}\u{19a}\x03\x02\x02\x02\u{199}\u{193}\x03\x02\x02\x02\u{199}\
	\u{194}\x03\x02\x02\x02\u{19a}\x37\x03\x02\x02\x02\u{19b}\u{19c}\x09\x06\
	\x02\x02\u{19c}\x39\x03\x02\x02\x02\u{19d}\u{19e}\x07\x39\x02\x02\u{19e}\
	\x3b\x03\x02\x02\x02\u{19f}\u{1a0}\x07\x3a\x02\x02\u{1a0}\u{1a1}\x07\x7d\
	\x02\x02\u{1a1}\u{1a2}\x07\x6c\x02\x02\u{1a2}\u{1a3}\x07\x7d\x02\x02\u{1a3}\
	\x3d\x03\x02\x02\x02\u{1a4}\u{1a5}\x07\x08\x02\x02\u{1a5}\u{1a7}\x05\x42\
	\x22\x02\u{1a6}\u{1a8}\x05\x44\x23\x02\u{1a7}\u{1a6}\x03\x02\x02\x02\u{1a7}\
	\u{1a8}\x03\x02\x02\x02\u{1a8}\u{1aa}\x03\x02\x02\x02\u{1a9}\u{1ab}\x05\
	\x48\x25\x02\u{1aa}\u{1a9}\x03\x02\x02\x02\u{1aa}\u{1ab}\x03\x02\x02\x02\
	\u{1ab}\u{1ad}\x03\x02\x02\x02\u{1ac}\u{1ae}\x05\x40\x21\x02\u{1ad}\u{1ac}\
	\x03\x02\x02\x02\u{1ad}\u{1ae}\x03\x02\x02\x02\u{1ae}\u{1af}\x03\x02\x02\
	\x02\u{1af}\u{1b0}\x05\x4c\x27\x02\u{1b0}\u{1b4}\x05\x58\x2d\x02\u{1b1}\
	\u{1b5}\x05\u{8e}\x48\x02\u{1b2}\u{1b5}\x05\u{92}\x4a\x02\u{1b3}\u{1b5}\
	\x05\u{96}\x4c\x02\u{1b4}\u{1b1}\x03\x02\x02\x02\u{1b4}\u{1b2}\x03\x02\x02\
	\x02\u{1b4}\u{1b3}\x03\x02\x02\x02\u{1b4}\u{1b5}\x03\x02\x02\x02\u{1b5}\
	\u{1b7}\x03\x02\x02\x02\u{1b6}\u{1b8}\x05\u{98}\x4d\x02\u{1b7}\u{1b6}\x03\
	\x02\x02\x02\u{1b7}\u{1b8}\x03\x02\x02\x02\u{1b8}\u{1ba}\x03\x02\x02\x02\
	\u{1b9}\u{1bb}\x05\u{9e}\x50\x02\u{1ba}\u{1b9}\x03\x02\x02\x02\u{1ba}\u{1bb}\
	\x03\x02\x02\x02\u{1bb}\u{1bc}\x03\x02\x02\x02\u{1bc}\u{1bd}\x07\x0a\x02\
	\x02\u{1bd}\x3f\x03\x02\x02\x02\u{1be}\u{1bf}\x07\x58\x02\x02\u{1bf}\u{1c0}\
	\x07\x69\x02\x02\u{1c0}\u{1c1}\x07\x75\x02\x02\u{1c1}\x41\x03\x02\x02\x02\
	\u{1c2}\u{1c3}\x07\x7d\x02\x02\u{1c3}\x43\x03\x02\x02\x02\u{1c4}\u{1c5}\
	\x07\x0f\x02\x02\u{1c5}\u{1c6}\x05\x46\x24\x02\u{1c6}\x45\x03\x02\x02\x02\
	\u{1c7}\u{1c8}\x09\x07\x02\x02\u{1c8}\x47\x03\x02\x02\x02\u{1c9}\u{1ca}\
	\x07\x56\x02\x02\u{1ca}\u{1cb}\x05\x4a\x26\x02\u{1cb}\x49\x03\x02\x02\x02\
	\u{1cc}\u{1cd}\x09\x08\x02\x02\u{1cd}\x4b\x03\x02\x02\x02\u{1ce}\u{1cf}\
	\x07\x0b\x02\x02\u{1cf}\u{1d1}\x07\x69\x02\x02\u{1d0}\u{1d2}\x05\x4e\x28\
	\x02\u{1d1}\u{1d0}\x03\x02\x02\x02\u{1d2}\u{1d3}\x03\x02\x02\x02\u{1d3}\
	\u{1d1}\x03\x02\x02\x02\u{1d3}\u{1d4}\x03\x02\x02\x02\u{1d4}\x4d\x03\x02\
	\x02\x02\u{1d5}\u{1d6}\x05\x50\x29\x02\u{1d6}\u{1d7}\x07\x69\x02\x02\u{1d7}\
	\u{1d9}\x05\x52\x2a\x02\u{1d8}\u{1da}\x05\x56\x2c\x02\u{1d9}\u{1d8}\x03\
	\x02\x02\x02\u{1d9}\u{1da}\x03\x02\x02\x02\u{1da}\x4f\x03\x02\x02\x02\u{1db}\
	\u{1dc}\x09\x09\x02\x02\u{1dc}\x51\x03\x02\x02\x02\u{1dd}\u{1e2}\x05\x54\
	\x2b\x02\u{1de}\u{1e2}\x07\x4f\x02\x02\u{1df}\u{1e2}\x07\x50\x02\x02\u{1e0}\
	\u{1e2}\x07\x7d\x02\x02\u{1e1}\u{1dd}\x03\x02\x02\x02\u{1e1}\u{1de}\x03\
	\x02\x02\x02\u{1e1}\u{1df}\x03\x02\x02\x02\u{1e1}\u{1e0}\x03\x02\x02\x02\
	\u{1e2}\x53\x03\x02\x02\x02\u{1e3}\u{1e4}\x09\x0a\x02\x02\u{1e4}\x55\x03\
	\x02\x02\x02\u{1e5}\u{1e6}\x07\x7f\x02\x02\u{1e6}\x57\x03\x02\x02\x02\u{1e7}\
	\u{1e8}\x07\x0c\x02\x02\u{1e8}\u{1e9}\x07\x69\x02\x02\u{1e9}\u{1ea}\x05\
	\x5a\x2e\x02\u{1ea}\x59\x03\x02\x02\x02\u{1eb}\u{1ec}\x05\x5c\x2f\x02\u{1ec}\
	\u{1ee}\x05\x64\x33\x02\u{1ed}\u{1ef}\x05\x66\x34\x02\u{1ee}\u{1ed}\x03\
	\x02\x02\x02\u{1ef}\u{1f0}\x03\x02\x02\x02\u{1f0}\u{1ee}\x03\x02\x02\x02\
	\u{1f0}\u{1f1}\x03\x02\x02\x02\u{1f1}\x5b\x03\x02\x02\x02\u{1f2}\u{1f4}\
	\x07\x68\x02\x02\u{1f3}\u{1f5}\x05\x5e\x30\x02\u{1f4}\u{1f3}\x03\x02\x02\
	\x02\u{1f4}\u{1f5}\x03\x02\x02\x02\u{1f5}\u{1f7}\x03\x02\x02\x02\u{1f6}\
	\u{1f8}\x05\x60\x31\x02\u{1f7}\u{1f6}\x03\x02\x02\x02\u{1f8}\u{1f9}\x03\
	\x02\x02\x02\u{1f9}\u{1f7}\x03\x02\x02\x02\u{1f9}\u{1fa}\x03\x02\x02\x02\
	\u{1fa}\x5d\x03\x02\x02\x02\u{1fb}\u{1fc}\x07\x57\x02\x02\u{1fc}\u{1fd}\
	\x07\x68\x02\x02\u{1fd}\x5f\x03\x02\x02\x02\u{1fe}\u{1ff}\x05\x62\x32\x02\
	\u{1ff}\u{200}\x07\x68\x02\x02\u{200}\x61\x03\x02\x02\x02\u{201}\u{202}\
	\x09\x0b\x02\x02\u{202}\x63\x03\x02\x02\x02\u{203}\u{204}\x07\x68\x02\x02\
	\u{204}\u{205}\x07\x73\x02\x02\u{205}\u{206}\x07\x68\x02\x02\u{206}\x65\
	\x03\x02\x02\x02\u{207}\u{209}\x07\x68\x02\x02\u{208}\u{20a}\x05\x68\x35\
	\x02\u{209}\u{208}\x03\x02\x02\x02\u{209}\u{20a}\x03\x02\x02\x02\u{20a}\
	\u{20c}\x03\x02\x02\x02\u{20b}\u{20d}\x05\x6a\x36\x02\u{20c}\u{20b}\x03\
	\x02\x02\x02\u{20d}\u{20e}\x03\x02\x02\x02\u{20e}\u{20c}\x03\x02\x02\x02\
	\u{20e}\u{20f}\x03\x02\x02\x02\u{20f}\x67\x03\x02\x02\x02\u{210}\u{211}\
	\x07\x76\x02\x02\u{211}\u{212}\x07\x68\x02\x02\u{212}\x69\x03\x02\x02\x02\
	\u{213}\u{214}\x05\x6c\x37\x02\u{214}\u{215}\x07\x68\x02\x02\u{215}\x6b\
	\x03\x02\x02\x02\u{216}\u{219}\x05\x6e\x38\x02\u{217}\u{219}\x05\u{80}\x41\
	\x02\u{218}\u{216}\x03\x02\x02\x02\u{218}\u{217}\x03\x02\x02\x02\u{219}\
	\x6d\x03\x02\x02\x02\u{21a}\u{224}\x07\x63\x02\x02\u{21b}\u{224}\x05\x70\
	\x39\x02\u{21c}\u{224}\x05\x72\x3a\x02\u{21d}\u{224}\x05\x74\x3b\x02\u{21e}\
	\u{224}\x05\x76\x3c\x02\u{21f}\u{224}\x05\x78\x3d\x02\u{220}\u{224}\x05\
	\x7a\x3e\x02\u{221}\u{224}\x05\x7c\x3f\x02\u{222}\u{224}\x05\x7e\x40\x02\
	\u{223}\u{21a}\x03\x02\x02\x02\u{223}\u{21b}\x03\x02\x02\x02\u{223}\u{21c}\
	\x03\x02\x02\x02\u{223}\u{21d}\x03\x02\x02\x02\u{223}\u{21e}\x03\x02\x02\
	\x02\u{223}\u{21f}\x03\x02\x02\x02\u{223}\u{220}\x03\x02\x02\x02\u{223}\
	\u{221}\x03\x02\x02\x02\u{223}\u{222}\x03\x02\x02\x02\u{224}\x6f\x03\x02\
	\x02\x02\u{225}\u{226}\x05\u{f2}\x7a\x02\u{226}\x71\x03\x02\x02\x02\u{227}\
	\u{228}\x05\u{f4}\x7b\x02\u{228}\u{229}\x07\x2d\x02\x02\u{229}\u{22a}\x05\
	\u{f4}\x7b\x02\u{22a}\u{22f}\x03\x02\x02\x02\u{22b}\u{22c}\x07\x78\x02\x02\
	\u{22c}\u{22d}\x07\x2d\x02\x02\u{22d}\u{22f}\x07\x78\x02\x02\u{22e}\u{227}\
	\x03\x02\x02\x02\u{22e}\u{22b}\x03\x02\x02\x02\u{22f}\x73\x03\x02\x02\x02\
	\u{230}\u{231}\x07\x22\x02\x02\u{231}\u{232}\x07\x6d\x02\x02\u{232}\u{233}\
	\x05\u{e8}\x75\x02\u{233}\u{234}\x07\x6e\x02\x02\u{234}\u{23c}\x03\x02\x02\
	\x02\u{235}\u{236}\x07\x21\x02\x02\u{236}\u{237}\x07\x22\x02\x02\u{237}\
	\u{238}\x07\x6d\x02\x02\u{238}\u{239}\x05\u{e8}\x75\x02\u{239}\u{23a}\x07\
	\x6e\x02\x02\u{23a}\u{23c}\x03\x02\x02\x02\u{23b}\u{230}\x03\x02\x02\x02\
	\u{23b}\u{235}\x03\x02\x02\x02\u{23c}\x75\x03\x02\x02\x02\u{23d}\u{23e}\
	\x07\x27\x02\x02\u{23e}\u{246}\x05\x4a\x26\x02\u{23f}\u{240}\x07\x2a\x02\
	\x02\u{240}\u{246}\x05\x4a\x26\x02\u{241}\u{242}\x07\x29\x02\x02\u{242}\
	\u{246}\x05\x4a\x26\x02\u{243}\u{244}\x07\x28\x02\x02\u{244}\u{246}\x05\
	\x4a\x26\x02\u{245}\u{23d}\x03\x02\x02\x02\u{245}\u{23f}\x03\x02\x02\x02\
	\u{245}\u{241}\x03\x02\x02\x02\u{245}\u{243}\x03\x02\x02\x02\u{246}\x77\
	\x03\x02\x02\x02\u{247}\u{248}\x07\x23\x02\x02\u{248}\u{24f}\x07\x24\x02\
	\x02\u{249}\u{24a}\x07\x23\x02\x02\u{24a}\u{24b}\x07\x21\x02\x02\u{24b}\
	\u{24f}\x07\x24\x02\x02\u{24c}\u{24f}\x07\x25\x02\x02\u{24d}\u{24f}\x07\
	\x26\x02\x02\u{24e}\u{247}\x03\x02\x02\x02\u{24e}\u{249}\x03\x02\x02\x02\
	\u{24e}\u{24c}\x03\x02\x02\x02\u{24e}\u{24d}\x03\x02\x02\x02\u{24f}\x79\
	\x03\x02\x02\x02\u{250}\u{251}\x05\u{c4}\x63\x02\u{251}\u{252}\x05\u{c6}\
	\x64\x02\u{252}\x7b\x03\x02\x02\x02\u{253}\u{254}\x07\x6d\x02\x02\u{254}\
	\u{255}\x05\u{bc}\x5f\x02\u{255}\u{256}\x07\x6e\x02\x02\u{256}\x7d\x03\x02\
	\x02\x02\u{257}\u{258}\x07\x52\x02\x02\u{258}\u{259}\x07\x7d\x02\x02\u{259}\
	\u{262}\x07\x53\x02\x02\u{25a}\u{25b}\x07\x52\x02\x02\u{25b}\u{25c}\x07\
	\x7d\x02\x02\u{25c}\u{262}\x07\x54\x02\x02\u{25d}\u{25e}\x07\x55\x02\x02\
	\u{25e}\u{25f}\x07\x7d\x02\x02\u{25f}\u{260}\x07\x1f\x02\x02\u{260}\u{262}\
	\x07\x7d\x02\x02\u{261}\u{257}\x03\x02\x02\x02\u{261}\u{25a}\x03\x02\x02\
	\x02\u{261}\u{25d}\x03\x02\x02\x02\u{262}\x7f\x03\x02\x02\x02\u{263}\u{26a}\
	\x07\x63\x02\x02\u{264}\u{26a}\x05\u{82}\x42\x02\u{265}\u{26a}\x05\u{84}\
	\x43\x02\u{266}\u{26a}\x05\u{86}\x44\x02\u{267}\u{26a}\x05\u{88}\x45\x02\
	\u{268}\u{26a}\x05\u{8c}\x47\x02\u{269}\u{263}\x03\x02\x02\x02\u{269}\u{264}\
	\x03\x02\x02\x02\u{269}\u{265}\x03\x02\x02\x02\u{269}\u{266}\x03\x02\x02\
	\x02\u{269}\u{267}\x03\x02\x02\x02\u{269}\u{268}\x03\x02\x02\x02\u{26a}\
	\u{81}\x03\x02\x02\x02\u{26b}\u{26c}\x05\u{f2}\x7a\x02\u{26c}\u{83}\x03\
	\x02\x02\x02\u{26d}\u{26e}\x05\u{e0}\x71\x02\u{26e}\u{85}\x03\x02\x02\x02\
	\u{26f}\u{270}\x07\x2b\x02\x02\u{270}\u{271}\x07\x6d\x02\x02\u{271}\u{276}\
	\x07\x7d\x02\x02\u{272}\u{273}\x07\x6a\x02\x02\u{273}\u{275}\x05\u{c6}\x64\
	\x02\u{274}\u{272}\x03\x02\x02\x02\u{275}\u{278}\x03\x02\x02\x02\u{276}\
	\u{274}\x03\x02\x02\x02\u{276}\u{277}\x03\x02\x02\x02\u{277}\u{27d}\x03\
	\x02\x02\x02\u{278}\u{276}\x03\x02\x02\x02\u{279}\u{27a}\x07\x6a\x02\x02\
	\u{27a}\u{27b}\x07\x2e\x02\x02\u{27b}\u{27c}\x07\x69\x02\x02\u{27c}\u{27e}\
	\x05\u{c6}\x64\x02\u{27d}\u{279}\x03\x02\x02\x02\u{27d}\u{27e}\x03\x02\x02\
	\x02\u{27e}\u{27f}\x03\x02\x02\x02\u{27f}\u{28a}\x07\x6e\x02\x02\u{280}\
	\u{281}\x07\x2b\x02\x02\u{281}\u{282}\x07\x6d\x02\x02\u{282}\u{283}\x07\
	\x7d\x02\x02\u{283}\u{284}\x07\x6a\x02\x02\u{284}\u{285}\x07\x2f\x02\x02\
	\u{285}\u{286}\x07\x69\x02\x02\u{286}\u{287}\x05\u{c6}\x64\x02\u{287}\u{288}\
	\x07\x6e\x02\x02\u{288}\u{28a}\x03\x02\x02\x02\u{289}\u{26f}\x03\x02\x02\
	\x02\u{289}\u{280}\x03\x02\x02\x02\u{28a}\u{87}\x03\x02\x02\x02\u{28b}\u{28c}\
	\x07\x7d\x02\x02\u{28c}\u{295}\x07\x6d\x02\x02\u{28d}\u{292}\x05\u{8a}\x46\
	\x02\u{28e}\u{28f}\x07\x6a\x02\x02\u{28f}\u{291}\x05\u{8a}\x46\x02\u{290}\
	\u{28e}\x03\x02\x02\x02\u{291}\u{294}\x03\x02\x02\x02\u{292}\u{290}\x03\
	\x02\x02\x02\u{292}\u{293}\x03\x02\x02\x02\u{293}\u{296}\x03\x02\x02\x02\
	\u{294}\u{292}\x03\x02\x02\x02\u{295}\u{28d}\x03\x02\x02\x02\u{295}\u{296}\
	\x03\x02\x02\x02\u{296}\u{297}\x03\x02\x02\x02\u{297}\u{298}\x07\x6e\x02\
	\x02\u{298}\u{89}\x03\x02\x02\x02\u{299}\u{29e}\x05\u{c6}\x64\x02\u{29a}\
	\u{29b}\x07\x7d\x02\x02\u{29b}\u{29c}\x07\x69\x02\x02\u{29c}\u{29e}\x05\
	\u{c6}\x64\x02\u{29d}\u{299}\x03\x02\x02\x02\u{29d}\u{29a}\x03\x02\x02\x02\
	\u{29e}\u{8b}\x03\x02\x02\x02\u{29f}\u{2a0}\x07\x2c\x02\x02\u{2a0}\u{2a1}\
	\x07\x2d\x02\x02\u{2a1}\u{2a2}\x07\x7d\x02\x02\u{2a2}\u{8d}\x03\x02\x02\
	\x02\u{2a3}\u{2a4}\x07\x0d\x02\x02\u{2a4}\u{2a6}\x07\x69\x02\x02\u{2a5}\
	\u{2a7}\x05\u{90}\x49\x02\u{2a6}\u{2a5}\x03\x02\x02\x02\u{2a7}\u{2a8}\x03\
	\x02\x02\x02\u{2a8}\u{2a6}\x03\x02\x02\x02\u{2a8}\u{2a9}\x03\x02\x02\x02\
	\u{2a9}\u{8f}\x03\x02\x02\x02\u{2aa}\u{2ab}\x05\x50\x29\x02\u{2ab}\u{2ac}\
	\x07\x69\x02\x02\u{2ac}\u{2ad}\x05\x52\x2a\x02\u{2ad}\u{91}\x03\x02\x02\
	\x02\u{2ae}\u{2af}\x07\x0e\x02\x02\u{2af}\u{2b0}\x07\x69\x02\x02\u{2b0}\
	\u{2b1}\x05\u{94}\x4b\x02\u{2b1}\u{93}\x03\x02\x02\x02\u{2b2}\u{2b3}\x09\
	\x0c\x02\x02\u{2b3}\u{95}\x03\x02\x02\x02\u{2b4}\u{2b5}\x05\u{8e}\x48\x02\
	\u{2b5}\u{2b6}\x05\u{92}\x4a\x02\u{2b6}\u{97}\x03\x02\x02\x02\u{2b7}\u{2b8}\
	\x07\x1d\x02\x02\u{2b8}\u{2ba}\x07\x69\x02\x02\u{2b9}\u{2bb}\x05\u{9a}\x4e\
	\x02\u{2ba}\u{2b9}\x03\x02\x02\x02\u{2bb}\u{2bc}\x03\x02\x02\x02\u{2bc}\
	\u{2ba}\x03\x02\x02\x02\u{2bc}\u{2bd}\x03\x02\x02\x02\u{2bd}\u{99}\x03\x02\
	\x02\x02\u{2be}\u{2c1}\x05\u{ac}\x57\x02\u{2bf}\u{2c1}\x05\u{9c}\x4f\x02\
	\u{2c0}\u{2be}\x03\x02\x02\x02\u{2c0}\u{2bf}\x03\x02\x02\x02\u{2c1}\u{9b}\
	\x03\x02\x02\x02\u{2c2}\u{2c3}\x07\x7d\x02\x02\u{2c3}\u{2c4}\x07\x5b\x02\
	\x02\u{2c4}\u{2c9}\x05\u{c6}\x64\x02\u{2c5}\u{2c6}\x07\x7d\x02\x02\u{2c6}\
	\u{2c7}\x07\x5b\x02\x02\u{2c7}\u{2c9}\x05\u{a2}\x52\x02\u{2c8}\u{2c2}\x03\
	\x02\x02\x02\u{2c8}\u{2c5}\x03\x02\x02\x02\u{2c9}\u{9d}\x03\x02\x02\x02\
	\u{2ca}\u{2cb}\x07\x1e\x02\x02\u{2cb}\u{2cd}\x07\x69\x02\x02\u{2cc}\u{2ce}\
	\x05\u{a0}\x51\x02\u{2cd}\u{2cc}\x03\x02\x02\x02\u{2ce}\u{2cf}\x03\x02\x02\
	\x02\u{2cf}\u{2cd}\x03\x02\x02\x02\u{2cf}\u{2d0}\x03\x02\x02\x02\u{2d0}\
	\u{9f}\x03\x02\x02\x02\u{2d1}\u{2d2}\x07\x7d\x02\x02\u{2d2}\u{2d3}\x07\x5b\
	\x02\x02\u{2d3}\u{2d4}\x05\u{c6}\x64\x02\u{2d4}\u{a1}\x03\x02\x02\x02\u{2d5}\
	\u{2d6}\x07\x1b\x02\x02\u{2d6}\u{2d7}\x05\u{bc}\x5f\x02\u{2d7}\u{2d8}\x07\
	\x15\x02\x02\u{2d8}\u{2e0}\x05\u{c6}\x64\x02\u{2d9}\u{2da}\x07\x1b\x02\x02\
	\u{2da}\u{2db}\x05\u{bc}\x5f\x02\u{2db}\u{2dc}\x07\x15\x02\x02\u{2dc}\u{2dd}\
	\x05\u{c6}\x64\x02\u{2dd}\u{2df}\x03\x02\x02\x02\u{2de}\u{2d9}\x03\x02\x02\
	\x02\u{2df}\u{2e2}\x03\x02\x02\x02\u{2e0}\u{2de}\x03\x02\x02\x02\u{2e0}\
	\u{2e1}\x03\x02\x02\x02\u{2e1}\u{2e3}\x03\x02\x02\x02\u{2e2}\u{2e0}\x03\
	\x02\x02\x02\u{2e3}\u{2e4}\x07\x1c\x02\x02\u{2e4}\u{2e5}\x05\u{c6}\x64\x02\
	\u{2e5}\u{a3}\x03\x02\x02\x02\u{2e6}\u{2e7}\x07\x09\x02\x02\u{2e7}\u{2e8}\
	\x05\u{a6}\x54\x02\u{2e8}\u{2ea}\x07\x69\x02\x02\u{2e9}\u{2eb}\x05\x48\x25\
	\x02\u{2ea}\u{2e9}\x03\x02\x02\x02\u{2ea}\u{2eb}\x03\x02\x02\x02\u{2eb}\
	\u{2ed}\x03\x02\x02\x02\u{2ec}\u{2ee}\x05\u{a8}\x55\x02\u{2ed}\u{2ec}\x03\
	\x02\x02\x02\u{2ee}\u{2ef}\x03\x02\x02\x02\u{2ef}\u{2ed}\x03\x02\x02\x02\
	\u{2ef}\u{2f0}\x03\x02\x02\x02\u{2f0}\u{2f1}\x03\x02\x02\x02\u{2f1}\u{2f2}\
	\x07\x0a\x02\x02\u{2f2}\u{a5}\x03\x02\x02\x02\u{2f3}\u{2f4}\x09\x0d\x02\
	\x02\u{2f4}\u{a7}\x03\x02\x02\x02\u{2f5}\u{2fb}\x05\u{ae}\x58\x02\u{2f6}\
	\u{2fb}\x05\u{aa}\x56\x02\u{2f7}\u{2fb}\x05\u{ac}\x57\x02\u{2f8}\u{2fb}\
	\x05\u{b2}\x5a\x02\u{2f9}\u{2fb}\x05\u{ba}\x5e\x02\u{2fa}\u{2f5}\x03\x02\
	\x02\x02\u{2fa}\u{2f6}\x03\x02\x02\x02\u{2fa}\u{2f7}\x03\x02\x02\x02\u{2fa}\
	\u{2f8}\x03\x02\x02\x02\u{2fa}\u{2f9}\x03\x02\x02\x02\u{2fb}\u{a9}\x03\x02\
	\x02\x02\u{2fc}\u{2fd}\x07\x19\x02\x02\u{2fd}\u{2fe}\x07\x7d\x02\x02\u{2fe}\
	\u{2ff}\x07\x5b\x02\x02\u{2ff}\u{300}\x05\u{c6}\x64\x02\u{300}\u{ab}\x03\
	\x02\x02\x02\u{301}\u{302}\x07\x1a\x02\x02\u{302}\u{303}\x07\x7d\x02\x02\
	\u{303}\u{304}\x07\x5b\x02\x02\u{304}\u{305}\x05\u{c6}\x64\x02\u{305}\u{ad}\
	\x03\x02\x02\x02\u{306}\u{307}\x07\x14\x02\x02\u{307}\u{308}\x05\u{bc}\x5f\
	\x02\u{308}\u{309}\x07\x15\x02\x02\u{309}\u{311}\x05\u{b0}\x59\x02\u{30a}\
	\u{30b}\x07\x16\x02\x02\u{30b}\u{30c}\x05\u{bc}\x5f\x02\u{30c}\u{30d}\x07\
	\x15\x02\x02\u{30d}\u{30e}\x05\u{b0}\x59\x02\u{30e}\u{310}\x03\x02\x02\x02\
	\u{30f}\u{30a}\x03\x02\x02\x02\u{310}\u{313}\x03\x02\x02\x02\u{311}\u{30f}\
	\x03\x02\x02\x02\u{311}\u{312}\x03\x02\x02\x02\u{312}\u{316}\x03\x02\x02\
	\x02\u{313}\u{311}\x03\x02\x02\x02\u{314}\u{315}\x07\x17\x02\x02\u{315}\
	\u{317}\x05\u{b0}\x59\x02\u{316}\u{314}\x03\x02\x02\x02\u{316}\u{317}\x03\
	\x02\x02\x02\u{317}\u{318}\x03\x02\x02\x02\u{318}\u{319}\x07\x18\x02\x02\
	\u{319}\u{af}\x03\x02\x02\x02\u{31a}\u{31c}\x05\u{a8}\x55\x02\u{31b}\u{31a}\
	\x03\x02\x02\x02\u{31c}\u{31d}\x03\x02\x02\x02\u{31d}\u{31b}\x03\x02\x02\
	\x02\u{31d}\u{31e}\x03\x02\x02\x02\u{31e}\u{b1}\x03\x02\x02\x02\u{31f}\u{324}\
	\x05\u{b4}\x5b\x02\u{320}\u{321}\x07\x6a\x02\x02\u{321}\u{323}\x05\u{b4}\
	\x5b\x02\u{322}\u{320}\x03\x02\x02\x02\u{323}\u{326}\x03\x02\x02\x02\u{324}\
	\u{322}\x03\x02\x02\x02\u{324}\u{325}\x03\x02\x02\x02\u{325}\u{b3}\x03\x02\
	\x02\x02\u{326}\u{324}\x03\x02\x02\x02\u{327}\u{32d}\x07\x7d\x02\x02\u{328}\
	\u{32a}\x07\x6d\x02\x02\u{329}\u{32b}\x05\u{b6}\x5c\x02\u{32a}\u{329}\x03\
	\x02\x02\x02\u{32a}\u{32b}\x03\x02\x02\x02\u{32b}\u{32c}\x03\x02\x02\x02\
	\u{32c}\u{32e}\x07\x6e\x02\x02\u{32d}\u{328}\x03\x02\x02\x02\u{32d}\u{32e}\
	\x03\x02\x02\x02\u{32e}\u{338}\x03\x02\x02\x02\u{32f}\u{335}\x07\x7a\x02\
	\x02\u{330}\u{332}\x07\x6d\x02\x02\u{331}\u{333}\x05\u{b6}\x5c\x02\u{332}\
	\u{331}\x03\x02\x02\x02\u{332}\u{333}\x03\x02\x02\x02\u{333}\u{334}\x03\
	\x02\x02\x02\u{334}\u{336}\x07\x6e\x02\x02\u{335}\u{330}\x03\x02\x02\x02\
	\u{335}\u{336}\x03\x02\x02\x02\u{336}\u{338}\x03\x02\x02\x02\u{337}\u{327}\
	\x03\x02\x02\x02\u{337}\u{32f}\x03\x02\x02\x02\u{338}\u{b5}\x03\x02\x02\
	\x02\u{339}\u{33e}\x05\u{b8}\x5d\x02\u{33a}\u{33b}\x07\x6a\x02\x02\u{33b}\
	\u{33d}\x05\u{b8}\x5d\x02\u{33c}\u{33a}\x03\x02\x02\x02\u{33d}\u{340}\x03\
	\x02\x02\x02\u{33e}\u{33c}\x03\x02\x02\x02\u{33e}\u{33f}\x03\x02\x02\x02\
	\u{33f}\u{b7}\x03\x02\x02\x02\u{340}\u{33e}\x03\x02\x02\x02\u{341}\u{342}\
	\x05\u{c6}\x64\x02\u{342}\u{b9}\x03\x02\x02\x02\u{343}\u{344}\x07\x0d\x02\
	\x02\u{344}\u{bb}\x03\x02\x02\x02\u{345}\u{34a}\x05\u{be}\x60\x02\u{346}\
	\u{347}\x09\x0e\x02\x02\u{347}\u{349}\x05\u{be}\x60\x02\u{348}\u{346}\x03\
	\x02\x02\x02\u{349}\u{34c}\x03\x02\x02\x02\u{34a}\u{348}\x03\x02\x02\x02\
	\u{34a}\u{34b}\x03\x02\x02\x02\u{34b}\u{bd}\x03\x02\x02\x02\u{34c}\u{34a}\
	\x03\x02\x02\x02\u{34d}\u{34f}\x07\x21\x02\x02\u{34e}\u{34d}\x03\x02\x02\
	\x02\u{34e}\u{34f}\x03\x02\x02\x02\u{34f}\u{350}\x03\x02\x02\x02\u{350}\
	\u{351}\x05\u{c0}\x61\x02\u{351}\u{bf}\x03\x02\x02\x02\u{352}\u{359}\x05\
	\u{c2}\x62\x02\u{353}\u{354}\x07\x6d\x02\x02\u{354}\u{355}\x05\u{bc}\x5f\
	\x02\u{355}\u{356}\x07\x6e\x02\x02\u{356}\u{359}\x03\x02\x02\x02\u{357}\
	\u{359}\x05\u{e2}\x72\x02\u{358}\u{352}\x03\x02\x02\x02\u{358}\u{353}\x03\
	\x02\x02\x02\u{358}\u{357}\x03\x02\x02\x02\u{359}\u{c1}\x03\x02\x02\x02\
	\u{35a}\u{35b}\x05\u{c6}\x64\x02\u{35b}\u{35c}\x05\u{c4}\x63\x02\u{35c}\
	\u{35d}\x05\u{c6}\x64\x02\u{35d}\u{38d}\x03\x02\x02\x02\u{35e}\u{35f}\x05\
	\u{c6}\x64\x02\u{35f}\u{360}\x07\x22\x02\x02\u{360}\u{361}\x07\x6d\x02\x02\
	\u{361}\u{362}\x05\u{e8}\x75\x02\u{362}\u{363}\x07\x6e\x02\x02\u{363}\u{38d}\
	\x03\x02\x02\x02\u{364}\u{365}\x05\u{c6}\x64\x02\u{365}\u{366}\x07\x21\x02\
	\x02\u{366}\u{367}\x07\x22\x02\x02\u{367}\u{368}\x07\x6d\x02\x02\u{368}\
	\u{369}\x05\u{e8}\x75\x02\u{369}\u{36a}\x07\x6e\x02\x02\u{36a}\u{38d}\x03\
	\x02\x02\x02\u{36b}\u{36c}\x05\u{c6}\x64\x02\u{36c}\u{36d}\x07\x22\x02\x02\
	\u{36d}\u{36e}\x05\u{ea}\x76\x02\u{36e}\u{38d}\x03\x02\x02\x02\u{36f}\u{370}\
	\x05\u{c6}\x64\x02\u{370}\u{371}\x07\x21\x02\x02\u{371}\u{372}\x07\x22\x02\
	\x02\u{372}\u{373}\x05\u{ea}\x76\x02\u{373}\u{38d}\x03\x02\x02\x02\u{374}\
	\u{375}\x05\u{c6}\x64\x02\u{375}\u{376}\x07\x22\x02\x02\u{376}\u{377}\x05\
	\u{e4}\x73\x02\u{377}\u{38d}\x03\x02\x02\x02\u{378}\u{379}\x05\u{c6}\x64\
	\x02\u{379}\u{37a}\x07\x21\x02\x02\u{37a}\u{37b}\x07\x22\x02\x02\u{37b}\
	\u{37c}\x05\u{e4}\x73\x02\u{37c}\u{38d}\x03\x02\x02\x02\u{37d}\u{37e}\x05\
	\u{c6}\x64\x02\u{37e}\u{37f}\x07\x23\x02\x02\u{37f}\u{380}\x07\x24\x02\x02\
	\u{380}\u{38d}\x03\x02\x02\x02\u{381}\u{382}\x05\u{c6}\x64\x02\u{382}\u{383}\
	\x07\x23\x02\x02\u{383}\u{384}\x07\x21\x02\x02\u{384}\u{385}\x07\x24\x02\
	\x02\u{385}\u{38d}\x03\x02\x02\x02\u{386}\u{387}\x05\u{c6}\x64\x02\u{387}\
	\u{388}\x07\x25\x02\x02\u{388}\u{38d}\x03\x02\x02\x02\u{389}\u{38a}\x05\
	\u{c6}\x64\x02\u{38a}\u{38b}\x07\x26\x02\x02\u{38b}\u{38d}\x03\x02\x02\x02\
	\u{38c}\u{35a}\x03\x02\x02\x02\u{38c}\u{35e}\x03\x02\x02\x02\u{38c}\u{364}\
	\x03\x02\x02\x02\u{38c}\u{36b}\x03\x02\x02\x02\u{38c}\u{36f}\x03\x02\x02\
	\x02\u{38c}\u{374}\x03\x02\x02\x02\u{38c}\u{378}\x03\x02\x02\x02\u{38c}\
	\u{37d}\x03\x02\x02\x02\u{38c}\u{381}\x03\x02\x02\x02\u{38c}\u{386}\x03\
	\x02\x02\x02\u{38c}\u{389}\x03\x02\x02\x02\u{38d}\u{c3}\x03\x02\x02\x02\
	\u{38e}\u{38f}\x09\x0f\x02\x02\u{38f}\u{c5}\x03\x02\x02\x02\u{390}\u{395}\
	\x05\u{c8}\x65\x02\u{391}\u{392}\x09\x10\x02\x02\u{392}\u{394}\x05\u{c8}\
	\x65\x02\u{393}\u{391}\x03\x02\x02\x02\u{394}\u{397}\x03\x02\x02\x02\u{395}\
	\u{393}\x03\x02\x02\x02\u{395}\u{396}\x03\x02\x02\x02\u{396}\u{c7}\x03\x02\
	\x02\x02\u{397}\u{395}\x03\x02\x02\x02\u{398}\u{39d}\x05\u{ca}\x66\x02\u{399}\
	\u{39a}\x09\x11\x02\x02\u{39a}\u{39c}\x05\u{ca}\x66\x02\u{39b}\u{399}\x03\
	\x02\x02\x02\u{39c}\u{39f}\x03\x02\x02\x02\u{39d}\u{39b}\x03\x02\x02\x02\
	\u{39d}\u{39e}\x03\x02\x02\x02\u{39e}\u{c9}\x03\x02\x02\x02\u{39f}\u{39d}\
	\x03\x02\x02\x02\u{3a0}\u{3a2}\x07\x62\x02\x02\u{3a1}\u{3a0}\x03\x02\x02\
	\x02\u{3a1}\u{3a2}\x03\x02\x02\x02\u{3a2}\u{3a3}\x03\x02\x02\x02\u{3a3}\
	\u{3a4}\x05\u{cc}\x67\x02\u{3a4}\u{cb}\x03\x02\x02\x02\u{3a5}\u{3b1}\x05\
	\u{f2}\x7a\x02\u{3a6}\u{3b1}\x05\u{e4}\x73\x02\u{3a7}\u{3b1}\x05\u{ce}\x68\
	\x02\u{3a8}\u{3b1}\x05\u{e2}\x72\x02\u{3a9}\u{3b1}\x05\u{ea}\x76\x02\u{3aa}\
	\u{3b1}\x05\u{ec}\x77\x02\u{3ab}\u{3b1}\x05\u{de}\x70\x02\u{3ac}\u{3ad}\
	\x07\x6d\x02\x02\u{3ad}\u{3ae}\x05\u{c6}\x64\x02\u{3ae}\u{3af}\x07\x6e\x02\
	\x02\u{3af}\u{3b1}\x03\x02\x02\x02\u{3b0}\u{3a5}\x03\x02\x02\x02\u{3b0}\
	\u{3a6}\x03\x02\x02\x02\u{3b0}\u{3a7}\x03\x02\x02\x02\u{3b0}\u{3a8}\x03\
	\x02\x02\x02\u{3b0}\u{3a9}\x03\x02\x02\x02\u{3b0}\u{3aa}\x03\x02\x02\x02\
	\u{3b0}\u{3ab}\x03\x02\x02\x02\u{3b0}\u{3ac}\x03\x02\x02\x02\u{3b1}\u{cd}\
	\x03\x02\x02\x02\u{3b2}\u{3b3}\x05\u{d0}\x69\x02\u{3b3}\u{3b4}\x07\x6d\x02\
	\x02\u{3b4}\u{3b5}\x05\u{c6}\x64\x02\u{3b5}\u{3b6}\x07\x6a\x02\x02\u{3b6}\
	\u{3b7}\x05\u{d6}\x6c\x02\u{3b7}\u{3b8}\x07\x6e\x02\x02\u{3b8}\u{3ca}\x03\
	\x02\x02\x02\u{3b9}\u{3ba}\x05\u{d2}\x6a\x02\u{3ba}\u{3bb}\x07\x6d\x02\x02\
	\u{3bb}\u{3be}\x05\u{c6}\x64\x02\u{3bc}\u{3bd}\x07\x6a\x02\x02\u{3bd}\u{3bf}\
	\x05\u{e4}\x73\x02\u{3be}\u{3bc}\x03\x02\x02\x02\u{3be}\u{3bf}\x03\x02\x02\
	\x02\u{3bf}\u{3c0}\x03\x02\x02\x02\u{3c0}\u{3c1}\x07\x6e\x02\x02\u{3c1}\
	\u{3ca}\x03\x02\x02\x02\u{3c2}\u{3c3}\x05\u{d4}\x6b\x02\u{3c3}\u{3c4}\x07\
	\x6d\x02\x02\u{3c4}\u{3c5}\x05\u{c6}\x64\x02\u{3c5}\u{3c6}\x07\x6a\x02\x02\
	\u{3c6}\u{3c7}\x05\u{d6}\x6c\x02\u{3c7}\u{3c8}\x07\x6e\x02\x02\u{3c8}\u{3ca}\
	\x03\x02\x02\x02\u{3c9}\u{3b2}\x03\x02\x02\x02\u{3c9}\u{3b9}\x03\x02\x02\
	\x02\u{3c9}\u{3c2}\x03\x02\x02\x02\u{3ca}\u{cf}\x03\x02\x02\x02\u{3cb}\u{3cc}\
	\x09\x12\x02\x02\u{3cc}\u{d1}\x03\x02\x02\x02\u{3cd}\u{3ce}\x09\x13\x02\
	\x02\u{3ce}\u{d3}\x03\x02\x02\x02\u{3cf}\u{3d0}\x09\x14\x02\x02\u{3d0}\u{d5}\
	\x03\x02\x02\x02\u{3d1}\u{3d4}\x05\u{de}\x70\x02\u{3d2}\u{3d4}\x05\u{d8}\
	\x6d\x02\u{3d3}\u{3d1}\x03\x02\x02\x02\u{3d3}\u{3d2}\x03\x02\x02\x02\u{3d4}\
	\u{d7}\x03\x02\x02\x02\u{3d5}\u{3da}\x05\u{da}\x6e\x02\u{3d6}\u{3d7}\x07\
	\x20\x02\x02\u{3d7}\u{3d9}\x05\u{da}\x6e\x02\u{3d8}\u{3d6}\x03\x02\x02\x02\
	\u{3d9}\u{3dc}\x03\x02\x02\x02\u{3da}\u{3d8}\x03\x02\x02\x02\u{3da}\u{3db}\
	\x03\x02\x02\x02\u{3db}\u{d9}\x03\x02\x02\x02\u{3dc}\u{3da}\x03\x02\x02\
	\x02\u{3dd}\u{3e2}\x05\u{dc}\x6f\x02\u{3de}\u{3df}\x07\x1f\x02\x02\u{3df}\
	\u{3e1}\x05\u{dc}\x6f\x02\u{3e0}\u{3de}\x03\x02\x02\x02\u{3e1}\u{3e4}\x03\
	\x02\x02\x02\u{3e2}\u{3e0}\x03\x02\x02\x02\u{3e2}\u{3e3}\x03\x02\x02\x02\
	\u{3e3}\u{db}\x03\x02\x02\x02\u{3e4}\u{3e2}\x03\x02\x02\x02\u{3e5}\u{3e6}\
	\x07\x21\x02\x02\u{3e6}\u{406}\x05\u{dc}\x6f\x02\u{3e7}\u{3e8}\x05\u{e4}\
	\x73\x02\u{3e8}\u{3e9}\x05\u{c4}\x63\x02\u{3e9}\u{3ea}\x05\u{c6}\x64\x02\
	\u{3ea}\u{406}\x03\x02\x02\x02\u{3eb}\u{3ec}\x05\u{e4}\x73\x02\u{3ec}\u{3ed}\
	\x07\x22\x02\x02\u{3ed}\u{3ee}\x07\x6d\x02\x02\u{3ee}\u{3ef}\x05\u{e8}\x75\
	\x02\u{3ef}\u{3f0}\x07\x6e\x02\x02\u{3f0}\u{406}\x03\x02\x02\x02\u{3f1}\
	\u{3f2}\x05\u{e4}\x73\x02\u{3f2}\u{3f3}\x07\x21\x02\x02\u{3f3}\u{3f4}\x07\
	\x22\x02\x02\u{3f4}\u{3f5}\x07\x6d\x02\x02\u{3f5}\u{3f6}\x05\u{e8}\x75\x02\
	\u{3f6}\u{3f7}\x07\x6e\x02\x02\u{3f7}\u{406}\x03\x02\x02\x02\u{3f8}\u{3f9}\
	\x05\u{e4}\x73\x02\u{3f9}\u{3fa}\x07\x23\x02\x02\u{3fa}\u{3fb}\x07\x24\x02\
	\x02\u{3fb}\u{406}\x03\x02\x02\x02\u{3fc}\u{3fd}\x05\u{e4}\x73\x02\u{3fd}\
	\u{3fe}\x07\x23\x02\x02\u{3fe}\u{3ff}\x07\x21\x02\x02\u{3ff}\u{400}\x07\
	\x24\x02\x02\u{400}\u{406}\x03\x02\x02\x02\u{401}\u{402}\x07\x6d\x02\x02\
	\u{402}\u{403}\x05\u{d8}\x6d\x02\u{403}\u{404}\x07\x6e\x02\x02\u{404}\u{406}\
	\x03\x02\x02\x02\u{405}\u{3e5}\x03\x02\x02\x02\u{405}\u{3e7}\x03\x02\x02\
	\x02\u{405}\u{3eb}\x03\x02\x02\x02\u{405}\u{3f1}\x03\x02\x02\x02\u{405}\
	\u{3f8}\x03\x02\x02\x02\u{405}\u{3fc}\x03\x02\x02\x02\u{405}\u{401}\x03\
	\x02\x02\x02\u{406}\u{dd}\x03\x02\x02\x02\u{407}\u{408}\x07\x7d\x02\x02\
	\u{408}\u{409}\x07\x67\x02\x02\u{409}\u{417}\x05\u{c6}\x64\x02\u{40a}\u{40b}\
	\x07\x6d\x02\x02\u{40b}\u{410}\x07\x7d\x02\x02\u{40c}\u{40d}\x07\x6a\x02\
	\x02\u{40d}\u{40f}\x07\x7d\x02\x02\u{40e}\u{40c}\x03\x02\x02\x02\u{40f}\
	\u{412}\x03\x02\x02\x02\u{410}\u{40e}\x03\x02\x02\x02\u{410}\u{411}\x03\
	\x02\x02\x02\u{411}\u{413}\x03\x02\x02\x02\u{412}\u{410}\x03\x02\x02\x02\
	\u{413}\u{414}\x07\x6e\x02\x02\u{414}\u{415}\x07\x67\x02\x02\u{415}\u{417}\
	\x05\u{c6}\x64\x02\u{416}\u{407}\x03\x02\x02\x02\u{416}\u{40a}\x03\x02\x02\
	\x02\u{417}\u{df}\x03\x02\x02\x02\u{418}\u{419}\x05\u{c6}\x64\x02\u{419}\
	\u{e1}\x03\x02\x02\x02\u{41a}\u{41b}\x07\x7d\x02\x02\u{41b}\u{424}\x07\x6d\
	\x02\x02\u{41c}\u{421}\x05\u{c6}\x64\x02\u{41d}\u{41e}\x07\x6a\x02\x02\u{41e}\
	\u{420}\x05\u{c6}\x64\x02\u{41f}\u{41d}\x03\x02\x02\x02\u{420}\u{423}\x03\
	\x02\x02\x02\u{421}\u{41f}\x03\x02\x02\x02\u{421}\u{422}\x03\x02\x02\x02\
	\u{422}\u{425}\x03\x02\x02\x02\u{423}\u{421}\x03\x02\x02\x02\u{424}\u{41c}\
	\x03\x02\x02\x02\u{424}\u{425}\x03\x02\x02\x02\u{425}\u{426}\x03\x02\x02\
	\x02\u{426}\u{427}\x07\x6e\x02\x02\u{427}\u{e3}\x03\x02\x02\x02\u{428}\u{42d}\
	\x05\u{e6}\x74\x02\u{429}\u{42a}\x07\x6c\x02\x02\u{42a}\u{42c}\x05\u{e6}\
	\x74\x02\u{42b}\u{429}\x03\x02\x02\x02\u{42c}\u{42f}\x03\x02\x02\x02\u{42d}\
	\u{42b}\x03\x02\x02\x02\u{42d}\u{42e}\x03\x02\x02\x02\u{42e}\u{43c}\x03\
	\x02\x02\x02\u{42f}\u{42d}\x03\x02\x02\x02\u{430}\u{431}\x07\x7d\x02\x02\
	\u{431}\u{432}\x07\x6f\x02\x02\u{432}\u{433}\x07\x76\x02\x02\u{433}\u{438}\
	\x07\x70\x02\x02\u{434}\u{435}\x07\x6c\x02\x02\u{435}\u{437}\x05\u{e6}\x74\
	\x02\u{436}\u{434}\x03\x02\x02\x02\u{437}\u{43a}\x03\x02\x02\x02\u{438}\
	\u{436}\x03\x02\x02\x02\u{438}\u{439}\x03\x02\x02\x02\u{439}\u{43c}\x03\
	\x02\x02\x02\u{43a}\u{438}\x03\x02\x02\x02\u{43b}\u{428}\x03\x02\x02\x02\
	\u{43b}\u{430}\x03\x02\x02\x02\u{43c}\u{e5}\x03\x02\x02\x02\u{43d}\u{43e}\
	\x09\x06\x02\x02\u{43e}\u{e7}\x03\x02\x02\x02\u{43f}\u{444}\x05\u{c6}\x64\
	\x02\u{440}\u{441}\x07\x6a\x02\x02\u{441}\u{443}\x05\u{c6}\x64\x02\u{442}\
	\u{440}\x03\x02\x02\x02\u{443}\u{446}\x03\x02\x02\x02\u{444}\u{442}\x03\
	\x02\x02\x02\u{444}\u{445}\x03\x02\x02\x02\u{445}\u{e9}\x03\x02\x02\x02\
	\u{446}\u{444}\x03\x02\x02\x02\u{447}\u{450}\x07\x6f\x02\x02\u{448}\u{44d}\
	\x05\u{c6}\x64\x02\u{449}\u{44a}\x07\x6a\x02\x02\u{44a}\u{44c}\x05\u{c6}\
	\x64\x02\u{44b}\u{449}\x03\x02\x02\x02\u{44c}\u{44f}\x03\x02\x02\x02\u{44d}\
	\u{44b}\x03\x02\x02\x02\u{44d}\u{44e}\x03\x02\x02\x02\u{44e}\u{451}\x03\
	\x02\x02\x02\u{44f}\u{44d}\x03\x02\x02\x02\u{450}\u{448}\x03\x02\x02\x02\
	\u{450}\u{451}\x03\x02\x02\x02\u{451}\u{452}\x03\x02\x02\x02\u{452}\u{453}\
	\x07\x70\x02\x02\u{453}\u{eb}\x03\x02\x02\x02\u{454}\u{45d}\x07\x71\x02\
	\x02\u{455}\u{45a}\x05\u{ee}\x78\x02\u{456}\u{457}\x07\x6a\x02\x02\u{457}\
	\u{459}\x05\u{ee}\x78\x02\u{458}\u{456}\x03\x02\x02\x02\u{459}\u{45c}\x03\
	\x02\x02\x02\u{45a}\u{458}\x03\x02\x02\x02\u{45a}\u{45b}\x03\x02\x02\x02\
	\u{45b}\u{45e}\x03\x02\x02\x02\u{45c}\u{45a}\x03\x02\x02\x02\u{45d}\u{455}\
	\x03\x02\x02\x02\u{45d}\u{45e}\x03\x02\x02\x02\u{45e}\u{45f}\x03\x02\x02\
	\x02\u{45f}\u{460}\x07\x72\x02\x02\u{460}\u{ed}\x03\x02\x02\x02\u{461}\u{462}\
	\x05\u{f0}\x79\x02\u{462}\u{463}\x07\x69\x02\x02\u{463}\u{464}\x05\u{c6}\
	\x64\x02\u{464}\u{ef}\x03\x02\x02\x02\u{465}\u{466}\x09\x15\x02\x02\u{466}\
	\u{f1}\x03\x02\x02\x02\u{467}\u{46e}\x05\x4a\x26\x02\u{468}\u{46e}\x05\u{f4}\
	\x7b\x02\u{469}\u{46e}\x07\x78\x02\x02\u{46a}\u{46e}\x07\x79\x02\x02\u{46b}\
	\u{46e}\x07\x74\x02\x02\u{46c}\u{46e}\x07\x24\x02\x02\u{46d}\u{467}\x03\
	\x02\x02\x02\u{46d}\u{468}\x03\x02\x02\x02\u{46d}\u{469}\x03\x02\x02\x02\
	\u{46d}\u{46a}\x03\x02\x02\x02\u{46d}\u{46b}\x03\x02\x02\x02\u{46d}\u{46c}\
	\x03\x02\x02\x02\u{46e}\u{f3}\x03\x02\x02\x02\u{46f}\u{476}\x07\x76\x02\
	\x02\u{470}\u{476}\x07\x77\x02\x02\u{471}\u{472}\x07\x62\x02\x02\u{472}\
	\u{476}\x07\x76\x02\x02\u{473}\u{474}\x07\x62\x02\x02\u{474}\u{476}\x07\
	\x77\x02\x02\u{475}\u{46f}\x03\x02\x02\x02\u{475}\u{470}\x03\x02\x02\x02\
	\u{475}\u{471}\x03\x02\x02\x02\u{475}\u{473}\x03\x02\x02\x02\u{476}\u{f5}\
	\x03\x02\x02\x02\x5f\u{f9}\u{fd}\u{100}\u{104}\u{106}\u{110}\u{11e}\u{12a}\
	\u{130}\u{13f}\u{146}\u{152}\u{15d}\u{169}\u{170}\u{17d}\u{188}\u{199}\u{1a7}\
	\u{1aa}\u{1ad}\u{1b4}\u{1b7}\u{1ba}\u{1d3}\u{1d9}\u{1e1}\u{1f0}\u{1f4}\u{1f9}\
	\u{209}\u{20e}\u{218}\u{223}\u{22e}\u{23b}\u{245}\u{24e}\u{261}\u{269}\u{276}\
	\u{27d}\u{289}\u{292}\u{295}\u{29d}\u{2a8}\u{2bc}\u{2c0}\u{2c8}\u{2cf}\u{2e0}\
	\u{2ea}\u{2ef}\u{2fa}\u{311}\u{316}\u{31d}\u{324}\u{32a}\u{32d}\u{332}\u{335}\
	\u{337}\u{33e}\u{34a}\u{34e}\u{358}\u{38c}\u{395}\u{39d}\u{3a1}\u{3b0}\u{3be}\
	\u{3c9}\u{3d3}\u{3da}\u{3e2}\u{405}\u{410}\u{416}\u{421}\u{424}\u{42d}\u{438}\
	\u{43b}\u{444}\u{44d}\u{450}\u{45a}\u{45d}\u{46d}\u{475}";

