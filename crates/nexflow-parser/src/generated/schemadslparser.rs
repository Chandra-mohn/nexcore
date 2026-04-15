// Generated from /Users/chandramohn/workspace/nexcore/grammar/nexflow/SchemaDSL.g4 by ANTLR 4.8
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
use super::schemadsllistener::*;
use super::schemadslvisitor::*;

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
		pub const T__122:isize=123; 
		pub const T__123:isize=124; 
		pub const T__124:isize=125; 
		pub const T__125:isize=126; 
		pub const T__126:isize=127; 
		pub const T__127:isize=128; 
		pub const T__128:isize=129; 
		pub const T__129:isize=130; 
		pub const T__130:isize=131; 
		pub const T__131:isize=132; 
		pub const T__132:isize=133; 
		pub const T__133:isize=134; 
		pub const T__134:isize=135; 
		pub const T__135:isize=136; 
		pub const T__136:isize=137; 
		pub const T__137:isize=138; 
		pub const T__138:isize=139; 
		pub const T__139:isize=140; 
		pub const T__140:isize=141; 
		pub const T__141:isize=142; 
		pub const T__142:isize=143; 
		pub const T__143:isize=144; 
		pub const T__144:isize=145; 
		pub const T__145:isize=146; 
		pub const T__146:isize=147; 
		pub const T__147:isize=148; 
		pub const T__148:isize=149; 
		pub const T__149:isize=150; 
		pub const T__150:isize=151; 
		pub const T__151:isize=152; 
		pub const T__152:isize=153; 
		pub const T__153:isize=154; 
		pub const T__154:isize=155; 
		pub const T__155:isize=156; 
		pub const T__156:isize=157; 
		pub const T__157:isize=158; 
		pub const T__158:isize=159; 
		pub const T__159:isize=160; 
		pub const T__160:isize=161; 
		pub const T__161:isize=162; 
		pub const T__162:isize=163; 
		pub const T__163:isize=164; 
		pub const T__164:isize=165; 
		pub const T__165:isize=166; 
		pub const T__166:isize=167; 
		pub const IMPORT:isize=168; 
		pub const PII:isize=169; 
		pub const VERSION_NUMBER:isize=170; 
		pub const INTEGER:isize=171; 
		pub const DECIMAL:isize=172; 
		pub const DURATION_LITERAL:isize=173; 
		pub const BOOLEAN:isize=174; 
		pub const IDENTIFIER:isize=175; 
		pub const UPPER_IDENTIFIER:isize=176; 
		pub const STRING:isize=177; 
		pub const MULTILINE_STRING:isize=178; 
		pub const COLON:isize=179; 
		pub const COMMA:isize=180; 
		pub const DOTDOT:isize=181; 
		pub const DOT:isize=182; 
		pub const LBRACKET:isize=183; 
		pub const RBRACKET:isize=184; 
		pub const LPAREN:isize=185; 
		pub const RPAREN:isize=186; 
		pub const LANGLE:isize=187; 
		pub const RANGLE:isize=188; 
		pub const EQ:isize=189; 
		pub const NE:isize=190; 
		pub const LE:isize=191; 
		pub const GE:isize=192; 
		pub const PLUS:isize=193; 
		pub const MINUS:isize=194; 
		pub const STAR:isize=195; 
		pub const SLASH:isize=196; 
		pub const ARROW:isize=197; 
		pub const LBRACE:isize=198; 
		pub const RBRACE:isize=199; 
		pub const COMMENT:isize=200; 
		pub const WS:isize=201;
	pub const RULE_program:usize = 0; 
	pub const RULE_importStatement:usize = 1; 
	pub const RULE_importPath:usize = 2; 
	pub const RULE_importPathSegment:usize = 3; 
	pub const RULE_importFileExtension:usize = 4; 
	pub const RULE_schemaDefinition:usize = 5; 
	pub const RULE_schemaName:usize = 6; 
	pub const RULE_patternDecl:usize = 7; 
	pub const RULE_mutationPattern:usize = 8; 
	pub const RULE_targetsDecl:usize = 9; 
	pub const RULE_targetList:usize = 10; 
	pub const RULE_target:usize = 11; 
	pub const RULE_versionBlock:usize = 12; 
	pub const RULE_compatibilityDecl:usize = 13; 
	pub const RULE_compatibilityMode:usize = 14; 
	pub const RULE_previousVersionDecl:usize = 15; 
	pub const RULE_deprecationDecl:usize = 16; 
	pub const RULE_migrationGuideDecl:usize = 17; 
	pub const RULE_retentionDecl:usize = 18; 
	pub const RULE_immutableDecl:usize = 19; 
	pub const RULE_constraintsBlock:usize = 20; 
	pub const RULE_constraintDecl:usize = 21; 
	pub const RULE_enumConstraint:usize = 22; 
	pub const RULE_enumValue:usize = 23; 
	pub const RULE_rangeConstraint:usize = 24; 
	pub const RULE_patternConstraint:usize = 25; 
	pub const RULE_lengthConstraint:usize = 26; 
	pub const RULE_identityBlock:usize = 27; 
	pub const RULE_identityFieldV2:usize = 28; 
	pub const RULE_streamingBlock:usize = 29; 
	pub const RULE_streamingDecl:usize = 30; 
	pub const RULE_keyFieldsDecl:usize = 31; 
	pub const RULE_timeFieldDecl:usize = 32; 
	pub const RULE_timeSemanticsDecl:usize = 33; 
	pub const RULE_timeSemanticsType:usize = 34; 
	pub const RULE_watermarkDecl:usize = 35; 
	pub const RULE_watermarkStrategy:usize = 36; 
	pub const RULE_lateDataDecl:usize = 37; 
	pub const RULE_lateDataStrategy:usize = 38; 
	pub const RULE_allowedLatenessDecl:usize = 39; 
	pub const RULE_idleDecl:usize = 40; 
	pub const RULE_idleBehavior:usize = 41; 
	pub const RULE_sparsityDecl:usize = 42; 
	pub const RULE_sparsityBlock:usize = 43; 
	pub const RULE_retentionBlockDecl:usize = 44; 
	pub const RULE_retentionOptions:usize = 45; 
	pub const RULE_retentionPolicy:usize = 46; 
	pub const RULE_serializationBlock:usize = 47; 
	pub const RULE_serializationDecl:usize = 48; 
	pub const RULE_formatDecl:usize = 49; 
	pub const RULE_serializationFormat:usize = 50; 
	pub const RULE_serializationCompatibilityDecl:usize = 51; 
	pub const RULE_subjectDecl:usize = 52; 
	pub const RULE_registryDecl:usize = 53; 
	pub const RULE_fieldsBlock:usize = 54; 
	pub const RULE_fieldDeclV2:usize = 55; 
	pub const RULE_fieldName:usize = 56; 
	pub const RULE_nestedObjectBlock:usize = 57; 
	pub const RULE_computedBlock:usize = 58; 
	pub const RULE_computedField:usize = 59; 
	pub const RULE_computedExpression:usize = 60; 
	pub const RULE_computedWhenExpression:usize = 61; 
	pub const RULE_stateMachineBlock:usize = 62; 
	pub const RULE_initialStateDecl:usize = 63; 
	pub const RULE_forEntityDecl:usize = 64; 
	pub const RULE_statesBlock:usize = 65; 
	pub const RULE_statesDecl:usize = 66; 
	pub const RULE_stateDefList:usize = 67; 
	pub const RULE_stateDef:usize = 68; 
	pub const RULE_stateQualifier:usize = 69; 
	pub const RULE_stateArray:usize = 70; 
	pub const RULE_transitionsBlock:usize = 71; 
	pub const RULE_transitionDecl:usize = 72; 
	pub const RULE_transitionArrowDecl:usize = 73; 
	pub const RULE_onTransitionBlock:usize = 74; 
	pub const RULE_transitionAction:usize = 75; 
	pub const RULE_actionCall:usize = 76; 
	pub const RULE_parametersBlock:usize = 77; 
	pub const RULE_parameterDeclV2:usize = 78; 
	pub const RULE_parameterOption:usize = 79; 
	pub const RULE_entriesBlock:usize = 80; 
	pub const RULE_entryDecl:usize = 81; 
	pub const RULE_entryFieldV2:usize = 82; 
	pub const RULE_ruleBlock:usize = 83; 
	pub const RULE_givenBlock:usize = 84; 
	pub const RULE_ruleFieldDeclV2:usize = 85; 
	pub const RULE_calculateBlock:usize = 86; 
	pub const RULE_calculation:usize = 87; 
	pub const RULE_returnBlock:usize = 88; 
	pub const RULE_migrationBlock:usize = 89; 
	pub const RULE_migrationStatement:usize = 90; 
	pub const RULE_typeAliasBlock:usize = 91; 
	pub const RULE_typeAliasV2:usize = 92; 
	pub const RULE_aliasName:usize = 93; 
	pub const RULE_fieldTypeV2:usize = 94; 
	pub const RULE_baseTypeV2:usize = 95; 
	pub const RULE_typeParams:usize = 96; 
	pub const RULE_collectionTypeV2:usize = 97; 
	pub const RULE_inlineObjectTypeV2:usize = 98; 
	pub const RULE_inlineFieldDeclV2:usize = 99; 
	pub const RULE_fieldQualifierV2:usize = 100; 
	pub const RULE_piiModifier:usize = 101; 
	pub const RULE_defaultClauseV2:usize = 102; 
	pub const RULE_deprecatedClauseV2:usize = 103; 
	pub const RULE_expression:usize = 104; 
	pub const RULE_whenExpression:usize = 105; 
	pub const RULE_condition:usize = 106; 
	pub const RULE_comparisonOp:usize = 107; 
	pub const RULE_operator:usize = 108; 
	pub const RULE_functionCall:usize = 109; 
	pub const RULE_fieldPath:usize = 110; 
	pub const RULE_fieldList:usize = 111; 
	pub const RULE_fieldArray:usize = 112; 
	pub const RULE_duration:usize = 113; 
	pub const RULE_timeUnit:usize = 114; 
	pub const RULE_sizeSpec:usize = 115; 
	pub const RULE_sizeUnit:usize = 116; 
	pub const RULE_literal:usize = 117; 
	pub const RULE_numberLiteral:usize = 118;
	pub const ruleNames: [&'static str; 119] =  [
		"program", "importStatement", "importPath", "importPathSegment", "importFileExtension", 
		"schemaDefinition", "schemaName", "patternDecl", "mutationPattern", "targetsDecl", 
		"targetList", "target", "versionBlock", "compatibilityDecl", "compatibilityMode", 
		"previousVersionDecl", "deprecationDecl", "migrationGuideDecl", "retentionDecl", 
		"immutableDecl", "constraintsBlock", "constraintDecl", "enumConstraint", 
		"enumValue", "rangeConstraint", "patternConstraint", "lengthConstraint", 
		"identityBlock", "identityFieldV2", "streamingBlock", "streamingDecl", 
		"keyFieldsDecl", "timeFieldDecl", "timeSemanticsDecl", "timeSemanticsType", 
		"watermarkDecl", "watermarkStrategy", "lateDataDecl", "lateDataStrategy", 
		"allowedLatenessDecl", "idleDecl", "idleBehavior", "sparsityDecl", "sparsityBlock", 
		"retentionBlockDecl", "retentionOptions", "retentionPolicy", "serializationBlock", 
		"serializationDecl", "formatDecl", "serializationFormat", "serializationCompatibilityDecl", 
		"subjectDecl", "registryDecl", "fieldsBlock", "fieldDeclV2", "fieldName", 
		"nestedObjectBlock", "computedBlock", "computedField", "computedExpression", 
		"computedWhenExpression", "stateMachineBlock", "initialStateDecl", "forEntityDecl", 
		"statesBlock", "statesDecl", "stateDefList", "stateDef", "stateQualifier", 
		"stateArray", "transitionsBlock", "transitionDecl", "transitionArrowDecl", 
		"onTransitionBlock", "transitionAction", "actionCall", "parametersBlock", 
		"parameterDeclV2", "parameterOption", "entriesBlock", "entryDecl", "entryFieldV2", 
		"ruleBlock", "givenBlock", "ruleFieldDeclV2", "calculateBlock", "calculation", 
		"returnBlock", "migrationBlock", "migrationStatement", "typeAliasBlock", 
		"typeAliasV2", "aliasName", "fieldTypeV2", "baseTypeV2", "typeParams", 
		"collectionTypeV2", "inlineObjectTypeV2", "inlineFieldDeclV2", "fieldQualifierV2", 
		"piiModifier", "defaultClauseV2", "deprecatedClauseV2", "expression", 
		"whenExpression", "condition", "comparisonOp", "operator", "functionCall", 
		"fieldPath", "fieldList", "fieldArray", "duration", "timeUnit", "sizeSpec", 
		"sizeUnit", "literal", "numberLiteral"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;200] = [
		None, Some("'schema'"), Some("'transform'"), Some("'flow'"), Some("'rules'"), 
		Some("'end'"), Some("'pattern'"), Some("'master_data'"), Some("'immutable_ledger'"), 
		Some("'versioned_configuration'"), Some("'operational_parameters'"), Some("'event_log'"), 
		Some("'state_machine'"), Some("'temporal_data'"), Some("'reference_data'"), 
		Some("'business_logic'"), Some("'command'"), Some("'response'"), Some("'aggregate'"), 
		Some("'document'"), Some("'audit_event'"), Some("'targets'"), Some("'kafka'"), 
		Some("'flink'"), Some("'mongo'"), Some("'java'"), Some("'all'"), Some("'version'"), 
		Some("'compatibility'"), Some("'evolution'"), Some("'backward'"), Some("'forward'"), 
		Some("'full'"), Some("'none'"), Some("'backward_compatible'"), Some("'forward_compatible'"), 
		Some("'previous_version'"), Some("'deprecated'"), Some("'deprecated_since'"), 
		Some("'removal_version'"), Some("'migration_guide'"), Some("'retention'"), 
		Some("'immutable'"), Some("'constraints'"), Some("'as'"), Some("'enum'"), 
		Some("'range'"), Some("'length'"), Some("'identity'"), Some("'streaming'"), 
		Some("'key_fields'"), Some("'time_field'"), Some("'time_semantics'"), 
		Some("'event_time'"), Some("'processing_time'"), Some("'ingestion_time'"), 
		Some("'watermark_delay'"), Some("'watermark_strategy'"), Some("'max_out_of_orderness'"), 
		Some("'watermark_interval'"), Some("'watermark_field'"), Some("'bounded_out_of_orderness'"), 
		Some("'periodic'"), Some("'punctuated'"), Some("'late_data_handling'"), 
		Some("'late_data_stream'"), Some("'side_output'"), Some("'drop'"), Some("'update'"), 
		Some("'allowed_lateness'"), Some("'idle_timeout'"), Some("'idle_behavior'"), 
		Some("'mark_idle'"), Some("'advance_to_infinity'"), Some("'keep_waiting'"), 
		Some("'sparsity'"), Some("'dense'"), Some("'moderate'"), Some("'sparse'"), 
		Some("'time'"), Some("'size'"), Some("'policy'"), Some("'delete_oldest'"), 
		Some("'archive'"), Some("'compact'"), Some("'serialization'"), Some("'format'"), 
		Some("'json'"), Some("'avro'"), Some("'confluent_avro'"), Some("'protobuf'"), 
		Some("'subject'"), Some("'registry'"), Some("'fields'"), Some("'object'"), 
		Some("'list'"), Some("'computed'"), Some("'='"), Some("'and'"), Some("'or'"), 
		Some("'not'"), Some("'when'"), Some("'then'"), Some("'else'"), Some("'initial_state'"), 
		Some("'for_entity'"), Some("'states'"), Some("'initial'"), Some("'terminal'"), 
		Some("'final'"), Some("'transitions'"), Some("'from'"), Some("'on_transition'"), 
		Some("'to'"), Some("'parameters'"), Some("'default'"), Some("'can_schedule'"), 
		Some("'change_frequency'"), Some("'entries'"), Some("'deprecated_reason'"), 
		Some("'rule'"), Some("'given'"), Some("'calculate'"), Some("'return'"), 
		Some("'migration'"), Some("'types'"), Some("'string'"), Some("'char'"), 
		Some("'text'"), Some("'integer'"), Some("'decimal'"), Some("'float'"), 
		Some("'boolean'"), Some("'date'"), Some("'timestamp'"), Some("'uuid'"), 
		Some("'bytes'"), Some("'bizdate'"), Some("'set'"), Some("'map'"), Some("'required'"), 
		Some("'optional'"), Some("'unique'"), Some("'cannot_change'"), Some("'encrypted'"), 
		Some("'removal'"), Some("'otherwise'"), Some("'seconds'"), Some("'second'"), 
		Some("'minutes'"), Some("'minute'"), Some("'hours'"), Some("'hour'"), 
		Some("'days'"), Some("'day'"), Some("'weeks'"), Some("'week'"), Some("'months'"), 
		Some("'month'"), Some("'years'"), Some("'year'"), Some("'milliseconds'"), 
		Some("'millisecond'"), Some("'KB'"), Some("'MB'"), Some("'GB'"), Some("'TB'"), 
		Some("'null'"), Some("'import'"), Some("'pii'"), None, None, None, None, 
		None, None, None, None, None, Some("':'"), Some("','"), Some("'..'"), 
		Some("'.'"), Some("'['"), Some("']'"), Some("'('"), Some("')'"), Some("'<'"), 
		Some("'>'"), None, Some("'!='"), Some("'<='"), Some("'>='"), Some("'+'"), 
		Some("'-'"), Some("'*'"), Some("'/'"), Some("'->'"), Some("'{'"), Some("'}'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;202]  = [
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
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		Some("IMPORT"), Some("PII"), Some("VERSION_NUMBER"), Some("INTEGER"), 
		Some("DECIMAL"), Some("DURATION_LITERAL"), Some("BOOLEAN"), Some("IDENTIFIER"), 
		Some("UPPER_IDENTIFIER"), Some("STRING"), Some("MULTILINE_STRING"), Some("COLON"), 
		Some("COMMA"), Some("DOTDOT"), Some("DOT"), Some("LBRACKET"), Some("RBRACKET"), 
		Some("LPAREN"), Some("RPAREN"), Some("LANGLE"), Some("RANGLE"), Some("EQ"), 
		Some("NE"), Some("LE"), Some("GE"), Some("PLUS"), Some("MINUS"), Some("STAR"), 
		Some("SLASH"), Some("ARROW"), Some("LBRACE"), Some("RBRACE"), Some("COMMENT"), 
		Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,SchemaDSLParserExt<'input>, I, SchemaDSLParserContextType , dyn SchemaDSLListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type SchemaDSLTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, SchemaDSLParserContextType , dyn SchemaDSLListener<'input> + 'a>;

/// Parser for SchemaDSL grammar
pub struct SchemaDSLParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
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
				SchemaDSLParserExt{
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

impl<'input, I> SchemaDSLParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> SchemaDSLParser<'input, I, DefaultErrorStrategy<'input,SchemaDSLParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for SchemaDSLParser
pub trait SchemaDSLParserContext<'input>:
	for<'x> Listenable<dyn SchemaDSLListener<'input> + 'x > + 
	for<'x> Visitable<dyn SchemaDSLVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=SchemaDSLParserContextType>
{}

antlr_rust::coerce_from!{ 'input : SchemaDSLParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn SchemaDSLParserContext<'input> + 'input
where
    T: SchemaDSLVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn SchemaDSLVisitor<'input> + 'x))
    }
}

impl<'input> SchemaDSLParserContext<'input> for TerminalNode<'input,SchemaDSLParserContextType> {}
impl<'input> SchemaDSLParserContext<'input> for ErrorNode<'input,SchemaDSLParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn SchemaDSLParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn SchemaDSLListener<'input> + 'input }

pub struct SchemaDSLParserContextType;
antlr_rust::tid!{SchemaDSLParserContextType}

impl<'input> ParserNodeType<'input> for SchemaDSLParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn SchemaDSLParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct SchemaDSLParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> SchemaDSLParserExt<'input>{
}
antlr_rust::tid! { SchemaDSLParserExt<'a> }

impl<'input> TokenAware<'input> for SchemaDSLParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for SchemaDSLParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for SchemaDSLParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "SchemaDSL.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn SchemaDSLParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					60 => SchemaDSLParser::<'input,I,_>::computedExpression_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					104 => SchemaDSLParser::<'input,I,_>::expression_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> SchemaDSLParser<'input, I, DefaultErrorStrategy<'input,SchemaDSLParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn computedExpression_sempred(_localctx: Option<&ComputedExpressionContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 11)
				}
				1=>{
					recog.precpred(None, 10)
				}
				2=>{
					recog.precpred(None, 9)
				}
				3=>{
					recog.precpred(None, 8)
				}
				4=>{
					recog.precpred(None, 7)
				}
			_ => true
		}
	}
	fn expression_sempred(_localctx: Option<&ExpressionContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				5=>{
					recog.precpred(None, 3)
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

impl<'input> SchemaDSLParserContext<'input> for ProgramContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for ProgramContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_program(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_program(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for ProgramContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_program(self);
	}
}

impl<'input> CustomRuleContext<'input> for ProgramContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_program }
	//fn type_rule_index() -> usize where Self: Sized { RULE_program }
}
antlr_rust::tid!{ProgramContextExt<'a>}

impl<'input> ProgramContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProgramContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProgramContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ProgramContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<ProgramContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn importStatement_all(&self) ->  Vec<Rc<ImportStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn importStatement(&self, i: usize) -> Option<Rc<ImportStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn schemaDefinition_all(&self) ->  Vec<Rc<SchemaDefinitionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn schemaDefinition(&self, i: usize) -> Option<Rc<SchemaDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn typeAliasBlock_all(&self) ->  Vec<Rc<TypeAliasBlockContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typeAliasBlock(&self, i: usize) -> Option<Rc<TypeAliasBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ProgramContextAttrs<'input> for ProgramContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
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
			recog.base.set_state(241);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==IMPORT {
				{
				{
				/*InvokeRule importStatement*/
				recog.base.set_state(238);
				recog.importStatement()?;

				}
				}
				recog.base.set_state(243);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(246); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				recog.base.set_state(246);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.base.input.la(1) {
				 T__0 
					=> {
						{
						/*InvokeRule schemaDefinition*/
						recog.base.set_state(244);
						recog.schemaDefinition()?;

						}
					}

				 T__124 
					=> {
						{
						/*InvokeRule typeAliasBlock*/
						recog.base.set_state(245);
						recog.typeAliasBlock()?;

						}
					}

					_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				}
				recog.base.set_state(248); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==T__0 || _la==T__124) {break}
			}
			recog.base.set_state(250);
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

impl<'input> SchemaDSLParserContext<'input> for ImportStatementContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for ImportStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_importStatement(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_importStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for ImportStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_importStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importStatement }
}
antlr_rust::tid!{ImportStatementContextExt<'a>}

impl<'input> ImportStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportStatementContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<ImportStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IMPORT
/// Returns `None` if there is no child corresponding to token IMPORT
fn IMPORT(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(IMPORT, 0)
}
fn importPath(&self) -> Option<Rc<ImportPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ImportStatementContextAttrs<'input> for ImportStatementContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
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
			recog.base.set_state(252);
			recog.base.match_token(IMPORT,&mut recog.err_handler)?;

			/*InvokeRule importPath*/
			recog.base.set_state(253);
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

impl<'input> SchemaDSLParserContext<'input> for ImportPathContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for ImportPathContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_importPath(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_importPath(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for ImportPathContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_importPath(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportPathContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importPath }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importPath }
}
antlr_rust::tid!{ImportPathContextExt<'a>}

impl<'input> ImportPathContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportPathContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportPathContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportPathContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<ImportPathContextExt<'input>>{

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

impl<'input, I, H> SchemaDSLParser<'input, I, H>
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
			recog.base.set_state(256); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					{
					/*InvokeRule importPathSegment*/
					recog.base.set_state(255);
					recog.importPathSegment()?;

					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(258); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(3,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
			}
			/*InvokeRule importFileExtension*/
			recog.base.set_state(260);
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

impl<'input> SchemaDSLParserContext<'input> for ImportPathSegmentContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for ImportPathSegmentContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_importPathSegment(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_importPathSegment(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for ImportPathSegmentContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_importPathSegment(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportPathSegmentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importPathSegment }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importPathSegment }
}
antlr_rust::tid!{ImportPathSegmentContextExt<'a>}

impl<'input> ImportPathSegmentContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportPathSegmentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportPathSegmentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportPathSegmentContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<ImportPathSegmentContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DOTDOT
/// Returns `None` if there is no child corresponding to token DOTDOT
fn DOTDOT(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(DOTDOT, 0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}
/// Retrieves first TerminalNode corresponding to token SLASH
/// Returns `None` if there is no child corresponding to token SLASH
fn SLASH(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(SLASH, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token UPPER_IDENTIFIER
/// Returns `None` if there is no child corresponding to token UPPER_IDENTIFIER
fn UPPER_IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(UPPER_IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}

}

impl<'input> ImportPathSegmentContextAttrs<'input> for ImportPathSegmentContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
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
			recog.base.set_state(262);
			_la = recog.base.input.la(1);
			if { !(((((_la - 175)) & !0x3f) == 0 && ((1usize << (_la - 175)) & ((1usize << (IDENTIFIER - 175)) | (1usize << (UPPER_IDENTIFIER - 175)) | (1usize << (DOTDOT - 175)) | (1usize << (DOT - 175)) | (1usize << (MINUS - 175)) | (1usize << (SLASH - 175)))) != 0)) } {
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

impl<'input> SchemaDSLParserContext<'input> for ImportFileExtensionContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for ImportFileExtensionContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_importFileExtension(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_importFileExtension(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for ImportFileExtensionContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_importFileExtension(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportFileExtensionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importFileExtension }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importFileExtension }
}
antlr_rust::tid!{ImportFileExtensionContextExt<'a>}

impl<'input> ImportFileExtensionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportFileExtensionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportFileExtensionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportFileExtensionContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<ImportFileExtensionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}

}

impl<'input> ImportFileExtensionContextAttrs<'input> for ImportFileExtensionContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
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
			recog.base.set_state(264);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			recog.base.set_state(265);
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
//------------------- schemaDefinition ----------------
pub type SchemaDefinitionContextAll<'input> = SchemaDefinitionContext<'input>;


pub type SchemaDefinitionContext<'input> = BaseParserRuleContext<'input,SchemaDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct SchemaDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for SchemaDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for SchemaDefinitionContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_schemaDefinition(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_schemaDefinition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for SchemaDefinitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_schemaDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for SchemaDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_schemaDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_schemaDefinition }
}
antlr_rust::tid!{SchemaDefinitionContextExt<'a>}

impl<'input> SchemaDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SchemaDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SchemaDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SchemaDefinitionContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<SchemaDefinitionContextExt<'input>>{

fn schemaName(&self) -> Option<Rc<SchemaNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn patternDecl(&self) -> Option<Rc<PatternDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn targetsDecl(&self) -> Option<Rc<TargetsDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn versionBlock(&self) -> Option<Rc<VersionBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn compatibilityDecl(&self) -> Option<Rc<CompatibilityDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn retentionDecl(&self) -> Option<Rc<RetentionDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identityBlock(&self) -> Option<Rc<IdentityBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn streamingBlock(&self) -> Option<Rc<StreamingBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn serializationBlock(&self) -> Option<Rc<SerializationBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldsBlock(&self) -> Option<Rc<FieldsBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn nestedObjectBlock_all(&self) ->  Vec<Rc<NestedObjectBlockContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn nestedObjectBlock(&self, i: usize) -> Option<Rc<NestedObjectBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn computedBlock(&self) -> Option<Rc<ComputedBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn constraintsBlock(&self) -> Option<Rc<ConstraintsBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn immutableDecl(&self) -> Option<Rc<ImmutableDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stateMachineBlock(&self) -> Option<Rc<StateMachineBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn parametersBlock(&self) -> Option<Rc<ParametersBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn entriesBlock(&self) -> Option<Rc<EntriesBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn ruleBlock_all(&self) ->  Vec<Rc<RuleBlockContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn ruleBlock(&self, i: usize) -> Option<Rc<RuleBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn migrationBlock(&self) -> Option<Rc<MigrationBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SchemaDefinitionContextAttrs<'input> for SchemaDefinitionContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn schemaDefinition(&mut self,)
	-> Result<Rc<SchemaDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SchemaDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_schemaDefinition);
        let mut _localctx: Rc<SchemaDefinitionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(267);
			recog.base.match_token(T__0,&mut recog.err_handler)?;

			/*InvokeRule schemaName*/
			recog.base.set_state(268);
			recog.schemaName()?;

			recog.base.set_state(270);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__5 {
				{
				/*InvokeRule patternDecl*/
				recog.base.set_state(269);
				recog.patternDecl()?;

				}
			}

			recog.base.set_state(273);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__20 {
				{
				/*InvokeRule targetsDecl*/
				recog.base.set_state(272);
				recog.targetsDecl()?;

				}
			}

			recog.base.set_state(276);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__26 {
				{
				/*InvokeRule versionBlock*/
				recog.base.set_state(275);
				recog.versionBlock()?;

				}
			}

			recog.base.set_state(279);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__27 || _la==T__28 {
				{
				/*InvokeRule compatibilityDecl*/
				recog.base.set_state(278);
				recog.compatibilityDecl()?;

				}
			}

			recog.base.set_state(282);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__40 {
				{
				/*InvokeRule retentionDecl*/
				recog.base.set_state(281);
				recog.retentionDecl()?;

				}
			}

			recog.base.set_state(285);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__47 {
				{
				/*InvokeRule identityBlock*/
				recog.base.set_state(284);
				recog.identityBlock()?;

				}
			}

			recog.base.set_state(288);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__48 {
				{
				/*InvokeRule streamingBlock*/
				recog.base.set_state(287);
				recog.streamingBlock()?;

				}
			}

			recog.base.set_state(291);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__84 {
				{
				/*InvokeRule serializationBlock*/
				recog.base.set_state(290);
				recog.serializationBlock()?;

				}
			}

			recog.base.set_state(294);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__92 {
				{
				/*InvokeRule fieldsBlock*/
				recog.base.set_state(293);
				recog.fieldsBlock()?;

				}
			}

			recog.base.set_state(299);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 53)) & !0x3f) == 0 && ((1usize << (_la - 53)) & ((1usize << (T__52 - 53)) | (1usize << (T__53 - 53)) | (1usize << (T__54 - 53)))) != 0) || ((((_la - 107)) & !0x3f) == 0 && ((1usize << (_la - 107)) & ((1usize << (T__106 - 107)) | (1usize << (T__107 - 107)) | (1usize << (T__108 - 107)))) != 0) || _la==IDENTIFIER {
				{
				{
				/*InvokeRule nestedObjectBlock*/
				recog.base.set_state(296);
				recog.nestedObjectBlock()?;

				}
				}
				recog.base.set_state(301);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(303);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__95 {
				{
				/*InvokeRule computedBlock*/
				recog.base.set_state(302);
				recog.computedBlock()?;

				}
			}

			recog.base.set_state(306);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__42 {
				{
				/*InvokeRule constraintsBlock*/
				recog.base.set_state(305);
				recog.constraintsBlock()?;

				}
			}

			recog.base.set_state(309);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__41 {
				{
				/*InvokeRule immutableDecl*/
				recog.base.set_state(308);
				recog.immutableDecl()?;

				}
			}

			recog.base.set_state(312);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__104 || _la==T__105 {
				{
				/*InvokeRule stateMachineBlock*/
				recog.base.set_state(311);
				recog.stateMachineBlock()?;

				}
			}

			recog.base.set_state(315);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__113 {
				{
				/*InvokeRule parametersBlock*/
				recog.base.set_state(314);
				recog.parametersBlock()?;

				}
			}

			recog.base.set_state(318);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__117 {
				{
				/*InvokeRule entriesBlock*/
				recog.base.set_state(317);
				recog.entriesBlock()?;

				}
			}

			recog.base.set_state(323);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__119 {
				{
				{
				/*InvokeRule ruleBlock*/
				recog.base.set_state(320);
				recog.ruleBlock()?;

				}
				}
				recog.base.set_state(325);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(327);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__123 {
				{
				/*InvokeRule migrationBlock*/
				recog.base.set_state(326);
				recog.migrationBlock()?;

				}
			}

			recog.base.set_state(329);
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
//------------------- schemaName ----------------
pub type SchemaNameContextAll<'input> = SchemaNameContext<'input>;


pub type SchemaNameContext<'input> = BaseParserRuleContext<'input,SchemaNameContextExt<'input>>;

#[derive(Clone)]
pub struct SchemaNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for SchemaNameContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for SchemaNameContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_schemaName(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_schemaName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for SchemaNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_schemaName(self);
	}
}

impl<'input> CustomRuleContext<'input> for SchemaNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_schemaName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_schemaName }
}
antlr_rust::tid!{SchemaNameContextExt<'a>}

impl<'input> SchemaNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SchemaNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SchemaNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SchemaNameContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<SchemaNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn mutationPattern(&self) -> Option<Rc<MutationPatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn timeSemanticsType(&self) -> Option<Rc<TimeSemanticsTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SchemaNameContextAttrs<'input> for SchemaNameContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn schemaName(&mut self,)
	-> Result<Rc<SchemaNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SchemaNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_schemaName);
        let mut _localctx: Rc<SchemaNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(334);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(331);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

			 T__6 | T__7 | T__8 | T__9 | T__10 | T__11 | T__12 | T__13 | T__14 | T__15 |
			 T__16 | T__17 | T__18 | T__19 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule mutationPattern*/
					recog.base.set_state(332);
					recog.mutationPattern()?;

					}
				}

			 T__52 | T__53 | T__54 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule timeSemanticsType*/
					recog.base.set_state(333);
					recog.timeSemanticsType()?;

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
//------------------- patternDecl ----------------
pub type PatternDeclContextAll<'input> = PatternDeclContext<'input>;


pub type PatternDeclContext<'input> = BaseParserRuleContext<'input,PatternDeclContextExt<'input>>;

#[derive(Clone)]
pub struct PatternDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for PatternDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for PatternDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_patternDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_patternDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for PatternDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_patternDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for PatternDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternDecl }
}
antlr_rust::tid!{PatternDeclContextExt<'a>}

impl<'input> PatternDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PatternDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PatternDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PatternDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<PatternDeclContextExt<'input>>{

fn mutationPattern_all(&self) ->  Vec<Rc<MutationPatternContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn mutationPattern(&self, i: usize) -> Option<Rc<MutationPatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,SchemaDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> PatternDeclContextAttrs<'input> for PatternDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn patternDecl(&mut self,)
	-> Result<Rc<PatternDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_patternDecl);
        let mut _localctx: Rc<PatternDeclContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(336);
			recog.base.match_token(T__5,&mut recog.err_handler)?;

			/*InvokeRule mutationPattern*/
			recog.base.set_state(337);
			recog.mutationPattern()?;

			recog.base.set_state(342);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(338);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule mutationPattern*/
				recog.base.set_state(339);
				recog.mutationPattern()?;

				}
				}
				recog.base.set_state(344);
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
//------------------- mutationPattern ----------------
pub type MutationPatternContextAll<'input> = MutationPatternContext<'input>;


pub type MutationPatternContext<'input> = BaseParserRuleContext<'input,MutationPatternContextExt<'input>>;

#[derive(Clone)]
pub struct MutationPatternContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for MutationPatternContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for MutationPatternContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mutationPattern(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_mutationPattern(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for MutationPatternContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_mutationPattern(self);
	}
}

impl<'input> CustomRuleContext<'input> for MutationPatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mutationPattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mutationPattern }
}
antlr_rust::tid!{MutationPatternContextExt<'a>}

impl<'input> MutationPatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MutationPatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MutationPatternContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MutationPatternContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<MutationPatternContextExt<'input>>{


}

impl<'input> MutationPatternContextAttrs<'input> for MutationPatternContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mutationPattern(&mut self,)
	-> Result<Rc<MutationPatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MutationPatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_mutationPattern);
        let mut _localctx: Rc<MutationPatternContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(345);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9) | (1usize << T__10) | (1usize << T__11) | (1usize << T__12) | (1usize << T__13) | (1usize << T__14) | (1usize << T__15) | (1usize << T__16) | (1usize << T__17) | (1usize << T__18) | (1usize << T__19))) != 0)) } {
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
//------------------- targetsDecl ----------------
pub type TargetsDeclContextAll<'input> = TargetsDeclContext<'input>;


pub type TargetsDeclContext<'input> = BaseParserRuleContext<'input,TargetsDeclContextExt<'input>>;

#[derive(Clone)]
pub struct TargetsDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for TargetsDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for TargetsDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_targetsDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_targetsDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for TargetsDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_targetsDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for TargetsDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_targetsDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_targetsDecl }
}
antlr_rust::tid!{TargetsDeclContextExt<'a>}

impl<'input> TargetsDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TargetsDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TargetsDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TargetsDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<TargetsDeclContextExt<'input>>{

fn targetList(&self) -> Option<Rc<TargetListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TargetsDeclContextAttrs<'input> for TargetsDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn targetsDecl(&mut self,)
	-> Result<Rc<TargetsDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TargetsDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_targetsDecl);
        let mut _localctx: Rc<TargetsDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(347);
			recog.base.match_token(T__20,&mut recog.err_handler)?;

			/*InvokeRule targetList*/
			recog.base.set_state(348);
			recog.targetList()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- targetList ----------------
pub type TargetListContextAll<'input> = TargetListContext<'input>;


pub type TargetListContext<'input> = BaseParserRuleContext<'input,TargetListContextExt<'input>>;

#[derive(Clone)]
pub struct TargetListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for TargetListContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for TargetListContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_targetList(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_targetList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for TargetListContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_targetList(self);
	}
}

impl<'input> CustomRuleContext<'input> for TargetListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_targetList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_targetList }
}
antlr_rust::tid!{TargetListContextExt<'a>}

impl<'input> TargetListContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TargetListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TargetListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TargetListContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<TargetListContextExt<'input>>{

fn target_all(&self) ->  Vec<Rc<TargetContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn target(&self, i: usize) -> Option<Rc<TargetContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,SchemaDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> TargetListContextAttrs<'input> for TargetListContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn targetList(&mut self,)
	-> Result<Rc<TargetListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TargetListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_targetList);
        let mut _localctx: Rc<TargetListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule target*/
			recog.base.set_state(350);
			recog.target()?;

			recog.base.set_state(355);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(351);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule target*/
				recog.base.set_state(352);
				recog.target()?;

				}
				}
				recog.base.set_state(357);
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
//------------------- target ----------------
pub type TargetContextAll<'input> = TargetContext<'input>;


pub type TargetContext<'input> = BaseParserRuleContext<'input,TargetContextExt<'input>>;

#[derive(Clone)]
pub struct TargetContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for TargetContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for TargetContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_target(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_target(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for TargetContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_target(self);
	}
}

impl<'input> CustomRuleContext<'input> for TargetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_target }
	//fn type_rule_index() -> usize where Self: Sized { RULE_target }
}
antlr_rust::tid!{TargetContextExt<'a>}

impl<'input> TargetContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TargetContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TargetContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TargetContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<TargetContextExt<'input>>{


}

impl<'input> TargetContextAttrs<'input> for TargetContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn target(&mut self,)
	-> Result<Rc<TargetContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TargetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_target);
        let mut _localctx: Rc<TargetContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(358);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__21) | (1usize << T__22) | (1usize << T__23) | (1usize << T__24) | (1usize << T__25))) != 0)) } {
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
//------------------- versionBlock ----------------
pub type VersionBlockContextAll<'input> = VersionBlockContext<'input>;


pub type VersionBlockContext<'input> = BaseParserRuleContext<'input,VersionBlockContextExt<'input>>;

#[derive(Clone)]
pub struct VersionBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for VersionBlockContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for VersionBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_versionBlock(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_versionBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for VersionBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_versionBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for VersionBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_versionBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_versionBlock }
}
antlr_rust::tid!{VersionBlockContextExt<'a>}

impl<'input> VersionBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VersionBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VersionBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VersionBlockContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<VersionBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token VERSION_NUMBER
/// Returns `None` if there is no child corresponding to token VERSION_NUMBER
fn VERSION_NUMBER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(VERSION_NUMBER, 0)
}
fn compatibilityDecl(&self) -> Option<Rc<CompatibilityDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn previousVersionDecl(&self) -> Option<Rc<PreviousVersionDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn deprecationDecl(&self) -> Option<Rc<DeprecationDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn migrationGuideDecl(&self) -> Option<Rc<MigrationGuideDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> VersionBlockContextAttrs<'input> for VersionBlockContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn versionBlock(&mut self,)
	-> Result<Rc<VersionBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VersionBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_versionBlock);
        let mut _localctx: Rc<VersionBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(360);
			recog.base.match_token(T__26,&mut recog.err_handler)?;

			recog.base.set_state(361);
			recog.base.match_token(VERSION_NUMBER,&mut recog.err_handler)?;

			recog.base.set_state(363);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(25,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule compatibilityDecl*/
					recog.base.set_state(362);
					recog.compatibilityDecl()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(366);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__35 {
				{
				/*InvokeRule previousVersionDecl*/
				recog.base.set_state(365);
				recog.previousVersionDecl()?;

				}
			}

			recog.base.set_state(369);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__36 {
				{
				/*InvokeRule deprecationDecl*/
				recog.base.set_state(368);
				recog.deprecationDecl()?;

				}
			}

			recog.base.set_state(372);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__39 {
				{
				/*InvokeRule migrationGuideDecl*/
				recog.base.set_state(371);
				recog.migrationGuideDecl()?;

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
//------------------- compatibilityDecl ----------------
pub type CompatibilityDeclContextAll<'input> = CompatibilityDeclContext<'input>;


pub type CompatibilityDeclContext<'input> = BaseParserRuleContext<'input,CompatibilityDeclContextExt<'input>>;

#[derive(Clone)]
pub struct CompatibilityDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for CompatibilityDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for CompatibilityDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_compatibilityDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_compatibilityDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for CompatibilityDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_compatibilityDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for CompatibilityDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compatibilityDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compatibilityDecl }
}
antlr_rust::tid!{CompatibilityDeclContextExt<'a>}

impl<'input> CompatibilityDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CompatibilityDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompatibilityDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CompatibilityDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<CompatibilityDeclContextExt<'input>>{

fn compatibilityMode(&self) -> Option<Rc<CompatibilityModeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CompatibilityDeclContextAttrs<'input> for CompatibilityDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compatibilityDecl(&mut self,)
	-> Result<Rc<CompatibilityDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CompatibilityDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_compatibilityDecl);
        let mut _localctx: Rc<CompatibilityDeclContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(374);
			_la = recog.base.input.la(1);
			if { !(_la==T__27 || _la==T__28) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			/*InvokeRule compatibilityMode*/
			recog.base.set_state(375);
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

impl<'input> SchemaDSLParserContext<'input> for CompatibilityModeContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for CompatibilityModeContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_compatibilityMode(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_compatibilityMode(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for CompatibilityModeContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_compatibilityMode(self);
	}
}

impl<'input> CustomRuleContext<'input> for CompatibilityModeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compatibilityMode }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compatibilityMode }
}
antlr_rust::tid!{CompatibilityModeContextExt<'a>}

impl<'input> CompatibilityModeContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CompatibilityModeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompatibilityModeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CompatibilityModeContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<CompatibilityModeContextExt<'input>>{


}

impl<'input> CompatibilityModeContextAttrs<'input> for CompatibilityModeContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compatibilityMode(&mut self,)
	-> Result<Rc<CompatibilityModeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CompatibilityModeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_compatibilityMode);
        let mut _localctx: Rc<CompatibilityModeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(377);
			_la = recog.base.input.la(1);
			if { !(((((_la - 30)) & !0x3f) == 0 && ((1usize << (_la - 30)) & ((1usize << (T__29 - 30)) | (1usize << (T__30 - 30)) | (1usize << (T__31 - 30)) | (1usize << (T__32 - 30)) | (1usize << (T__33 - 30)) | (1usize << (T__34 - 30)))) != 0)) } {
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
//------------------- previousVersionDecl ----------------
pub type PreviousVersionDeclContextAll<'input> = PreviousVersionDeclContext<'input>;


pub type PreviousVersionDeclContext<'input> = BaseParserRuleContext<'input,PreviousVersionDeclContextExt<'input>>;

#[derive(Clone)]
pub struct PreviousVersionDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for PreviousVersionDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for PreviousVersionDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_previousVersionDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_previousVersionDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for PreviousVersionDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_previousVersionDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for PreviousVersionDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_previousVersionDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_previousVersionDecl }
}
antlr_rust::tid!{PreviousVersionDeclContextExt<'a>}

impl<'input> PreviousVersionDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PreviousVersionDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PreviousVersionDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PreviousVersionDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<PreviousVersionDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token VERSION_NUMBER
/// Returns `None` if there is no child corresponding to token VERSION_NUMBER
fn VERSION_NUMBER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(VERSION_NUMBER, 0)
}

}

impl<'input> PreviousVersionDeclContextAttrs<'input> for PreviousVersionDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn previousVersionDecl(&mut self,)
	-> Result<Rc<PreviousVersionDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PreviousVersionDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_previousVersionDecl);
        let mut _localctx: Rc<PreviousVersionDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(379);
			recog.base.match_token(T__35,&mut recog.err_handler)?;

			recog.base.set_state(380);
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
//------------------- deprecationDecl ----------------
pub type DeprecationDeclContextAll<'input> = DeprecationDeclContext<'input>;


pub type DeprecationDeclContext<'input> = BaseParserRuleContext<'input,DeprecationDeclContextExt<'input>>;

#[derive(Clone)]
pub struct DeprecationDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for DeprecationDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for DeprecationDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_deprecationDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_deprecationDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for DeprecationDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_deprecationDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for DeprecationDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_deprecationDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_deprecationDecl }
}
antlr_rust::tid!{DeprecationDeclContextExt<'a>}

impl<'input> DeprecationDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DeprecationDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeprecationDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DeprecationDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<DeprecationDeclContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token COLON in current rule
fn COLON_all(&self) -> Vec<Rc<TerminalNode<'input,SchemaDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COLON, starting from 0.
/// Returns `None` if number of children corresponding to token COLON is less or equal than `i`.
fn COLON(&self, i: usize) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, i)
}
/// Retrieves all `TerminalNode`s corresponding to token STRING in current rule
fn STRING_all(&self) -> Vec<Rc<TerminalNode<'input,SchemaDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token STRING, starting from 0.
/// Returns `None` if number of children corresponding to token STRING is less or equal than `i`.
fn STRING(&self, i: usize) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, i)
}
/// Retrieves first TerminalNode corresponding to token VERSION_NUMBER
/// Returns `None` if there is no child corresponding to token VERSION_NUMBER
fn VERSION_NUMBER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(VERSION_NUMBER, 0)
}

}

impl<'input> DeprecationDeclContextAttrs<'input> for DeprecationDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn deprecationDecl(&mut self,)
	-> Result<Rc<DeprecationDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeprecationDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_deprecationDecl);
        let mut _localctx: Rc<DeprecationDeclContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(382);
			recog.base.match_token(T__36,&mut recog.err_handler)?;

			recog.base.set_state(383);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(384);
			recog.base.match_token(STRING,&mut recog.err_handler)?;

			recog.base.set_state(388);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__37 {
				{
				recog.base.set_state(385);
				recog.base.match_token(T__37,&mut recog.err_handler)?;

				recog.base.set_state(386);
				recog.base.match_token(COLON,&mut recog.err_handler)?;

				recog.base.set_state(387);
				recog.base.match_token(STRING,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(393);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__38 {
				{
				recog.base.set_state(390);
				recog.base.match_token(T__38,&mut recog.err_handler)?;

				recog.base.set_state(391);
				recog.base.match_token(COLON,&mut recog.err_handler)?;

				recog.base.set_state(392);
				recog.base.match_token(VERSION_NUMBER,&mut recog.err_handler)?;

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
//------------------- migrationGuideDecl ----------------
pub type MigrationGuideDeclContextAll<'input> = MigrationGuideDeclContext<'input>;


pub type MigrationGuideDeclContext<'input> = BaseParserRuleContext<'input,MigrationGuideDeclContextExt<'input>>;

#[derive(Clone)]
pub struct MigrationGuideDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for MigrationGuideDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for MigrationGuideDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_migrationGuideDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_migrationGuideDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for MigrationGuideDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_migrationGuideDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for MigrationGuideDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_migrationGuideDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_migrationGuideDecl }
}
antlr_rust::tid!{MigrationGuideDeclContextExt<'a>}

impl<'input> MigrationGuideDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MigrationGuideDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MigrationGuideDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MigrationGuideDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<MigrationGuideDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token MULTILINE_STRING
/// Returns `None` if there is no child corresponding to token MULTILINE_STRING
fn MULTILINE_STRING(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(MULTILINE_STRING, 0)
}

}

impl<'input> MigrationGuideDeclContextAttrs<'input> for MigrationGuideDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn migrationGuideDecl(&mut self,)
	-> Result<Rc<MigrationGuideDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MigrationGuideDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_migrationGuideDecl);
        let mut _localctx: Rc<MigrationGuideDeclContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(395);
			recog.base.match_token(T__39,&mut recog.err_handler)?;

			recog.base.set_state(396);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(397);
			_la = recog.base.input.la(1);
			if { !(_la==STRING || _la==MULTILINE_STRING) } {
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
//------------------- retentionDecl ----------------
pub type RetentionDeclContextAll<'input> = RetentionDeclContext<'input>;


pub type RetentionDeclContext<'input> = BaseParserRuleContext<'input,RetentionDeclContextExt<'input>>;

#[derive(Clone)]
pub struct RetentionDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for RetentionDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for RetentionDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_retentionDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_retentionDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for RetentionDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_retentionDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for RetentionDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_retentionDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_retentionDecl }
}
antlr_rust::tid!{RetentionDeclContextExt<'a>}

impl<'input> RetentionDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RetentionDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RetentionDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RetentionDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<RetentionDeclContextExt<'input>>{

fn duration(&self) -> Option<Rc<DurationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RetentionDeclContextAttrs<'input> for RetentionDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn retentionDecl(&mut self,)
	-> Result<Rc<RetentionDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RetentionDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_retentionDecl);
        let mut _localctx: Rc<RetentionDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(399);
			recog.base.match_token(T__40,&mut recog.err_handler)?;

			/*InvokeRule duration*/
			recog.base.set_state(400);
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
//------------------- immutableDecl ----------------
pub type ImmutableDeclContextAll<'input> = ImmutableDeclContext<'input>;


pub type ImmutableDeclContext<'input> = BaseParserRuleContext<'input,ImmutableDeclContextExt<'input>>;

#[derive(Clone)]
pub struct ImmutableDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for ImmutableDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for ImmutableDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_immutableDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_immutableDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for ImmutableDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_immutableDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImmutableDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_immutableDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_immutableDecl }
}
antlr_rust::tid!{ImmutableDeclContextExt<'a>}

impl<'input> ImmutableDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImmutableDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImmutableDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImmutableDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<ImmutableDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BOOLEAN
/// Returns `None` if there is no child corresponding to token BOOLEAN
fn BOOLEAN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(BOOLEAN, 0)
}

}

impl<'input> ImmutableDeclContextAttrs<'input> for ImmutableDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn immutableDecl(&mut self,)
	-> Result<Rc<ImmutableDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ImmutableDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_immutableDecl);
        let mut _localctx: Rc<ImmutableDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(402);
			recog.base.match_token(T__41,&mut recog.err_handler)?;

			recog.base.set_state(403);
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
//------------------- constraintsBlock ----------------
pub type ConstraintsBlockContextAll<'input> = ConstraintsBlockContext<'input>;


pub type ConstraintsBlockContext<'input> = BaseParserRuleContext<'input,ConstraintsBlockContextExt<'input>>;

#[derive(Clone)]
pub struct ConstraintsBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for ConstraintsBlockContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for ConstraintsBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_constraintsBlock(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_constraintsBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for ConstraintsBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_constraintsBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConstraintsBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constraintsBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constraintsBlock }
}
antlr_rust::tid!{ConstraintsBlockContextExt<'a>}

impl<'input> ConstraintsBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConstraintsBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConstraintsBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConstraintsBlockContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<ConstraintsBlockContextExt<'input>>{

fn constraintDecl_all(&self) ->  Vec<Rc<ConstraintDeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn constraintDecl(&self, i: usize) -> Option<Rc<ConstraintDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ConstraintsBlockContextAttrs<'input> for ConstraintsBlockContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn constraintsBlock(&mut self,)
	-> Result<Rc<ConstraintsBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConstraintsBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_constraintsBlock);
        let mut _localctx: Rc<ConstraintsBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(405);
			recog.base.match_token(T__42,&mut recog.err_handler)?;

			recog.base.set_state(407); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule constraintDecl*/
				recog.base.set_state(406);
				recog.constraintDecl()?;

				}
				}
				recog.base.set_state(409); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==T__100 || ((((_la - 147)) & !0x3f) == 0 && ((1usize << (_la - 147)) & ((1usize << (T__146 - 147)) | (1usize << (T__147 - 147)) | (1usize << (T__148 - 147)) | (1usize << (T__149 - 147)) | (1usize << (T__150 - 147)) | (1usize << (T__151 - 147)) | (1usize << (T__152 - 147)) | (1usize << (T__153 - 147)) | (1usize << (T__154 - 147)) | (1usize << (T__155 - 147)) | (1usize << (T__156 - 147)) | (1usize << (T__157 - 147)) | (1usize << (T__158 - 147)) | (1usize << (T__159 - 147)) | (1usize << (T__160 - 147)) | (1usize << (T__161 - 147)) | (1usize << (T__166 - 147)) | (1usize << (INTEGER - 147)) | (1usize << (DECIMAL - 147)) | (1usize << (BOOLEAN - 147)) | (1usize << (IDENTIFIER - 147)) | (1usize << (STRING - 147)))) != 0) || _la==LPAREN || _la==MINUS) {break}
			}
			recog.base.set_state(411);
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
//------------------- constraintDecl ----------------
pub type ConstraintDeclContextAll<'input> = ConstraintDeclContext<'input>;


pub type ConstraintDeclContext<'input> = BaseParserRuleContext<'input,ConstraintDeclContextExt<'input>>;

#[derive(Clone)]
pub struct ConstraintDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for ConstraintDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for ConstraintDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_constraintDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_constraintDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for ConstraintDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_constraintDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConstraintDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constraintDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constraintDecl }
}
antlr_rust::tid!{ConstraintDeclContextExt<'a>}

impl<'input> ConstraintDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConstraintDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConstraintDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConstraintDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<ConstraintDeclContextExt<'input>>{

fn fieldPath(&self) -> Option<Rc<FieldPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumConstraint(&self) -> Option<Rc<EnumConstraintContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn rangeConstraint(&self) -> Option<Rc<RangeConstraintContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn patternConstraint(&self) -> Option<Rc<PatternConstraintContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lengthConstraint(&self) -> Option<Rc<LengthConstraintContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn condition(&self) -> Option<Rc<ConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}

}

impl<'input> ConstraintDeclContextAttrs<'input> for ConstraintDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn constraintDecl(&mut self,)
	-> Result<Rc<ConstraintDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConstraintDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_constraintDecl);
        let mut _localctx: Rc<ConstraintDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(429);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(32,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fieldPath*/
					recog.base.set_state(413);
					recog.fieldPath()?;

					/*InvokeRule enumConstraint*/
					recog.base.set_state(414);
					recog.enumConstraint()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule fieldPath*/
					recog.base.set_state(416);
					recog.fieldPath()?;

					/*InvokeRule rangeConstraint*/
					recog.base.set_state(417);
					recog.rangeConstraint()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule fieldPath*/
					recog.base.set_state(419);
					recog.fieldPath()?;

					/*InvokeRule patternConstraint*/
					recog.base.set_state(420);
					recog.patternConstraint()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule fieldPath*/
					recog.base.set_state(422);
					recog.fieldPath()?;

					/*InvokeRule lengthConstraint*/
					recog.base.set_state(423);
					recog.lengthConstraint()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule condition*/
					recog.base.set_state(425);
					recog.condition()?;

					recog.base.set_state(426);
					recog.base.match_token(T__43,&mut recog.err_handler)?;

					recog.base.set_state(427);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

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
//------------------- enumConstraint ----------------
pub type EnumConstraintContextAll<'input> = EnumConstraintContext<'input>;


pub type EnumConstraintContext<'input> = BaseParserRuleContext<'input,EnumConstraintContextExt<'input>>;

#[derive(Clone)]
pub struct EnumConstraintContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for EnumConstraintContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for EnumConstraintContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_enumConstraint(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_enumConstraint(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for EnumConstraintContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_enumConstraint(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumConstraintContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumConstraint }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumConstraint }
}
antlr_rust::tid!{EnumConstraintContextExt<'a>}

impl<'input> EnumConstraintContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EnumConstraintContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumConstraintContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EnumConstraintContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<EnumConstraintContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn enumValue_all(&self) ->  Vec<Rc<EnumValueContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn enumValue(&self, i: usize) -> Option<Rc<EnumValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,SchemaDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> EnumConstraintContextAttrs<'input> for EnumConstraintContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn enumConstraint(&mut self,)
	-> Result<Rc<EnumConstraintContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumConstraintContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_enumConstraint);
        let mut _localctx: Rc<EnumConstraintContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(431);
			recog.base.match_token(T__44,&mut recog.err_handler)?;

			recog.base.set_state(432);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule enumValue*/
			recog.base.set_state(433);
			recog.enumValue()?;

			recog.base.set_state(438);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(434);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule enumValue*/
				recog.base.set_state(435);
				recog.enumValue()?;

				}
				}
				recog.base.set_state(440);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(441);
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
//------------------- enumValue ----------------
pub type EnumValueContextAll<'input> = EnumValueContext<'input>;


pub type EnumValueContext<'input> = BaseParserRuleContext<'input,EnumValueContextExt<'input>>;

#[derive(Clone)]
pub struct EnumValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for EnumValueContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for EnumValueContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_enumValue(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_enumValue(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for EnumValueContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_enumValue(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumValue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumValue }
}
antlr_rust::tid!{EnumValueContextExt<'a>}

impl<'input> EnumValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EnumValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumValueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EnumValueContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<EnumValueContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
/// Retrieves first TerminalNode corresponding to token BOOLEAN
/// Returns `None` if there is no child corresponding to token BOOLEAN
fn BOOLEAN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(BOOLEAN, 0)
}

}

impl<'input> EnumValueContextAttrs<'input> for EnumValueContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn enumValue(&mut self,)
	-> Result<Rc<EnumValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_enumValue);
        let mut _localctx: Rc<EnumValueContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(443);
			_la = recog.base.input.la(1);
			if { !(((((_la - 171)) & !0x3f) == 0 && ((1usize << (_la - 171)) & ((1usize << (INTEGER - 171)) | (1usize << (BOOLEAN - 171)) | (1usize << (IDENTIFIER - 171)) | (1usize << (STRING - 171)))) != 0)) } {
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
//------------------- rangeConstraint ----------------
pub type RangeConstraintContextAll<'input> = RangeConstraintContext<'input>;


pub type RangeConstraintContext<'input> = BaseParserRuleContext<'input,RangeConstraintContextExt<'input>>;

#[derive(Clone)]
pub struct RangeConstraintContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for RangeConstraintContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for RangeConstraintContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_rangeConstraint(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_rangeConstraint(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for RangeConstraintContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_rangeConstraint(self);
	}
}

impl<'input> CustomRuleContext<'input> for RangeConstraintContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_rangeConstraint }
	//fn type_rule_index() -> usize where Self: Sized { RULE_rangeConstraint }
}
antlr_rust::tid!{RangeConstraintContextExt<'a>}

impl<'input> RangeConstraintContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RangeConstraintContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RangeConstraintContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RangeConstraintContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<RangeConstraintContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn numberLiteral_all(&self) ->  Vec<Rc<NumberLiteralContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn numberLiteral(&self, i: usize) -> Option<Rc<NumberLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> RangeConstraintContextAttrs<'input> for RangeConstraintContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn rangeConstraint(&mut self,)
	-> Result<Rc<RangeConstraintContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RangeConstraintContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_rangeConstraint);
        let mut _localctx: Rc<RangeConstraintContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(445);
			recog.base.match_token(T__45,&mut recog.err_handler)?;

			recog.base.set_state(446);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule numberLiteral*/
			recog.base.set_state(447);
			recog.numberLiteral()?;

			recog.base.set_state(448);
			recog.base.match_token(COMMA,&mut recog.err_handler)?;

			/*InvokeRule numberLiteral*/
			recog.base.set_state(449);
			recog.numberLiteral()?;

			recog.base.set_state(450);
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
//------------------- patternConstraint ----------------
pub type PatternConstraintContextAll<'input> = PatternConstraintContext<'input>;


pub type PatternConstraintContext<'input> = BaseParserRuleContext<'input,PatternConstraintContextExt<'input>>;

#[derive(Clone)]
pub struct PatternConstraintContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for PatternConstraintContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for PatternConstraintContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_patternConstraint(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_patternConstraint(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for PatternConstraintContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_patternConstraint(self);
	}
}

impl<'input> CustomRuleContext<'input> for PatternConstraintContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternConstraint }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternConstraint }
}
antlr_rust::tid!{PatternConstraintContextExt<'a>}

impl<'input> PatternConstraintContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PatternConstraintContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PatternConstraintContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PatternConstraintContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<PatternConstraintContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> PatternConstraintContextAttrs<'input> for PatternConstraintContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn patternConstraint(&mut self,)
	-> Result<Rc<PatternConstraintContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternConstraintContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_patternConstraint);
        let mut _localctx: Rc<PatternConstraintContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(452);
			recog.base.match_token(T__5,&mut recog.err_handler)?;

			recog.base.set_state(453);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			recog.base.set_state(454);
			recog.base.match_token(STRING,&mut recog.err_handler)?;

			recog.base.set_state(455);
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
//------------------- lengthConstraint ----------------
pub type LengthConstraintContextAll<'input> = LengthConstraintContext<'input>;


pub type LengthConstraintContext<'input> = BaseParserRuleContext<'input,LengthConstraintContextExt<'input>>;

#[derive(Clone)]
pub struct LengthConstraintContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for LengthConstraintContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for LengthConstraintContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lengthConstraint(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_lengthConstraint(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for LengthConstraintContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_lengthConstraint(self);
	}
}

impl<'input> CustomRuleContext<'input> for LengthConstraintContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lengthConstraint }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lengthConstraint }
}
antlr_rust::tid!{LengthConstraintContextExt<'a>}

impl<'input> LengthConstraintContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LengthConstraintContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LengthConstraintContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LengthConstraintContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<LengthConstraintContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token INTEGER in current rule
fn INTEGER_all(&self) -> Vec<Rc<TerminalNode<'input,SchemaDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token INTEGER, starting from 0.
/// Returns `None` if number of children corresponding to token INTEGER is less or equal than `i`.
fn INTEGER(&self, i: usize) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, i)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> LengthConstraintContextAttrs<'input> for LengthConstraintContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lengthConstraint(&mut self,)
	-> Result<Rc<LengthConstraintContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LengthConstraintContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_lengthConstraint);
        let mut _localctx: Rc<LengthConstraintContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(467);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(34,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(457);
					recog.base.match_token(T__46,&mut recog.err_handler)?;

					recog.base.set_state(458);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					recog.base.set_state(459);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					recog.base.set_state(460);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					recog.base.set_state(461);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					recog.base.set_state(462);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(463);
					recog.base.match_token(T__46,&mut recog.err_handler)?;

					recog.base.set_state(464);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					recog.base.set_state(465);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					recog.base.set_state(466);
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
//------------------- identityBlock ----------------
pub type IdentityBlockContextAll<'input> = IdentityBlockContext<'input>;


pub type IdentityBlockContext<'input> = BaseParserRuleContext<'input,IdentityBlockContextExt<'input>>;

#[derive(Clone)]
pub struct IdentityBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for IdentityBlockContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for IdentityBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_identityBlock(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_identityBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for IdentityBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_identityBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdentityBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_identityBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_identityBlock }
}
antlr_rust::tid!{IdentityBlockContextExt<'a>}

impl<'input> IdentityBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IdentityBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdentityBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IdentityBlockContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<IdentityBlockContextExt<'input>>{

fn identityFieldV2_all(&self) ->  Vec<Rc<IdentityFieldV2ContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn identityFieldV2(&self, i: usize) -> Option<Rc<IdentityFieldV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> IdentityBlockContextAttrs<'input> for IdentityBlockContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn identityBlock(&mut self,)
	-> Result<Rc<IdentityBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdentityBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_identityBlock);
        let mut _localctx: Rc<IdentityBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(469);
			recog.base.match_token(T__47,&mut recog.err_handler)?;

			recog.base.set_state(471); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule identityFieldV2*/
				recog.base.set_state(470);
				recog.identityFieldV2()?;

				}
				}
				recog.base.set_state(473); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(((((_la - 53)) & !0x3f) == 0 && ((1usize << (_la - 53)) & ((1usize << (T__52 - 53)) | (1usize << (T__53 - 53)) | (1usize << (T__54 - 53)))) != 0) || ((((_la - 107)) & !0x3f) == 0 && ((1usize << (_la - 107)) & ((1usize << (T__106 - 107)) | (1usize << (T__107 - 107)) | (1usize << (T__108 - 107)))) != 0) || _la==IDENTIFIER) {break}
			}
			recog.base.set_state(475);
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
//------------------- identityFieldV2 ----------------
pub type IdentityFieldV2ContextAll<'input> = IdentityFieldV2Context<'input>;


pub type IdentityFieldV2Context<'input> = BaseParserRuleContext<'input,IdentityFieldV2ContextExt<'input>>;

#[derive(Clone)]
pub struct IdentityFieldV2ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for IdentityFieldV2Context<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for IdentityFieldV2Context<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_identityFieldV2(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_identityFieldV2(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for IdentityFieldV2Context<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_identityFieldV2(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdentityFieldV2ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_identityFieldV2 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_identityFieldV2 }
}
antlr_rust::tid!{IdentityFieldV2ContextExt<'a>}

impl<'input> IdentityFieldV2ContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IdentityFieldV2ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdentityFieldV2ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IdentityFieldV2ContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<IdentityFieldV2ContextExt<'input>>{

fn fieldName(&self) -> Option<Rc<FieldNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldTypeV2(&self) -> Option<Rc<FieldTypeV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldQualifierV2_all(&self) ->  Vec<Rc<FieldQualifierV2ContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fieldQualifierV2(&self, i: usize) -> Option<Rc<FieldQualifierV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}

}

impl<'input> IdentityFieldV2ContextAttrs<'input> for IdentityFieldV2Context<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn identityFieldV2(&mut self,)
	-> Result<Rc<IdentityFieldV2ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdentityFieldV2ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_identityFieldV2);
        let mut _localctx: Rc<IdentityFieldV2ContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fieldName*/
			recog.base.set_state(477);
			recog.fieldName()?;

			/*InvokeRule fieldTypeV2*/
			recog.base.set_state(478);
			recog.fieldTypeV2()?;

			recog.base.set_state(482);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__36 || ((((_la - 115)) & !0x3f) == 0 && ((1usize << (_la - 115)) & ((1usize << (T__114 - 115)) | (1usize << (T__139 - 115)) | (1usize << (T__140 - 115)) | (1usize << (T__141 - 115)) | (1usize << (T__142 - 115)) | (1usize << (T__143 - 115)) | (1usize << (T__144 - 115)))) != 0) || _la==PII {
				{
				{
				/*InvokeRule fieldQualifierV2*/
				recog.base.set_state(479);
				recog.fieldQualifierV2()?;

				}
				}
				recog.base.set_state(484);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(486);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==COMMA {
				{
				recog.base.set_state(485);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

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
//------------------- streamingBlock ----------------
pub type StreamingBlockContextAll<'input> = StreamingBlockContext<'input>;


pub type StreamingBlockContext<'input> = BaseParserRuleContext<'input,StreamingBlockContextExt<'input>>;

#[derive(Clone)]
pub struct StreamingBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for StreamingBlockContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for StreamingBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_streamingBlock(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_streamingBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for StreamingBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_streamingBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for StreamingBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_streamingBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_streamingBlock }
}
antlr_rust::tid!{StreamingBlockContextExt<'a>}

impl<'input> StreamingBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StreamingBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StreamingBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StreamingBlockContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<StreamingBlockContextExt<'input>>{

fn streamingDecl_all(&self) ->  Vec<Rc<StreamingDeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn streamingDecl(&self, i: usize) -> Option<Rc<StreamingDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> StreamingBlockContextAttrs<'input> for StreamingBlockContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn streamingBlock(&mut self,)
	-> Result<Rc<StreamingBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StreamingBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_streamingBlock);
        let mut _localctx: Rc<StreamingBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(488);
			recog.base.match_token(T__48,&mut recog.err_handler)?;

			recog.base.set_state(490); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule streamingDecl*/
				recog.base.set_state(489);
				recog.streamingDecl()?;

				}
				}
				recog.base.set_state(492); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(((((_la - 41)) & !0x3f) == 0 && ((1usize << (_la - 41)) & ((1usize << (T__40 - 41)) | (1usize << (T__49 - 41)) | (1usize << (T__50 - 41)) | (1usize << (T__51 - 41)) | (1usize << (T__55 - 41)) | (1usize << (T__56 - 41)) | (1usize << (T__57 - 41)) | (1usize << (T__58 - 41)) | (1usize << (T__59 - 41)) | (1usize << (T__63 - 41)) | (1usize << (T__64 - 41)) | (1usize << (T__68 - 41)) | (1usize << (T__69 - 41)) | (1usize << (T__70 - 41)))) != 0) || _la==T__74) {break}
			}
			recog.base.set_state(494);
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
//------------------- streamingDecl ----------------
pub type StreamingDeclContextAll<'input> = StreamingDeclContext<'input>;


pub type StreamingDeclContext<'input> = BaseParserRuleContext<'input,StreamingDeclContextExt<'input>>;

#[derive(Clone)]
pub struct StreamingDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for StreamingDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for StreamingDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_streamingDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_streamingDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for StreamingDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_streamingDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for StreamingDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_streamingDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_streamingDecl }
}
antlr_rust::tid!{StreamingDeclContextExt<'a>}

impl<'input> StreamingDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StreamingDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StreamingDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StreamingDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<StreamingDeclContextExt<'input>>{

fn keyFieldsDecl(&self) -> Option<Rc<KeyFieldsDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn timeFieldDecl(&self) -> Option<Rc<TimeFieldDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn timeSemanticsDecl(&self) -> Option<Rc<TimeSemanticsDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn watermarkDecl(&self) -> Option<Rc<WatermarkDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lateDataDecl(&self) -> Option<Rc<LateDataDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn allowedLatenessDecl(&self) -> Option<Rc<AllowedLatenessDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn idleDecl(&self) -> Option<Rc<IdleDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn sparsityDecl(&self) -> Option<Rc<SparsityDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn retentionBlockDecl(&self) -> Option<Rc<RetentionBlockDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StreamingDeclContextAttrs<'input> for StreamingDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn streamingDecl(&mut self,)
	-> Result<Rc<StreamingDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StreamingDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_streamingDecl);
        let mut _localctx: Rc<StreamingDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(505);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__49 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule keyFieldsDecl*/
					recog.base.set_state(496);
					recog.keyFieldsDecl()?;

					}
				}

			 T__50 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule timeFieldDecl*/
					recog.base.set_state(497);
					recog.timeFieldDecl()?;

					}
				}

			 T__51 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule timeSemanticsDecl*/
					recog.base.set_state(498);
					recog.timeSemanticsDecl()?;

					}
				}

			 T__55 | T__56 | T__57 | T__58 | T__59 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule watermarkDecl*/
					recog.base.set_state(499);
					recog.watermarkDecl()?;

					}
				}

			 T__63 | T__64 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule lateDataDecl*/
					recog.base.set_state(500);
					recog.lateDataDecl()?;

					}
				}

			 T__68 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule allowedLatenessDecl*/
					recog.base.set_state(501);
					recog.allowedLatenessDecl()?;

					}
				}

			 T__69 | T__70 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule idleDecl*/
					recog.base.set_state(502);
					recog.idleDecl()?;

					}
				}

			 T__74 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule sparsityDecl*/
					recog.base.set_state(503);
					recog.sparsityDecl()?;

					}
				}

			 T__40 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					/*InvokeRule retentionBlockDecl*/
					recog.base.set_state(504);
					recog.retentionBlockDecl()?;

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
//------------------- keyFieldsDecl ----------------
pub type KeyFieldsDeclContextAll<'input> = KeyFieldsDeclContext<'input>;


pub type KeyFieldsDeclContext<'input> = BaseParserRuleContext<'input,KeyFieldsDeclContextExt<'input>>;

#[derive(Clone)]
pub struct KeyFieldsDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for KeyFieldsDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for KeyFieldsDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_keyFieldsDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_keyFieldsDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for KeyFieldsDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_keyFieldsDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for KeyFieldsDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_keyFieldsDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_keyFieldsDecl }
}
antlr_rust::tid!{KeyFieldsDeclContextExt<'a>}

impl<'input> KeyFieldsDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<KeyFieldsDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,KeyFieldsDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait KeyFieldsDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<KeyFieldsDeclContextExt<'input>>{

fn fieldArray(&self) -> Option<Rc<FieldArrayContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> KeyFieldsDeclContextAttrs<'input> for KeyFieldsDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn keyFieldsDecl(&mut self,)
	-> Result<Rc<KeyFieldsDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = KeyFieldsDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_keyFieldsDecl);
        let mut _localctx: Rc<KeyFieldsDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(507);
			recog.base.match_token(T__49,&mut recog.err_handler)?;

			/*InvokeRule fieldArray*/
			recog.base.set_state(508);
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
//------------------- timeFieldDecl ----------------
pub type TimeFieldDeclContextAll<'input> = TimeFieldDeclContext<'input>;


pub type TimeFieldDeclContext<'input> = BaseParserRuleContext<'input,TimeFieldDeclContextExt<'input>>;

#[derive(Clone)]
pub struct TimeFieldDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for TimeFieldDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for TimeFieldDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_timeFieldDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_timeFieldDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for TimeFieldDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_timeFieldDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for TimeFieldDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_timeFieldDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_timeFieldDecl }
}
antlr_rust::tid!{TimeFieldDeclContextExt<'a>}

impl<'input> TimeFieldDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TimeFieldDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TimeFieldDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TimeFieldDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<TimeFieldDeclContextExt<'input>>{

fn fieldPath(&self) -> Option<Rc<FieldPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn timeSemanticsType(&self) -> Option<Rc<TimeSemanticsTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TimeFieldDeclContextAttrs<'input> for TimeFieldDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn timeFieldDecl(&mut self,)
	-> Result<Rc<TimeFieldDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TimeFieldDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_timeFieldDecl);
        let mut _localctx: Rc<TimeFieldDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(510);
			recog.base.match_token(T__50,&mut recog.err_handler)?;

			recog.base.set_state(513);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENTIFIER 
				=> {
					{
					/*InvokeRule fieldPath*/
					recog.base.set_state(511);
					recog.fieldPath()?;

					}
				}

			 T__52 | T__53 | T__54 
				=> {
					{
					/*InvokeRule timeSemanticsType*/
					recog.base.set_state(512);
					recog.timeSemanticsType()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
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
//------------------- timeSemanticsDecl ----------------
pub type TimeSemanticsDeclContextAll<'input> = TimeSemanticsDeclContext<'input>;


pub type TimeSemanticsDeclContext<'input> = BaseParserRuleContext<'input,TimeSemanticsDeclContextExt<'input>>;

#[derive(Clone)]
pub struct TimeSemanticsDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for TimeSemanticsDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for TimeSemanticsDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_timeSemanticsDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_timeSemanticsDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for TimeSemanticsDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_timeSemanticsDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for TimeSemanticsDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_timeSemanticsDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_timeSemanticsDecl }
}
antlr_rust::tid!{TimeSemanticsDeclContextExt<'a>}

impl<'input> TimeSemanticsDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TimeSemanticsDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TimeSemanticsDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TimeSemanticsDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<TimeSemanticsDeclContextExt<'input>>{

fn timeSemanticsType(&self) -> Option<Rc<TimeSemanticsTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TimeSemanticsDeclContextAttrs<'input> for TimeSemanticsDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn timeSemanticsDecl(&mut self,)
	-> Result<Rc<TimeSemanticsDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TimeSemanticsDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_timeSemanticsDecl);
        let mut _localctx: Rc<TimeSemanticsDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(515);
			recog.base.match_token(T__51,&mut recog.err_handler)?;

			/*InvokeRule timeSemanticsType*/
			recog.base.set_state(516);
			recog.timeSemanticsType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- timeSemanticsType ----------------
pub type TimeSemanticsTypeContextAll<'input> = TimeSemanticsTypeContext<'input>;


pub type TimeSemanticsTypeContext<'input> = BaseParserRuleContext<'input,TimeSemanticsTypeContextExt<'input>>;

#[derive(Clone)]
pub struct TimeSemanticsTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for TimeSemanticsTypeContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for TimeSemanticsTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_timeSemanticsType(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_timeSemanticsType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for TimeSemanticsTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_timeSemanticsType(self);
	}
}

impl<'input> CustomRuleContext<'input> for TimeSemanticsTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_timeSemanticsType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_timeSemanticsType }
}
antlr_rust::tid!{TimeSemanticsTypeContextExt<'a>}

impl<'input> TimeSemanticsTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TimeSemanticsTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TimeSemanticsTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TimeSemanticsTypeContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<TimeSemanticsTypeContextExt<'input>>{


}

impl<'input> TimeSemanticsTypeContextAttrs<'input> for TimeSemanticsTypeContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn timeSemanticsType(&mut self,)
	-> Result<Rc<TimeSemanticsTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TimeSemanticsTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_timeSemanticsType);
        let mut _localctx: Rc<TimeSemanticsTypeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(518);
			_la = recog.base.input.la(1);
			if { !(((((_la - 53)) & !0x3f) == 0 && ((1usize << (_la - 53)) & ((1usize << (T__52 - 53)) | (1usize << (T__53 - 53)) | (1usize << (T__54 - 53)))) != 0)) } {
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
//------------------- watermarkDecl ----------------
pub type WatermarkDeclContextAll<'input> = WatermarkDeclContext<'input>;


pub type WatermarkDeclContext<'input> = BaseParserRuleContext<'input,WatermarkDeclContextExt<'input>>;

#[derive(Clone)]
pub struct WatermarkDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for WatermarkDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for WatermarkDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_watermarkDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_watermarkDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for WatermarkDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_watermarkDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for WatermarkDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_watermarkDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_watermarkDecl }
}
antlr_rust::tid!{WatermarkDeclContextExt<'a>}

impl<'input> WatermarkDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<WatermarkDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,WatermarkDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait WatermarkDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<WatermarkDeclContextExt<'input>>{

fn duration(&self) -> Option<Rc<DurationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn watermarkStrategy(&self) -> Option<Rc<WatermarkStrategyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldPath(&self) -> Option<Rc<FieldPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> WatermarkDeclContextAttrs<'input> for WatermarkDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn watermarkDecl(&mut self,)
	-> Result<Rc<WatermarkDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = WatermarkDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_watermarkDecl);
        let mut _localctx: Rc<WatermarkDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(530);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__55 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(520);
					recog.base.match_token(T__55,&mut recog.err_handler)?;

					/*InvokeRule duration*/
					recog.base.set_state(521);
					recog.duration()?;

					}
				}

			 T__56 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(522);
					recog.base.match_token(T__56,&mut recog.err_handler)?;

					/*InvokeRule watermarkStrategy*/
					recog.base.set_state(523);
					recog.watermarkStrategy()?;

					}
				}

			 T__57 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(524);
					recog.base.match_token(T__57,&mut recog.err_handler)?;

					/*InvokeRule duration*/
					recog.base.set_state(525);
					recog.duration()?;

					}
				}

			 T__58 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(526);
					recog.base.match_token(T__58,&mut recog.err_handler)?;

					/*InvokeRule duration*/
					recog.base.set_state(527);
					recog.duration()?;

					}
				}

			 T__59 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(528);
					recog.base.match_token(T__59,&mut recog.err_handler)?;

					/*InvokeRule fieldPath*/
					recog.base.set_state(529);
					recog.fieldPath()?;

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
//------------------- watermarkStrategy ----------------
pub type WatermarkStrategyContextAll<'input> = WatermarkStrategyContext<'input>;


pub type WatermarkStrategyContext<'input> = BaseParserRuleContext<'input,WatermarkStrategyContextExt<'input>>;

#[derive(Clone)]
pub struct WatermarkStrategyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for WatermarkStrategyContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for WatermarkStrategyContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_watermarkStrategy(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_watermarkStrategy(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for WatermarkStrategyContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_watermarkStrategy(self);
	}
}

impl<'input> CustomRuleContext<'input> for WatermarkStrategyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_watermarkStrategy }
	//fn type_rule_index() -> usize where Self: Sized { RULE_watermarkStrategy }
}
antlr_rust::tid!{WatermarkStrategyContextExt<'a>}

impl<'input> WatermarkStrategyContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<WatermarkStrategyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,WatermarkStrategyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait WatermarkStrategyContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<WatermarkStrategyContextExt<'input>>{


}

impl<'input> WatermarkStrategyContextAttrs<'input> for WatermarkStrategyContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn watermarkStrategy(&mut self,)
	-> Result<Rc<WatermarkStrategyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = WatermarkStrategyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_watermarkStrategy);
        let mut _localctx: Rc<WatermarkStrategyContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(532);
			_la = recog.base.input.la(1);
			if { !(((((_la - 61)) & !0x3f) == 0 && ((1usize << (_la - 61)) & ((1usize << (T__60 - 61)) | (1usize << (T__61 - 61)) | (1usize << (T__62 - 61)))) != 0)) } {
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
//------------------- lateDataDecl ----------------
pub type LateDataDeclContextAll<'input> = LateDataDeclContext<'input>;


pub type LateDataDeclContext<'input> = BaseParserRuleContext<'input,LateDataDeclContextExt<'input>>;

#[derive(Clone)]
pub struct LateDataDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for LateDataDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for LateDataDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lateDataDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_lateDataDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for LateDataDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_lateDataDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for LateDataDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lateDataDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lateDataDecl }
}
antlr_rust::tid!{LateDataDeclContextExt<'a>}

impl<'input> LateDataDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LateDataDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LateDataDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LateDataDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<LateDataDeclContextExt<'input>>{

fn lateDataStrategy(&self) -> Option<Rc<LateDataStrategyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> LateDataDeclContextAttrs<'input> for LateDataDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lateDataDecl(&mut self,)
	-> Result<Rc<LateDataDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LateDataDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_lateDataDecl);
        let mut _localctx: Rc<LateDataDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(538);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__63 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(534);
					recog.base.match_token(T__63,&mut recog.err_handler)?;

					/*InvokeRule lateDataStrategy*/
					recog.base.set_state(535);
					recog.lateDataStrategy()?;

					}
				}

			 T__64 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(536);
					recog.base.match_token(T__64,&mut recog.err_handler)?;

					recog.base.set_state(537);
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
//------------------- lateDataStrategy ----------------
pub type LateDataStrategyContextAll<'input> = LateDataStrategyContext<'input>;


pub type LateDataStrategyContext<'input> = BaseParserRuleContext<'input,LateDataStrategyContextExt<'input>>;

#[derive(Clone)]
pub struct LateDataStrategyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for LateDataStrategyContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for LateDataStrategyContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lateDataStrategy(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_lateDataStrategy(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for LateDataStrategyContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_lateDataStrategy(self);
	}
}

impl<'input> CustomRuleContext<'input> for LateDataStrategyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lateDataStrategy }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lateDataStrategy }
}
antlr_rust::tid!{LateDataStrategyContextExt<'a>}

impl<'input> LateDataStrategyContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LateDataStrategyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LateDataStrategyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LateDataStrategyContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<LateDataStrategyContextExt<'input>>{


}

impl<'input> LateDataStrategyContextAttrs<'input> for LateDataStrategyContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lateDataStrategy(&mut self,)
	-> Result<Rc<LateDataStrategyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LateDataStrategyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_lateDataStrategy);
        let mut _localctx: Rc<LateDataStrategyContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(540);
			_la = recog.base.input.la(1);
			if { !(((((_la - 66)) & !0x3f) == 0 && ((1usize << (_la - 66)) & ((1usize << (T__65 - 66)) | (1usize << (T__66 - 66)) | (1usize << (T__67 - 66)))) != 0)) } {
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
//------------------- allowedLatenessDecl ----------------
pub type AllowedLatenessDeclContextAll<'input> = AllowedLatenessDeclContext<'input>;


pub type AllowedLatenessDeclContext<'input> = BaseParserRuleContext<'input,AllowedLatenessDeclContextExt<'input>>;

#[derive(Clone)]
pub struct AllowedLatenessDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for AllowedLatenessDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for AllowedLatenessDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_allowedLatenessDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_allowedLatenessDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for AllowedLatenessDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_allowedLatenessDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for AllowedLatenessDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_allowedLatenessDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_allowedLatenessDecl }
}
antlr_rust::tid!{AllowedLatenessDeclContextExt<'a>}

impl<'input> AllowedLatenessDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AllowedLatenessDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AllowedLatenessDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AllowedLatenessDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<AllowedLatenessDeclContextExt<'input>>{

fn duration(&self) -> Option<Rc<DurationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AllowedLatenessDeclContextAttrs<'input> for AllowedLatenessDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn allowedLatenessDecl(&mut self,)
	-> Result<Rc<AllowedLatenessDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AllowedLatenessDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_allowedLatenessDecl);
        let mut _localctx: Rc<AllowedLatenessDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(542);
			recog.base.match_token(T__68,&mut recog.err_handler)?;

			/*InvokeRule duration*/
			recog.base.set_state(543);
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
//------------------- idleDecl ----------------
pub type IdleDeclContextAll<'input> = IdleDeclContext<'input>;


pub type IdleDeclContext<'input> = BaseParserRuleContext<'input,IdleDeclContextExt<'input>>;

#[derive(Clone)]
pub struct IdleDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for IdleDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for IdleDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_idleDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_idleDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for IdleDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_idleDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdleDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_idleDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_idleDecl }
}
antlr_rust::tid!{IdleDeclContextExt<'a>}

impl<'input> IdleDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IdleDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdleDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IdleDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<IdleDeclContextExt<'input>>{

fn duration(&self) -> Option<Rc<DurationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn idleBehavior(&self) -> Option<Rc<IdleBehaviorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> IdleDeclContextAttrs<'input> for IdleDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn idleDecl(&mut self,)
	-> Result<Rc<IdleDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdleDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_idleDecl);
        let mut _localctx: Rc<IdleDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(549);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__69 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(545);
					recog.base.match_token(T__69,&mut recog.err_handler)?;

					/*InvokeRule duration*/
					recog.base.set_state(546);
					recog.duration()?;

					}
				}

			 T__70 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(547);
					recog.base.match_token(T__70,&mut recog.err_handler)?;

					/*InvokeRule idleBehavior*/
					recog.base.set_state(548);
					recog.idleBehavior()?;

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
//------------------- idleBehavior ----------------
pub type IdleBehaviorContextAll<'input> = IdleBehaviorContext<'input>;


pub type IdleBehaviorContext<'input> = BaseParserRuleContext<'input,IdleBehaviorContextExt<'input>>;

#[derive(Clone)]
pub struct IdleBehaviorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for IdleBehaviorContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for IdleBehaviorContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_idleBehavior(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_idleBehavior(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for IdleBehaviorContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_idleBehavior(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdleBehaviorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_idleBehavior }
	//fn type_rule_index() -> usize where Self: Sized { RULE_idleBehavior }
}
antlr_rust::tid!{IdleBehaviorContextExt<'a>}

impl<'input> IdleBehaviorContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IdleBehaviorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdleBehaviorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IdleBehaviorContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<IdleBehaviorContextExt<'input>>{


}

impl<'input> IdleBehaviorContextAttrs<'input> for IdleBehaviorContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn idleBehavior(&mut self,)
	-> Result<Rc<IdleBehaviorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdleBehaviorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_idleBehavior);
        let mut _localctx: Rc<IdleBehaviorContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(551);
			_la = recog.base.input.la(1);
			if { !(((((_la - 72)) & !0x3f) == 0 && ((1usize << (_la - 72)) & ((1usize << (T__71 - 72)) | (1usize << (T__72 - 72)) | (1usize << (T__73 - 72)))) != 0)) } {
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
//------------------- sparsityDecl ----------------
pub type SparsityDeclContextAll<'input> = SparsityDeclContext<'input>;


pub type SparsityDeclContext<'input> = BaseParserRuleContext<'input,SparsityDeclContextExt<'input>>;

#[derive(Clone)]
pub struct SparsityDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for SparsityDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for SparsityDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_sparsityDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_sparsityDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for SparsityDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_sparsityDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for SparsityDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sparsityDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sparsityDecl }
}
antlr_rust::tid!{SparsityDeclContextExt<'a>}

impl<'input> SparsityDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SparsityDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SparsityDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SparsityDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<SparsityDeclContextExt<'input>>{

fn sparsityBlock(&self) -> Option<Rc<SparsityBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SparsityDeclContextAttrs<'input> for SparsityDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn sparsityDecl(&mut self,)
	-> Result<Rc<SparsityDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SparsityDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_sparsityDecl);
        let mut _localctx: Rc<SparsityDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(553);
			recog.base.match_token(T__74,&mut recog.err_handler)?;

			/*InvokeRule sparsityBlock*/
			recog.base.set_state(554);
			recog.sparsityBlock()?;

			recog.base.set_state(555);
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
//------------------- sparsityBlock ----------------
pub type SparsityBlockContextAll<'input> = SparsityBlockContext<'input>;


pub type SparsityBlockContext<'input> = BaseParserRuleContext<'input,SparsityBlockContextExt<'input>>;

#[derive(Clone)]
pub struct SparsityBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for SparsityBlockContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for SparsityBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_sparsityBlock(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_sparsityBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for SparsityBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_sparsityBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for SparsityBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sparsityBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sparsityBlock }
}
antlr_rust::tid!{SparsityBlockContextExt<'a>}

impl<'input> SparsityBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SparsityBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SparsityBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SparsityBlockContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<SparsityBlockContextExt<'input>>{

fn fieldArray_all(&self) ->  Vec<Rc<FieldArrayContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fieldArray(&self, i: usize) -> Option<Rc<FieldArrayContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> SparsityBlockContextAttrs<'input> for SparsityBlockContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn sparsityBlock(&mut self,)
	-> Result<Rc<SparsityBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SparsityBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_sparsityBlock);
        let mut _localctx: Rc<SparsityBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(559);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__75 {
				{
				recog.base.set_state(557);
				recog.base.match_token(T__75,&mut recog.err_handler)?;

				/*InvokeRule fieldArray*/
				recog.base.set_state(558);
				recog.fieldArray()?;

				}
			}

			recog.base.set_state(563);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__76 {
				{
				recog.base.set_state(561);
				recog.base.match_token(T__76,&mut recog.err_handler)?;

				/*InvokeRule fieldArray*/
				recog.base.set_state(562);
				recog.fieldArray()?;

				}
			}

			recog.base.set_state(567);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__77 {
				{
				recog.base.set_state(565);
				recog.base.match_token(T__77,&mut recog.err_handler)?;

				/*InvokeRule fieldArray*/
				recog.base.set_state(566);
				recog.fieldArray()?;

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
//------------------- retentionBlockDecl ----------------
pub type RetentionBlockDeclContextAll<'input> = RetentionBlockDeclContext<'input>;


pub type RetentionBlockDeclContext<'input> = BaseParserRuleContext<'input,RetentionBlockDeclContextExt<'input>>;

#[derive(Clone)]
pub struct RetentionBlockDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for RetentionBlockDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for RetentionBlockDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_retentionBlockDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_retentionBlockDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for RetentionBlockDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_retentionBlockDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for RetentionBlockDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_retentionBlockDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_retentionBlockDecl }
}
antlr_rust::tid!{RetentionBlockDeclContextExt<'a>}

impl<'input> RetentionBlockDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RetentionBlockDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RetentionBlockDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RetentionBlockDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<RetentionBlockDeclContextExt<'input>>{

fn retentionOptions(&self) -> Option<Rc<RetentionOptionsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RetentionBlockDeclContextAttrs<'input> for RetentionBlockDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn retentionBlockDecl(&mut self,)
	-> Result<Rc<RetentionBlockDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RetentionBlockDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_retentionBlockDecl);
        let mut _localctx: Rc<RetentionBlockDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(569);
			recog.base.match_token(T__40,&mut recog.err_handler)?;

			/*InvokeRule retentionOptions*/
			recog.base.set_state(570);
			recog.retentionOptions()?;

			recog.base.set_state(571);
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
//------------------- retentionOptions ----------------
pub type RetentionOptionsContextAll<'input> = RetentionOptionsContext<'input>;


pub type RetentionOptionsContext<'input> = BaseParserRuleContext<'input,RetentionOptionsContextExt<'input>>;

#[derive(Clone)]
pub struct RetentionOptionsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for RetentionOptionsContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for RetentionOptionsContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_retentionOptions(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_retentionOptions(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for RetentionOptionsContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_retentionOptions(self);
	}
}

impl<'input> CustomRuleContext<'input> for RetentionOptionsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_retentionOptions }
	//fn type_rule_index() -> usize where Self: Sized { RULE_retentionOptions }
}
antlr_rust::tid!{RetentionOptionsContextExt<'a>}

impl<'input> RetentionOptionsContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RetentionOptionsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RetentionOptionsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RetentionOptionsContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<RetentionOptionsContextExt<'input>>{

fn duration(&self) -> Option<Rc<DurationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn sizeSpec(&self) -> Option<Rc<SizeSpecContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn retentionPolicy(&self) -> Option<Rc<RetentionPolicyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RetentionOptionsContextAttrs<'input> for RetentionOptionsContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn retentionOptions(&mut self,)
	-> Result<Rc<RetentionOptionsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RetentionOptionsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_retentionOptions);
        let mut _localctx: Rc<RetentionOptionsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(575);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__78 {
				{
				recog.base.set_state(573);
				recog.base.match_token(T__78,&mut recog.err_handler)?;

				/*InvokeRule duration*/
				recog.base.set_state(574);
				recog.duration()?;

				}
			}

			recog.base.set_state(579);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__79 {
				{
				recog.base.set_state(577);
				recog.base.match_token(T__79,&mut recog.err_handler)?;

				/*InvokeRule sizeSpec*/
				recog.base.set_state(578);
				recog.sizeSpec()?;

				}
			}

			recog.base.set_state(583);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__80 {
				{
				recog.base.set_state(581);
				recog.base.match_token(T__80,&mut recog.err_handler)?;

				/*InvokeRule retentionPolicy*/
				recog.base.set_state(582);
				recog.retentionPolicy()?;

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
//------------------- retentionPolicy ----------------
pub type RetentionPolicyContextAll<'input> = RetentionPolicyContext<'input>;


pub type RetentionPolicyContext<'input> = BaseParserRuleContext<'input,RetentionPolicyContextExt<'input>>;

#[derive(Clone)]
pub struct RetentionPolicyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for RetentionPolicyContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for RetentionPolicyContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_retentionPolicy(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_retentionPolicy(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for RetentionPolicyContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_retentionPolicy(self);
	}
}

impl<'input> CustomRuleContext<'input> for RetentionPolicyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_retentionPolicy }
	//fn type_rule_index() -> usize where Self: Sized { RULE_retentionPolicy }
}
antlr_rust::tid!{RetentionPolicyContextExt<'a>}

impl<'input> RetentionPolicyContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RetentionPolicyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RetentionPolicyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RetentionPolicyContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<RetentionPolicyContextExt<'input>>{


}

impl<'input> RetentionPolicyContextAttrs<'input> for RetentionPolicyContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn retentionPolicy(&mut self,)
	-> Result<Rc<RetentionPolicyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RetentionPolicyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_retentionPolicy);
        let mut _localctx: Rc<RetentionPolicyContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(585);
			_la = recog.base.input.la(1);
			if { !(((((_la - 82)) & !0x3f) == 0 && ((1usize << (_la - 82)) & ((1usize << (T__81 - 82)) | (1usize << (T__82 - 82)) | (1usize << (T__83 - 82)))) != 0)) } {
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
//------------------- serializationBlock ----------------
pub type SerializationBlockContextAll<'input> = SerializationBlockContext<'input>;


pub type SerializationBlockContext<'input> = BaseParserRuleContext<'input,SerializationBlockContextExt<'input>>;

#[derive(Clone)]
pub struct SerializationBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for SerializationBlockContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for SerializationBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_serializationBlock(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_serializationBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for SerializationBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_serializationBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for SerializationBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_serializationBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_serializationBlock }
}
antlr_rust::tid!{SerializationBlockContextExt<'a>}

impl<'input> SerializationBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SerializationBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SerializationBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SerializationBlockContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<SerializationBlockContextExt<'input>>{

fn serializationDecl_all(&self) ->  Vec<Rc<SerializationDeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn serializationDecl(&self, i: usize) -> Option<Rc<SerializationDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> SerializationBlockContextAttrs<'input> for SerializationBlockContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn serializationBlock(&mut self,)
	-> Result<Rc<SerializationBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SerializationBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_serializationBlock);
        let mut _localctx: Rc<SerializationBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(587);
			recog.base.match_token(T__84,&mut recog.err_handler)?;

			recog.base.set_state(589); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule serializationDecl*/
				recog.base.set_state(588);
				recog.serializationDecl()?;

				}
				}
				recog.base.set_state(591); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==T__27 || ((((_la - 86)) & !0x3f) == 0 && ((1usize << (_la - 86)) & ((1usize << (T__85 - 86)) | (1usize << (T__90 - 86)) | (1usize << (T__91 - 86)))) != 0)) {break}
			}
			recog.base.set_state(593);
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
//------------------- serializationDecl ----------------
pub type SerializationDeclContextAll<'input> = SerializationDeclContext<'input>;


pub type SerializationDeclContext<'input> = BaseParserRuleContext<'input,SerializationDeclContextExt<'input>>;

#[derive(Clone)]
pub struct SerializationDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for SerializationDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for SerializationDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_serializationDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_serializationDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for SerializationDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_serializationDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for SerializationDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_serializationDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_serializationDecl }
}
antlr_rust::tid!{SerializationDeclContextExt<'a>}

impl<'input> SerializationDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SerializationDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SerializationDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SerializationDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<SerializationDeclContextExt<'input>>{

fn formatDecl(&self) -> Option<Rc<FormatDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn serializationCompatibilityDecl(&self) -> Option<Rc<SerializationCompatibilityDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn subjectDecl(&self) -> Option<Rc<SubjectDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn registryDecl(&self) -> Option<Rc<RegistryDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SerializationDeclContextAttrs<'input> for SerializationDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn serializationDecl(&mut self,)
	-> Result<Rc<SerializationDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SerializationDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 96, RULE_serializationDecl);
        let mut _localctx: Rc<SerializationDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(599);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__85 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule formatDecl*/
					recog.base.set_state(595);
					recog.formatDecl()?;

					}
				}

			 T__27 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule serializationCompatibilityDecl*/
					recog.base.set_state(596);
					recog.serializationCompatibilityDecl()?;

					}
				}

			 T__90 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule subjectDecl*/
					recog.base.set_state(597);
					recog.subjectDecl()?;

					}
				}

			 T__91 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule registryDecl*/
					recog.base.set_state(598);
					recog.registryDecl()?;

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
//------------------- formatDecl ----------------
pub type FormatDeclContextAll<'input> = FormatDeclContext<'input>;


pub type FormatDeclContext<'input> = BaseParserRuleContext<'input,FormatDeclContextExt<'input>>;

#[derive(Clone)]
pub struct FormatDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for FormatDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for FormatDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_formatDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_formatDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for FormatDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_formatDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for FormatDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_formatDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_formatDecl }
}
antlr_rust::tid!{FormatDeclContextExt<'a>}

impl<'input> FormatDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FormatDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FormatDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FormatDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<FormatDeclContextExt<'input>>{

fn serializationFormat(&self) -> Option<Rc<SerializationFormatContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FormatDeclContextAttrs<'input> for FormatDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn formatDecl(&mut self,)
	-> Result<Rc<FormatDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FormatDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 98, RULE_formatDecl);
        let mut _localctx: Rc<FormatDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(601);
			recog.base.match_token(T__85,&mut recog.err_handler)?;

			/*InvokeRule serializationFormat*/
			recog.base.set_state(602);
			recog.serializationFormat()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- serializationFormat ----------------
pub type SerializationFormatContextAll<'input> = SerializationFormatContext<'input>;


pub type SerializationFormatContext<'input> = BaseParserRuleContext<'input,SerializationFormatContextExt<'input>>;

#[derive(Clone)]
pub struct SerializationFormatContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for SerializationFormatContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for SerializationFormatContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_serializationFormat(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_serializationFormat(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for SerializationFormatContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_serializationFormat(self);
	}
}

impl<'input> CustomRuleContext<'input> for SerializationFormatContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_serializationFormat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_serializationFormat }
}
antlr_rust::tid!{SerializationFormatContextExt<'a>}

impl<'input> SerializationFormatContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SerializationFormatContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SerializationFormatContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SerializationFormatContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<SerializationFormatContextExt<'input>>{


}

impl<'input> SerializationFormatContextAttrs<'input> for SerializationFormatContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn serializationFormat(&mut self,)
	-> Result<Rc<SerializationFormatContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SerializationFormatContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 100, RULE_serializationFormat);
        let mut _localctx: Rc<SerializationFormatContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(604);
			_la = recog.base.input.la(1);
			if { !(((((_la - 87)) & !0x3f) == 0 && ((1usize << (_la - 87)) & ((1usize << (T__86 - 87)) | (1usize << (T__87 - 87)) | (1usize << (T__88 - 87)) | (1usize << (T__89 - 87)))) != 0)) } {
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
//------------------- serializationCompatibilityDecl ----------------
pub type SerializationCompatibilityDeclContextAll<'input> = SerializationCompatibilityDeclContext<'input>;


pub type SerializationCompatibilityDeclContext<'input> = BaseParserRuleContext<'input,SerializationCompatibilityDeclContextExt<'input>>;

#[derive(Clone)]
pub struct SerializationCompatibilityDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for SerializationCompatibilityDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for SerializationCompatibilityDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_serializationCompatibilityDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_serializationCompatibilityDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for SerializationCompatibilityDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_serializationCompatibilityDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for SerializationCompatibilityDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_serializationCompatibilityDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_serializationCompatibilityDecl }
}
antlr_rust::tid!{SerializationCompatibilityDeclContextExt<'a>}

impl<'input> SerializationCompatibilityDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SerializationCompatibilityDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SerializationCompatibilityDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SerializationCompatibilityDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<SerializationCompatibilityDeclContextExt<'input>>{

fn compatibilityMode(&self) -> Option<Rc<CompatibilityModeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SerializationCompatibilityDeclContextAttrs<'input> for SerializationCompatibilityDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn serializationCompatibilityDecl(&mut self,)
	-> Result<Rc<SerializationCompatibilityDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SerializationCompatibilityDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 102, RULE_serializationCompatibilityDecl);
        let mut _localctx: Rc<SerializationCompatibilityDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(606);
			recog.base.match_token(T__27,&mut recog.err_handler)?;

			/*InvokeRule compatibilityMode*/
			recog.base.set_state(607);
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
//------------------- subjectDecl ----------------
pub type SubjectDeclContextAll<'input> = SubjectDeclContext<'input>;


pub type SubjectDeclContext<'input> = BaseParserRuleContext<'input,SubjectDeclContextExt<'input>>;

#[derive(Clone)]
pub struct SubjectDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for SubjectDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for SubjectDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_subjectDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_subjectDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for SubjectDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_subjectDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for SubjectDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_subjectDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_subjectDecl }
}
antlr_rust::tid!{SubjectDeclContextExt<'a>}

impl<'input> SubjectDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SubjectDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SubjectDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SubjectDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<SubjectDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}

}

impl<'input> SubjectDeclContextAttrs<'input> for SubjectDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn subjectDecl(&mut self,)
	-> Result<Rc<SubjectDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SubjectDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 104, RULE_subjectDecl);
        let mut _localctx: Rc<SubjectDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(609);
			recog.base.match_token(T__90,&mut recog.err_handler)?;

			recog.base.set_state(610);
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
//------------------- registryDecl ----------------
pub type RegistryDeclContextAll<'input> = RegistryDeclContext<'input>;


pub type RegistryDeclContext<'input> = BaseParserRuleContext<'input,RegistryDeclContextExt<'input>>;

#[derive(Clone)]
pub struct RegistryDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for RegistryDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for RegistryDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_registryDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_registryDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for RegistryDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_registryDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for RegistryDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_registryDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_registryDecl }
}
antlr_rust::tid!{RegistryDeclContextExt<'a>}

impl<'input> RegistryDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RegistryDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RegistryDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RegistryDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<RegistryDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}

}

impl<'input> RegistryDeclContextAttrs<'input> for RegistryDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn registryDecl(&mut self,)
	-> Result<Rc<RegistryDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RegistryDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 106, RULE_registryDecl);
        let mut _localctx: Rc<RegistryDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(612);
			recog.base.match_token(T__91,&mut recog.err_handler)?;

			recog.base.set_state(613);
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
//------------------- fieldsBlock ----------------
pub type FieldsBlockContextAll<'input> = FieldsBlockContext<'input>;


pub type FieldsBlockContext<'input> = BaseParserRuleContext<'input,FieldsBlockContextExt<'input>>;

#[derive(Clone)]
pub struct FieldsBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for FieldsBlockContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for FieldsBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fieldsBlock(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_fieldsBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for FieldsBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_fieldsBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for FieldsBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fieldsBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fieldsBlock }
}
antlr_rust::tid!{FieldsBlockContextExt<'a>}

impl<'input> FieldsBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FieldsBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FieldsBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FieldsBlockContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<FieldsBlockContextExt<'input>>{

fn fieldDeclV2_all(&self) ->  Vec<Rc<FieldDeclV2ContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fieldDeclV2(&self, i: usize) -> Option<Rc<FieldDeclV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> FieldsBlockContextAttrs<'input> for FieldsBlockContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fieldsBlock(&mut self,)
	-> Result<Rc<FieldsBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FieldsBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 108, RULE_fieldsBlock);
        let mut _localctx: Rc<FieldsBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(615);
			recog.base.match_token(T__92,&mut recog.err_handler)?;

			recog.base.set_state(617); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule fieldDeclV2*/
				recog.base.set_state(616);
				recog.fieldDeclV2()?;

				}
				}
				recog.base.set_state(619); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(((((_la - 53)) & !0x3f) == 0 && ((1usize << (_la - 53)) & ((1usize << (T__52 - 53)) | (1usize << (T__53 - 53)) | (1usize << (T__54 - 53)))) != 0) || ((((_la - 107)) & !0x3f) == 0 && ((1usize << (_la - 107)) & ((1usize << (T__106 - 107)) | (1usize << (T__107 - 107)) | (1usize << (T__108 - 107)))) != 0) || _la==IDENTIFIER) {break}
			}
			recog.base.set_state(621);
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
//------------------- fieldDeclV2 ----------------
pub type FieldDeclV2ContextAll<'input> = FieldDeclV2Context<'input>;


pub type FieldDeclV2Context<'input> = BaseParserRuleContext<'input,FieldDeclV2ContextExt<'input>>;

#[derive(Clone)]
pub struct FieldDeclV2ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for FieldDeclV2Context<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for FieldDeclV2Context<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fieldDeclV2(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_fieldDeclV2(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for FieldDeclV2Context<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_fieldDeclV2(self);
	}
}

impl<'input> CustomRuleContext<'input> for FieldDeclV2ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fieldDeclV2 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fieldDeclV2 }
}
antlr_rust::tid!{FieldDeclV2ContextExt<'a>}

impl<'input> FieldDeclV2ContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FieldDeclV2ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FieldDeclV2ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FieldDeclV2ContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<FieldDeclV2ContextExt<'input>>{

fn fieldName(&self) -> Option<Rc<FieldNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldTypeV2(&self) -> Option<Rc<FieldTypeV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldQualifierV2_all(&self) ->  Vec<Rc<FieldQualifierV2ContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fieldQualifierV2(&self, i: usize) -> Option<Rc<FieldQualifierV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}

}

impl<'input> FieldDeclV2ContextAttrs<'input> for FieldDeclV2Context<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fieldDeclV2(&mut self,)
	-> Result<Rc<FieldDeclV2ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FieldDeclV2ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 110, RULE_fieldDeclV2);
        let mut _localctx: Rc<FieldDeclV2ContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fieldName*/
			recog.base.set_state(623);
			recog.fieldName()?;

			/*InvokeRule fieldTypeV2*/
			recog.base.set_state(624);
			recog.fieldTypeV2()?;

			recog.base.set_state(628);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__36 || ((((_la - 115)) & !0x3f) == 0 && ((1usize << (_la - 115)) & ((1usize << (T__114 - 115)) | (1usize << (T__139 - 115)) | (1usize << (T__140 - 115)) | (1usize << (T__141 - 115)) | (1usize << (T__142 - 115)) | (1usize << (T__143 - 115)) | (1usize << (T__144 - 115)))) != 0) || _la==PII {
				{
				{
				/*InvokeRule fieldQualifierV2*/
				recog.base.set_state(625);
				recog.fieldQualifierV2()?;

				}
				}
				recog.base.set_state(630);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(632);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==COMMA {
				{
				recog.base.set_state(631);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

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
//------------------- fieldName ----------------
pub type FieldNameContextAll<'input> = FieldNameContext<'input>;


pub type FieldNameContext<'input> = BaseParserRuleContext<'input,FieldNameContextExt<'input>>;

#[derive(Clone)]
pub struct FieldNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for FieldNameContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for FieldNameContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fieldName(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_fieldName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for FieldNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_fieldName(self);
	}
}

impl<'input> CustomRuleContext<'input> for FieldNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fieldName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fieldName }
}
antlr_rust::tid!{FieldNameContextExt<'a>}

impl<'input> FieldNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FieldNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FieldNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FieldNameContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<FieldNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn timeSemanticsType(&self) -> Option<Rc<TimeSemanticsTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stateQualifier(&self) -> Option<Rc<StateQualifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FieldNameContextAttrs<'input> for FieldNameContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fieldName(&mut self,)
	-> Result<Rc<FieldNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FieldNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 112, RULE_fieldName);
        let mut _localctx: Rc<FieldNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(637);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(634);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

			 T__52 | T__53 | T__54 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule timeSemanticsType*/
					recog.base.set_state(635);
					recog.timeSemanticsType()?;

					}
				}

			 T__106 | T__107 | T__108 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule stateQualifier*/
					recog.base.set_state(636);
					recog.stateQualifier()?;

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
//------------------- nestedObjectBlock ----------------
pub type NestedObjectBlockContextAll<'input> = NestedObjectBlockContext<'input>;


pub type NestedObjectBlockContext<'input> = BaseParserRuleContext<'input,NestedObjectBlockContextExt<'input>>;

#[derive(Clone)]
pub struct NestedObjectBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for NestedObjectBlockContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for NestedObjectBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_nestedObjectBlock(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_nestedObjectBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for NestedObjectBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_nestedObjectBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for NestedObjectBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nestedObjectBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nestedObjectBlock }
}
antlr_rust::tid!{NestedObjectBlockContextExt<'a>}

impl<'input> NestedObjectBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NestedObjectBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NestedObjectBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NestedObjectBlockContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<NestedObjectBlockContextExt<'input>>{

fn fieldName(&self) -> Option<Rc<FieldNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldDeclV2_all(&self) ->  Vec<Rc<FieldDeclV2ContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fieldDeclV2(&self, i: usize) -> Option<Rc<FieldDeclV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn nestedObjectBlock_all(&self) ->  Vec<Rc<NestedObjectBlockContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn nestedObjectBlock(&self, i: usize) -> Option<Rc<NestedObjectBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token LANGLE
/// Returns `None` if there is no child corresponding to token LANGLE
fn LANGLE(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(LANGLE, 0)
}
/// Retrieves first TerminalNode corresponding to token RANGLE
/// Returns `None` if there is no child corresponding to token RANGLE
fn RANGLE(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(RANGLE, 0)
}

}

impl<'input> NestedObjectBlockContextAttrs<'input> for NestedObjectBlockContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn nestedObjectBlock(&mut self,)
	-> Result<Rc<NestedObjectBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NestedObjectBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 114, RULE_nestedObjectBlock);
        let mut _localctx: Rc<NestedObjectBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			recog.base.set_state(674);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(60,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fieldName*/
					recog.base.set_state(639);
					recog.fieldName()?;

					recog.base.set_state(640);
					recog.base.match_token(T__93,&mut recog.err_handler)?;

					recog.base.set_state(644);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(56,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule fieldDeclV2*/
							recog.base.set_state(641);
							recog.fieldDeclV2()?;

							}
							} 
						}
						recog.base.set_state(646);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(56,&mut recog.base)?;
					}
					recog.base.set_state(650);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while ((((_la - 53)) & !0x3f) == 0 && ((1usize << (_la - 53)) & ((1usize << (T__52 - 53)) | (1usize << (T__53 - 53)) | (1usize << (T__54 - 53)))) != 0) || ((((_la - 107)) & !0x3f) == 0 && ((1usize << (_la - 107)) & ((1usize << (T__106 - 107)) | (1usize << (T__107 - 107)) | (1usize << (T__108 - 107)))) != 0) || _la==IDENTIFIER {
						{
						{
						/*InvokeRule nestedObjectBlock*/
						recog.base.set_state(647);
						recog.nestedObjectBlock()?;

						}
						}
						recog.base.set_state(652);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(653);
					recog.base.match_token(T__4,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule fieldName*/
					recog.base.set_state(655);
					recog.fieldName()?;

					recog.base.set_state(656);
					recog.base.match_token(T__94,&mut recog.err_handler)?;

					recog.base.set_state(657);
					recog.base.match_token(LANGLE,&mut recog.err_handler)?;

					recog.base.set_state(658);
					recog.base.match_token(T__93,&mut recog.err_handler)?;

					recog.base.set_state(659);
					recog.base.match_token(RANGLE,&mut recog.err_handler)?;

					recog.base.set_state(663);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(58,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule fieldDeclV2*/
							recog.base.set_state(660);
							recog.fieldDeclV2()?;

							}
							} 
						}
						recog.base.set_state(665);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(58,&mut recog.base)?;
					}
					recog.base.set_state(669);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while ((((_la - 53)) & !0x3f) == 0 && ((1usize << (_la - 53)) & ((1usize << (T__52 - 53)) | (1usize << (T__53 - 53)) | (1usize << (T__54 - 53)))) != 0) || ((((_la - 107)) & !0x3f) == 0 && ((1usize << (_la - 107)) & ((1usize << (T__106 - 107)) | (1usize << (T__107 - 107)) | (1usize << (T__108 - 107)))) != 0) || _la==IDENTIFIER {
						{
						{
						/*InvokeRule nestedObjectBlock*/
						recog.base.set_state(666);
						recog.nestedObjectBlock()?;

						}
						}
						recog.base.set_state(671);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(672);
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
//------------------- computedBlock ----------------
pub type ComputedBlockContextAll<'input> = ComputedBlockContext<'input>;


pub type ComputedBlockContext<'input> = BaseParserRuleContext<'input,ComputedBlockContextExt<'input>>;

#[derive(Clone)]
pub struct ComputedBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for ComputedBlockContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for ComputedBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_computedBlock(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_computedBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for ComputedBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_computedBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComputedBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_computedBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_computedBlock }
}
antlr_rust::tid!{ComputedBlockContextExt<'a>}

impl<'input> ComputedBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComputedBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComputedBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComputedBlockContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<ComputedBlockContextExt<'input>>{

fn computedField_all(&self) ->  Vec<Rc<ComputedFieldContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn computedField(&self, i: usize) -> Option<Rc<ComputedFieldContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ComputedBlockContextAttrs<'input> for ComputedBlockContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn computedBlock(&mut self,)
	-> Result<Rc<ComputedBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComputedBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_computedBlock);
        let mut _localctx: Rc<ComputedBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(676);
			recog.base.match_token(T__95,&mut recog.err_handler)?;

			recog.base.set_state(678); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule computedField*/
				recog.base.set_state(677);
				recog.computedField()?;

				}
				}
				recog.base.set_state(680); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(((((_la - 53)) & !0x3f) == 0 && ((1usize << (_la - 53)) & ((1usize << (T__52 - 53)) | (1usize << (T__53 - 53)) | (1usize << (T__54 - 53)))) != 0) || ((((_la - 107)) & !0x3f) == 0 && ((1usize << (_la - 107)) & ((1usize << (T__106 - 107)) | (1usize << (T__107 - 107)) | (1usize << (T__108 - 107)))) != 0) || _la==IDENTIFIER) {break}
			}
			recog.base.set_state(682);
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
//------------------- computedField ----------------
pub type ComputedFieldContextAll<'input> = ComputedFieldContext<'input>;


pub type ComputedFieldContext<'input> = BaseParserRuleContext<'input,ComputedFieldContextExt<'input>>;

#[derive(Clone)]
pub struct ComputedFieldContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for ComputedFieldContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for ComputedFieldContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_computedField(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_computedField(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for ComputedFieldContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_computedField(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComputedFieldContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_computedField }
	//fn type_rule_index() -> usize where Self: Sized { RULE_computedField }
}
antlr_rust::tid!{ComputedFieldContextExt<'a>}

impl<'input> ComputedFieldContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComputedFieldContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComputedFieldContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComputedFieldContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<ComputedFieldContextExt<'input>>{

fn fieldName(&self) -> Option<Rc<FieldNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn computedExpression(&self) -> Option<Rc<ComputedExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ComputedFieldContextAttrs<'input> for ComputedFieldContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn computedField(&mut self,)
	-> Result<Rc<ComputedFieldContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComputedFieldContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_computedField);
        let mut _localctx: Rc<ComputedFieldContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fieldName*/
			recog.base.set_state(684);
			recog.fieldName()?;

			recog.base.set_state(685);
			recog.base.match_token(T__96,&mut recog.err_handler)?;

			/*InvokeRule computedExpression*/
			recog.base.set_state(686);
			recog.computedExpression_rec(0)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- computedExpression ----------------
pub type ComputedExpressionContextAll<'input> = ComputedExpressionContext<'input>;


pub type ComputedExpressionContext<'input> = BaseParserRuleContext<'input,ComputedExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ComputedExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for ComputedExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for ComputedExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_computedExpression(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_computedExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for ComputedExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_computedExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComputedExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_computedExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_computedExpression }
}
antlr_rust::tid!{ComputedExpressionContextExt<'a>}

impl<'input> ComputedExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComputedExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComputedExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComputedExpressionContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<ComputedExpressionContextExt<'input>>{

fn computedExpression_all(&self) ->  Vec<Rc<ComputedExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn computedExpression(&self, i: usize) -> Option<Rc<ComputedExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn computedWhenExpression(&self) -> Option<Rc<ComputedWhenExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functionCall(&self) -> Option<Rc<FunctionCallContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldPath(&self) -> Option<Rc<FieldPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token STAR
/// Returns `None` if there is no child corresponding to token STAR
fn STAR(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(STAR, 0)
}
/// Retrieves first TerminalNode corresponding to token SLASH
/// Returns `None` if there is no child corresponding to token SLASH
fn SLASH(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(SLASH, 0)
}
/// Retrieves first TerminalNode corresponding to token PLUS
/// Returns `None` if there is no child corresponding to token PLUS
fn PLUS(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(PLUS, 0)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}
fn comparisonOp(&self) -> Option<Rc<ComparisonOpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ComputedExpressionContextAttrs<'input> for ComputedExpressionContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  computedExpression(&mut self,)
	-> Result<Rc<ComputedExpressionContextAll<'input>>,ANTLRError> {
		self.computedExpression_rec(0)
	}

	fn computedExpression_rec(&mut self, _p: isize)
	-> Result<Rc<ComputedExpressionContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ComputedExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 120, RULE_computedExpression, _p);
	    let mut _localctx: Rc<ComputedExpressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 120;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(699);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(62,&mut recog.base)? {
				1 =>{
					{
					recog.base.set_state(689);
					recog.base.match_token(T__99,&mut recog.err_handler)?;

					/*InvokeRule computedExpression*/
					recog.base.set_state(690);
					recog.computedExpression_rec(6)?;

					}
				}
			,
				2 =>{
					{
					recog.base.set_state(691);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule computedExpression*/
					recog.base.set_state(692);
					recog.computedExpression_rec(0)?;

					recog.base.set_state(693);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					{
					/*InvokeRule computedWhenExpression*/
					recog.base.set_state(695);
					recog.computedWhenExpression()?;

					}
				}
			,
				4 =>{
					{
					/*InvokeRule functionCall*/
					recog.base.set_state(696);
					recog.functionCall()?;

					}
				}
			,
				5 =>{
					{
					/*InvokeRule fieldPath*/
					recog.base.set_state(697);
					recog.fieldPath()?;

					}
				}
			,
				6 =>{
					{
					/*InvokeRule literal*/
					recog.base.set_state(698);
					recog.literal()?;

					}
				}

				_ => {}
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(719);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(64,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(717);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(63,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ComputedExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_computedExpression);
							_localctx = tmp;
							recog.base.set_state(701);
							if !({recog.precpred(None, 11)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 11)".to_owned()), None))?;
							}
							recog.base.set_state(702);
							_la = recog.base.input.la(1);
							if { !(_la==STAR || _la==SLASH) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule computedExpression*/
							recog.base.set_state(703);
							recog.computedExpression_rec(12)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ComputedExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_computedExpression);
							_localctx = tmp;
							recog.base.set_state(704);
							if !({recog.precpred(None, 10)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 10)".to_owned()), None))?;
							}
							recog.base.set_state(705);
							_la = recog.base.input.la(1);
							if { !(_la==PLUS || _la==MINUS) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule computedExpression*/
							recog.base.set_state(706);
							recog.computedExpression_rec(11)?;

							}
						}
					,
						3 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ComputedExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_computedExpression);
							_localctx = tmp;
							recog.base.set_state(707);
							if !({recog.precpred(None, 9)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 9)".to_owned()), None))?;
							}
							/*InvokeRule comparisonOp*/
							recog.base.set_state(708);
							recog.comparisonOp()?;

							/*InvokeRule computedExpression*/
							recog.base.set_state(709);
							recog.computedExpression_rec(10)?;

							}
						}
					,
						4 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ComputedExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_computedExpression);
							_localctx = tmp;
							recog.base.set_state(711);
							if !({recog.precpred(None, 8)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 8)".to_owned()), None))?;
							}
							recog.base.set_state(712);
							recog.base.match_token(T__97,&mut recog.err_handler)?;

							/*InvokeRule computedExpression*/
							recog.base.set_state(713);
							recog.computedExpression_rec(9)?;

							}
						}
					,
						5 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ComputedExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_computedExpression);
							_localctx = tmp;
							recog.base.set_state(714);
							if !({recog.precpred(None, 7)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 7)".to_owned()), None))?;
							}
							recog.base.set_state(715);
							recog.base.match_token(T__98,&mut recog.err_handler)?;

							/*InvokeRule computedExpression*/
							recog.base.set_state(716);
							recog.computedExpression_rec(8)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(721);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(64,&mut recog.base)?;
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
//------------------- computedWhenExpression ----------------
pub type ComputedWhenExpressionContextAll<'input> = ComputedWhenExpressionContext<'input>;


pub type ComputedWhenExpressionContext<'input> = BaseParserRuleContext<'input,ComputedWhenExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ComputedWhenExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for ComputedWhenExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for ComputedWhenExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_computedWhenExpression(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_computedWhenExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for ComputedWhenExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_computedWhenExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComputedWhenExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_computedWhenExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_computedWhenExpression }
}
antlr_rust::tid!{ComputedWhenExpressionContextExt<'a>}

impl<'input> ComputedWhenExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComputedWhenExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComputedWhenExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComputedWhenExpressionContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<ComputedWhenExpressionContextExt<'input>>{

fn computedExpression_all(&self) ->  Vec<Rc<ComputedExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn computedExpression(&self, i: usize) -> Option<Rc<ComputedExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ComputedWhenExpressionContextAttrs<'input> for ComputedWhenExpressionContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn computedWhenExpression(&mut self,)
	-> Result<Rc<ComputedWhenExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComputedWhenExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_computedWhenExpression);
        let mut _localctx: Rc<ComputedWhenExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(722);
			recog.base.match_token(T__100,&mut recog.err_handler)?;

			/*InvokeRule computedExpression*/
			recog.base.set_state(723);
			recog.computedExpression_rec(0)?;

			recog.base.set_state(724);
			recog.base.match_token(T__101,&mut recog.err_handler)?;

			/*InvokeRule computedExpression*/
			recog.base.set_state(725);
			recog.computedExpression_rec(0)?;

			recog.base.set_state(733);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__100 {
				{
				{
				recog.base.set_state(726);
				recog.base.match_token(T__100,&mut recog.err_handler)?;

				/*InvokeRule computedExpression*/
				recog.base.set_state(727);
				recog.computedExpression_rec(0)?;

				recog.base.set_state(728);
				recog.base.match_token(T__101,&mut recog.err_handler)?;

				/*InvokeRule computedExpression*/
				recog.base.set_state(729);
				recog.computedExpression_rec(0)?;

				}
				}
				recog.base.set_state(735);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(736);
			recog.base.match_token(T__102,&mut recog.err_handler)?;

			/*InvokeRule computedExpression*/
			recog.base.set_state(737);
			recog.computedExpression_rec(0)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- stateMachineBlock ----------------
pub type StateMachineBlockContextAll<'input> = StateMachineBlockContext<'input>;


pub type StateMachineBlockContext<'input> = BaseParserRuleContext<'input,StateMachineBlockContextExt<'input>>;

#[derive(Clone)]
pub struct StateMachineBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for StateMachineBlockContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for StateMachineBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stateMachineBlock(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_stateMachineBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for StateMachineBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_stateMachineBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for StateMachineBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stateMachineBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stateMachineBlock }
}
antlr_rust::tid!{StateMachineBlockContextExt<'a>}

impl<'input> StateMachineBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StateMachineBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StateMachineBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StateMachineBlockContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<StateMachineBlockContextExt<'input>>{

fn statesBlock(&self) -> Option<Rc<StatesBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn forEntityDecl(&self) -> Option<Rc<ForEntityDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn initialStateDecl(&self) -> Option<Rc<InitialStateDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn transitionsBlock(&self) -> Option<Rc<TransitionsBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn onTransitionBlock(&self) -> Option<Rc<OnTransitionBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StateMachineBlockContextAttrs<'input> for StateMachineBlockContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stateMachineBlock(&mut self,)
	-> Result<Rc<StateMachineBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StateMachineBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 124, RULE_stateMachineBlock);
        let mut _localctx: Rc<StateMachineBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(740);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__104 {
				{
				/*InvokeRule forEntityDecl*/
				recog.base.set_state(739);
				recog.forEntityDecl()?;

				}
			}

			/*InvokeRule statesBlock*/
			recog.base.set_state(742);
			recog.statesBlock()?;

			recog.base.set_state(744);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__103 {
				{
				/*InvokeRule initialStateDecl*/
				recog.base.set_state(743);
				recog.initialStateDecl()?;

				}
			}

			recog.base.set_state(747);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__109 {
				{
				/*InvokeRule transitionsBlock*/
				recog.base.set_state(746);
				recog.transitionsBlock()?;

				}
			}

			recog.base.set_state(750);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__111 {
				{
				/*InvokeRule onTransitionBlock*/
				recog.base.set_state(749);
				recog.onTransitionBlock()?;

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
//------------------- initialStateDecl ----------------
pub type InitialStateDeclContextAll<'input> = InitialStateDeclContext<'input>;


pub type InitialStateDeclContext<'input> = BaseParserRuleContext<'input,InitialStateDeclContextExt<'input>>;

#[derive(Clone)]
pub struct InitialStateDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for InitialStateDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for InitialStateDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_initialStateDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_initialStateDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for InitialStateDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_initialStateDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for InitialStateDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_initialStateDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_initialStateDecl }
}
antlr_rust::tid!{InitialStateDeclContextExt<'a>}

impl<'input> InitialStateDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InitialStateDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InitialStateDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InitialStateDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<InitialStateDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> InitialStateDeclContextAttrs<'input> for InitialStateDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn initialStateDecl(&mut self,)
	-> Result<Rc<InitialStateDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InitialStateDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_initialStateDecl);
        let mut _localctx: Rc<InitialStateDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(752);
			recog.base.match_token(T__103,&mut recog.err_handler)?;

			recog.base.set_state(753);
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
//------------------- forEntityDecl ----------------
pub type ForEntityDeclContextAll<'input> = ForEntityDeclContext<'input>;


pub type ForEntityDeclContext<'input> = BaseParserRuleContext<'input,ForEntityDeclContextExt<'input>>;

#[derive(Clone)]
pub struct ForEntityDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for ForEntityDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for ForEntityDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_forEntityDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_forEntityDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for ForEntityDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_forEntityDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for ForEntityDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_forEntityDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_forEntityDecl }
}
antlr_rust::tid!{ForEntityDeclContextExt<'a>}

impl<'input> ForEntityDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ForEntityDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ForEntityDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ForEntityDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<ForEntityDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> ForEntityDeclContextAttrs<'input> for ForEntityDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn forEntityDecl(&mut self,)
	-> Result<Rc<ForEntityDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ForEntityDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 128, RULE_forEntityDecl);
        let mut _localctx: Rc<ForEntityDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(755);
			recog.base.match_token(T__104,&mut recog.err_handler)?;

			recog.base.set_state(756);
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
//------------------- statesBlock ----------------
pub type StatesBlockContextAll<'input> = StatesBlockContext<'input>;


pub type StatesBlockContext<'input> = BaseParserRuleContext<'input,StatesBlockContextExt<'input>>;

#[derive(Clone)]
pub struct StatesBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for StatesBlockContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for StatesBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_statesBlock(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_statesBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for StatesBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_statesBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for StatesBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statesBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statesBlock }
}
antlr_rust::tid!{StatesBlockContextExt<'a>}

impl<'input> StatesBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatesBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatesBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StatesBlockContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<StatesBlockContextExt<'input>>{

fn statesDecl(&self) -> Option<Rc<StatesDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stateDefList(&self) -> Option<Rc<StateDefListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StatesBlockContextAttrs<'input> for StatesBlockContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn statesBlock(&mut self,)
	-> Result<Rc<StatesBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatesBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 130, RULE_statesBlock);
        let mut _localctx: Rc<StatesBlockContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(758);
			recog.base.match_token(T__105,&mut recog.err_handler)?;

			recog.base.set_state(761);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LBRACKET 
				=> {
					{
					/*InvokeRule statesDecl*/
					recog.base.set_state(759);
					recog.statesDecl()?;

					}
				}

			 IDENTIFIER 
				=> {
					{
					/*InvokeRule stateDefList*/
					recog.base.set_state(760);
					recog.stateDefList()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(764);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(71,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(763);
					recog.base.match_token(T__4,&mut recog.err_handler)?;

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
//------------------- statesDecl ----------------
pub type StatesDeclContextAll<'input> = StatesDeclContext<'input>;


pub type StatesDeclContext<'input> = BaseParserRuleContext<'input,StatesDeclContextExt<'input>>;

#[derive(Clone)]
pub struct StatesDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for StatesDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for StatesDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_statesDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_statesDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for StatesDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_statesDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for StatesDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statesDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statesDecl }
}
antlr_rust::tid!{StatesDeclContextExt<'a>}

impl<'input> StatesDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatesDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatesDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StatesDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<StatesDeclContextExt<'input>>{

fn stateArray(&self) -> Option<Rc<StateArrayContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StatesDeclContextAttrs<'input> for StatesDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn statesDecl(&mut self,)
	-> Result<Rc<StatesDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatesDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 132, RULE_statesDecl);
        let mut _localctx: Rc<StatesDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule stateArray*/
			recog.base.set_state(766);
			recog.stateArray()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- stateDefList ----------------
pub type StateDefListContextAll<'input> = StateDefListContext<'input>;


pub type StateDefListContext<'input> = BaseParserRuleContext<'input,StateDefListContextExt<'input>>;

#[derive(Clone)]
pub struct StateDefListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for StateDefListContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for StateDefListContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stateDefList(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_stateDefList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for StateDefListContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_stateDefList(self);
	}
}

impl<'input> CustomRuleContext<'input> for StateDefListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stateDefList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stateDefList }
}
antlr_rust::tid!{StateDefListContextExt<'a>}

impl<'input> StateDefListContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StateDefListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StateDefListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StateDefListContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<StateDefListContextExt<'input>>{

fn stateDef_all(&self) ->  Vec<Rc<StateDefContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn stateDef(&self, i: usize) -> Option<Rc<StateDefContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> StateDefListContextAttrs<'input> for StateDefListContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stateDefList(&mut self,)
	-> Result<Rc<StateDefListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StateDefListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 134, RULE_stateDefList);
        let mut _localctx: Rc<StateDefListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(769); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule stateDef*/
				recog.base.set_state(768);
				recog.stateDef()?;

				}
				}
				recog.base.set_state(771); 
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
//------------------- stateDef ----------------
pub type StateDefContextAll<'input> = StateDefContext<'input>;


pub type StateDefContext<'input> = BaseParserRuleContext<'input,StateDefContextExt<'input>>;

#[derive(Clone)]
pub struct StateDefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for StateDefContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for StateDefContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stateDef(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_stateDef(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for StateDefContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_stateDef(self);
	}
}

impl<'input> CustomRuleContext<'input> for StateDefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stateDef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stateDef }
}
antlr_rust::tid!{StateDefContextExt<'a>}

impl<'input> StateDefContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StateDefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StateDefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StateDefContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<StateDefContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn stateQualifier(&self) -> Option<Rc<StateQualifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StateDefContextAttrs<'input> for StateDefContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stateDef(&mut self,)
	-> Result<Rc<StateDefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StateDefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 136, RULE_stateDef);
        let mut _localctx: Rc<StateDefContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(773);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(776);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==COLON {
				{
				recog.base.set_state(774);
				recog.base.match_token(COLON,&mut recog.err_handler)?;

				/*InvokeRule stateQualifier*/
				recog.base.set_state(775);
				recog.stateQualifier()?;

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
//------------------- stateQualifier ----------------
pub type StateQualifierContextAll<'input> = StateQualifierContext<'input>;


pub type StateQualifierContext<'input> = BaseParserRuleContext<'input,StateQualifierContextExt<'input>>;

#[derive(Clone)]
pub struct StateQualifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for StateQualifierContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for StateQualifierContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stateQualifier(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_stateQualifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for StateQualifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_stateQualifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for StateQualifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stateQualifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stateQualifier }
}
antlr_rust::tid!{StateQualifierContextExt<'a>}

impl<'input> StateQualifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StateQualifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StateQualifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StateQualifierContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<StateQualifierContextExt<'input>>{


}

impl<'input> StateQualifierContextAttrs<'input> for StateQualifierContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stateQualifier(&mut self,)
	-> Result<Rc<StateQualifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StateQualifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 138, RULE_stateQualifier);
        let mut _localctx: Rc<StateQualifierContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(778);
			_la = recog.base.input.la(1);
			if { !(((((_la - 107)) & !0x3f) == 0 && ((1usize << (_la - 107)) & ((1usize << (T__106 - 107)) | (1usize << (T__107 - 107)) | (1usize << (T__108 - 107)))) != 0)) } {
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
//------------------- stateArray ----------------
pub type StateArrayContextAll<'input> = StateArrayContext<'input>;


pub type StateArrayContext<'input> = BaseParserRuleContext<'input,StateArrayContextExt<'input>>;

#[derive(Clone)]
pub struct StateArrayContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for StateArrayContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for StateArrayContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stateArray(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_stateArray(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for StateArrayContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_stateArray(self);
	}
}

impl<'input> CustomRuleContext<'input> for StateArrayContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stateArray }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stateArray }
}
antlr_rust::tid!{StateArrayContextExt<'a>}

impl<'input> StateArrayContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StateArrayContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StateArrayContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StateArrayContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<StateArrayContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACKET
/// Returns `None` if there is no child corresponding to token LBRACKET
fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(LBRACKET, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,SchemaDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves first TerminalNode corresponding to token RBRACKET
/// Returns `None` if there is no child corresponding to token RBRACKET
fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(RBRACKET, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,SchemaDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> StateArrayContextAttrs<'input> for StateArrayContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stateArray(&mut self,)
	-> Result<Rc<StateArrayContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StateArrayContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 140, RULE_stateArray);
        let mut _localctx: Rc<StateArrayContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(780);
			recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

			recog.base.set_state(781);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(786);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(782);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				recog.base.set_state(783);
				recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(788);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(789);
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
//------------------- transitionsBlock ----------------
pub type TransitionsBlockContextAll<'input> = TransitionsBlockContext<'input>;


pub type TransitionsBlockContext<'input> = BaseParserRuleContext<'input,TransitionsBlockContextExt<'input>>;

#[derive(Clone)]
pub struct TransitionsBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for TransitionsBlockContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for TransitionsBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_transitionsBlock(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_transitionsBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for TransitionsBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_transitionsBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for TransitionsBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_transitionsBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_transitionsBlock }
}
antlr_rust::tid!{TransitionsBlockContextExt<'a>}

impl<'input> TransitionsBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TransitionsBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TransitionsBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TransitionsBlockContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<TransitionsBlockContextExt<'input>>{

fn transitionDecl_all(&self) ->  Vec<Rc<TransitionDeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn transitionDecl(&self, i: usize) -> Option<Rc<TransitionDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn transitionArrowDecl_all(&self) ->  Vec<Rc<TransitionArrowDeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn transitionArrowDecl(&self, i: usize) -> Option<Rc<TransitionArrowDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TransitionsBlockContextAttrs<'input> for TransitionsBlockContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn transitionsBlock(&mut self,)
	-> Result<Rc<TransitionsBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TransitionsBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 142, RULE_transitionsBlock);
        let mut _localctx: Rc<TransitionsBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(791);
			recog.base.match_token(T__109,&mut recog.err_handler)?;

			recog.base.set_state(794); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				recog.base.set_state(794);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.base.input.la(1) {
				 T__110 
					=> {
						{
						/*InvokeRule transitionDecl*/
						recog.base.set_state(792);
						recog.transitionDecl()?;

						}
					}

				 IDENTIFIER | STAR 
					=> {
						{
						/*InvokeRule transitionArrowDecl*/
						recog.base.set_state(793);
						recog.transitionArrowDecl()?;

						}
					}

					_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				}
				recog.base.set_state(796); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==T__110 || _la==IDENTIFIER || _la==STAR) {break}
			}
			recog.base.set_state(798);
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
//------------------- transitionDecl ----------------
pub type TransitionDeclContextAll<'input> = TransitionDeclContext<'input>;


pub type TransitionDeclContext<'input> = BaseParserRuleContext<'input,TransitionDeclContextExt<'input>>;

#[derive(Clone)]
pub struct TransitionDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for TransitionDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for TransitionDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_transitionDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_transitionDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for TransitionDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_transitionDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for TransitionDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_transitionDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_transitionDecl }
}
antlr_rust::tid!{TransitionDeclContextExt<'a>}

impl<'input> TransitionDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TransitionDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TransitionDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TransitionDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<TransitionDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn stateArray(&self) -> Option<Rc<StateArrayContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TransitionDeclContextAttrs<'input> for TransitionDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn transitionDecl(&mut self,)
	-> Result<Rc<TransitionDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TransitionDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 144, RULE_transitionDecl);
        let mut _localctx: Rc<TransitionDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(800);
			recog.base.match_token(T__110,&mut recog.err_handler)?;

			recog.base.set_state(801);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			/*InvokeRule stateArray*/
			recog.base.set_state(802);
			recog.stateArray()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- transitionArrowDecl ----------------
pub type TransitionArrowDeclContextAll<'input> = TransitionArrowDeclContext<'input>;


pub type TransitionArrowDeclContext<'input> = BaseParserRuleContext<'input,TransitionArrowDeclContextExt<'input>>;

#[derive(Clone)]
pub struct TransitionArrowDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for TransitionArrowDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for TransitionArrowDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_transitionArrowDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_transitionArrowDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for TransitionArrowDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_transitionArrowDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for TransitionArrowDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_transitionArrowDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_transitionArrowDecl }
}
antlr_rust::tid!{TransitionArrowDeclContextExt<'a>}

impl<'input> TransitionArrowDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TransitionArrowDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TransitionArrowDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TransitionArrowDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<TransitionArrowDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ARROW
/// Returns `None` if there is no child corresponding to token ARROW
fn ARROW(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(ARROW, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,SchemaDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves first TerminalNode corresponding to token STAR
/// Returns `None` if there is no child corresponding to token STAR
fn STAR(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(STAR, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}

}

impl<'input> TransitionArrowDeclContextAttrs<'input> for TransitionArrowDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn transitionArrowDecl(&mut self,)
	-> Result<Rc<TransitionArrowDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TransitionArrowDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 146, RULE_transitionArrowDecl);
        let mut _localctx: Rc<TransitionArrowDeclContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(804);
			_la = recog.base.input.la(1);
			if { !(_la==IDENTIFIER || _la==STAR) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(805);
			recog.base.match_token(ARROW,&mut recog.err_handler)?;

			recog.base.set_state(806);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(809);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==COLON {
				{
				recog.base.set_state(807);
				recog.base.match_token(COLON,&mut recog.err_handler)?;

				recog.base.set_state(808);
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
//------------------- onTransitionBlock ----------------
pub type OnTransitionBlockContextAll<'input> = OnTransitionBlockContext<'input>;


pub type OnTransitionBlockContext<'input> = BaseParserRuleContext<'input,OnTransitionBlockContextExt<'input>>;

#[derive(Clone)]
pub struct OnTransitionBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for OnTransitionBlockContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for OnTransitionBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_onTransitionBlock(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_onTransitionBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for OnTransitionBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_onTransitionBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for OnTransitionBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_onTransitionBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_onTransitionBlock }
}
antlr_rust::tid!{OnTransitionBlockContextExt<'a>}

impl<'input> OnTransitionBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OnTransitionBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OnTransitionBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OnTransitionBlockContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<OnTransitionBlockContextExt<'input>>{

fn transitionAction_all(&self) ->  Vec<Rc<TransitionActionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn transitionAction(&self, i: usize) -> Option<Rc<TransitionActionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OnTransitionBlockContextAttrs<'input> for OnTransitionBlockContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn onTransitionBlock(&mut self,)
	-> Result<Rc<OnTransitionBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OnTransitionBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 148, RULE_onTransitionBlock);
        let mut _localctx: Rc<OnTransitionBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(811);
			recog.base.match_token(T__111,&mut recog.err_handler)?;

			recog.base.set_state(813); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule transitionAction*/
				recog.base.set_state(812);
				recog.transitionAction()?;

				}
				}
				recog.base.set_state(815); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==T__112) {break}
			}
			recog.base.set_state(817);
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
//------------------- transitionAction ----------------
pub type TransitionActionContextAll<'input> = TransitionActionContext<'input>;


pub type TransitionActionContext<'input> = BaseParserRuleContext<'input,TransitionActionContextExt<'input>>;

#[derive(Clone)]
pub struct TransitionActionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for TransitionActionContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for TransitionActionContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_transitionAction(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_transitionAction(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for TransitionActionContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_transitionAction(self);
	}
}

impl<'input> CustomRuleContext<'input> for TransitionActionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_transitionAction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_transitionAction }
}
antlr_rust::tid!{TransitionActionContextExt<'a>}

impl<'input> TransitionActionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TransitionActionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TransitionActionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TransitionActionContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<TransitionActionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn actionCall(&self) -> Option<Rc<ActionCallContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TransitionActionContextAttrs<'input> for TransitionActionContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn transitionAction(&mut self,)
	-> Result<Rc<TransitionActionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TransitionActionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 150, RULE_transitionAction);
        let mut _localctx: Rc<TransitionActionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(819);
			recog.base.match_token(T__112,&mut recog.err_handler)?;

			recog.base.set_state(820);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(821);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule actionCall*/
			recog.base.set_state(822);
			recog.actionCall()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
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

impl<'input> SchemaDSLParserContext<'input> for ActionCallContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for ActionCallContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_actionCall(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_actionCall(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for ActionCallContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_actionCall(self);
	}
}

impl<'input> CustomRuleContext<'input> for ActionCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_actionCall }
	//fn type_rule_index() -> usize where Self: Sized { RULE_actionCall }
}
antlr_rust::tid!{ActionCallContextExt<'a>}

impl<'input> ActionCallContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ActionCallContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ActionCallContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ActionCallContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<ActionCallContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token STRING in current rule
fn STRING_all(&self) -> Vec<Rc<TerminalNode<'input,SchemaDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token STRING, starting from 0.
/// Returns `None` if number of children corresponding to token STRING is less or equal than `i`.
fn STRING(&self, i: usize) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,SchemaDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ActionCallContextAttrs<'input> for ActionCallContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn actionCall(&mut self,)
	-> Result<Rc<ActionCallContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ActionCallContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 152, RULE_actionCall);
        let mut _localctx: Rc<ActionCallContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(824);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(825);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			recog.base.set_state(834);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==STRING {
				{
				recog.base.set_state(826);
				recog.base.match_token(STRING,&mut recog.err_handler)?;

				recog.base.set_state(831);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==COMMA {
					{
					{
					recog.base.set_state(827);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					recog.base.set_state(828);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
					}
					recog.base.set_state(833);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(836);
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
//------------------- parametersBlock ----------------
pub type ParametersBlockContextAll<'input> = ParametersBlockContext<'input>;


pub type ParametersBlockContext<'input> = BaseParserRuleContext<'input,ParametersBlockContextExt<'input>>;

#[derive(Clone)]
pub struct ParametersBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for ParametersBlockContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for ParametersBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_parametersBlock(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_parametersBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for ParametersBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_parametersBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParametersBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parametersBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parametersBlock }
}
antlr_rust::tid!{ParametersBlockContextExt<'a>}

impl<'input> ParametersBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParametersBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParametersBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParametersBlockContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<ParametersBlockContextExt<'input>>{

fn parameterDeclV2_all(&self) ->  Vec<Rc<ParameterDeclV2ContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn parameterDeclV2(&self, i: usize) -> Option<Rc<ParameterDeclV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ParametersBlockContextAttrs<'input> for ParametersBlockContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parametersBlock(&mut self,)
	-> Result<Rc<ParametersBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParametersBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 154, RULE_parametersBlock);
        let mut _localctx: Rc<ParametersBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(838);
			recog.base.match_token(T__113,&mut recog.err_handler)?;

			recog.base.set_state(840); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule parameterDeclV2*/
				recog.base.set_state(839);
				recog.parameterDeclV2()?;

				}
				}
				recog.base.set_state(842); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(((((_la - 53)) & !0x3f) == 0 && ((1usize << (_la - 53)) & ((1usize << (T__52 - 53)) | (1usize << (T__53 - 53)) | (1usize << (T__54 - 53)))) != 0) || ((((_la - 107)) & !0x3f) == 0 && ((1usize << (_la - 107)) & ((1usize << (T__106 - 107)) | (1usize << (T__107 - 107)) | (1usize << (T__108 - 107)))) != 0) || _la==IDENTIFIER) {break}
			}
			recog.base.set_state(844);
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
//------------------- parameterDeclV2 ----------------
pub type ParameterDeclV2ContextAll<'input> = ParameterDeclV2Context<'input>;


pub type ParameterDeclV2Context<'input> = BaseParserRuleContext<'input,ParameterDeclV2ContextExt<'input>>;

#[derive(Clone)]
pub struct ParameterDeclV2ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for ParameterDeclV2Context<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for ParameterDeclV2Context<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_parameterDeclV2(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_parameterDeclV2(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for ParameterDeclV2Context<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_parameterDeclV2(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParameterDeclV2ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterDeclV2 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterDeclV2 }
}
antlr_rust::tid!{ParameterDeclV2ContextExt<'a>}

impl<'input> ParameterDeclV2ContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParameterDeclV2ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParameterDeclV2ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParameterDeclV2ContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<ParameterDeclV2ContextExt<'input>>{

fn fieldName(&self) -> Option<Rc<FieldNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldTypeV2(&self) -> Option<Rc<FieldTypeV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn parameterOption_all(&self) ->  Vec<Rc<ParameterOptionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn parameterOption(&self, i: usize) -> Option<Rc<ParameterOptionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ParameterDeclV2ContextAttrs<'input> for ParameterDeclV2Context<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parameterDeclV2(&mut self,)
	-> Result<Rc<ParameterDeclV2ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterDeclV2ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 156, RULE_parameterDeclV2);
        let mut _localctx: Rc<ParameterDeclV2ContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fieldName*/
			recog.base.set_state(846);
			recog.fieldName()?;

			/*InvokeRule fieldTypeV2*/
			recog.base.set_state(847);
			recog.fieldTypeV2()?;

			recog.base.set_state(851);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__45 || ((((_la - 115)) & !0x3f) == 0 && ((1usize << (_la - 115)) & ((1usize << (T__114 - 115)) | (1usize << (T__115 - 115)) | (1usize << (T__116 - 115)))) != 0) {
				{
				{
				/*InvokeRule parameterOption*/
				recog.base.set_state(848);
				recog.parameterOption()?;

				}
				}
				recog.base.set_state(853);
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
//------------------- parameterOption ----------------
pub type ParameterOptionContextAll<'input> = ParameterOptionContext<'input>;


pub type ParameterOptionContext<'input> = BaseParserRuleContext<'input,ParameterOptionContextExt<'input>>;

#[derive(Clone)]
pub struct ParameterOptionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for ParameterOptionContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for ParameterOptionContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_parameterOption(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_parameterOption(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for ParameterOptionContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_parameterOption(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParameterOptionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterOption }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterOption }
}
antlr_rust::tid!{ParameterOptionContextExt<'a>}

impl<'input> ParameterOptionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParameterOptionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParameterOptionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParameterOptionContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<ParameterOptionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn numberLiteral_all(&self) ->  Vec<Rc<NumberLiteralContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn numberLiteral(&self, i: usize) -> Option<Rc<NumberLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}
/// Retrieves first TerminalNode corresponding to token BOOLEAN
/// Returns `None` if there is no child corresponding to token BOOLEAN
fn BOOLEAN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(BOOLEAN, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> ParameterOptionContextAttrs<'input> for ParameterOptionContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parameterOption(&mut self,)
	-> Result<Rc<ParameterOptionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterOptionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 158, RULE_parameterOption);
        let mut _localctx: Rc<ParameterOptionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(870);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__114 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(854);
					recog.base.match_token(T__114,&mut recog.err_handler)?;

					recog.base.set_state(855);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule literal*/
					recog.base.set_state(856);
					recog.literal()?;

					recog.base.set_state(857);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}

			 T__45 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(859);
					recog.base.match_token(T__45,&mut recog.err_handler)?;

					recog.base.set_state(860);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule numberLiteral*/
					recog.base.set_state(861);
					recog.numberLiteral()?;

					recog.base.set_state(862);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule numberLiteral*/
					recog.base.set_state(863);
					recog.numberLiteral()?;

					recog.base.set_state(864);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}

			 T__115 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(866);
					recog.base.match_token(T__115,&mut recog.err_handler)?;

					recog.base.set_state(867);
					recog.base.match_token(BOOLEAN,&mut recog.err_handler)?;

					}
				}

			 T__116 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(868);
					recog.base.match_token(T__116,&mut recog.err_handler)?;

					recog.base.set_state(869);
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
//------------------- entriesBlock ----------------
pub type EntriesBlockContextAll<'input> = EntriesBlockContext<'input>;


pub type EntriesBlockContext<'input> = BaseParserRuleContext<'input,EntriesBlockContextExt<'input>>;

#[derive(Clone)]
pub struct EntriesBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for EntriesBlockContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for EntriesBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_entriesBlock(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_entriesBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for EntriesBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_entriesBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for EntriesBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_entriesBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_entriesBlock }
}
antlr_rust::tid!{EntriesBlockContextExt<'a>}

impl<'input> EntriesBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EntriesBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EntriesBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EntriesBlockContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<EntriesBlockContextExt<'input>>{

fn entryDecl_all(&self) ->  Vec<Rc<EntryDeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn entryDecl(&self, i: usize) -> Option<Rc<EntryDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> EntriesBlockContextAttrs<'input> for EntriesBlockContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn entriesBlock(&mut self,)
	-> Result<Rc<EntriesBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EntriesBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 160, RULE_entriesBlock);
        let mut _localctx: Rc<EntriesBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(872);
			recog.base.match_token(T__117,&mut recog.err_handler)?;

			recog.base.set_state(874); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule entryDecl*/
				recog.base.set_state(873);
				recog.entryDecl()?;

				}
				}
				recog.base.set_state(876); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==IDENTIFIER) {break}
			}
			recog.base.set_state(878);
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
//------------------- entryDecl ----------------
pub type EntryDeclContextAll<'input> = EntryDeclContext<'input>;


pub type EntryDeclContext<'input> = BaseParserRuleContext<'input,EntryDeclContextExt<'input>>;

#[derive(Clone)]
pub struct EntryDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for EntryDeclContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for EntryDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_entryDecl(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_entryDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for EntryDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_entryDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for EntryDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_entryDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_entryDecl }
}
antlr_rust::tid!{EntryDeclContextExt<'a>}

impl<'input> EntryDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EntryDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EntryDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EntryDeclContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<EntryDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn entryFieldV2_all(&self) ->  Vec<Rc<EntryFieldV2ContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn entryFieldV2(&self, i: usize) -> Option<Rc<EntryFieldV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> EntryDeclContextAttrs<'input> for EntryDeclContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn entryDecl(&mut self,)
	-> Result<Rc<EntryDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EntryDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 162, RULE_entryDecl);
        let mut _localctx: Rc<EntryDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(880);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(881);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(883); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					{
					/*InvokeRule entryFieldV2*/
					recog.base.set_state(882);
					recog.entryFieldV2()?;

					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(885); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(85,&mut recog.base)?;
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
//------------------- entryFieldV2 ----------------
pub type EntryFieldV2ContextAll<'input> = EntryFieldV2Context<'input>;


pub type EntryFieldV2Context<'input> = BaseParserRuleContext<'input,EntryFieldV2ContextExt<'input>>;

#[derive(Clone)]
pub struct EntryFieldV2ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for EntryFieldV2Context<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for EntryFieldV2Context<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_entryFieldV2(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_entryFieldV2(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for EntryFieldV2Context<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_entryFieldV2(self);
	}
}

impl<'input> CustomRuleContext<'input> for EntryFieldV2ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_entryFieldV2 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_entryFieldV2 }
}
antlr_rust::tid!{EntryFieldV2ContextExt<'a>}

impl<'input> EntryFieldV2ContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EntryFieldV2ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EntryFieldV2ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EntryFieldV2ContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<EntryFieldV2ContextExt<'input>>{

fn fieldName(&self) -> Option<Rc<FieldNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token BOOLEAN
/// Returns `None` if there is no child corresponding to token BOOLEAN
fn BOOLEAN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(BOOLEAN, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}

}

impl<'input> EntryFieldV2ContextAttrs<'input> for EntryFieldV2Context<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn entryFieldV2(&mut self,)
	-> Result<Rc<EntryFieldV2ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EntryFieldV2ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 164, RULE_entryFieldV2);
        let mut _localctx: Rc<EntryFieldV2ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(894);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__52 | T__53 | T__54 | T__106 | T__107 | T__108 | IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fieldName*/
					recog.base.set_state(887);
					recog.fieldName()?;

					/*InvokeRule literal*/
					recog.base.set_state(888);
					recog.literal()?;

					}
				}

			 T__36 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(890);
					recog.base.match_token(T__36,&mut recog.err_handler)?;

					recog.base.set_state(891);
					recog.base.match_token(BOOLEAN,&mut recog.err_handler)?;

					}
				}

			 T__118 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(892);
					recog.base.match_token(T__118,&mut recog.err_handler)?;

					recog.base.set_state(893);
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
//------------------- ruleBlock ----------------
pub type RuleBlockContextAll<'input> = RuleBlockContext<'input>;


pub type RuleBlockContext<'input> = BaseParserRuleContext<'input,RuleBlockContextExt<'input>>;

#[derive(Clone)]
pub struct RuleBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for RuleBlockContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for RuleBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_ruleBlock(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_ruleBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for RuleBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_ruleBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for RuleBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ruleBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ruleBlock }
}
antlr_rust::tid!{RuleBlockContextExt<'a>}

impl<'input> RuleBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RuleBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RuleBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RuleBlockContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<RuleBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn givenBlock(&self) -> Option<Rc<GivenBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn returnBlock(&self) -> Option<Rc<ReturnBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn calculateBlock(&self) -> Option<Rc<CalculateBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RuleBlockContextAttrs<'input> for RuleBlockContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn ruleBlock(&mut self,)
	-> Result<Rc<RuleBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RuleBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 166, RULE_ruleBlock);
        let mut _localctx: Rc<RuleBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(896);
			recog.base.match_token(T__119,&mut recog.err_handler)?;

			recog.base.set_state(897);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			/*InvokeRule givenBlock*/
			recog.base.set_state(898);
			recog.givenBlock()?;

			recog.base.set_state(900);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__121 {
				{
				/*InvokeRule calculateBlock*/
				recog.base.set_state(899);
				recog.calculateBlock()?;

				}
			}

			/*InvokeRule returnBlock*/
			recog.base.set_state(902);
			recog.returnBlock()?;

			recog.base.set_state(903);
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
//------------------- givenBlock ----------------
pub type GivenBlockContextAll<'input> = GivenBlockContext<'input>;


pub type GivenBlockContext<'input> = BaseParserRuleContext<'input,GivenBlockContextExt<'input>>;

#[derive(Clone)]
pub struct GivenBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for GivenBlockContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for GivenBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_givenBlock(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_givenBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for GivenBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_givenBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for GivenBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_givenBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_givenBlock }
}
antlr_rust::tid!{GivenBlockContextExt<'a>}

impl<'input> GivenBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<GivenBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GivenBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait GivenBlockContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<GivenBlockContextExt<'input>>{

fn ruleFieldDeclV2_all(&self) ->  Vec<Rc<RuleFieldDeclV2ContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn ruleFieldDeclV2(&self, i: usize) -> Option<Rc<RuleFieldDeclV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> GivenBlockContextAttrs<'input> for GivenBlockContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn givenBlock(&mut self,)
	-> Result<Rc<GivenBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GivenBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 168, RULE_givenBlock);
        let mut _localctx: Rc<GivenBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(905);
			recog.base.match_token(T__120,&mut recog.err_handler)?;

			recog.base.set_state(907); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule ruleFieldDeclV2*/
				recog.base.set_state(906);
				recog.ruleFieldDeclV2()?;

				}
				}
				recog.base.set_state(909); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(((((_la - 53)) & !0x3f) == 0 && ((1usize << (_la - 53)) & ((1usize << (T__52 - 53)) | (1usize << (T__53 - 53)) | (1usize << (T__54 - 53)))) != 0) || ((((_la - 107)) & !0x3f) == 0 && ((1usize << (_la - 107)) & ((1usize << (T__106 - 107)) | (1usize << (T__107 - 107)) | (1usize << (T__108 - 107)))) != 0) || _la==IDENTIFIER) {break}
			}
			recog.base.set_state(911);
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
//------------------- ruleFieldDeclV2 ----------------
pub type RuleFieldDeclV2ContextAll<'input> = RuleFieldDeclV2Context<'input>;


pub type RuleFieldDeclV2Context<'input> = BaseParserRuleContext<'input,RuleFieldDeclV2ContextExt<'input>>;

#[derive(Clone)]
pub struct RuleFieldDeclV2ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for RuleFieldDeclV2Context<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for RuleFieldDeclV2Context<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_ruleFieldDeclV2(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_ruleFieldDeclV2(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for RuleFieldDeclV2Context<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_ruleFieldDeclV2(self);
	}
}

impl<'input> CustomRuleContext<'input> for RuleFieldDeclV2ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ruleFieldDeclV2 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ruleFieldDeclV2 }
}
antlr_rust::tid!{RuleFieldDeclV2ContextExt<'a>}

impl<'input> RuleFieldDeclV2ContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RuleFieldDeclV2ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RuleFieldDeclV2ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RuleFieldDeclV2ContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<RuleFieldDeclV2ContextExt<'input>>{

fn fieldName(&self) -> Option<Rc<FieldNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldTypeV2(&self) -> Option<Rc<FieldTypeV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RuleFieldDeclV2ContextAttrs<'input> for RuleFieldDeclV2Context<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn ruleFieldDeclV2(&mut self,)
	-> Result<Rc<RuleFieldDeclV2ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RuleFieldDeclV2ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 170, RULE_ruleFieldDeclV2);
        let mut _localctx: Rc<RuleFieldDeclV2ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fieldName*/
			recog.base.set_state(913);
			recog.fieldName()?;

			/*InvokeRule fieldTypeV2*/
			recog.base.set_state(914);
			recog.fieldTypeV2()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- calculateBlock ----------------
pub type CalculateBlockContextAll<'input> = CalculateBlockContext<'input>;


pub type CalculateBlockContext<'input> = BaseParserRuleContext<'input,CalculateBlockContextExt<'input>>;

#[derive(Clone)]
pub struct CalculateBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for CalculateBlockContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for CalculateBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_calculateBlock(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_calculateBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for CalculateBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_calculateBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for CalculateBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_calculateBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_calculateBlock }
}
antlr_rust::tid!{CalculateBlockContextExt<'a>}

impl<'input> CalculateBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CalculateBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CalculateBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CalculateBlockContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<CalculateBlockContextExt<'input>>{

fn calculation_all(&self) ->  Vec<Rc<CalculationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn calculation(&self, i: usize) -> Option<Rc<CalculationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> CalculateBlockContextAttrs<'input> for CalculateBlockContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn calculateBlock(&mut self,)
	-> Result<Rc<CalculateBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CalculateBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 172, RULE_calculateBlock);
        let mut _localctx: Rc<CalculateBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(916);
			recog.base.match_token(T__121,&mut recog.err_handler)?;

			recog.base.set_state(918); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule calculation*/
				recog.base.set_state(917);
				recog.calculation()?;

				}
				}
				recog.base.set_state(920); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(((((_la - 53)) & !0x3f) == 0 && ((1usize << (_la - 53)) & ((1usize << (T__52 - 53)) | (1usize << (T__53 - 53)) | (1usize << (T__54 - 53)))) != 0) || ((((_la - 107)) & !0x3f) == 0 && ((1usize << (_la - 107)) & ((1usize << (T__106 - 107)) | (1usize << (T__107 - 107)) | (1usize << (T__108 - 107)))) != 0) || _la==IDENTIFIER) {break}
			}
			recog.base.set_state(922);
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
//------------------- calculation ----------------
pub type CalculationContextAll<'input> = CalculationContext<'input>;


pub type CalculationContext<'input> = BaseParserRuleContext<'input,CalculationContextExt<'input>>;

#[derive(Clone)]
pub struct CalculationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for CalculationContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for CalculationContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_calculation(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_calculation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for CalculationContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_calculation(self);
	}
}

impl<'input> CustomRuleContext<'input> for CalculationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_calculation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_calculation }
}
antlr_rust::tid!{CalculationContextExt<'a>}

impl<'input> CalculationContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CalculationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CalculationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CalculationContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<CalculationContextExt<'input>>{

fn fieldName(&self) -> Option<Rc<FieldNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CalculationContextAttrs<'input> for CalculationContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn calculation(&mut self,)
	-> Result<Rc<CalculationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CalculationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 174, RULE_calculation);
        let mut _localctx: Rc<CalculationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fieldName*/
			recog.base.set_state(924);
			recog.fieldName()?;

			recog.base.set_state(925);
			recog.base.match_token(T__96,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(926);
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
//------------------- returnBlock ----------------
pub type ReturnBlockContextAll<'input> = ReturnBlockContext<'input>;


pub type ReturnBlockContext<'input> = BaseParserRuleContext<'input,ReturnBlockContextExt<'input>>;

#[derive(Clone)]
pub struct ReturnBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for ReturnBlockContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for ReturnBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_returnBlock(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_returnBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for ReturnBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_returnBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReturnBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_returnBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_returnBlock }
}
antlr_rust::tid!{ReturnBlockContextExt<'a>}

impl<'input> ReturnBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReturnBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReturnBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReturnBlockContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<ReturnBlockContextExt<'input>>{

fn ruleFieldDeclV2_all(&self) ->  Vec<Rc<RuleFieldDeclV2ContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn ruleFieldDeclV2(&self, i: usize) -> Option<Rc<RuleFieldDeclV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ReturnBlockContextAttrs<'input> for ReturnBlockContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn returnBlock(&mut self,)
	-> Result<Rc<ReturnBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReturnBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 176, RULE_returnBlock);
        let mut _localctx: Rc<ReturnBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(928);
			recog.base.match_token(T__122,&mut recog.err_handler)?;

			recog.base.set_state(930); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule ruleFieldDeclV2*/
				recog.base.set_state(929);
				recog.ruleFieldDeclV2()?;

				}
				}
				recog.base.set_state(932); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(((((_la - 53)) & !0x3f) == 0 && ((1usize << (_la - 53)) & ((1usize << (T__52 - 53)) | (1usize << (T__53 - 53)) | (1usize << (T__54 - 53)))) != 0) || ((((_la - 107)) & !0x3f) == 0 && ((1usize << (_la - 107)) & ((1usize << (T__106 - 107)) | (1usize << (T__107 - 107)) | (1usize << (T__108 - 107)))) != 0) || _la==IDENTIFIER) {break}
			}
			recog.base.set_state(934);
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
//------------------- migrationBlock ----------------
pub type MigrationBlockContextAll<'input> = MigrationBlockContext<'input>;


pub type MigrationBlockContext<'input> = BaseParserRuleContext<'input,MigrationBlockContextExt<'input>>;

#[derive(Clone)]
pub struct MigrationBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for MigrationBlockContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for MigrationBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_migrationBlock(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_migrationBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for MigrationBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_migrationBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for MigrationBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_migrationBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_migrationBlock }
}
antlr_rust::tid!{MigrationBlockContextExt<'a>}

impl<'input> MigrationBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MigrationBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MigrationBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MigrationBlockContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<MigrationBlockContextExt<'input>>{

fn migrationStatement_all(&self) ->  Vec<Rc<MigrationStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn migrationStatement(&self, i: usize) -> Option<Rc<MigrationStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> MigrationBlockContextAttrs<'input> for MigrationBlockContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn migrationBlock(&mut self,)
	-> Result<Rc<MigrationBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MigrationBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 178, RULE_migrationBlock);
        let mut _localctx: Rc<MigrationBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(936);
			recog.base.match_token(T__123,&mut recog.err_handler)?;

			recog.base.set_state(938); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule migrationStatement*/
				recog.base.set_state(937);
				recog.migrationStatement()?;

				}
				}
				recog.base.set_state(940); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==IDENTIFIER || _la==LPAREN) {break}
			}
			recog.base.set_state(942);
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
//------------------- migrationStatement ----------------
pub type MigrationStatementContextAll<'input> = MigrationStatementContext<'input>;


pub type MigrationStatementContext<'input> = BaseParserRuleContext<'input,MigrationStatementContextExt<'input>>;

#[derive(Clone)]
pub struct MigrationStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for MigrationStatementContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for MigrationStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_migrationStatement(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_migrationStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for MigrationStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_migrationStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for MigrationStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_migrationStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_migrationStatement }
}
antlr_rust::tid!{MigrationStatementContextExt<'a>}

impl<'input> MigrationStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MigrationStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MigrationStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MigrationStatementContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<MigrationStatementContextExt<'input>>{

fn fieldPath(&self) -> Option<Rc<FieldPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn fieldList(&self) -> Option<Rc<FieldListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn functionCall(&self) -> Option<Rc<FunctionCallContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MigrationStatementContextAttrs<'input> for MigrationStatementContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn migrationStatement(&mut self,)
	-> Result<Rc<MigrationStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MigrationStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 180, RULE_migrationStatement);
        let mut _localctx: Rc<MigrationStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(954);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fieldPath*/
					recog.base.set_state(944);
					recog.fieldPath()?;

					recog.base.set_state(945);
					recog.base.match_token(T__96,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(946);
					recog.expression_rec(0)?;

					}
				}

			 LPAREN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(948);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule fieldList*/
					recog.base.set_state(949);
					recog.fieldList()?;

					recog.base.set_state(950);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					recog.base.set_state(951);
					recog.base.match_token(T__96,&mut recog.err_handler)?;

					/*InvokeRule functionCall*/
					recog.base.set_state(952);
					recog.functionCall()?;

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
//------------------- typeAliasBlock ----------------
pub type TypeAliasBlockContextAll<'input> = TypeAliasBlockContext<'input>;


pub type TypeAliasBlockContext<'input> = BaseParserRuleContext<'input,TypeAliasBlockContextExt<'input>>;

#[derive(Clone)]
pub struct TypeAliasBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for TypeAliasBlockContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for TypeAliasBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_typeAliasBlock(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_typeAliasBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for TypeAliasBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_typeAliasBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeAliasBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeAliasBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeAliasBlock }
}
antlr_rust::tid!{TypeAliasBlockContextExt<'a>}

impl<'input> TypeAliasBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeAliasBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeAliasBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeAliasBlockContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<TypeAliasBlockContextExt<'input>>{

fn typeAliasV2_all(&self) ->  Vec<Rc<TypeAliasV2ContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typeAliasV2(&self, i: usize) -> Option<Rc<TypeAliasV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TypeAliasBlockContextAttrs<'input> for TypeAliasBlockContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeAliasBlock(&mut self,)
	-> Result<Rc<TypeAliasBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeAliasBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 182, RULE_typeAliasBlock);
        let mut _localctx: Rc<TypeAliasBlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(956);
			recog.base.match_token(T__124,&mut recog.err_handler)?;

			recog.base.set_state(958); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule typeAliasV2*/
				recog.base.set_state(957);
				recog.typeAliasV2()?;

				}
				}
				recog.base.set_state(960); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==UPPER_IDENTIFIER) {break}
			}
			recog.base.set_state(962);
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
//------------------- typeAliasV2 ----------------
pub type TypeAliasV2ContextAll<'input> = TypeAliasV2Context<'input>;


pub type TypeAliasV2Context<'input> = BaseParserRuleContext<'input,TypeAliasV2ContextExt<'input>>;

#[derive(Clone)]
pub struct TypeAliasV2ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for TypeAliasV2Context<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for TypeAliasV2Context<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_typeAliasV2(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_typeAliasV2(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for TypeAliasV2Context<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_typeAliasV2(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeAliasV2ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeAliasV2 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeAliasV2 }
}
antlr_rust::tid!{TypeAliasV2ContextExt<'a>}

impl<'input> TypeAliasV2ContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeAliasV2ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeAliasV2ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeAliasV2ContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<TypeAliasV2ContextExt<'input>>{

fn aliasName(&self) -> Option<Rc<AliasNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldTypeV2(&self) -> Option<Rc<FieldTypeV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldDeclV2_all(&self) ->  Vec<Rc<FieldDeclV2ContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fieldDeclV2(&self, i: usize) -> Option<Rc<FieldDeclV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TypeAliasV2ContextAttrs<'input> for TypeAliasV2Context<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeAliasV2(&mut self,)
	-> Result<Rc<TypeAliasV2ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeAliasV2ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 184, RULE_typeAliasV2);
        let mut _localctx: Rc<TypeAliasV2ContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(977);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(95,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule aliasName*/
					recog.base.set_state(964);
					recog.aliasName()?;

					/*InvokeRule fieldTypeV2*/
					recog.base.set_state(965);
					recog.fieldTypeV2()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule aliasName*/
					recog.base.set_state(967);
					recog.aliasName()?;

					recog.base.set_state(968);
					recog.base.match_token(T__93,&mut recog.err_handler)?;

					recog.base.set_state(972);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while ((((_la - 53)) & !0x3f) == 0 && ((1usize << (_la - 53)) & ((1usize << (T__52 - 53)) | (1usize << (T__53 - 53)) | (1usize << (T__54 - 53)))) != 0) || ((((_la - 107)) & !0x3f) == 0 && ((1usize << (_la - 107)) & ((1usize << (T__106 - 107)) | (1usize << (T__107 - 107)) | (1usize << (T__108 - 107)))) != 0) || _la==IDENTIFIER {
						{
						{
						/*InvokeRule fieldDeclV2*/
						recog.base.set_state(969);
						recog.fieldDeclV2()?;

						}
						}
						recog.base.set_state(974);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(975);
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
//------------------- aliasName ----------------
pub type AliasNameContextAll<'input> = AliasNameContext<'input>;


pub type AliasNameContext<'input> = BaseParserRuleContext<'input,AliasNameContextExt<'input>>;

#[derive(Clone)]
pub struct AliasNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for AliasNameContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for AliasNameContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_aliasName(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_aliasName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for AliasNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_aliasName(self);
	}
}

impl<'input> CustomRuleContext<'input> for AliasNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_aliasName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_aliasName }
}
antlr_rust::tid!{AliasNameContextExt<'a>}

impl<'input> AliasNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AliasNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AliasNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AliasNameContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<AliasNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token UPPER_IDENTIFIER
/// Returns `None` if there is no child corresponding to token UPPER_IDENTIFIER
fn UPPER_IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(UPPER_IDENTIFIER, 0)
}

}

impl<'input> AliasNameContextAttrs<'input> for AliasNameContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn aliasName(&mut self,)
	-> Result<Rc<AliasNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AliasNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 186, RULE_aliasName);
        let mut _localctx: Rc<AliasNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(979);
			recog.base.match_token(UPPER_IDENTIFIER,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fieldTypeV2 ----------------
pub type FieldTypeV2ContextAll<'input> = FieldTypeV2Context<'input>;


pub type FieldTypeV2Context<'input> = BaseParserRuleContext<'input,FieldTypeV2ContextExt<'input>>;

#[derive(Clone)]
pub struct FieldTypeV2ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for FieldTypeV2Context<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for FieldTypeV2Context<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fieldTypeV2(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_fieldTypeV2(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for FieldTypeV2Context<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_fieldTypeV2(self);
	}
}

impl<'input> CustomRuleContext<'input> for FieldTypeV2ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fieldTypeV2 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fieldTypeV2 }
}
antlr_rust::tid!{FieldTypeV2ContextExt<'a>}

impl<'input> FieldTypeV2ContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FieldTypeV2ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FieldTypeV2ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FieldTypeV2ContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<FieldTypeV2ContextExt<'input>>{

fn baseTypeV2(&self) -> Option<Rc<BaseTypeV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn collectionTypeV2(&self) -> Option<Rc<CollectionTypeV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn inlineObjectTypeV2(&self) -> Option<Rc<InlineObjectTypeV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token UPPER_IDENTIFIER
/// Returns `None` if there is no child corresponding to token UPPER_IDENTIFIER
fn UPPER_IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(UPPER_IDENTIFIER, 0)
}

}

impl<'input> FieldTypeV2ContextAttrs<'input> for FieldTypeV2Context<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fieldTypeV2(&mut self,)
	-> Result<Rc<FieldTypeV2ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FieldTypeV2ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 188, RULE_fieldTypeV2);
        let mut _localctx: Rc<FieldTypeV2ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(986);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__86 | T__125 | T__126 | T__127 | T__128 | T__129 | T__130 | T__131 |
			 T__132 | T__133 | T__134 | T__135 | T__136 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule baseTypeV2*/
					recog.base.set_state(981);
					recog.baseTypeV2()?;

					}
				}

			 T__94 | T__137 | T__138 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule collectionTypeV2*/
					recog.base.set_state(982);
					recog.collectionTypeV2()?;

					}
				}

			 T__93 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule inlineObjectTypeV2*/
					recog.base.set_state(983);
					recog.inlineObjectTypeV2()?;

					}
				}

			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(984);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

			 UPPER_IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(985);
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
//------------------- baseTypeV2 ----------------
pub type BaseTypeV2ContextAll<'input> = BaseTypeV2Context<'input>;


pub type BaseTypeV2Context<'input> = BaseParserRuleContext<'input,BaseTypeV2ContextExt<'input>>;

#[derive(Clone)]
pub struct BaseTypeV2ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for BaseTypeV2Context<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for BaseTypeV2Context<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_baseTypeV2(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_baseTypeV2(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for BaseTypeV2Context<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_baseTypeV2(self);
	}
}

impl<'input> CustomRuleContext<'input> for BaseTypeV2ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_baseTypeV2 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_baseTypeV2 }
}
antlr_rust::tid!{BaseTypeV2ContextExt<'a>}

impl<'input> BaseTypeV2ContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BaseTypeV2ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BaseTypeV2ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BaseTypeV2ContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<BaseTypeV2ContextExt<'input>>{

fn typeParams(&self) -> Option<Rc<TypeParamsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> BaseTypeV2ContextAttrs<'input> for BaseTypeV2Context<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn baseTypeV2(&mut self,)
	-> Result<Rc<BaseTypeV2ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BaseTypeV2ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 190, RULE_baseTypeV2);
        let mut _localctx: Rc<BaseTypeV2ContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1011);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__125 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(988);
					recog.base.match_token(T__125,&mut recog.err_handler)?;

					recog.base.set_state(990);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LPAREN {
						{
						/*InvokeRule typeParams*/
						recog.base.set_state(989);
						recog.typeParams()?;

						}
					}

					}
				}

			 T__126 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(992);
					recog.base.match_token(T__126,&mut recog.err_handler)?;

					/*InvokeRule typeParams*/
					recog.base.set_state(993);
					recog.typeParams()?;

					}
				}

			 T__127 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(994);
					recog.base.match_token(T__127,&mut recog.err_handler)?;

					}
				}

			 T__128 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(995);
					recog.base.match_token(T__128,&mut recog.err_handler)?;

					recog.base.set_state(997);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LPAREN {
						{
						/*InvokeRule typeParams*/
						recog.base.set_state(996);
						recog.typeParams()?;

						}
					}

					}
				}

			 T__129 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(999);
					recog.base.match_token(T__129,&mut recog.err_handler)?;

					recog.base.set_state(1001);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LPAREN {
						{
						/*InvokeRule typeParams*/
						recog.base.set_state(1000);
						recog.typeParams()?;

						}
					}

					}
				}

			 T__130 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(1003);
					recog.base.match_token(T__130,&mut recog.err_handler)?;

					}
				}

			 T__131 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					recog.base.set_state(1004);
					recog.base.match_token(T__131,&mut recog.err_handler)?;

					}
				}

			 T__132 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					recog.base.set_state(1005);
					recog.base.match_token(T__132,&mut recog.err_handler)?;

					}
				}

			 T__133 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					recog.base.set_state(1006);
					recog.base.match_token(T__133,&mut recog.err_handler)?;

					}
				}

			 T__134 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 10);
					recog.base.enter_outer_alt(None, 10);
					{
					recog.base.set_state(1007);
					recog.base.match_token(T__134,&mut recog.err_handler)?;

					}
				}

			 T__135 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 11);
					recog.base.enter_outer_alt(None, 11);
					{
					recog.base.set_state(1008);
					recog.base.match_token(T__135,&mut recog.err_handler)?;

					}
				}

			 T__136 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 12);
					recog.base.enter_outer_alt(None, 12);
					{
					recog.base.set_state(1009);
					recog.base.match_token(T__136,&mut recog.err_handler)?;

					}
				}

			 T__86 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 13);
					recog.base.enter_outer_alt(None, 13);
					{
					recog.base.set_state(1010);
					recog.base.match_token(T__86,&mut recog.err_handler)?;

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
//------------------- typeParams ----------------
pub type TypeParamsContextAll<'input> = TypeParamsContext<'input>;


pub type TypeParamsContext<'input> = BaseParserRuleContext<'input,TypeParamsContextExt<'input>>;

#[derive(Clone)]
pub struct TypeParamsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for TypeParamsContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for TypeParamsContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_typeParams(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_typeParams(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for TypeParamsContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_typeParams(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeParamsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeParams }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeParams }
}
antlr_rust::tid!{TypeParamsContextExt<'a>}

impl<'input> TypeParamsContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeParamsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeParamsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeParamsContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<TypeParamsContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token INTEGER in current rule
fn INTEGER_all(&self) -> Vec<Rc<TerminalNode<'input,SchemaDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token INTEGER, starting from 0.
/// Returns `None` if number of children corresponding to token INTEGER is less or equal than `i`.
fn INTEGER(&self, i: usize) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, i)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}

}

impl<'input> TypeParamsContextAttrs<'input> for TypeParamsContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeParams(&mut self,)
	-> Result<Rc<TypeParamsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeParamsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 192, RULE_typeParams);
        let mut _localctx: Rc<TypeParamsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1013);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			recog.base.set_state(1014);
			recog.base.match_token(INTEGER,&mut recog.err_handler)?;

			recog.base.set_state(1017);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==COMMA {
				{
				recog.base.set_state(1015);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				recog.base.set_state(1016);
				recog.base.match_token(INTEGER,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(1019);
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
//------------------- collectionTypeV2 ----------------
pub type CollectionTypeV2ContextAll<'input> = CollectionTypeV2Context<'input>;


pub type CollectionTypeV2Context<'input> = BaseParserRuleContext<'input,CollectionTypeV2ContextExt<'input>>;

#[derive(Clone)]
pub struct CollectionTypeV2ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for CollectionTypeV2Context<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for CollectionTypeV2Context<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_collectionTypeV2(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_collectionTypeV2(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for CollectionTypeV2Context<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_collectionTypeV2(self);
	}
}

impl<'input> CustomRuleContext<'input> for CollectionTypeV2ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_collectionTypeV2 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_collectionTypeV2 }
}
antlr_rust::tid!{CollectionTypeV2ContextExt<'a>}

impl<'input> CollectionTypeV2ContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CollectionTypeV2ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CollectionTypeV2ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CollectionTypeV2ContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<CollectionTypeV2ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn fieldTypeV2_all(&self) ->  Vec<Rc<FieldTypeV2ContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fieldTypeV2(&self, i: usize) -> Option<Rc<FieldTypeV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}

}

impl<'input> CollectionTypeV2ContextAttrs<'input> for CollectionTypeV2Context<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn collectionTypeV2(&mut self,)
	-> Result<Rc<CollectionTypeV2ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CollectionTypeV2ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 194, RULE_collectionTypeV2);
        let mut _localctx: Rc<CollectionTypeV2ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1038);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__94 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1021);
					recog.base.match_token(T__94,&mut recog.err_handler)?;

					recog.base.set_state(1022);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule fieldTypeV2*/
					recog.base.set_state(1023);
					recog.fieldTypeV2()?;

					recog.base.set_state(1024);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}

			 T__137 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1026);
					recog.base.match_token(T__137,&mut recog.err_handler)?;

					recog.base.set_state(1027);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule fieldTypeV2*/
					recog.base.set_state(1028);
					recog.fieldTypeV2()?;

					recog.base.set_state(1029);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}

			 T__138 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(1031);
					recog.base.match_token(T__138,&mut recog.err_handler)?;

					recog.base.set_state(1032);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule fieldTypeV2*/
					recog.base.set_state(1033);
					recog.fieldTypeV2()?;

					recog.base.set_state(1034);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule fieldTypeV2*/
					recog.base.set_state(1035);
					recog.fieldTypeV2()?;

					recog.base.set_state(1036);
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
//------------------- inlineObjectTypeV2 ----------------
pub type InlineObjectTypeV2ContextAll<'input> = InlineObjectTypeV2Context<'input>;


pub type InlineObjectTypeV2Context<'input> = BaseParserRuleContext<'input,InlineObjectTypeV2ContextExt<'input>>;

#[derive(Clone)]
pub struct InlineObjectTypeV2ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for InlineObjectTypeV2Context<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for InlineObjectTypeV2Context<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_inlineObjectTypeV2(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_inlineObjectTypeV2(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for InlineObjectTypeV2Context<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_inlineObjectTypeV2(self);
	}
}

impl<'input> CustomRuleContext<'input> for InlineObjectTypeV2ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineObjectTypeV2 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineObjectTypeV2 }
}
antlr_rust::tid!{InlineObjectTypeV2ContextExt<'a>}

impl<'input> InlineObjectTypeV2ContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InlineObjectTypeV2ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InlineObjectTypeV2ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InlineObjectTypeV2ContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<InlineObjectTypeV2ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn inlineFieldDeclV2_all(&self) ->  Vec<Rc<InlineFieldDeclV2ContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn inlineFieldDeclV2(&self, i: usize) -> Option<Rc<InlineFieldDeclV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> InlineObjectTypeV2ContextAttrs<'input> for InlineObjectTypeV2Context<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inlineObjectTypeV2(&mut self,)
	-> Result<Rc<InlineObjectTypeV2ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InlineObjectTypeV2ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 196, RULE_inlineObjectTypeV2);
        let mut _localctx: Rc<InlineObjectTypeV2ContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1040);
			recog.base.match_token(T__93,&mut recog.err_handler)?;

			recog.base.set_state(1041);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(1045);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 53)) & !0x3f) == 0 && ((1usize << (_la - 53)) & ((1usize << (T__52 - 53)) | (1usize << (T__53 - 53)) | (1usize << (T__54 - 53)))) != 0) || ((((_la - 107)) & !0x3f) == 0 && ((1usize << (_la - 107)) & ((1usize << (T__106 - 107)) | (1usize << (T__107 - 107)) | (1usize << (T__108 - 107)))) != 0) || _la==IDENTIFIER {
				{
				{
				/*InvokeRule inlineFieldDeclV2*/
				recog.base.set_state(1042);
				recog.inlineFieldDeclV2()?;

				}
				}
				recog.base.set_state(1047);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1048);
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
//------------------- inlineFieldDeclV2 ----------------
pub type InlineFieldDeclV2ContextAll<'input> = InlineFieldDeclV2Context<'input>;


pub type InlineFieldDeclV2Context<'input> = BaseParserRuleContext<'input,InlineFieldDeclV2ContextExt<'input>>;

#[derive(Clone)]
pub struct InlineFieldDeclV2ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for InlineFieldDeclV2Context<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for InlineFieldDeclV2Context<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_inlineFieldDeclV2(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_inlineFieldDeclV2(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for InlineFieldDeclV2Context<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_inlineFieldDeclV2(self);
	}
}

impl<'input> CustomRuleContext<'input> for InlineFieldDeclV2ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineFieldDeclV2 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineFieldDeclV2 }
}
antlr_rust::tid!{InlineFieldDeclV2ContextExt<'a>}

impl<'input> InlineFieldDeclV2ContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InlineFieldDeclV2ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InlineFieldDeclV2ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InlineFieldDeclV2ContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<InlineFieldDeclV2ContextExt<'input>>{

fn fieldName(&self) -> Option<Rc<FieldNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldTypeV2(&self) -> Option<Rc<FieldTypeV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldQualifierV2_all(&self) ->  Vec<Rc<FieldQualifierV2ContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fieldQualifierV2(&self, i: usize) -> Option<Rc<FieldQualifierV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> InlineFieldDeclV2ContextAttrs<'input> for InlineFieldDeclV2Context<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inlineFieldDeclV2(&mut self,)
	-> Result<Rc<InlineFieldDeclV2ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InlineFieldDeclV2ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 198, RULE_inlineFieldDeclV2);
        let mut _localctx: Rc<InlineFieldDeclV2ContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fieldName*/
			recog.base.set_state(1050);
			recog.fieldName()?;

			/*InvokeRule fieldTypeV2*/
			recog.base.set_state(1051);
			recog.fieldTypeV2()?;

			recog.base.set_state(1055);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__36 || ((((_la - 115)) & !0x3f) == 0 && ((1usize << (_la - 115)) & ((1usize << (T__114 - 115)) | (1usize << (T__139 - 115)) | (1usize << (T__140 - 115)) | (1usize << (T__141 - 115)) | (1usize << (T__142 - 115)) | (1usize << (T__143 - 115)) | (1usize << (T__144 - 115)))) != 0) || _la==PII {
				{
				{
				/*InvokeRule fieldQualifierV2*/
				recog.base.set_state(1052);
				recog.fieldQualifierV2()?;

				}
				}
				recog.base.set_state(1057);
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
//------------------- fieldQualifierV2 ----------------
pub type FieldQualifierV2ContextAll<'input> = FieldQualifierV2Context<'input>;


pub type FieldQualifierV2Context<'input> = BaseParserRuleContext<'input,FieldQualifierV2ContextExt<'input>>;

#[derive(Clone)]
pub struct FieldQualifierV2ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for FieldQualifierV2Context<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for FieldQualifierV2Context<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fieldQualifierV2(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_fieldQualifierV2(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for FieldQualifierV2Context<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_fieldQualifierV2(self);
	}
}

impl<'input> CustomRuleContext<'input> for FieldQualifierV2ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fieldQualifierV2 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fieldQualifierV2 }
}
antlr_rust::tid!{FieldQualifierV2ContextExt<'a>}

impl<'input> FieldQualifierV2ContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FieldQualifierV2ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FieldQualifierV2ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FieldQualifierV2ContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<FieldQualifierV2ContextExt<'input>>{

fn piiModifier(&self) -> Option<Rc<PiiModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn defaultClauseV2(&self) -> Option<Rc<DefaultClauseV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn deprecatedClauseV2(&self) -> Option<Rc<DeprecatedClauseV2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FieldQualifierV2ContextAttrs<'input> for FieldQualifierV2Context<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fieldQualifierV2(&mut self,)
	-> Result<Rc<FieldQualifierV2ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FieldQualifierV2ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 200, RULE_fieldQualifierV2);
        let mut _localctx: Rc<FieldQualifierV2ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1066);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__139 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1058);
					recog.base.match_token(T__139,&mut recog.err_handler)?;

					}
				}

			 T__140 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1059);
					recog.base.match_token(T__140,&mut recog.err_handler)?;

					}
				}

			 T__141 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(1060);
					recog.base.match_token(T__141,&mut recog.err_handler)?;

					}
				}

			 T__142 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(1061);
					recog.base.match_token(T__142,&mut recog.err_handler)?;

					}
				}

			 T__143 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(1062);
					recog.base.match_token(T__143,&mut recog.err_handler)?;

					}
				}

			 PII 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule piiModifier*/
					recog.base.set_state(1063);
					recog.piiModifier()?;

					}
				}

			 T__114 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule defaultClauseV2*/
					recog.base.set_state(1064);
					recog.defaultClauseV2()?;

					}
				}

			 T__36 | T__144 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule deprecatedClauseV2*/
					recog.base.set_state(1065);
					recog.deprecatedClauseV2()?;

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
//------------------- piiModifier ----------------
pub type PiiModifierContextAll<'input> = PiiModifierContext<'input>;


pub type PiiModifierContext<'input> = BaseParserRuleContext<'input,PiiModifierContextExt<'input>>;

#[derive(Clone)]
pub struct PiiModifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for PiiModifierContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for PiiModifierContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_piiModifier(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_piiModifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for PiiModifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_piiModifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for PiiModifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_piiModifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_piiModifier }
}
antlr_rust::tid!{PiiModifierContextExt<'a>}

impl<'input> PiiModifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PiiModifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PiiModifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PiiModifierContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<PiiModifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PII
/// Returns `None` if there is no child corresponding to token PII
fn PII(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(PII, 0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> PiiModifierContextAttrs<'input> for PiiModifierContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn piiModifier(&mut self,)
	-> Result<Rc<PiiModifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PiiModifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 202, RULE_piiModifier);
        let mut _localctx: Rc<PiiModifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1072);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(106,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1068);
					recog.base.match_token(PII,&mut recog.err_handler)?;

					recog.base.set_state(1069);
					recog.base.match_token(DOT,&mut recog.err_handler)?;

					recog.base.set_state(1070);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1071);
					recog.base.match_token(PII,&mut recog.err_handler)?;

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
//------------------- defaultClauseV2 ----------------
pub type DefaultClauseV2ContextAll<'input> = DefaultClauseV2Context<'input>;


pub type DefaultClauseV2Context<'input> = BaseParserRuleContext<'input,DefaultClauseV2ContextExt<'input>>;

#[derive(Clone)]
pub struct DefaultClauseV2ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for DefaultClauseV2Context<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for DefaultClauseV2Context<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_defaultClauseV2(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_defaultClauseV2(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for DefaultClauseV2Context<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_defaultClauseV2(self);
	}
}

impl<'input> CustomRuleContext<'input> for DefaultClauseV2ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_defaultClauseV2 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_defaultClauseV2 }
}
antlr_rust::tid!{DefaultClauseV2ContextExt<'a>}

impl<'input> DefaultClauseV2ContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DefaultClauseV2ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DefaultClauseV2ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DefaultClauseV2ContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<DefaultClauseV2ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functionCall(&self) -> Option<Rc<FunctionCallContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DefaultClauseV2ContextAttrs<'input> for DefaultClauseV2Context<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn defaultClauseV2(&mut self,)
	-> Result<Rc<DefaultClauseV2ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DefaultClauseV2ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 204, RULE_defaultClauseV2);
        let mut _localctx: Rc<DefaultClauseV2ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1074);
			recog.base.match_token(T__114,&mut recog.err_handler)?;

			recog.base.set_state(1075);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			recog.base.set_state(1078);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__166 | INTEGER | DECIMAL | BOOLEAN | STRING | MINUS 
				=> {
					{
					/*InvokeRule literal*/
					recog.base.set_state(1076);
					recog.literal()?;

					}
				}

			 IDENTIFIER 
				=> {
					{
					/*InvokeRule functionCall*/
					recog.base.set_state(1077);
					recog.functionCall()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(1080);
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
//------------------- deprecatedClauseV2 ----------------
pub type DeprecatedClauseV2ContextAll<'input> = DeprecatedClauseV2Context<'input>;


pub type DeprecatedClauseV2Context<'input> = BaseParserRuleContext<'input,DeprecatedClauseV2ContextExt<'input>>;

#[derive(Clone)]
pub struct DeprecatedClauseV2ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for DeprecatedClauseV2Context<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for DeprecatedClauseV2Context<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_deprecatedClauseV2(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_deprecatedClauseV2(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for DeprecatedClauseV2Context<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_deprecatedClauseV2(self);
	}
}

impl<'input> CustomRuleContext<'input> for DeprecatedClauseV2ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_deprecatedClauseV2 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_deprecatedClauseV2 }
}
antlr_rust::tid!{DeprecatedClauseV2ContextExt<'a>}

impl<'input> DeprecatedClauseV2ContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DeprecatedClauseV2ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeprecatedClauseV2ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DeprecatedClauseV2ContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<DeprecatedClauseV2ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token VERSION_NUMBER
/// Returns `None` if there is no child corresponding to token VERSION_NUMBER
fn VERSION_NUMBER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(VERSION_NUMBER, 0)
}

}

impl<'input> DeprecatedClauseV2ContextAttrs<'input> for DeprecatedClauseV2Context<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn deprecatedClauseV2(&mut self,)
	-> Result<Rc<DeprecatedClauseV2ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeprecatedClauseV2ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 206, RULE_deprecatedClauseV2);
        let mut _localctx: Rc<DeprecatedClauseV2ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1090);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__36 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1082);
					recog.base.match_token(T__36,&mut recog.err_handler)?;

					recog.base.set_state(1083);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					recog.base.set_state(1084);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					recog.base.set_state(1085);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}

			 T__144 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1086);
					recog.base.match_token(T__144,&mut recog.err_handler)?;

					recog.base.set_state(1087);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					recog.base.set_state(1088);
					recog.base.match_token(VERSION_NUMBER,&mut recog.err_handler)?;

					recog.base.set_state(1089);
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
//------------------- expression ----------------
pub type ExpressionContextAll<'input> = ExpressionContext<'input>;


pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for ExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expression(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for ExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::tid!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{

fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldPath(&self) -> Option<Rc<FieldPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functionCall(&self) -> Option<Rc<FunctionCallContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn timeUnit(&self) -> Option<Rc<TimeUnitContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn whenExpression(&self) -> Option<Rc<WhenExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn operator(&self) -> Option<Rc<OperatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
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
		recog.base.enter_recursion_rule(_localctx.clone(), 208, RULE_expression, _p);
	    let mut _localctx: Rc<ExpressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 208;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1102);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(109,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule literal*/
					recog.base.set_state(1093);
					recog.literal()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule fieldPath*/
					recog.base.set_state(1094);
					recog.fieldPath()?;

					}
				}
			,
				3 =>{
					{
					/*InvokeRule functionCall*/
					recog.base.set_state(1095);
					recog.functionCall()?;

					}
				}
			,
				4 =>{
					{
					/*InvokeRule timeUnit*/
					recog.base.set_state(1096);
					recog.timeUnit()?;

					}
				}
			,
				5 =>{
					{
					recog.base.set_state(1097);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(1098);
					recog.expression_rec(0)?;

					recog.base.set_state(1099);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					{
					/*InvokeRule whenExpression*/
					recog.base.set_state(1101);
					recog.whenExpression()?;

					}
				}

				_ => {}
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(1110);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(110,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
					_localctx = tmp;
					recog.base.set_state(1104);
					if !({recog.precpred(None, 3)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
					}
					/*InvokeRule operator*/
					recog.base.set_state(1105);
					recog.operator()?;

					/*InvokeRule expression*/
					recog.base.set_state(1106);
					recog.expression_rec(4)?;

					}
					} 
				}
				recog.base.set_state(1112);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(110,&mut recog.base)?;
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
//------------------- whenExpression ----------------
pub type WhenExpressionContextAll<'input> = WhenExpressionContext<'input>;


pub type WhenExpressionContext<'input> = BaseParserRuleContext<'input,WhenExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct WhenExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for WhenExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for WhenExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_whenExpression(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_whenExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for WhenExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_whenExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for WhenExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_whenExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_whenExpression }
}
antlr_rust::tid!{WhenExpressionContextExt<'a>}

impl<'input> WhenExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<WhenExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,WhenExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait WhenExpressionContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<WhenExpressionContextExt<'input>>{

fn condition_all(&self) ->  Vec<Rc<ConditionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn condition(&self, i: usize) -> Option<Rc<ConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COLON in current rule
fn COLON_all(&self) -> Vec<Rc<TerminalNode<'input,SchemaDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COLON, starting from 0.
/// Returns `None` if number of children corresponding to token COLON is less or equal than `i`.
fn COLON(&self, i: usize) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(COLON, i)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> WhenExpressionContextAttrs<'input> for WhenExpressionContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn whenExpression(&mut self,)
	-> Result<Rc<WhenExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = WhenExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 210, RULE_whenExpression);
        let mut _localctx: Rc<WhenExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1113);
			recog.base.match_token(T__100,&mut recog.err_handler)?;

			/*InvokeRule condition*/
			recog.base.set_state(1114);
			recog.condition()?;

			recog.base.set_state(1115);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1116);
			recog.expression_rec(0)?;

			recog.base.set_state(1124);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__100 {
				{
				{
				recog.base.set_state(1117);
				recog.base.match_token(T__100,&mut recog.err_handler)?;

				/*InvokeRule condition*/
				recog.base.set_state(1118);
				recog.condition()?;

				recog.base.set_state(1119);
				recog.base.match_token(COLON,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(1120);
				recog.expression_rec(0)?;

				}
				}
				recog.base.set_state(1126);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1127);
			recog.base.match_token(T__145,&mut recog.err_handler)?;

			recog.base.set_state(1128);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1129);
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
//------------------- condition ----------------
pub type ConditionContextAll<'input> = ConditionContext<'input>;


pub type ConditionContext<'input> = BaseParserRuleContext<'input,ConditionContextExt<'input>>;

#[derive(Clone)]
pub struct ConditionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for ConditionContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for ConditionContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_condition(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_condition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for ConditionContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_condition(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConditionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_condition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_condition }
}
antlr_rust::tid!{ConditionContextExt<'a>}

impl<'input> ConditionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConditionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConditionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConditionContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<ConditionContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn comparisonOp(&self) -> Option<Rc<ComparisonOpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn condition(&self) -> Option<Rc<ConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> ConditionContextAttrs<'input> for ConditionContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn condition(&mut self,)
	-> Result<Rc<ConditionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 212, RULE_condition);
        let mut _localctx: Rc<ConditionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1147);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(112,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule expression*/
					recog.base.set_state(1131);
					recog.expression_rec(0)?;

					/*InvokeRule comparisonOp*/
					recog.base.set_state(1132);
					recog.comparisonOp()?;

					/*InvokeRule expression*/
					recog.base.set_state(1133);
					recog.expression_rec(0)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule expression*/
					recog.base.set_state(1135);
					recog.expression_rec(0)?;

					recog.base.set_state(1136);
					recog.base.match_token(T__97,&mut recog.err_handler)?;

					/*InvokeRule condition*/
					recog.base.set_state(1137);
					recog.condition()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule expression*/
					recog.base.set_state(1139);
					recog.expression_rec(0)?;

					recog.base.set_state(1140);
					recog.base.match_token(T__98,&mut recog.err_handler)?;

					/*InvokeRule condition*/
					recog.base.set_state(1141);
					recog.condition()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(1143);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule condition*/
					recog.base.set_state(1144);
					recog.condition()?;

					recog.base.set_state(1145);
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
//------------------- comparisonOp ----------------
pub type ComparisonOpContextAll<'input> = ComparisonOpContext<'input>;


pub type ComparisonOpContext<'input> = BaseParserRuleContext<'input,ComparisonOpContextExt<'input>>;

#[derive(Clone)]
pub struct ComparisonOpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for ComparisonOpContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for ComparisonOpContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_comparisonOp(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_comparisonOp(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for ComparisonOpContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_comparisonOp(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComparisonOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comparisonOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comparisonOp }
}
antlr_rust::tid!{ComparisonOpContextExt<'a>}

impl<'input> ComparisonOpContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComparisonOpContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComparisonOpContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComparisonOpContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<ComparisonOpContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EQ
/// Returns `None` if there is no child corresponding to token EQ
fn EQ(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(EQ, 0)
}
/// Retrieves first TerminalNode corresponding to token NE
/// Returns `None` if there is no child corresponding to token NE
fn NE(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(NE, 0)
}
/// Retrieves first TerminalNode corresponding to token LANGLE
/// Returns `None` if there is no child corresponding to token LANGLE
fn LANGLE(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(LANGLE, 0)
}
/// Retrieves first TerminalNode corresponding to token RANGLE
/// Returns `None` if there is no child corresponding to token RANGLE
fn RANGLE(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(RANGLE, 0)
}
/// Retrieves first TerminalNode corresponding to token LE
/// Returns `None` if there is no child corresponding to token LE
fn LE(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(LE, 0)
}
/// Retrieves first TerminalNode corresponding to token GE
/// Returns `None` if there is no child corresponding to token GE
fn GE(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(GE, 0)
}

}

impl<'input> ComparisonOpContextAttrs<'input> for ComparisonOpContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn comparisonOp(&mut self,)
	-> Result<Rc<ComparisonOpContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComparisonOpContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 214, RULE_comparisonOp);
        let mut _localctx: Rc<ComparisonOpContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1149);
			_la = recog.base.input.la(1);
			if { !(((((_la - 187)) & !0x3f) == 0 && ((1usize << (_la - 187)) & ((1usize << (LANGLE - 187)) | (1usize << (RANGLE - 187)) | (1usize << (EQ - 187)) | (1usize << (NE - 187)) | (1usize << (LE - 187)) | (1usize << (GE - 187)))) != 0)) } {
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
//------------------- operator ----------------
pub type OperatorContextAll<'input> = OperatorContext<'input>;


pub type OperatorContext<'input> = BaseParserRuleContext<'input,OperatorContextExt<'input>>;

#[derive(Clone)]
pub struct OperatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for OperatorContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for OperatorContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_operator(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_operator(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for OperatorContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_operator(self);
	}
}

impl<'input> CustomRuleContext<'input> for OperatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_operator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_operator }
}
antlr_rust::tid!{OperatorContextExt<'a>}

impl<'input> OperatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OperatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OperatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OperatorContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<OperatorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PLUS
/// Returns `None` if there is no child corresponding to token PLUS
fn PLUS(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(PLUS, 0)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}
/// Retrieves first TerminalNode corresponding to token STAR
/// Returns `None` if there is no child corresponding to token STAR
fn STAR(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(STAR, 0)
}
/// Retrieves first TerminalNode corresponding to token SLASH
/// Returns `None` if there is no child corresponding to token SLASH
fn SLASH(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(SLASH, 0)
}

}

impl<'input> OperatorContextAttrs<'input> for OperatorContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn operator(&mut self,)
	-> Result<Rc<OperatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OperatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 216, RULE_operator);
        let mut _localctx: Rc<OperatorContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1151);
			_la = recog.base.input.la(1);
			if { !(((((_la - 193)) & !0x3f) == 0 && ((1usize << (_la - 193)) & ((1usize << (PLUS - 193)) | (1usize << (MINUS - 193)) | (1usize << (STAR - 193)) | (1usize << (SLASH - 193)))) != 0)) } {
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

impl<'input> SchemaDSLParserContext<'input> for FunctionCallContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for FunctionCallContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_functionCall(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_functionCall(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for FunctionCallContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_functionCall(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionCall }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionCall }
}
antlr_rust::tid!{FunctionCallContextExt<'a>}

impl<'input> FunctionCallContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionCallContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionCallContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionCallContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<FunctionCallContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,SchemaDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> FunctionCallContextAttrs<'input> for FunctionCallContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionCall(&mut self,)
	-> Result<Rc<FunctionCallContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionCallContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 218, RULE_functionCall);
        let mut _localctx: Rc<FunctionCallContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1153);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(1154);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			recog.base.set_state(1163);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__100 || ((((_la - 147)) & !0x3f) == 0 && ((1usize << (_la - 147)) & ((1usize << (T__146 - 147)) | (1usize << (T__147 - 147)) | (1usize << (T__148 - 147)) | (1usize << (T__149 - 147)) | (1usize << (T__150 - 147)) | (1usize << (T__151 - 147)) | (1usize << (T__152 - 147)) | (1usize << (T__153 - 147)) | (1usize << (T__154 - 147)) | (1usize << (T__155 - 147)) | (1usize << (T__156 - 147)) | (1usize << (T__157 - 147)) | (1usize << (T__158 - 147)) | (1usize << (T__159 - 147)) | (1usize << (T__160 - 147)) | (1usize << (T__161 - 147)) | (1usize << (T__166 - 147)) | (1usize << (INTEGER - 147)) | (1usize << (DECIMAL - 147)) | (1usize << (BOOLEAN - 147)) | (1usize << (IDENTIFIER - 147)) | (1usize << (STRING - 147)))) != 0) || _la==LPAREN || _la==MINUS {
				{
				/*InvokeRule expression*/
				recog.base.set_state(1155);
				recog.expression_rec(0)?;

				recog.base.set_state(1160);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==COMMA {
					{
					{
					recog.base.set_state(1156);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(1157);
					recog.expression_rec(0)?;

					}
					}
					recog.base.set_state(1162);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(1165);
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

impl<'input> SchemaDSLParserContext<'input> for FieldPathContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for FieldPathContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fieldPath(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_fieldPath(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for FieldPathContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_fieldPath(self);
	}
}

impl<'input> CustomRuleContext<'input> for FieldPathContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fieldPath }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fieldPath }
}
antlr_rust::tid!{FieldPathContextExt<'a>}

impl<'input> FieldPathContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FieldPathContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FieldPathContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FieldPathContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<FieldPathContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,SchemaDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input,SchemaDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
/// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(DOT, i)
}

}

impl<'input> FieldPathContextAttrs<'input> for FieldPathContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fieldPath(&mut self,)
	-> Result<Rc<FieldPathContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FieldPathContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 220, RULE_fieldPath);
        let mut _localctx: Rc<FieldPathContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1167);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(1172);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(115,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(1168);
					recog.base.match_token(DOT,&mut recog.err_handler)?;

					recog.base.set_state(1169);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(1174);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(115,&mut recog.base)?;
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
//------------------- fieldList ----------------
pub type FieldListContextAll<'input> = FieldListContext<'input>;


pub type FieldListContext<'input> = BaseParserRuleContext<'input,FieldListContextExt<'input>>;

#[derive(Clone)]
pub struct FieldListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for FieldListContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for FieldListContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fieldList(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_fieldList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for FieldListContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_fieldList(self);
	}
}

impl<'input> CustomRuleContext<'input> for FieldListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fieldList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fieldList }
}
antlr_rust::tid!{FieldListContextExt<'a>}

impl<'input> FieldListContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FieldListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FieldListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FieldListContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<FieldListContextExt<'input>>{

fn fieldPath_all(&self) ->  Vec<Rc<FieldPathContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fieldPath(&self, i: usize) -> Option<Rc<FieldPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,SchemaDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> FieldListContextAttrs<'input> for FieldListContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fieldList(&mut self,)
	-> Result<Rc<FieldListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FieldListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 222, RULE_fieldList);
        let mut _localctx: Rc<FieldListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fieldPath*/
			recog.base.set_state(1175);
			recog.fieldPath()?;

			recog.base.set_state(1180);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(1176);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule fieldPath*/
				recog.base.set_state(1177);
				recog.fieldPath()?;

				}
				}
				recog.base.set_state(1182);
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
//------------------- fieldArray ----------------
pub type FieldArrayContextAll<'input> = FieldArrayContext<'input>;


pub type FieldArrayContext<'input> = BaseParserRuleContext<'input,FieldArrayContextExt<'input>>;

#[derive(Clone)]
pub struct FieldArrayContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for FieldArrayContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for FieldArrayContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fieldArray(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_fieldArray(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for FieldArrayContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_fieldArray(self);
	}
}

impl<'input> CustomRuleContext<'input> for FieldArrayContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fieldArray }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fieldArray }
}
antlr_rust::tid!{FieldArrayContextExt<'a>}

impl<'input> FieldArrayContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FieldArrayContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FieldArrayContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FieldArrayContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<FieldArrayContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACKET
/// Returns `None` if there is no child corresponding to token LBRACKET
fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
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
fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(RBRACKET, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,SchemaDSLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> FieldArrayContextAttrs<'input> for FieldArrayContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fieldArray(&mut self,)
	-> Result<Rc<FieldArrayContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FieldArrayContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 224, RULE_fieldArray);
        let mut _localctx: Rc<FieldArrayContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1196);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(118,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1183);
					recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

					/*InvokeRule fieldPath*/
					recog.base.set_state(1184);
					recog.fieldPath()?;

					recog.base.set_state(1189);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==COMMA {
						{
						{
						recog.base.set_state(1185);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						/*InvokeRule fieldPath*/
						recog.base.set_state(1186);
						recog.fieldPath()?;

						}
						}
						recog.base.set_state(1191);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(1192);
					recog.base.match_token(RBRACKET,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1194);
					recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

					recog.base.set_state(1195);
					recog.base.match_token(RBRACKET,&mut recog.err_handler)?;

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
//------------------- duration ----------------
pub type DurationContextAll<'input> = DurationContext<'input>;


pub type DurationContext<'input> = BaseParserRuleContext<'input,DurationContextExt<'input>>;

#[derive(Clone)]
pub struct DurationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for DurationContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for DurationContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_duration(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_duration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for DurationContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_duration(self);
	}
}

impl<'input> CustomRuleContext<'input> for DurationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_duration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_duration }
}
antlr_rust::tid!{DurationContextExt<'a>}

impl<'input> DurationContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DurationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DurationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DurationContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<DurationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
fn timeUnit(&self) -> Option<Rc<TimeUnitContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DURATION_LITERAL
/// Returns `None` if there is no child corresponding to token DURATION_LITERAL
fn DURATION_LITERAL(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(DURATION_LITERAL, 0)
}

}

impl<'input> DurationContextAttrs<'input> for DurationContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn duration(&mut self,)
	-> Result<Rc<DurationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DurationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 226, RULE_duration);
        let mut _localctx: Rc<DurationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1201);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 INTEGER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1198);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					/*InvokeRule timeUnit*/
					recog.base.set_state(1199);
					recog.timeUnit()?;

					}
				}

			 DURATION_LITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1200);
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

impl<'input> SchemaDSLParserContext<'input> for TimeUnitContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for TimeUnitContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_timeUnit(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_timeUnit(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for TimeUnitContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_timeUnit(self);
	}
}

impl<'input> CustomRuleContext<'input> for TimeUnitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_timeUnit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_timeUnit }
}
antlr_rust::tid!{TimeUnitContextExt<'a>}

impl<'input> TimeUnitContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TimeUnitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TimeUnitContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TimeUnitContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<TimeUnitContextExt<'input>>{


}

impl<'input> TimeUnitContextAttrs<'input> for TimeUnitContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn timeUnit(&mut self,)
	-> Result<Rc<TimeUnitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TimeUnitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 228, RULE_timeUnit);
        let mut _localctx: Rc<TimeUnitContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1203);
			_la = recog.base.input.la(1);
			if { !(((((_la - 147)) & !0x3f) == 0 && ((1usize << (_la - 147)) & ((1usize << (T__146 - 147)) | (1usize << (T__147 - 147)) | (1usize << (T__148 - 147)) | (1usize << (T__149 - 147)) | (1usize << (T__150 - 147)) | (1usize << (T__151 - 147)) | (1usize << (T__152 - 147)) | (1usize << (T__153 - 147)) | (1usize << (T__154 - 147)) | (1usize << (T__155 - 147)) | (1usize << (T__156 - 147)) | (1usize << (T__157 - 147)) | (1usize << (T__158 - 147)) | (1usize << (T__159 - 147)) | (1usize << (T__160 - 147)) | (1usize << (T__161 - 147)))) != 0)) } {
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
//------------------- sizeSpec ----------------
pub type SizeSpecContextAll<'input> = SizeSpecContext<'input>;


pub type SizeSpecContext<'input> = BaseParserRuleContext<'input,SizeSpecContextExt<'input>>;

#[derive(Clone)]
pub struct SizeSpecContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for SizeSpecContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for SizeSpecContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_sizeSpec(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_sizeSpec(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for SizeSpecContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_sizeSpec(self);
	}
}

impl<'input> CustomRuleContext<'input> for SizeSpecContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sizeSpec }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sizeSpec }
}
antlr_rust::tid!{SizeSpecContextExt<'a>}

impl<'input> SizeSpecContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SizeSpecContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SizeSpecContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SizeSpecContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<SizeSpecContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
fn sizeUnit(&self) -> Option<Rc<SizeUnitContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SizeSpecContextAttrs<'input> for SizeSpecContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn sizeSpec(&mut self,)
	-> Result<Rc<SizeSpecContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SizeSpecContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 230, RULE_sizeSpec);
        let mut _localctx: Rc<SizeSpecContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1205);
			recog.base.match_token(INTEGER,&mut recog.err_handler)?;

			/*InvokeRule sizeUnit*/
			recog.base.set_state(1206);
			recog.sizeUnit()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- sizeUnit ----------------
pub type SizeUnitContextAll<'input> = SizeUnitContext<'input>;


pub type SizeUnitContext<'input> = BaseParserRuleContext<'input,SizeUnitContextExt<'input>>;

#[derive(Clone)]
pub struct SizeUnitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SchemaDSLParserContext<'input> for SizeUnitContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for SizeUnitContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_sizeUnit(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_sizeUnit(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for SizeUnitContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_sizeUnit(self);
	}
}

impl<'input> CustomRuleContext<'input> for SizeUnitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sizeUnit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sizeUnit }
}
antlr_rust::tid!{SizeUnitContextExt<'a>}

impl<'input> SizeUnitContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SizeUnitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SizeUnitContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SizeUnitContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<SizeUnitContextExt<'input>>{


}

impl<'input> SizeUnitContextAttrs<'input> for SizeUnitContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn sizeUnit(&mut self,)
	-> Result<Rc<SizeUnitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SizeUnitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 232, RULE_sizeUnit);
        let mut _localctx: Rc<SizeUnitContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1208);
			_la = recog.base.input.la(1);
			if { !(((((_la - 136)) & !0x3f) == 0 && ((1usize << (_la - 136)) & ((1usize << (T__135 - 136)) | (1usize << (T__162 - 136)) | (1usize << (T__163 - 136)) | (1usize << (T__164 - 136)) | (1usize << (T__165 - 136)))) != 0)) } {
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

impl<'input> SchemaDSLParserContext<'input> for LiteralContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for LiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_literal(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_literal(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for LiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_literal(self);
	}
}

impl<'input> CustomRuleContext<'input> for LiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literal }
}
antlr_rust::tid!{LiteralContextExt<'a>}

impl<'input> LiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LiteralContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<LiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
fn numberLiteral(&self) -> Option<Rc<NumberLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token BOOLEAN
/// Returns `None` if there is no child corresponding to token BOOLEAN
fn BOOLEAN(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(BOOLEAN, 0)
}

}

impl<'input> LiteralContextAttrs<'input> for LiteralContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn literal(&mut self,)
	-> Result<Rc<LiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 234, RULE_literal);
        let mut _localctx: Rc<LiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1214);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 STRING 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1210);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}

			 INTEGER | DECIMAL | MINUS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule numberLiteral*/
					recog.base.set_state(1211);
					recog.numberLiteral()?;

					}
				}

			 BOOLEAN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(1212);
					recog.base.match_token(BOOLEAN,&mut recog.err_handler)?;

					}
				}

			 T__166 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(1213);
					recog.base.match_token(T__166,&mut recog.err_handler)?;

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

impl<'input> SchemaDSLParserContext<'input> for NumberLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn SchemaDSLListener<'input> + 'a> for NumberLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_numberLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn SchemaDSLListener<'input> + 'a)) {
			listener.exit_numberLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SchemaDSLVisitor<'input> + 'a> for NumberLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn SchemaDSLVisitor<'input> + 'a)) {
		visitor.visit_numberLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumberLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SchemaDSLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_numberLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_numberLiteral }
}
antlr_rust::tid!{NumberLiteralContextExt<'a>}

impl<'input> NumberLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn SchemaDSLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NumberLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NumberLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NumberLiteralContextAttrs<'input>: SchemaDSLParserContext<'input> + BorrowMut<NumberLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
/// Retrieves first TerminalNode corresponding to token DECIMAL
/// Returns `None` if there is no child corresponding to token DECIMAL
fn DECIMAL(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(DECIMAL, 0)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,SchemaDSLParserContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}

}

impl<'input> NumberLiteralContextAttrs<'input> for NumberLiteralContext<'input>{}

impl<'input, I, H> SchemaDSLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn numberLiteral(&mut self,)
	-> Result<Rc<NumberLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NumberLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 236, RULE_numberLiteral);
        let mut _localctx: Rc<NumberLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1222);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(121,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1216);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1217);
					recog.base.match_token(DECIMAL,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(1218);
					recog.base.match_token(MINUS,&mut recog.err_handler)?;

					recog.base.set_state(1219);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(1220);
					recog.base.match_token(MINUS,&mut recog.err_handler)?;

					recog.base.set_state(1221);
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
	\u{cb}\u{4cb}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
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
	\x76\x09\x76\x04\x77\x09\x77\x04\x78\x09\x78\x03\x02\x07\x02\u{f2}\x0a\x02\
	\x0c\x02\x0e\x02\u{f5}\x0b\x02\x03\x02\x03\x02\x06\x02\u{f9}\x0a\x02\x0d\
	\x02\x0e\x02\u{fa}\x03\x02\x03\x02\x03\x03\x03\x03\x03\x03\x03\x04\x06\x04\
	\u{103}\x0a\x04\x0d\x04\x0e\x04\u{104}\x03\x04\x03\x04\x03\x05\x03\x05\x03\
	\x06\x03\x06\x03\x06\x03\x07\x03\x07\x03\x07\x05\x07\u{111}\x0a\x07\x03\
	\x07\x05\x07\u{114}\x0a\x07\x03\x07\x05\x07\u{117}\x0a\x07\x03\x07\x05\x07\
	\u{11a}\x0a\x07\x03\x07\x05\x07\u{11d}\x0a\x07\x03\x07\x05\x07\u{120}\x0a\
	\x07\x03\x07\x05\x07\u{123}\x0a\x07\x03\x07\x05\x07\u{126}\x0a\x07\x03\x07\
	\x05\x07\u{129}\x0a\x07\x03\x07\x07\x07\u{12c}\x0a\x07\x0c\x07\x0e\x07\u{12f}\
	\x0b\x07\x03\x07\x05\x07\u{132}\x0a\x07\x03\x07\x05\x07\u{135}\x0a\x07\x03\
	\x07\x05\x07\u{138}\x0a\x07\x03\x07\x05\x07\u{13b}\x0a\x07\x03\x07\x05\x07\
	\u{13e}\x0a\x07\x03\x07\x05\x07\u{141}\x0a\x07\x03\x07\x07\x07\u{144}\x0a\
	\x07\x0c\x07\x0e\x07\u{147}\x0b\x07\x03\x07\x05\x07\u{14a}\x0a\x07\x03\x07\
	\x03\x07\x03\x08\x03\x08\x03\x08\x05\x08\u{151}\x0a\x08\x03\x09\x03\x09\
	\x03\x09\x03\x09\x07\x09\u{157}\x0a\x09\x0c\x09\x0e\x09\u{15a}\x0b\x09\x03\
	\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0c\x07\x0c\u{164}\
	\x0a\x0c\x0c\x0c\x0e\x0c\u{167}\x0b\x0c\x03\x0d\x03\x0d\x03\x0e\x03\x0e\
	\x03\x0e\x05\x0e\u{16e}\x0a\x0e\x03\x0e\x05\x0e\u{171}\x0a\x0e\x03\x0e\x05\
	\x0e\u{174}\x0a\x0e\x03\x0e\x05\x0e\u{177}\x0a\x0e\x03\x0f\x03\x0f\x03\x0f\
	\x03\x10\x03\x10\x03\x11\x03\x11\x03\x11\x03\x12\x03\x12\x03\x12\x03\x12\
	\x03\x12\x03\x12\x05\x12\u{187}\x0a\x12\x03\x12\x03\x12\x03\x12\x05\x12\
	\u{18c}\x0a\x12\x03\x13\x03\x13\x03\x13\x03\x13\x03\x14\x03\x14\x03\x14\
	\x03\x15\x03\x15\x03\x15\x03\x16\x03\x16\x06\x16\u{19a}\x0a\x16\x0d\x16\
	\x0e\x16\u{19b}\x03\x16\x03\x16\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\
	\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\
	\x03\x17\x03\x17\x05\x17\u{1b0}\x0a\x17\x03\x18\x03\x18\x03\x18\x03\x18\
	\x03\x18\x07\x18\u{1b7}\x0a\x18\x0c\x18\x0e\x18\u{1ba}\x0b\x18\x03\x18\x03\
	\x18\x03\x19\x03\x19\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\
	\x1a\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1c\x03\x1c\x03\x1c\x03\
	\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x05\x1c\u{1d6}\x0a\
	\x1c\x03\x1d\x03\x1d\x06\x1d\u{1da}\x0a\x1d\x0d\x1d\x0e\x1d\u{1db}\x03\x1d\
	\x03\x1d\x03\x1e\x03\x1e\x03\x1e\x07\x1e\u{1e3}\x0a\x1e\x0c\x1e\x0e\x1e\
	\u{1e6}\x0b\x1e\x03\x1e\x05\x1e\u{1e9}\x0a\x1e\x03\x1f\x03\x1f\x06\x1f\u{1ed}\
	\x0a\x1f\x0d\x1f\x0e\x1f\u{1ee}\x03\x1f\x03\x1f\x03\x20\x03\x20\x03\x20\
	\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x05\x20\u{1fc}\x0a\x20\
	\x03\x21\x03\x21\x03\x21\x03\x22\x03\x22\x03\x22\x05\x22\u{204}\x0a\x22\
	\x03\x23\x03\x23\x03\x23\x03\x24\x03\x24\x03\x25\x03\x25\x03\x25\x03\x25\
	\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x05\x25\u{215}\x0a\x25\
	\x03\x26\x03\x26\x03\x27\x03\x27\x03\x27\x03\x27\x05\x27\u{21d}\x0a\x27\
	\x03\x28\x03\x28\x03\x29\x03\x29\x03\x29\x03\x2a\x03\x2a\x03\x2a\x03\x2a\
	\x05\x2a\u{228}\x0a\x2a\x03\x2b\x03\x2b\x03\x2c\x03\x2c\x03\x2c\x03\x2c\
	\x03\x2d\x03\x2d\x05\x2d\u{232}\x0a\x2d\x03\x2d\x03\x2d\x05\x2d\u{236}\x0a\
	\x2d\x03\x2d\x03\x2d\x05\x2d\u{23a}\x0a\x2d\x03\x2e\x03\x2e\x03\x2e\x03\
	\x2e\x03\x2f\x03\x2f\x05\x2f\u{242}\x0a\x2f\x03\x2f\x03\x2f\x05\x2f\u{246}\
	\x0a\x2f\x03\x2f\x03\x2f\x05\x2f\u{24a}\x0a\x2f\x03\x30\x03\x30\x03\x31\
	\x03\x31\x06\x31\u{250}\x0a\x31\x0d\x31\x0e\x31\u{251}\x03\x31\x03\x31\x03\
	\x32\x03\x32\x03\x32\x03\x32\x05\x32\u{25a}\x0a\x32\x03\x33\x03\x33\x03\
	\x33\x03\x34\x03\x34\x03\x35\x03\x35\x03\x35\x03\x36\x03\x36\x03\x36\x03\
	\x37\x03\x37\x03\x37\x03\x38\x03\x38\x06\x38\u{26c}\x0a\x38\x0d\x38\x0e\
	\x38\u{26d}\x03\x38\x03\x38\x03\x39\x03\x39\x03\x39\x07\x39\u{275}\x0a\x39\
	\x0c\x39\x0e\x39\u{278}\x0b\x39\x03\x39\x05\x39\u{27b}\x0a\x39\x03\x3a\x03\
	\x3a\x03\x3a\x05\x3a\u{280}\x0a\x3a\x03\x3b\x03\x3b\x03\x3b\x07\x3b\u{285}\
	\x0a\x3b\x0c\x3b\x0e\x3b\u{288}\x0b\x3b\x03\x3b\x07\x3b\u{28b}\x0a\x3b\x0c\
	\x3b\x0e\x3b\u{28e}\x0b\x3b\x03\x3b\x03\x3b\x03\x3b\x03\x3b\x03\x3b\x03\
	\x3b\x03\x3b\x03\x3b\x07\x3b\u{298}\x0a\x3b\x0c\x3b\x0e\x3b\u{29b}\x0b\x3b\
	\x03\x3b\x07\x3b\u{29e}\x0a\x3b\x0c\x3b\x0e\x3b\u{2a1}\x0b\x3b\x03\x3b\x03\
	\x3b\x05\x3b\u{2a5}\x0a\x3b\x03\x3c\x03\x3c\x06\x3c\u{2a9}\x0a\x3c\x0d\x3c\
	\x0e\x3c\u{2aa}\x03\x3c\x03\x3c\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3e\
	\x03\x3e\x03\x3e\x03\x3e\x03\x3e\x03\x3e\x03\x3e\x03\x3e\x03\x3e\x03\x3e\
	\x03\x3e\x05\x3e\u{2be}\x0a\x3e\x03\x3e\x03\x3e\x03\x3e\x03\x3e\x03\x3e\
	\x03\x3e\x03\x3e\x03\x3e\x03\x3e\x03\x3e\x03\x3e\x03\x3e\x03\x3e\x03\x3e\
	\x03\x3e\x03\x3e\x07\x3e\u{2d0}\x0a\x3e\x0c\x3e\x0e\x3e\u{2d3}\x0b\x3e\x03\
	\x3f\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x07\
	\x3f\u{2de}\x0a\x3f\x0c\x3f\x0e\x3f\u{2e1}\x0b\x3f\x03\x3f\x03\x3f\x03\x3f\
	\x03\x40\x05\x40\u{2e7}\x0a\x40\x03\x40\x03\x40\x05\x40\u{2eb}\x0a\x40\x03\
	\x40\x05\x40\u{2ee}\x0a\x40\x03\x40\x05\x40\u{2f1}\x0a\x40\x03\x41\x03\x41\
	\x03\x41\x03\x42\x03\x42\x03\x42\x03\x43\x03\x43\x03\x43\x05\x43\u{2fc}\
	\x0a\x43\x03\x43\x05\x43\u{2ff}\x0a\x43\x03\x44\x03\x44\x03\x45\x06\x45\
	\u{304}\x0a\x45\x0d\x45\x0e\x45\u{305}\x03\x46\x03\x46\x03\x46\x05\x46\u{30b}\
	\x0a\x46\x03\x47\x03\x47\x03\x48\x03\x48\x03\x48\x03\x48\x07\x48\u{313}\
	\x0a\x48\x0c\x48\x0e\x48\u{316}\x0b\x48\x03\x48\x03\x48\x03\x49\x03\x49\
	\x03\x49\x06\x49\u{31d}\x0a\x49\x0d\x49\x0e\x49\u{31e}\x03\x49\x03\x49\x03\
	\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x05\
	\x4b\u{32c}\x0a\x4b\x03\x4c\x03\x4c\x06\x4c\u{330}\x0a\x4c\x0d\x4c\x0e\x4c\
	\u{331}\x03\x4c\x03\x4c\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4e\
	\x03\x4e\x03\x4e\x03\x4e\x03\x4e\x07\x4e\u{340}\x0a\x4e\x0c\x4e\x0e\x4e\
	\u{343}\x0b\x4e\x05\x4e\u{345}\x0a\x4e\x03\x4e\x03\x4e\x03\x4f\x03\x4f\x06\
	\x4f\u{34b}\x0a\x4f\x0d\x4f\x0e\x4f\u{34c}\x03\x4f\x03\x4f\x03\x50\x03\x50\
	\x03\x50\x07\x50\u{354}\x0a\x50\x0c\x50\x0e\x50\u{357}\x0b\x50\x03\x51\x03\
	\x51\x03\x51\x03\x51\x03\x51\x03\x51\x03\x51\x03\x51\x03\x51\x03\x51\x03\
	\x51\x03\x51\x03\x51\x03\x51\x03\x51\x03\x51\x05\x51\u{369}\x0a\x51\x03\
	\x52\x03\x52\x06\x52\u{36d}\x0a\x52\x0d\x52\x0e\x52\u{36e}\x03\x52\x03\x52\
	\x03\x53\x03\x53\x03\x53\x06\x53\u{376}\x0a\x53\x0d\x53\x0e\x53\u{377}\x03\
	\x54\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\x05\x54\u{381}\x0a\
	\x54\x03\x55\x03\x55\x03\x55\x03\x55\x05\x55\u{387}\x0a\x55\x03\x55\x03\
	\x55\x03\x55\x03\x56\x03\x56\x06\x56\u{38e}\x0a\x56\x0d\x56\x0e\x56\u{38f}\
	\x03\x56\x03\x56\x03\x57\x03\x57\x03\x57\x03\x58\x03\x58\x06\x58\u{399}\
	\x0a\x58\x0d\x58\x0e\x58\u{39a}\x03\x58\x03\x58\x03\x59\x03\x59\x03\x59\
	\x03\x59\x03\x5a\x03\x5a\x06\x5a\u{3a5}\x0a\x5a\x0d\x5a\x0e\x5a\u{3a6}\x03\
	\x5a\x03\x5a\x03\x5b\x03\x5b\x06\x5b\u{3ad}\x0a\x5b\x0d\x5b\x0e\x5b\u{3ae}\
	\x03\x5b\x03\x5b\x03\x5c\x03\x5c\x03\x5c\x03\x5c\x03\x5c\x03\x5c\x03\x5c\
	\x03\x5c\x03\x5c\x03\x5c\x05\x5c\u{3bd}\x0a\x5c\x03\x5d\x03\x5d\x06\x5d\
	\u{3c1}\x0a\x5d\x0d\x5d\x0e\x5d\u{3c2}\x03\x5d\x03\x5d\x03\x5e\x03\x5e\x03\
	\x5e\x03\x5e\x03\x5e\x03\x5e\x07\x5e\u{3cd}\x0a\x5e\x0c\x5e\x0e\x5e\u{3d0}\
	\x0b\x5e\x03\x5e\x03\x5e\x05\x5e\u{3d4}\x0a\x5e\x03\x5f\x03\x5f\x03\x60\
	\x03\x60\x03\x60\x03\x60\x03\x60\x05\x60\u{3dd}\x0a\x60\x03\x61\x03\x61\
	\x05\x61\u{3e1}\x0a\x61\x03\x61\x03\x61\x03\x61\x03\x61\x03\x61\x05\x61\
	\u{3e8}\x0a\x61\x03\x61\x03\x61\x05\x61\u{3ec}\x0a\x61\x03\x61\x03\x61\x03\
	\x61\x03\x61\x03\x61\x03\x61\x03\x61\x03\x61\x05\x61\u{3f6}\x0a\x61\x03\
	\x62\x03\x62\x03\x62\x03\x62\x05\x62\u{3fc}\x0a\x62\x03\x62\x03\x62\x03\
	\x63\x03\x63\x03\x63\x03\x63\x03\x63\x03\x63\x03\x63\x03\x63\x03\x63\x03\
	\x63\x03\x63\x03\x63\x03\x63\x03\x63\x03\x63\x03\x63\x03\x63\x05\x63\u{411}\
	\x0a\x63\x03\x64\x03\x64\x03\x64\x07\x64\u{416}\x0a\x64\x0c\x64\x0e\x64\
	\u{419}\x0b\x64\x03\x64\x03\x64\x03\x65\x03\x65\x03\x65\x07\x65\u{420}\x0a\
	\x65\x0c\x65\x0e\x65\u{423}\x0b\x65\x03\x66\x03\x66\x03\x66\x03\x66\x03\
	\x66\x03\x66\x03\x66\x03\x66\x05\x66\u{42d}\x0a\x66\x03\x67\x03\x67\x03\
	\x67\x03\x67\x05\x67\u{433}\x0a\x67\x03\x68\x03\x68\x03\x68\x03\x68\x05\
	\x68\u{439}\x0a\x68\x03\x68\x03\x68\x03\x69\x03\x69\x03\x69\x03\x69\x03\
	\x69\x03\x69\x03\x69\x03\x69\x05\x69\u{445}\x0a\x69\x03\x6a\x03\x6a\x03\
	\x6a\x03\x6a\x03\x6a\x03\x6a\x03\x6a\x03\x6a\x03\x6a\x03\x6a\x05\x6a\u{451}\
	\x0a\x6a\x03\x6a\x03\x6a\x03\x6a\x03\x6a\x07\x6a\u{457}\x0a\x6a\x0c\x6a\
	\x0e\x6a\u{45a}\x0b\x6a\x03\x6b\x03\x6b\x03\x6b\x03\x6b\x03\x6b\x03\x6b\
	\x03\x6b\x03\x6b\x03\x6b\x07\x6b\u{465}\x0a\x6b\x0c\x6b\x0e\x6b\u{468}\x0b\
	\x6b\x03\x6b\x03\x6b\x03\x6b\x03\x6b\x03\x6c\x03\x6c\x03\x6c\x03\x6c\x03\
	\x6c\x03\x6c\x03\x6c\x03\x6c\x03\x6c\x03\x6c\x03\x6c\x03\x6c\x03\x6c\x03\
	\x6c\x03\x6c\x03\x6c\x05\x6c\u{47e}\x0a\x6c\x03\x6d\x03\x6d\x03\x6e\x03\
	\x6e\x03\x6f\x03\x6f\x03\x6f\x03\x6f\x03\x6f\x07\x6f\u{489}\x0a\x6f\x0c\
	\x6f\x0e\x6f\u{48c}\x0b\x6f\x05\x6f\u{48e}\x0a\x6f\x03\x6f\x03\x6f\x03\x70\
	\x03\x70\x03\x70\x07\x70\u{495}\x0a\x70\x0c\x70\x0e\x70\u{498}\x0b\x70\x03\
	\x71\x03\x71\x03\x71\x07\x71\u{49d}\x0a\x71\x0c\x71\x0e\x71\u{4a0}\x0b\x71\
	\x03\x72\x03\x72\x03\x72\x03\x72\x07\x72\u{4a6}\x0a\x72\x0c\x72\x0e\x72\
	\u{4a9}\x0b\x72\x03\x72\x03\x72\x03\x72\x03\x72\x05\x72\u{4af}\x0a\x72\x03\
	\x73\x03\x73\x03\x73\x05\x73\u{4b4}\x0a\x73\x03\x74\x03\x74\x03\x75\x03\
	\x75\x03\x75\x03\x76\x03\x76\x03\x77\x03\x77\x03\x77\x03\x77\x05\x77\u{4c1}\
	\x0a\x77\x03\x78\x03\x78\x03\x78\x03\x78\x03\x78\x03\x78\x05\x78\u{4c9}\
	\x0a\x78\x03\x78\x02\x04\x7a\u{d2}\x79\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\
	\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x30\x32\x34\x36\
	\x38\x3a\x3c\x3e\x40\x42\x44\x46\x48\x4a\x4c\x4e\x50\x52\x54\x56\x58\x5a\
	\x5c\x5e\x60\x62\x64\x66\x68\x6a\x6c\x6e\x70\x72\x74\x76\x78\x7a\x7c\x7e\
	\u{80}\u{82}\u{84}\u{86}\u{88}\u{8a}\u{8c}\u{8e}\u{90}\u{92}\u{94}\u{96}\
	\u{98}\u{9a}\u{9c}\u{9e}\u{a0}\u{a2}\u{a4}\u{a6}\u{a8}\u{aa}\u{ac}\u{ae}\
	\u{b0}\u{b2}\u{b4}\u{b6}\u{b8}\u{ba}\u{bc}\u{be}\u{c0}\u{c2}\u{c4}\u{c6}\
	\u{c8}\u{ca}\u{cc}\u{ce}\u{d0}\u{d2}\u{d4}\u{d6}\u{d8}\u{da}\u{dc}\u{de}\
	\u{e0}\u{e2}\u{e4}\u{e6}\u{e8}\u{ea}\u{ec}\u{ee}\x02\x18\x06\x02\u{b1}\u{b2}\
	\u{b7}\u{b8}\u{c4}\u{c4}\u{c6}\u{c6}\x03\x02\x03\x06\x03\x02\x09\x16\x03\
	\x02\x18\x1c\x03\x02\x1e\x1f\x03\x02\x20\x25\x03\x02\u{b3}\u{b4}\x05\x02\
	\u{ad}\u{ad}\u{b0}\u{b1}\u{b3}\u{b3}\x03\x02\x37\x39\x03\x02\x3f\x41\x03\
	\x02\x44\x46\x03\x02\x4a\x4c\x03\x02\x54\x56\x03\x02\x59\x5c\x03\x02\u{c5}\
	\u{c6}\x03\x02\u{c3}\u{c4}\x03\x02\x6d\x6f\x04\x02\u{b1}\u{b1}\u{c5}\u{c5}\
	\x03\x02\u{bd}\u{c2}\x03\x02\u{c3}\u{c6}\x03\x02\u{95}\u{a4}\x04\x02\u{8a}\
	\u{8a}\u{a5}\u{a8}\x02\u{507}\x02\u{f3}\x03\x02\x02\x02\x04\u{fe}\x03\x02\
	\x02\x02\x06\u{102}\x03\x02\x02\x02\x08\u{108}\x03\x02\x02\x02\x0a\u{10a}\
	\x03\x02\x02\x02\x0c\u{10d}\x03\x02\x02\x02\x0e\u{150}\x03\x02\x02\x02\x10\
	\u{152}\x03\x02\x02\x02\x12\u{15b}\x03\x02\x02\x02\x14\u{15d}\x03\x02\x02\
	\x02\x16\u{160}\x03\x02\x02\x02\x18\u{168}\x03\x02\x02\x02\x1a\u{16a}\x03\
	\x02\x02\x02\x1c\u{178}\x03\x02\x02\x02\x1e\u{17b}\x03\x02\x02\x02\x20\u{17d}\
	\x03\x02\x02\x02\x22\u{180}\x03\x02\x02\x02\x24\u{18d}\x03\x02\x02\x02\x26\
	\u{191}\x03\x02\x02\x02\x28\u{194}\x03\x02\x02\x02\x2a\u{197}\x03\x02\x02\
	\x02\x2c\u{1af}\x03\x02\x02\x02\x2e\u{1b1}\x03\x02\x02\x02\x30\u{1bd}\x03\
	\x02\x02\x02\x32\u{1bf}\x03\x02\x02\x02\x34\u{1c6}\x03\x02\x02\x02\x36\u{1d5}\
	\x03\x02\x02\x02\x38\u{1d7}\x03\x02\x02\x02\x3a\u{1df}\x03\x02\x02\x02\x3c\
	\u{1ea}\x03\x02\x02\x02\x3e\u{1fb}\x03\x02\x02\x02\x40\u{1fd}\x03\x02\x02\
	\x02\x42\u{200}\x03\x02\x02\x02\x44\u{205}\x03\x02\x02\x02\x46\u{208}\x03\
	\x02\x02\x02\x48\u{214}\x03\x02\x02\x02\x4a\u{216}\x03\x02\x02\x02\x4c\u{21c}\
	\x03\x02\x02\x02\x4e\u{21e}\x03\x02\x02\x02\x50\u{220}\x03\x02\x02\x02\x52\
	\u{227}\x03\x02\x02\x02\x54\u{229}\x03\x02\x02\x02\x56\u{22b}\x03\x02\x02\
	\x02\x58\u{231}\x03\x02\x02\x02\x5a\u{23b}\x03\x02\x02\x02\x5c\u{241}\x03\
	\x02\x02\x02\x5e\u{24b}\x03\x02\x02\x02\x60\u{24d}\x03\x02\x02\x02\x62\u{259}\
	\x03\x02\x02\x02\x64\u{25b}\x03\x02\x02\x02\x66\u{25e}\x03\x02\x02\x02\x68\
	\u{260}\x03\x02\x02\x02\x6a\u{263}\x03\x02\x02\x02\x6c\u{266}\x03\x02\x02\
	\x02\x6e\u{269}\x03\x02\x02\x02\x70\u{271}\x03\x02\x02\x02\x72\u{27f}\x03\
	\x02\x02\x02\x74\u{2a4}\x03\x02\x02\x02\x76\u{2a6}\x03\x02\x02\x02\x78\u{2ae}\
	\x03\x02\x02\x02\x7a\u{2bd}\x03\x02\x02\x02\x7c\u{2d4}\x03\x02\x02\x02\x7e\
	\u{2e6}\x03\x02\x02\x02\u{80}\u{2f2}\x03\x02\x02\x02\u{82}\u{2f5}\x03\x02\
	\x02\x02\u{84}\u{2f8}\x03\x02\x02\x02\u{86}\u{300}\x03\x02\x02\x02\u{88}\
	\u{303}\x03\x02\x02\x02\u{8a}\u{307}\x03\x02\x02\x02\u{8c}\u{30c}\x03\x02\
	\x02\x02\u{8e}\u{30e}\x03\x02\x02\x02\u{90}\u{319}\x03\x02\x02\x02\u{92}\
	\u{322}\x03\x02\x02\x02\u{94}\u{326}\x03\x02\x02\x02\u{96}\u{32d}\x03\x02\
	\x02\x02\u{98}\u{335}\x03\x02\x02\x02\u{9a}\u{33a}\x03\x02\x02\x02\u{9c}\
	\u{348}\x03\x02\x02\x02\u{9e}\u{350}\x03\x02\x02\x02\u{a0}\u{368}\x03\x02\
	\x02\x02\u{a2}\u{36a}\x03\x02\x02\x02\u{a4}\u{372}\x03\x02\x02\x02\u{a6}\
	\u{380}\x03\x02\x02\x02\u{a8}\u{382}\x03\x02\x02\x02\u{aa}\u{38b}\x03\x02\
	\x02\x02\u{ac}\u{393}\x03\x02\x02\x02\u{ae}\u{396}\x03\x02\x02\x02\u{b0}\
	\u{39e}\x03\x02\x02\x02\u{b2}\u{3a2}\x03\x02\x02\x02\u{b4}\u{3aa}\x03\x02\
	\x02\x02\u{b6}\u{3bc}\x03\x02\x02\x02\u{b8}\u{3be}\x03\x02\x02\x02\u{ba}\
	\u{3d3}\x03\x02\x02\x02\u{bc}\u{3d5}\x03\x02\x02\x02\u{be}\u{3dc}\x03\x02\
	\x02\x02\u{c0}\u{3f5}\x03\x02\x02\x02\u{c2}\u{3f7}\x03\x02\x02\x02\u{c4}\
	\u{410}\x03\x02\x02\x02\u{c6}\u{412}\x03\x02\x02\x02\u{c8}\u{41c}\x03\x02\
	\x02\x02\u{ca}\u{42c}\x03\x02\x02\x02\u{cc}\u{432}\x03\x02\x02\x02\u{ce}\
	\u{434}\x03\x02\x02\x02\u{d0}\u{444}\x03\x02\x02\x02\u{d2}\u{450}\x03\x02\
	\x02\x02\u{d4}\u{45b}\x03\x02\x02\x02\u{d6}\u{47d}\x03\x02\x02\x02\u{d8}\
	\u{47f}\x03\x02\x02\x02\u{da}\u{481}\x03\x02\x02\x02\u{dc}\u{483}\x03\x02\
	\x02\x02\u{de}\u{491}\x03\x02\x02\x02\u{e0}\u{499}\x03\x02\x02\x02\u{e2}\
	\u{4ae}\x03\x02\x02\x02\u{e4}\u{4b3}\x03\x02\x02\x02\u{e6}\u{4b5}\x03\x02\
	\x02\x02\u{e8}\u{4b7}\x03\x02\x02\x02\u{ea}\u{4ba}\x03\x02\x02\x02\u{ec}\
	\u{4c0}\x03\x02\x02\x02\u{ee}\u{4c8}\x03\x02\x02\x02\u{f0}\u{f2}\x05\x04\
	\x03\x02\u{f1}\u{f0}\x03\x02\x02\x02\u{f2}\u{f5}\x03\x02\x02\x02\u{f3}\u{f1}\
	\x03\x02\x02\x02\u{f3}\u{f4}\x03\x02\x02\x02\u{f4}\u{f8}\x03\x02\x02\x02\
	\u{f5}\u{f3}\x03\x02\x02\x02\u{f6}\u{f9}\x05\x0c\x07\x02\u{f7}\u{f9}\x05\
	\u{b8}\x5d\x02\u{f8}\u{f6}\x03\x02\x02\x02\u{f8}\u{f7}\x03\x02\x02\x02\u{f9}\
	\u{fa}\x03\x02\x02\x02\u{fa}\u{f8}\x03\x02\x02\x02\u{fa}\u{fb}\x03\x02\x02\
	\x02\u{fb}\u{fc}\x03\x02\x02\x02\u{fc}\u{fd}\x07\x02\x02\x03\u{fd}\x03\x03\
	\x02\x02\x02\u{fe}\u{ff}\x07\u{aa}\x02\x02\u{ff}\u{100}\x05\x06\x04\x02\
	\u{100}\x05\x03\x02\x02\x02\u{101}\u{103}\x05\x08\x05\x02\u{102}\u{101}\
	\x03\x02\x02\x02\u{103}\u{104}\x03\x02\x02\x02\u{104}\u{102}\x03\x02\x02\
	\x02\u{104}\u{105}\x03\x02\x02\x02\u{105}\u{106}\x03\x02\x02\x02\u{106}\
	\u{107}\x05\x0a\x06\x02\u{107}\x07\x03\x02\x02\x02\u{108}\u{109}\x09\x02\
	\x02\x02\u{109}\x09\x03\x02\x02\x02\u{10a}\u{10b}\x07\u{b8}\x02\x02\u{10b}\
	\u{10c}\x09\x03\x02\x02\u{10c}\x0b\x03\x02\x02\x02\u{10d}\u{10e}\x07\x03\
	\x02\x02\u{10e}\u{110}\x05\x0e\x08\x02\u{10f}\u{111}\x05\x10\x09\x02\u{110}\
	\u{10f}\x03\x02\x02\x02\u{110}\u{111}\x03\x02\x02\x02\u{111}\u{113}\x03\
	\x02\x02\x02\u{112}\u{114}\x05\x14\x0b\x02\u{113}\u{112}\x03\x02\x02\x02\
	\u{113}\u{114}\x03\x02\x02\x02\u{114}\u{116}\x03\x02\x02\x02\u{115}\u{117}\
	\x05\x1a\x0e\x02\u{116}\u{115}\x03\x02\x02\x02\u{116}\u{117}\x03\x02\x02\
	\x02\u{117}\u{119}\x03\x02\x02\x02\u{118}\u{11a}\x05\x1c\x0f\x02\u{119}\
	\u{118}\x03\x02\x02\x02\u{119}\u{11a}\x03\x02\x02\x02\u{11a}\u{11c}\x03\
	\x02\x02\x02\u{11b}\u{11d}\x05\x26\x14\x02\u{11c}\u{11b}\x03\x02\x02\x02\
	\u{11c}\u{11d}\x03\x02\x02\x02\u{11d}\u{11f}\x03\x02\x02\x02\u{11e}\u{120}\
	\x05\x38\x1d\x02\u{11f}\u{11e}\x03\x02\x02\x02\u{11f}\u{120}\x03\x02\x02\
	\x02\u{120}\u{122}\x03\x02\x02\x02\u{121}\u{123}\x05\x3c\x1f\x02\u{122}\
	\u{121}\x03\x02\x02\x02\u{122}\u{123}\x03\x02\x02\x02\u{123}\u{125}\x03\
	\x02\x02\x02\u{124}\u{126}\x05\x60\x31\x02\u{125}\u{124}\x03\x02\x02\x02\
	\u{125}\u{126}\x03\x02\x02\x02\u{126}\u{128}\x03\x02\x02\x02\u{127}\u{129}\
	\x05\x6e\x38\x02\u{128}\u{127}\x03\x02\x02\x02\u{128}\u{129}\x03\x02\x02\
	\x02\u{129}\u{12d}\x03\x02\x02\x02\u{12a}\u{12c}\x05\x74\x3b\x02\u{12b}\
	\u{12a}\x03\x02\x02\x02\u{12c}\u{12f}\x03\x02\x02\x02\u{12d}\u{12b}\x03\
	\x02\x02\x02\u{12d}\u{12e}\x03\x02\x02\x02\u{12e}\u{131}\x03\x02\x02\x02\
	\u{12f}\u{12d}\x03\x02\x02\x02\u{130}\u{132}\x05\x76\x3c\x02\u{131}\u{130}\
	\x03\x02\x02\x02\u{131}\u{132}\x03\x02\x02\x02\u{132}\u{134}\x03\x02\x02\
	\x02\u{133}\u{135}\x05\x2a\x16\x02\u{134}\u{133}\x03\x02\x02\x02\u{134}\
	\u{135}\x03\x02\x02\x02\u{135}\u{137}\x03\x02\x02\x02\u{136}\u{138}\x05\
	\x28\x15\x02\u{137}\u{136}\x03\x02\x02\x02\u{137}\u{138}\x03\x02\x02\x02\
	\u{138}\u{13a}\x03\x02\x02\x02\u{139}\u{13b}\x05\x7e\x40\x02\u{13a}\u{139}\
	\x03\x02\x02\x02\u{13a}\u{13b}\x03\x02\x02\x02\u{13b}\u{13d}\x03\x02\x02\
	\x02\u{13c}\u{13e}\x05\u{9c}\x4f\x02\u{13d}\u{13c}\x03\x02\x02\x02\u{13d}\
	\u{13e}\x03\x02\x02\x02\u{13e}\u{140}\x03\x02\x02\x02\u{13f}\u{141}\x05\
	\u{a2}\x52\x02\u{140}\u{13f}\x03\x02\x02\x02\u{140}\u{141}\x03\x02\x02\x02\
	\u{141}\u{145}\x03\x02\x02\x02\u{142}\u{144}\x05\u{a8}\x55\x02\u{143}\u{142}\
	\x03\x02\x02\x02\u{144}\u{147}\x03\x02\x02\x02\u{145}\u{143}\x03\x02\x02\
	\x02\u{145}\u{146}\x03\x02\x02\x02\u{146}\u{149}\x03\x02\x02\x02\u{147}\
	\u{145}\x03\x02\x02\x02\u{148}\u{14a}\x05\u{b4}\x5b\x02\u{149}\u{148}\x03\
	\x02\x02\x02\u{149}\u{14a}\x03\x02\x02\x02\u{14a}\u{14b}\x03\x02\x02\x02\
	\u{14b}\u{14c}\x07\x07\x02\x02\u{14c}\x0d\x03\x02\x02\x02\u{14d}\u{151}\
	\x07\u{b1}\x02\x02\u{14e}\u{151}\x05\x12\x0a\x02\u{14f}\u{151}\x05\x46\x24\
	\x02\u{150}\u{14d}\x03\x02\x02\x02\u{150}\u{14e}\x03\x02\x02\x02\u{150}\
	\u{14f}\x03\x02\x02\x02\u{151}\x0f\x03\x02\x02\x02\u{152}\u{153}\x07\x08\
	\x02\x02\u{153}\u{158}\x05\x12\x0a\x02\u{154}\u{155}\x07\u{b6}\x02\x02\u{155}\
	\u{157}\x05\x12\x0a\x02\u{156}\u{154}\x03\x02\x02\x02\u{157}\u{15a}\x03\
	\x02\x02\x02\u{158}\u{156}\x03\x02\x02\x02\u{158}\u{159}\x03\x02\x02\x02\
	\u{159}\x11\x03\x02\x02\x02\u{15a}\u{158}\x03\x02\x02\x02\u{15b}\u{15c}\
	\x09\x04\x02\x02\u{15c}\x13\x03\x02\x02\x02\u{15d}\u{15e}\x07\x17\x02\x02\
	\u{15e}\u{15f}\x05\x16\x0c\x02\u{15f}\x15\x03\x02\x02\x02\u{160}\u{165}\
	\x05\x18\x0d\x02\u{161}\u{162}\x07\u{b6}\x02\x02\u{162}\u{164}\x05\x18\x0d\
	\x02\u{163}\u{161}\x03\x02\x02\x02\u{164}\u{167}\x03\x02\x02\x02\u{165}\
	\u{163}\x03\x02\x02\x02\u{165}\u{166}\x03\x02\x02\x02\u{166}\x17\x03\x02\
	\x02\x02\u{167}\u{165}\x03\x02\x02\x02\u{168}\u{169}\x09\x05\x02\x02\u{169}\
	\x19\x03\x02\x02\x02\u{16a}\u{16b}\x07\x1d\x02\x02\u{16b}\u{16d}\x07\u{ac}\
	\x02\x02\u{16c}\u{16e}\x05\x1c\x0f\x02\u{16d}\u{16c}\x03\x02\x02\x02\u{16d}\
	\u{16e}\x03\x02\x02\x02\u{16e}\u{170}\x03\x02\x02\x02\u{16f}\u{171}\x05\
	\x20\x11\x02\u{170}\u{16f}\x03\x02\x02\x02\u{170}\u{171}\x03\x02\x02\x02\
	\u{171}\u{173}\x03\x02\x02\x02\u{172}\u{174}\x05\x22\x12\x02\u{173}\u{172}\
	\x03\x02\x02\x02\u{173}\u{174}\x03\x02\x02\x02\u{174}\u{176}\x03\x02\x02\
	\x02\u{175}\u{177}\x05\x24\x13\x02\u{176}\u{175}\x03\x02\x02\x02\u{176}\
	\u{177}\x03\x02\x02\x02\u{177}\x1b\x03\x02\x02\x02\u{178}\u{179}\x09\x06\
	\x02\x02\u{179}\u{17a}\x05\x1e\x10\x02\u{17a}\x1d\x03\x02\x02\x02\u{17b}\
	\u{17c}\x09\x07\x02\x02\u{17c}\x1f\x03\x02\x02\x02\u{17d}\u{17e}\x07\x26\
	\x02\x02\u{17e}\u{17f}\x07\u{ac}\x02\x02\u{17f}\x21\x03\x02\x02\x02\u{180}\
	\u{181}\x07\x27\x02\x02\u{181}\u{182}\x07\u{b5}\x02\x02\u{182}\u{186}\x07\
	\u{b3}\x02\x02\u{183}\u{184}\x07\x28\x02\x02\u{184}\u{185}\x07\u{b5}\x02\
	\x02\u{185}\u{187}\x07\u{b3}\x02\x02\u{186}\u{183}\x03\x02\x02\x02\u{186}\
	\u{187}\x03\x02\x02\x02\u{187}\u{18b}\x03\x02\x02\x02\u{188}\u{189}\x07\
	\x29\x02\x02\u{189}\u{18a}\x07\u{b5}\x02\x02\u{18a}\u{18c}\x07\u{ac}\x02\
	\x02\u{18b}\u{188}\x03\x02\x02\x02\u{18b}\u{18c}\x03\x02\x02\x02\u{18c}\
	\x23\x03\x02\x02\x02\u{18d}\u{18e}\x07\x2a\x02\x02\u{18e}\u{18f}\x07\u{b5}\
	\x02\x02\u{18f}\u{190}\x09\x08\x02\x02\u{190}\x25\x03\x02\x02\x02\u{191}\
	\u{192}\x07\x2b\x02\x02\u{192}\u{193}\x05\u{e4}\x73\x02\u{193}\x27\x03\x02\
	\x02\x02\u{194}\u{195}\x07\x2c\x02\x02\u{195}\u{196}\x07\u{b0}\x02\x02\u{196}\
	\x29\x03\x02\x02\x02\u{197}\u{199}\x07\x2d\x02\x02\u{198}\u{19a}\x05\x2c\
	\x17\x02\u{199}\u{198}\x03\x02\x02\x02\u{19a}\u{19b}\x03\x02\x02\x02\u{19b}\
	\u{199}\x03\x02\x02\x02\u{19b}\u{19c}\x03\x02\x02\x02\u{19c}\u{19d}\x03\
	\x02\x02\x02\u{19d}\u{19e}\x07\x07\x02\x02\u{19e}\x2b\x03\x02\x02\x02\u{19f}\
	\u{1a0}\x05\u{de}\x70\x02\u{1a0}\u{1a1}\x05\x2e\x18\x02\u{1a1}\u{1b0}\x03\
	\x02\x02\x02\u{1a2}\u{1a3}\x05\u{de}\x70\x02\u{1a3}\u{1a4}\x05\x32\x1a\x02\
	\u{1a4}\u{1b0}\x03\x02\x02\x02\u{1a5}\u{1a6}\x05\u{de}\x70\x02\u{1a6}\u{1a7}\
	\x05\x34\x1b\x02\u{1a7}\u{1b0}\x03\x02\x02\x02\u{1a8}\u{1a9}\x05\u{de}\x70\
	\x02\u{1a9}\u{1aa}\x05\x36\x1c\x02\u{1aa}\u{1b0}\x03\x02\x02\x02\u{1ab}\
	\u{1ac}\x05\u{d6}\x6c\x02\u{1ac}\u{1ad}\x07\x2e\x02\x02\u{1ad}\u{1ae}\x07\
	\u{b3}\x02\x02\u{1ae}\u{1b0}\x03\x02\x02\x02\u{1af}\u{19f}\x03\x02\x02\x02\
	\u{1af}\u{1a2}\x03\x02\x02\x02\u{1af}\u{1a5}\x03\x02\x02\x02\u{1af}\u{1a8}\
	\x03\x02\x02\x02\u{1af}\u{1ab}\x03\x02\x02\x02\u{1b0}\x2d\x03\x02\x02\x02\
	\u{1b1}\u{1b2}\x07\x2f\x02\x02\u{1b2}\u{1b3}\x07\u{bb}\x02\x02\u{1b3}\u{1b8}\
	\x05\x30\x19\x02\u{1b4}\u{1b5}\x07\u{b6}\x02\x02\u{1b5}\u{1b7}\x05\x30\x19\
	\x02\u{1b6}\u{1b4}\x03\x02\x02\x02\u{1b7}\u{1ba}\x03\x02\x02\x02\u{1b8}\
	\u{1b6}\x03\x02\x02\x02\u{1b8}\u{1b9}\x03\x02\x02\x02\u{1b9}\u{1bb}\x03\
	\x02\x02\x02\u{1ba}\u{1b8}\x03\x02\x02\x02\u{1bb}\u{1bc}\x07\u{bc}\x02\x02\
	\u{1bc}\x2f\x03\x02\x02\x02\u{1bd}\u{1be}\x09\x09\x02\x02\u{1be}\x31\x03\
	\x02\x02\x02\u{1bf}\u{1c0}\x07\x30\x02\x02\u{1c0}\u{1c1}\x07\u{bb}\x02\x02\
	\u{1c1}\u{1c2}\x05\u{ee}\x78\x02\u{1c2}\u{1c3}\x07\u{b6}\x02\x02\u{1c3}\
	\u{1c4}\x05\u{ee}\x78\x02\u{1c4}\u{1c5}\x07\u{bc}\x02\x02\u{1c5}\x33\x03\
	\x02\x02\x02\u{1c6}\u{1c7}\x07\x08\x02\x02\u{1c7}\u{1c8}\x07\u{bb}\x02\x02\
	\u{1c8}\u{1c9}\x07\u{b3}\x02\x02\u{1c9}\u{1ca}\x07\u{bc}\x02\x02\u{1ca}\
	\x35\x03\x02\x02\x02\u{1cb}\u{1cc}\x07\x31\x02\x02\u{1cc}\u{1cd}\x07\u{bb}\
	\x02\x02\u{1cd}\u{1ce}\x07\u{ad}\x02\x02\u{1ce}\u{1cf}\x07\u{b6}\x02\x02\
	\u{1cf}\u{1d0}\x07\u{ad}\x02\x02\u{1d0}\u{1d6}\x07\u{bc}\x02\x02\u{1d1}\
	\u{1d2}\x07\x31\x02\x02\u{1d2}\u{1d3}\x07\u{bb}\x02\x02\u{1d3}\u{1d4}\x07\
	\u{ad}\x02\x02\u{1d4}\u{1d6}\x07\u{bc}\x02\x02\u{1d5}\u{1cb}\x03\x02\x02\
	\x02\u{1d5}\u{1d1}\x03\x02\x02\x02\u{1d6}\x37\x03\x02\x02\x02\u{1d7}\u{1d9}\
	\x07\x32\x02\x02\u{1d8}\u{1da}\x05\x3a\x1e\x02\u{1d9}\u{1d8}\x03\x02\x02\
	\x02\u{1da}\u{1db}\x03\x02\x02\x02\u{1db}\u{1d9}\x03\x02\x02\x02\u{1db}\
	\u{1dc}\x03\x02\x02\x02\u{1dc}\u{1dd}\x03\x02\x02\x02\u{1dd}\u{1de}\x07\
	\x07\x02\x02\u{1de}\x39\x03\x02\x02\x02\u{1df}\u{1e0}\x05\x72\x3a\x02\u{1e0}\
	\u{1e4}\x05\u{be}\x60\x02\u{1e1}\u{1e3}\x05\u{ca}\x66\x02\u{1e2}\u{1e1}\
	\x03\x02\x02\x02\u{1e3}\u{1e6}\x03\x02\x02\x02\u{1e4}\u{1e2}\x03\x02\x02\
	\x02\u{1e4}\u{1e5}\x03\x02\x02\x02\u{1e5}\u{1e8}\x03\x02\x02\x02\u{1e6}\
	\u{1e4}\x03\x02\x02\x02\u{1e7}\u{1e9}\x07\u{b6}\x02\x02\u{1e8}\u{1e7}\x03\
	\x02\x02\x02\u{1e8}\u{1e9}\x03\x02\x02\x02\u{1e9}\x3b\x03\x02\x02\x02\u{1ea}\
	\u{1ec}\x07\x33\x02\x02\u{1eb}\u{1ed}\x05\x3e\x20\x02\u{1ec}\u{1eb}\x03\
	\x02\x02\x02\u{1ed}\u{1ee}\x03\x02\x02\x02\u{1ee}\u{1ec}\x03\x02\x02\x02\
	\u{1ee}\u{1ef}\x03\x02\x02\x02\u{1ef}\u{1f0}\x03\x02\x02\x02\u{1f0}\u{1f1}\
	\x07\x07\x02\x02\u{1f1}\x3d\x03\x02\x02\x02\u{1f2}\u{1fc}\x05\x40\x21\x02\
	\u{1f3}\u{1fc}\x05\x42\x22\x02\u{1f4}\u{1fc}\x05\x44\x23\x02\u{1f5}\u{1fc}\
	\x05\x48\x25\x02\u{1f6}\u{1fc}\x05\x4c\x27\x02\u{1f7}\u{1fc}\x05\x50\x29\
	\x02\u{1f8}\u{1fc}\x05\x52\x2a\x02\u{1f9}\u{1fc}\x05\x56\x2c\x02\u{1fa}\
	\u{1fc}\x05\x5a\x2e\x02\u{1fb}\u{1f2}\x03\x02\x02\x02\u{1fb}\u{1f3}\x03\
	\x02\x02\x02\u{1fb}\u{1f4}\x03\x02\x02\x02\u{1fb}\u{1f5}\x03\x02\x02\x02\
	\u{1fb}\u{1f6}\x03\x02\x02\x02\u{1fb}\u{1f7}\x03\x02\x02\x02\u{1fb}\u{1f8}\
	\x03\x02\x02\x02\u{1fb}\u{1f9}\x03\x02\x02\x02\u{1fb}\u{1fa}\x03\x02\x02\
	\x02\u{1fc}\x3f\x03\x02\x02\x02\u{1fd}\u{1fe}\x07\x34\x02\x02\u{1fe}\u{1ff}\
	\x05\u{e2}\x72\x02\u{1ff}\x41\x03\x02\x02\x02\u{200}\u{203}\x07\x35\x02\
	\x02\u{201}\u{204}\x05\u{de}\x70\x02\u{202}\u{204}\x05\x46\x24\x02\u{203}\
	\u{201}\x03\x02\x02\x02\u{203}\u{202}\x03\x02\x02\x02\u{204}\x43\x03\x02\
	\x02\x02\u{205}\u{206}\x07\x36\x02\x02\u{206}\u{207}\x05\x46\x24\x02\u{207}\
	\x45\x03\x02\x02\x02\u{208}\u{209}\x09\x0a\x02\x02\u{209}\x47\x03\x02\x02\
	\x02\u{20a}\u{20b}\x07\x3a\x02\x02\u{20b}\u{215}\x05\u{e4}\x73\x02\u{20c}\
	\u{20d}\x07\x3b\x02\x02\u{20d}\u{215}\x05\x4a\x26\x02\u{20e}\u{20f}\x07\
	\x3c\x02\x02\u{20f}\u{215}\x05\u{e4}\x73\x02\u{210}\u{211}\x07\x3d\x02\x02\
	\u{211}\u{215}\x05\u{e4}\x73\x02\u{212}\u{213}\x07\x3e\x02\x02\u{213}\u{215}\
	\x05\u{de}\x70\x02\u{214}\u{20a}\x03\x02\x02\x02\u{214}\u{20c}\x03\x02\x02\
	\x02\u{214}\u{20e}\x03\x02\x02\x02\u{214}\u{210}\x03\x02\x02\x02\u{214}\
	\u{212}\x03\x02\x02\x02\u{215}\x49\x03\x02\x02\x02\u{216}\u{217}\x09\x0b\
	\x02\x02\u{217}\x4b\x03\x02\x02\x02\u{218}\u{219}\x07\x42\x02\x02\u{219}\
	\u{21d}\x05\x4e\x28\x02\u{21a}\u{21b}\x07\x43\x02\x02\u{21b}\u{21d}\x07\
	\u{b1}\x02\x02\u{21c}\u{218}\x03\x02\x02\x02\u{21c}\u{21a}\x03\x02\x02\x02\
	\u{21d}\x4d\x03\x02\x02\x02\u{21e}\u{21f}\x09\x0c\x02\x02\u{21f}\x4f\x03\
	\x02\x02\x02\u{220}\u{221}\x07\x47\x02\x02\u{221}\u{222}\x05\u{e4}\x73\x02\
	\u{222}\x51\x03\x02\x02\x02\u{223}\u{224}\x07\x48\x02\x02\u{224}\u{228}\
	\x05\u{e4}\x73\x02\u{225}\u{226}\x07\x49\x02\x02\u{226}\u{228}\x05\x54\x2b\
	\x02\u{227}\u{223}\x03\x02\x02\x02\u{227}\u{225}\x03\x02\x02\x02\u{228}\
	\x53\x03\x02\x02\x02\u{229}\u{22a}\x09\x0d\x02\x02\u{22a}\x55\x03\x02\x02\
	\x02\u{22b}\u{22c}\x07\x4d\x02\x02\u{22c}\u{22d}\x05\x58\x2d\x02\u{22d}\
	\u{22e}\x07\x07\x02\x02\u{22e}\x57\x03\x02\x02\x02\u{22f}\u{230}\x07\x4e\
	\x02\x02\u{230}\u{232}\x05\u{e2}\x72\x02\u{231}\u{22f}\x03\x02\x02\x02\u{231}\
	\u{232}\x03\x02\x02\x02\u{232}\u{235}\x03\x02\x02\x02\u{233}\u{234}\x07\
	\x4f\x02\x02\u{234}\u{236}\x05\u{e2}\x72\x02\u{235}\u{233}\x03\x02\x02\x02\
	\u{235}\u{236}\x03\x02\x02\x02\u{236}\u{239}\x03\x02\x02\x02\u{237}\u{238}\
	\x07\x50\x02\x02\u{238}\u{23a}\x05\u{e2}\x72\x02\u{239}\u{237}\x03\x02\x02\
	\x02\u{239}\u{23a}\x03\x02\x02\x02\u{23a}\x59\x03\x02\x02\x02\u{23b}\u{23c}\
	\x07\x2b\x02\x02\u{23c}\u{23d}\x05\x5c\x2f\x02\u{23d}\u{23e}\x07\x07\x02\
	\x02\u{23e}\x5b\x03\x02\x02\x02\u{23f}\u{240}\x07\x51\x02\x02\u{240}\u{242}\
	\x05\u{e4}\x73\x02\u{241}\u{23f}\x03\x02\x02\x02\u{241}\u{242}\x03\x02\x02\
	\x02\u{242}\u{245}\x03\x02\x02\x02\u{243}\u{244}\x07\x52\x02\x02\u{244}\
	\u{246}\x05\u{e8}\x75\x02\u{245}\u{243}\x03\x02\x02\x02\u{245}\u{246}\x03\
	\x02\x02\x02\u{246}\u{249}\x03\x02\x02\x02\u{247}\u{248}\x07\x53\x02\x02\
	\u{248}\u{24a}\x05\x5e\x30\x02\u{249}\u{247}\x03\x02\x02\x02\u{249}\u{24a}\
	\x03\x02\x02\x02\u{24a}\x5d\x03\x02\x02\x02\u{24b}\u{24c}\x09\x0e\x02\x02\
	\u{24c}\x5f\x03\x02\x02\x02\u{24d}\u{24f}\x07\x57\x02\x02\u{24e}\u{250}\
	\x05\x62\x32\x02\u{24f}\u{24e}\x03\x02\x02\x02\u{250}\u{251}\x03\x02\x02\
	\x02\u{251}\u{24f}\x03\x02\x02\x02\u{251}\u{252}\x03\x02\x02\x02\u{252}\
	\u{253}\x03\x02\x02\x02\u{253}\u{254}\x07\x07\x02\x02\u{254}\x61\x03\x02\
	\x02\x02\u{255}\u{25a}\x05\x64\x33\x02\u{256}\u{25a}\x05\x68\x35\x02\u{257}\
	\u{25a}\x05\x6a\x36\x02\u{258}\u{25a}\x05\x6c\x37\x02\u{259}\u{255}\x03\
	\x02\x02\x02\u{259}\u{256}\x03\x02\x02\x02\u{259}\u{257}\x03\x02\x02\x02\
	\u{259}\u{258}\x03\x02\x02\x02\u{25a}\x63\x03\x02\x02\x02\u{25b}\u{25c}\
	\x07\x58\x02\x02\u{25c}\u{25d}\x05\x66\x34\x02\u{25d}\x65\x03\x02\x02\x02\
	\u{25e}\u{25f}\x09\x0f\x02\x02\u{25f}\x67\x03\x02\x02\x02\u{260}\u{261}\
	\x07\x1e\x02\x02\u{261}\u{262}\x05\x1e\x10\x02\u{262}\x69\x03\x02\x02\x02\
	\u{263}\u{264}\x07\x5d\x02\x02\u{264}\u{265}\x07\u{b3}\x02\x02\u{265}\x6b\
	\x03\x02\x02\x02\u{266}\u{267}\x07\x5e\x02\x02\u{267}\u{268}\x07\u{b3}\x02\
	\x02\u{268}\x6d\x03\x02\x02\x02\u{269}\u{26b}\x07\x5f\x02\x02\u{26a}\u{26c}\
	\x05\x70\x39\x02\u{26b}\u{26a}\x03\x02\x02\x02\u{26c}\u{26d}\x03\x02\x02\
	\x02\u{26d}\u{26b}\x03\x02\x02\x02\u{26d}\u{26e}\x03\x02\x02\x02\u{26e}\
	\u{26f}\x03\x02\x02\x02\u{26f}\u{270}\x07\x07\x02\x02\u{270}\x6f\x03\x02\
	\x02\x02\u{271}\u{272}\x05\x72\x3a\x02\u{272}\u{276}\x05\u{be}\x60\x02\u{273}\
	\u{275}\x05\u{ca}\x66\x02\u{274}\u{273}\x03\x02\x02\x02\u{275}\u{278}\x03\
	\x02\x02\x02\u{276}\u{274}\x03\x02\x02\x02\u{276}\u{277}\x03\x02\x02\x02\
	\u{277}\u{27a}\x03\x02\x02\x02\u{278}\u{276}\x03\x02\x02\x02\u{279}\u{27b}\
	\x07\u{b6}\x02\x02\u{27a}\u{279}\x03\x02\x02\x02\u{27a}\u{27b}\x03\x02\x02\
	\x02\u{27b}\x71\x03\x02\x02\x02\u{27c}\u{280}\x07\u{b1}\x02\x02\u{27d}\u{280}\
	\x05\x46\x24\x02\u{27e}\u{280}\x05\u{8c}\x47\x02\u{27f}\u{27c}\x03\x02\x02\
	\x02\u{27f}\u{27d}\x03\x02\x02\x02\u{27f}\u{27e}\x03\x02\x02\x02\u{280}\
	\x73\x03\x02\x02\x02\u{281}\u{282}\x05\x72\x3a\x02\u{282}\u{286}\x07\x60\
	\x02\x02\u{283}\u{285}\x05\x70\x39\x02\u{284}\u{283}\x03\x02\x02\x02\u{285}\
	\u{288}\x03\x02\x02\x02\u{286}\u{284}\x03\x02\x02\x02\u{286}\u{287}\x03\
	\x02\x02\x02\u{287}\u{28c}\x03\x02\x02\x02\u{288}\u{286}\x03\x02\x02\x02\
	\u{289}\u{28b}\x05\x74\x3b\x02\u{28a}\u{289}\x03\x02\x02\x02\u{28b}\u{28e}\
	\x03\x02\x02\x02\u{28c}\u{28a}\x03\x02\x02\x02\u{28c}\u{28d}\x03\x02\x02\
	\x02\u{28d}\u{28f}\x03\x02\x02\x02\u{28e}\u{28c}\x03\x02\x02\x02\u{28f}\
	\u{290}\x07\x07\x02\x02\u{290}\u{2a5}\x03\x02\x02\x02\u{291}\u{292}\x05\
	\x72\x3a\x02\u{292}\u{293}\x07\x61\x02\x02\u{293}\u{294}\x07\u{bd}\x02\x02\
	\u{294}\u{295}\x07\x60\x02\x02\u{295}\u{299}\x07\u{be}\x02\x02\u{296}\u{298}\
	\x05\x70\x39\x02\u{297}\u{296}\x03\x02\x02\x02\u{298}\u{29b}\x03\x02\x02\
	\x02\u{299}\u{297}\x03\x02\x02\x02\u{299}\u{29a}\x03\x02\x02\x02\u{29a}\
	\u{29f}\x03\x02\x02\x02\u{29b}\u{299}\x03\x02\x02\x02\u{29c}\u{29e}\x05\
	\x74\x3b\x02\u{29d}\u{29c}\x03\x02\x02\x02\u{29e}\u{2a1}\x03\x02\x02\x02\
	\u{29f}\u{29d}\x03\x02\x02\x02\u{29f}\u{2a0}\x03\x02\x02\x02\u{2a0}\u{2a2}\
	\x03\x02\x02\x02\u{2a1}\u{29f}\x03\x02\x02\x02\u{2a2}\u{2a3}\x07\x07\x02\
	\x02\u{2a3}\u{2a5}\x03\x02\x02\x02\u{2a4}\u{281}\x03\x02\x02\x02\u{2a4}\
	\u{291}\x03\x02\x02\x02\u{2a5}\x75\x03\x02\x02\x02\u{2a6}\u{2a8}\x07\x62\
	\x02\x02\u{2a7}\u{2a9}\x05\x78\x3d\x02\u{2a8}\u{2a7}\x03\x02\x02\x02\u{2a9}\
	\u{2aa}\x03\x02\x02\x02\u{2aa}\u{2a8}\x03\x02\x02\x02\u{2aa}\u{2ab}\x03\
	\x02\x02\x02\u{2ab}\u{2ac}\x03\x02\x02\x02\u{2ac}\u{2ad}\x07\x07\x02\x02\
	\u{2ad}\x77\x03\x02\x02\x02\u{2ae}\u{2af}\x05\x72\x3a\x02\u{2af}\u{2b0}\
	\x07\x63\x02\x02\u{2b0}\u{2b1}\x05\x7a\x3e\x02\u{2b1}\x79\x03\x02\x02\x02\
	\u{2b2}\u{2b3}\x08\x3e\x01\x02\u{2b3}\u{2b4}\x07\x66\x02\x02\u{2b4}\u{2be}\
	\x05\x7a\x3e\x08\u{2b5}\u{2b6}\x07\u{bb}\x02\x02\u{2b6}\u{2b7}\x05\x7a\x3e\
	\x02\u{2b7}\u{2b8}\x07\u{bc}\x02\x02\u{2b8}\u{2be}\x03\x02\x02\x02\u{2b9}\
	\u{2be}\x05\x7c\x3f\x02\u{2ba}\u{2be}\x05\u{dc}\x6f\x02\u{2bb}\u{2be}\x05\
	\u{de}\x70\x02\u{2bc}\u{2be}\x05\u{ec}\x77\x02\u{2bd}\u{2b2}\x03\x02\x02\
	\x02\u{2bd}\u{2b5}\x03\x02\x02\x02\u{2bd}\u{2b9}\x03\x02\x02\x02\u{2bd}\
	\u{2ba}\x03\x02\x02\x02\u{2bd}\u{2bb}\x03\x02\x02\x02\u{2bd}\u{2bc}\x03\
	\x02\x02\x02\u{2be}\u{2d1}\x03\x02\x02\x02\u{2bf}\u{2c0}\x0c\x0d\x02\x02\
	\u{2c0}\u{2c1}\x09\x10\x02\x02\u{2c1}\u{2d0}\x05\x7a\x3e\x0e\u{2c2}\u{2c3}\
	\x0c\x0c\x02\x02\u{2c3}\u{2c4}\x09\x11\x02\x02\u{2c4}\u{2d0}\x05\x7a\x3e\
	\x0d\u{2c5}\u{2c6}\x0c\x0b\x02\x02\u{2c6}\u{2c7}\x05\u{d8}\x6d\x02\u{2c7}\
	\u{2c8}\x05\x7a\x3e\x0c\u{2c8}\u{2d0}\x03\x02\x02\x02\u{2c9}\u{2ca}\x0c\
	\x0a\x02\x02\u{2ca}\u{2cb}\x07\x64\x02\x02\u{2cb}\u{2d0}\x05\x7a\x3e\x0b\
	\u{2cc}\u{2cd}\x0c\x09\x02\x02\u{2cd}\u{2ce}\x07\x65\x02\x02\u{2ce}\u{2d0}\
	\x05\x7a\x3e\x0a\u{2cf}\u{2bf}\x03\x02\x02\x02\u{2cf}\u{2c2}\x03\x02\x02\
	\x02\u{2cf}\u{2c5}\x03\x02\x02\x02\u{2cf}\u{2c9}\x03\x02\x02\x02\u{2cf}\
	\u{2cc}\x03\x02\x02\x02\u{2d0}\u{2d3}\x03\x02\x02\x02\u{2d1}\u{2cf}\x03\
	\x02\x02\x02\u{2d1}\u{2d2}\x03\x02\x02\x02\u{2d2}\x7b\x03\x02\x02\x02\u{2d3}\
	\u{2d1}\x03\x02\x02\x02\u{2d4}\u{2d5}\x07\x67\x02\x02\u{2d5}\u{2d6}\x05\
	\x7a\x3e\x02\u{2d6}\u{2d7}\x07\x68\x02\x02\u{2d7}\u{2df}\x05\x7a\x3e\x02\
	\u{2d8}\u{2d9}\x07\x67\x02\x02\u{2d9}\u{2da}\x05\x7a\x3e\x02\u{2da}\u{2db}\
	\x07\x68\x02\x02\u{2db}\u{2dc}\x05\x7a\x3e\x02\u{2dc}\u{2de}\x03\x02\x02\
	\x02\u{2dd}\u{2d8}\x03\x02\x02\x02\u{2de}\u{2e1}\x03\x02\x02\x02\u{2df}\
	\u{2dd}\x03\x02\x02\x02\u{2df}\u{2e0}\x03\x02\x02\x02\u{2e0}\u{2e2}\x03\
	\x02\x02\x02\u{2e1}\u{2df}\x03\x02\x02\x02\u{2e2}\u{2e3}\x07\x69\x02\x02\
	\u{2e3}\u{2e4}\x05\x7a\x3e\x02\u{2e4}\x7d\x03\x02\x02\x02\u{2e5}\u{2e7}\
	\x05\u{82}\x42\x02\u{2e6}\u{2e5}\x03\x02\x02\x02\u{2e6}\u{2e7}\x03\x02\x02\
	\x02\u{2e7}\u{2e8}\x03\x02\x02\x02\u{2e8}\u{2ea}\x05\u{84}\x43\x02\u{2e9}\
	\u{2eb}\x05\u{80}\x41\x02\u{2ea}\u{2e9}\x03\x02\x02\x02\u{2ea}\u{2eb}\x03\
	\x02\x02\x02\u{2eb}\u{2ed}\x03\x02\x02\x02\u{2ec}\u{2ee}\x05\u{90}\x49\x02\
	\u{2ed}\u{2ec}\x03\x02\x02\x02\u{2ed}\u{2ee}\x03\x02\x02\x02\u{2ee}\u{2f0}\
	\x03\x02\x02\x02\u{2ef}\u{2f1}\x05\u{96}\x4c\x02\u{2f0}\u{2ef}\x03\x02\x02\
	\x02\u{2f0}\u{2f1}\x03\x02\x02\x02\u{2f1}\x7f\x03\x02\x02\x02\u{2f2}\u{2f3}\
	\x07\x6a\x02\x02\u{2f3}\u{2f4}\x07\u{b1}\x02\x02\u{2f4}\u{81}\x03\x02\x02\
	\x02\u{2f5}\u{2f6}\x07\x6b\x02\x02\u{2f6}\u{2f7}\x07\u{b1}\x02\x02\u{2f7}\
	\u{83}\x03\x02\x02\x02\u{2f8}\u{2fb}\x07\x6c\x02\x02\u{2f9}\u{2fc}\x05\u{86}\
	\x44\x02\u{2fa}\u{2fc}\x05\u{88}\x45\x02\u{2fb}\u{2f9}\x03\x02\x02\x02\u{2fb}\
	\u{2fa}\x03\x02\x02\x02\u{2fc}\u{2fe}\x03\x02\x02\x02\u{2fd}\u{2ff}\x07\
	\x07\x02\x02\u{2fe}\u{2fd}\x03\x02\x02\x02\u{2fe}\u{2ff}\x03\x02\x02\x02\
	\u{2ff}\u{85}\x03\x02\x02\x02\u{300}\u{301}\x05\u{8e}\x48\x02\u{301}\u{87}\
	\x03\x02\x02\x02\u{302}\u{304}\x05\u{8a}\x46\x02\u{303}\u{302}\x03\x02\x02\
	\x02\u{304}\u{305}\x03\x02\x02\x02\u{305}\u{303}\x03\x02\x02\x02\u{305}\
	\u{306}\x03\x02\x02\x02\u{306}\u{89}\x03\x02\x02\x02\u{307}\u{30a}\x07\u{b1}\
	\x02\x02\u{308}\u{309}\x07\u{b5}\x02\x02\u{309}\u{30b}\x05\u{8c}\x47\x02\
	\u{30a}\u{308}\x03\x02\x02\x02\u{30a}\u{30b}\x03\x02\x02\x02\u{30b}\u{8b}\
	\x03\x02\x02\x02\u{30c}\u{30d}\x09\x12\x02\x02\u{30d}\u{8d}\x03\x02\x02\
	\x02\u{30e}\u{30f}\x07\u{b9}\x02\x02\u{30f}\u{314}\x07\u{b1}\x02\x02\u{310}\
	\u{311}\x07\u{b6}\x02\x02\u{311}\u{313}\x07\u{b1}\x02\x02\u{312}\u{310}\
	\x03\x02\x02\x02\u{313}\u{316}\x03\x02\x02\x02\u{314}\u{312}\x03\x02\x02\
	\x02\u{314}\u{315}\x03\x02\x02\x02\u{315}\u{317}\x03\x02\x02\x02\u{316}\
	\u{314}\x03\x02\x02\x02\u{317}\u{318}\x07\u{ba}\x02\x02\u{318}\u{8f}\x03\
	\x02\x02\x02\u{319}\u{31c}\x07\x70\x02\x02\u{31a}\u{31d}\x05\u{92}\x4a\x02\
	\u{31b}\u{31d}\x05\u{94}\x4b\x02\u{31c}\u{31a}\x03\x02\x02\x02\u{31c}\u{31b}\
	\x03\x02\x02\x02\u{31d}\u{31e}\x03\x02\x02\x02\u{31e}\u{31c}\x03\x02\x02\
	\x02\u{31e}\u{31f}\x03\x02\x02\x02\u{31f}\u{320}\x03\x02\x02\x02\u{320}\
	\u{321}\x07\x07\x02\x02\u{321}\u{91}\x03\x02\x02\x02\u{322}\u{323}\x07\x71\
	\x02\x02\u{323}\u{324}\x07\u{b1}\x02\x02\u{324}\u{325}\x05\u{8e}\x48\x02\
	\u{325}\u{93}\x03\x02\x02\x02\u{326}\u{327}\x09\x13\x02\x02\u{327}\u{328}\
	\x07\u{c7}\x02\x02\u{328}\u{32b}\x07\u{b1}\x02\x02\u{329}\u{32a}\x07\u{b5}\
	\x02\x02\u{32a}\u{32c}\x07\u{b1}\x02\x02\u{32b}\u{329}\x03\x02\x02\x02\u{32b}\
	\u{32c}\x03\x02\x02\x02\u{32c}\u{95}\x03\x02\x02\x02\u{32d}\u{32f}\x07\x72\
	\x02\x02\u{32e}\u{330}\x05\u{98}\x4d\x02\u{32f}\u{32e}\x03\x02\x02\x02\u{330}\
	\u{331}\x03\x02\x02\x02\u{331}\u{32f}\x03\x02\x02\x02\u{331}\u{332}\x03\
	\x02\x02\x02\u{332}\u{333}\x03\x02\x02\x02\u{333}\u{334}\x07\x07\x02\x02\
	\u{334}\u{97}\x03\x02\x02\x02\u{335}\u{336}\x07\x73\x02\x02\u{336}\u{337}\
	\x07\u{b1}\x02\x02\u{337}\u{338}\x07\u{b5}\x02\x02\u{338}\u{339}\x05\u{9a}\
	\x4e\x02\u{339}\u{99}\x03\x02\x02\x02\u{33a}\u{33b}\x07\u{b1}\x02\x02\u{33b}\
	\u{344}\x07\u{bb}\x02\x02\u{33c}\u{341}\x07\u{b3}\x02\x02\u{33d}\u{33e}\
	\x07\u{b6}\x02\x02\u{33e}\u{340}\x07\u{b3}\x02\x02\u{33f}\u{33d}\x03\x02\
	\x02\x02\u{340}\u{343}\x03\x02\x02\x02\u{341}\u{33f}\x03\x02\x02\x02\u{341}\
	\u{342}\x03\x02\x02\x02\u{342}\u{345}\x03\x02\x02\x02\u{343}\u{341}\x03\
	\x02\x02\x02\u{344}\u{33c}\x03\x02\x02\x02\u{344}\u{345}\x03\x02\x02\x02\
	\u{345}\u{346}\x03\x02\x02\x02\u{346}\u{347}\x07\u{bc}\x02\x02\u{347}\u{9b}\
	\x03\x02\x02\x02\u{348}\u{34a}\x07\x74\x02\x02\u{349}\u{34b}\x05\u{9e}\x50\
	\x02\u{34a}\u{349}\x03\x02\x02\x02\u{34b}\u{34c}\x03\x02\x02\x02\u{34c}\
	\u{34a}\x03\x02\x02\x02\u{34c}\u{34d}\x03\x02\x02\x02\u{34d}\u{34e}\x03\
	\x02\x02\x02\u{34e}\u{34f}\x07\x07\x02\x02\u{34f}\u{9d}\x03\x02\x02\x02\
	\u{350}\u{351}\x05\x72\x3a\x02\u{351}\u{355}\x05\u{be}\x60\x02\u{352}\u{354}\
	\x05\u{a0}\x51\x02\u{353}\u{352}\x03\x02\x02\x02\u{354}\u{357}\x03\x02\x02\
	\x02\u{355}\u{353}\x03\x02\x02\x02\u{355}\u{356}\x03\x02\x02\x02\u{356}\
	\u{9f}\x03\x02\x02\x02\u{357}\u{355}\x03\x02\x02\x02\u{358}\u{359}\x07\x75\
	\x02\x02\u{359}\u{35a}\x07\u{bb}\x02\x02\u{35a}\u{35b}\x05\u{ec}\x77\x02\
	\u{35b}\u{35c}\x07\u{bc}\x02\x02\u{35c}\u{369}\x03\x02\x02\x02\u{35d}\u{35e}\
	\x07\x30\x02\x02\u{35e}\u{35f}\x07\u{bb}\x02\x02\u{35f}\u{360}\x05\u{ee}\
	\x78\x02\u{360}\u{361}\x07\u{b6}\x02\x02\u{361}\u{362}\x05\u{ee}\x78\x02\
	\u{362}\u{363}\x07\u{bc}\x02\x02\u{363}\u{369}\x03\x02\x02\x02\u{364}\u{365}\
	\x07\x76\x02\x02\u{365}\u{369}\x07\u{b0}\x02\x02\u{366}\u{367}\x07\x77\x02\
	\x02\u{367}\u{369}\x07\u{b1}\x02\x02\u{368}\u{358}\x03\x02\x02\x02\u{368}\
	\u{35d}\x03\x02\x02\x02\u{368}\u{364}\x03\x02\x02\x02\u{368}\u{366}\x03\
	\x02\x02\x02\u{369}\u{a1}\x03\x02\x02\x02\u{36a}\u{36c}\x07\x78\x02\x02\
	\u{36b}\u{36d}\x05\u{a4}\x53\x02\u{36c}\u{36b}\x03\x02\x02\x02\u{36d}\u{36e}\
	\x03\x02\x02\x02\u{36e}\u{36c}\x03\x02\x02\x02\u{36e}\u{36f}\x03\x02\x02\
	\x02\u{36f}\u{370}\x03\x02\x02\x02\u{370}\u{371}\x07\x07\x02\x02\u{371}\
	\u{a3}\x03\x02\x02\x02\u{372}\u{373}\x07\u{b1}\x02\x02\u{373}\u{375}\x07\
	\u{b5}\x02\x02\u{374}\u{376}\x05\u{a6}\x54\x02\u{375}\u{374}\x03\x02\x02\
	\x02\u{376}\u{377}\x03\x02\x02\x02\u{377}\u{375}\x03\x02\x02\x02\u{377}\
	\u{378}\x03\x02\x02\x02\u{378}\u{a5}\x03\x02\x02\x02\u{379}\u{37a}\x05\x72\
	\x3a\x02\u{37a}\u{37b}\x05\u{ec}\x77\x02\u{37b}\u{381}\x03\x02\x02\x02\u{37c}\
	\u{37d}\x07\x27\x02\x02\u{37d}\u{381}\x07\u{b0}\x02\x02\u{37e}\u{37f}\x07\
	\x79\x02\x02\u{37f}\u{381}\x07\u{b3}\x02\x02\u{380}\u{379}\x03\x02\x02\x02\
	\u{380}\u{37c}\x03\x02\x02\x02\u{380}\u{37e}\x03\x02\x02\x02\u{381}\u{a7}\
	\x03\x02\x02\x02\u{382}\u{383}\x07\x7a\x02\x02\u{383}\u{384}\x07\u{b1}\x02\
	\x02\u{384}\u{386}\x05\u{aa}\x56\x02\u{385}\u{387}\x05\u{ae}\x58\x02\u{386}\
	\u{385}\x03\x02\x02\x02\u{386}\u{387}\x03\x02\x02\x02\u{387}\u{388}\x03\
	\x02\x02\x02\u{388}\u{389}\x05\u{b2}\x5a\x02\u{389}\u{38a}\x07\x07\x02\x02\
	\u{38a}\u{a9}\x03\x02\x02\x02\u{38b}\u{38d}\x07\x7b\x02\x02\u{38c}\u{38e}\
	\x05\u{ac}\x57\x02\u{38d}\u{38c}\x03\x02\x02\x02\u{38e}\u{38f}\x03\x02\x02\
	\x02\u{38f}\u{38d}\x03\x02\x02\x02\u{38f}\u{390}\x03\x02\x02\x02\u{390}\
	\u{391}\x03\x02\x02\x02\u{391}\u{392}\x07\x07\x02\x02\u{392}\u{ab}\x03\x02\
	\x02\x02\u{393}\u{394}\x05\x72\x3a\x02\u{394}\u{395}\x05\u{be}\x60\x02\u{395}\
	\u{ad}\x03\x02\x02\x02\u{396}\u{398}\x07\x7c\x02\x02\u{397}\u{399}\x05\u{b0}\
	\x59\x02\u{398}\u{397}\x03\x02\x02\x02\u{399}\u{39a}\x03\x02\x02\x02\u{39a}\
	\u{398}\x03\x02\x02\x02\u{39a}\u{39b}\x03\x02\x02\x02\u{39b}\u{39c}\x03\
	\x02\x02\x02\u{39c}\u{39d}\x07\x07\x02\x02\u{39d}\u{af}\x03\x02\x02\x02\
	\u{39e}\u{39f}\x05\x72\x3a\x02\u{39f}\u{3a0}\x07\x63\x02\x02\u{3a0}\u{3a1}\
	\x05\u{d2}\x6a\x02\u{3a1}\u{b1}\x03\x02\x02\x02\u{3a2}\u{3a4}\x07\x7d\x02\
	\x02\u{3a3}\u{3a5}\x05\u{ac}\x57\x02\u{3a4}\u{3a3}\x03\x02\x02\x02\u{3a5}\
	\u{3a6}\x03\x02\x02\x02\u{3a6}\u{3a4}\x03\x02\x02\x02\u{3a6}\u{3a7}\x03\
	\x02\x02\x02\u{3a7}\u{3a8}\x03\x02\x02\x02\u{3a8}\u{3a9}\x07\x07\x02\x02\
	\u{3a9}\u{b3}\x03\x02\x02\x02\u{3aa}\u{3ac}\x07\x7e\x02\x02\u{3ab}\u{3ad}\
	\x05\u{b6}\x5c\x02\u{3ac}\u{3ab}\x03\x02\x02\x02\u{3ad}\u{3ae}\x03\x02\x02\
	\x02\u{3ae}\u{3ac}\x03\x02\x02\x02\u{3ae}\u{3af}\x03\x02\x02\x02\u{3af}\
	\u{3b0}\x03\x02\x02\x02\u{3b0}\u{3b1}\x07\x07\x02\x02\u{3b1}\u{b5}\x03\x02\
	\x02\x02\u{3b2}\u{3b3}\x05\u{de}\x70\x02\u{3b3}\u{3b4}\x07\x63\x02\x02\u{3b4}\
	\u{3b5}\x05\u{d2}\x6a\x02\u{3b5}\u{3bd}\x03\x02\x02\x02\u{3b6}\u{3b7}\x07\
	\u{bb}\x02\x02\u{3b7}\u{3b8}\x05\u{e0}\x71\x02\u{3b8}\u{3b9}\x07\u{bc}\x02\
	\x02\u{3b9}\u{3ba}\x07\x63\x02\x02\u{3ba}\u{3bb}\x05\u{dc}\x6f\x02\u{3bb}\
	\u{3bd}\x03\x02\x02\x02\u{3bc}\u{3b2}\x03\x02\x02\x02\u{3bc}\u{3b6}\x03\
	\x02\x02\x02\u{3bd}\u{b7}\x03\x02\x02\x02\u{3be}\u{3c0}\x07\x7f\x02\x02\
	\u{3bf}\u{3c1}\x05\u{ba}\x5e\x02\u{3c0}\u{3bf}\x03\x02\x02\x02\u{3c1}\u{3c2}\
	\x03\x02\x02\x02\u{3c2}\u{3c0}\x03\x02\x02\x02\u{3c2}\u{3c3}\x03\x02\x02\
	\x02\u{3c3}\u{3c4}\x03\x02\x02\x02\u{3c4}\u{3c5}\x07\x07\x02\x02\u{3c5}\
	\u{b9}\x03\x02\x02\x02\u{3c6}\u{3c7}\x05\u{bc}\x5f\x02\u{3c7}\u{3c8}\x05\
	\u{be}\x60\x02\u{3c8}\u{3d4}\x03\x02\x02\x02\u{3c9}\u{3ca}\x05\u{bc}\x5f\
	\x02\u{3ca}\u{3ce}\x07\x60\x02\x02\u{3cb}\u{3cd}\x05\x70\x39\x02\u{3cc}\
	\u{3cb}\x03\x02\x02\x02\u{3cd}\u{3d0}\x03\x02\x02\x02\u{3ce}\u{3cc}\x03\
	\x02\x02\x02\u{3ce}\u{3cf}\x03\x02\x02\x02\u{3cf}\u{3d1}\x03\x02\x02\x02\
	\u{3d0}\u{3ce}\x03\x02\x02\x02\u{3d1}\u{3d2}\x07\x07\x02\x02\u{3d2}\u{3d4}\
	\x03\x02\x02\x02\u{3d3}\u{3c6}\x03\x02\x02\x02\u{3d3}\u{3c9}\x03\x02\x02\
	\x02\u{3d4}\u{bb}\x03\x02\x02\x02\u{3d5}\u{3d6}\x07\u{b2}\x02\x02\u{3d6}\
	\u{bd}\x03\x02\x02\x02\u{3d7}\u{3dd}\x05\u{c0}\x61\x02\u{3d8}\u{3dd}\x05\
	\u{c4}\x63\x02\u{3d9}\u{3dd}\x05\u{c6}\x64\x02\u{3da}\u{3dd}\x07\u{b1}\x02\
	\x02\u{3db}\u{3dd}\x07\u{b2}\x02\x02\u{3dc}\u{3d7}\x03\x02\x02\x02\u{3dc}\
	\u{3d8}\x03\x02\x02\x02\u{3dc}\u{3d9}\x03\x02\x02\x02\u{3dc}\u{3da}\x03\
	\x02\x02\x02\u{3dc}\u{3db}\x03\x02\x02\x02\u{3dd}\u{bf}\x03\x02\x02\x02\
	\u{3de}\u{3e0}\x07\u{80}\x02\x02\u{3df}\u{3e1}\x05\u{c2}\x62\x02\u{3e0}\
	\u{3df}\x03\x02\x02\x02\u{3e0}\u{3e1}\x03\x02\x02\x02\u{3e1}\u{3f6}\x03\
	\x02\x02\x02\u{3e2}\u{3e3}\x07\u{81}\x02\x02\u{3e3}\u{3f6}\x05\u{c2}\x62\
	\x02\u{3e4}\u{3f6}\x07\u{82}\x02\x02\u{3e5}\u{3e7}\x07\u{83}\x02\x02\u{3e6}\
	\u{3e8}\x05\u{c2}\x62\x02\u{3e7}\u{3e6}\x03\x02\x02\x02\u{3e7}\u{3e8}\x03\
	\x02\x02\x02\u{3e8}\u{3f6}\x03\x02\x02\x02\u{3e9}\u{3eb}\x07\u{84}\x02\x02\
	\u{3ea}\u{3ec}\x05\u{c2}\x62\x02\u{3eb}\u{3ea}\x03\x02\x02\x02\u{3eb}\u{3ec}\
	\x03\x02\x02\x02\u{3ec}\u{3f6}\x03\x02\x02\x02\u{3ed}\u{3f6}\x07\u{85}\x02\
	\x02\u{3ee}\u{3f6}\x07\u{86}\x02\x02\u{3ef}\u{3f6}\x07\u{87}\x02\x02\u{3f0}\
	\u{3f6}\x07\u{88}\x02\x02\u{3f1}\u{3f6}\x07\u{89}\x02\x02\u{3f2}\u{3f6}\
	\x07\u{8a}\x02\x02\u{3f3}\u{3f6}\x07\u{8b}\x02\x02\u{3f4}\u{3f6}\x07\x59\
	\x02\x02\u{3f5}\u{3de}\x03\x02\x02\x02\u{3f5}\u{3e2}\x03\x02\x02\x02\u{3f5}\
	\u{3e4}\x03\x02\x02\x02\u{3f5}\u{3e5}\x03\x02\x02\x02\u{3f5}\u{3e9}\x03\
	\x02\x02\x02\u{3f5}\u{3ed}\x03\x02\x02\x02\u{3f5}\u{3ee}\x03\x02\x02\x02\
	\u{3f5}\u{3ef}\x03\x02\x02\x02\u{3f5}\u{3f0}\x03\x02\x02\x02\u{3f5}\u{3f1}\
	\x03\x02\x02\x02\u{3f5}\u{3f2}\x03\x02\x02\x02\u{3f5}\u{3f3}\x03\x02\x02\
	\x02\u{3f5}\u{3f4}\x03\x02\x02\x02\u{3f6}\u{c1}\x03\x02\x02\x02\u{3f7}\u{3f8}\
	\x07\u{bb}\x02\x02\u{3f8}\u{3fb}\x07\u{ad}\x02\x02\u{3f9}\u{3fa}\x07\u{b6}\
	\x02\x02\u{3fa}\u{3fc}\x07\u{ad}\x02\x02\u{3fb}\u{3f9}\x03\x02\x02\x02\u{3fb}\
	\u{3fc}\x03\x02\x02\x02\u{3fc}\u{3fd}\x03\x02\x02\x02\u{3fd}\u{3fe}\x07\
	\u{bc}\x02\x02\u{3fe}\u{c3}\x03\x02\x02\x02\u{3ff}\u{400}\x07\x61\x02\x02\
	\u{400}\u{401}\x07\u{bb}\x02\x02\u{401}\u{402}\x05\u{be}\x60\x02\u{402}\
	\u{403}\x07\u{bc}\x02\x02\u{403}\u{411}\x03\x02\x02\x02\u{404}\u{405}\x07\
	\u{8c}\x02\x02\u{405}\u{406}\x07\u{bb}\x02\x02\u{406}\u{407}\x05\u{be}\x60\
	\x02\u{407}\u{408}\x07\u{bc}\x02\x02\u{408}\u{411}\x03\x02\x02\x02\u{409}\
	\u{40a}\x07\u{8d}\x02\x02\u{40a}\u{40b}\x07\u{bb}\x02\x02\u{40b}\u{40c}\
	\x05\u{be}\x60\x02\u{40c}\u{40d}\x07\u{b6}\x02\x02\u{40d}\u{40e}\x05\u{be}\
	\x60\x02\u{40e}\u{40f}\x07\u{bc}\x02\x02\u{40f}\u{411}\x03\x02\x02\x02\u{410}\
	\u{3ff}\x03\x02\x02\x02\u{410}\u{404}\x03\x02\x02\x02\u{410}\u{409}\x03\
	\x02\x02\x02\u{411}\u{c5}\x03\x02\x02\x02\u{412}\u{413}\x07\x60\x02\x02\
	\u{413}\u{417}\x07\u{c8}\x02\x02\u{414}\u{416}\x05\u{c8}\x65\x02\u{415}\
	\u{414}\x03\x02\x02\x02\u{416}\u{419}\x03\x02\x02\x02\u{417}\u{415}\x03\
	\x02\x02\x02\u{417}\u{418}\x03\x02\x02\x02\u{418}\u{41a}\x03\x02\x02\x02\
	\u{419}\u{417}\x03\x02\x02\x02\u{41a}\u{41b}\x07\u{c9}\x02\x02\u{41b}\u{c7}\
	\x03\x02\x02\x02\u{41c}\u{41d}\x05\x72\x3a\x02\u{41d}\u{421}\x05\u{be}\x60\
	\x02\u{41e}\u{420}\x05\u{ca}\x66\x02\u{41f}\u{41e}\x03\x02\x02\x02\u{420}\
	\u{423}\x03\x02\x02\x02\u{421}\u{41f}\x03\x02\x02\x02\u{421}\u{422}\x03\
	\x02\x02\x02\u{422}\u{c9}\x03\x02\x02\x02\u{423}\u{421}\x03\x02\x02\x02\
	\u{424}\u{42d}\x07\u{8e}\x02\x02\u{425}\u{42d}\x07\u{8f}\x02\x02\u{426}\
	\u{42d}\x07\u{90}\x02\x02\u{427}\u{42d}\x07\u{91}\x02\x02\u{428}\u{42d}\
	\x07\u{92}\x02\x02\u{429}\u{42d}\x05\u{cc}\x67\x02\u{42a}\u{42d}\x05\u{ce}\
	\x68\x02\u{42b}\u{42d}\x05\u{d0}\x69\x02\u{42c}\u{424}\x03\x02\x02\x02\u{42c}\
	\u{425}\x03\x02\x02\x02\u{42c}\u{426}\x03\x02\x02\x02\u{42c}\u{427}\x03\
	\x02\x02\x02\u{42c}\u{428}\x03\x02\x02\x02\u{42c}\u{429}\x03\x02\x02\x02\
	\u{42c}\u{42a}\x03\x02\x02\x02\u{42c}\u{42b}\x03\x02\x02\x02\u{42d}\u{cb}\
	\x03\x02\x02\x02\u{42e}\u{42f}\x07\u{ab}\x02\x02\u{42f}\u{430}\x07\u{b8}\
	\x02\x02\u{430}\u{433}\x07\u{b1}\x02\x02\u{431}\u{433}\x07\u{ab}\x02\x02\
	\u{432}\u{42e}\x03\x02\x02\x02\u{432}\u{431}\x03\x02\x02\x02\u{433}\u{cd}\
	\x03\x02\x02\x02\u{434}\u{435}\x07\x75\x02\x02\u{435}\u{438}\x07\u{bb}\x02\
	\x02\u{436}\u{439}\x05\u{ec}\x77\x02\u{437}\u{439}\x05\u{dc}\x6f\x02\u{438}\
	\u{436}\x03\x02\x02\x02\u{438}\u{437}\x03\x02\x02\x02\u{439}\u{43a}\x03\
	\x02\x02\x02\u{43a}\u{43b}\x07\u{bc}\x02\x02\u{43b}\u{cf}\x03\x02\x02\x02\
	\u{43c}\u{43d}\x07\x27\x02\x02\u{43d}\u{43e}\x07\u{bb}\x02\x02\u{43e}\u{43f}\
	\x07\u{b3}\x02\x02\u{43f}\u{445}\x07\u{bc}\x02\x02\u{440}\u{441}\x07\u{93}\
	\x02\x02\u{441}\u{442}\x07\u{bb}\x02\x02\u{442}\u{443}\x07\u{ac}\x02\x02\
	\u{443}\u{445}\x07\u{bc}\x02\x02\u{444}\u{43c}\x03\x02\x02\x02\u{444}\u{440}\
	\x03\x02\x02\x02\u{445}\u{d1}\x03\x02\x02\x02\u{446}\u{447}\x08\x6a\x01\
	\x02\u{447}\u{451}\x05\u{ec}\x77\x02\u{448}\u{451}\x05\u{de}\x70\x02\u{449}\
	\u{451}\x05\u{dc}\x6f\x02\u{44a}\u{451}\x05\u{e6}\x74\x02\u{44b}\u{44c}\
	\x07\u{bb}\x02\x02\u{44c}\u{44d}\x05\u{d2}\x6a\x02\u{44d}\u{44e}\x07\u{bc}\
	\x02\x02\u{44e}\u{451}\x03\x02\x02\x02\u{44f}\u{451}\x05\u{d4}\x6b\x02\u{450}\
	\u{446}\x03\x02\x02\x02\u{450}\u{448}\x03\x02\x02\x02\u{450}\u{449}\x03\
	\x02\x02\x02\u{450}\u{44a}\x03\x02\x02\x02\u{450}\u{44b}\x03\x02\x02\x02\
	\u{450}\u{44f}\x03\x02\x02\x02\u{451}\u{458}\x03\x02\x02\x02\u{452}\u{453}\
	\x0c\x05\x02\x02\u{453}\u{454}\x05\u{da}\x6e\x02\u{454}\u{455}\x05\u{d2}\
	\x6a\x06\u{455}\u{457}\x03\x02\x02\x02\u{456}\u{452}\x03\x02\x02\x02\u{457}\
	\u{45a}\x03\x02\x02\x02\u{458}\u{456}\x03\x02\x02\x02\u{458}\u{459}\x03\
	\x02\x02\x02\u{459}\u{d3}\x03\x02\x02\x02\u{45a}\u{458}\x03\x02\x02\x02\
	\u{45b}\u{45c}\x07\x67\x02\x02\u{45c}\u{45d}\x05\u{d6}\x6c\x02\u{45d}\u{45e}\
	\x07\u{b5}\x02\x02\u{45e}\u{466}\x05\u{d2}\x6a\x02\u{45f}\u{460}\x07\x67\
	\x02\x02\u{460}\u{461}\x05\u{d6}\x6c\x02\u{461}\u{462}\x07\u{b5}\x02\x02\
	\u{462}\u{463}\x05\u{d2}\x6a\x02\u{463}\u{465}\x03\x02\x02\x02\u{464}\u{45f}\
	\x03\x02\x02\x02\u{465}\u{468}\x03\x02\x02\x02\u{466}\u{464}\x03\x02\x02\
	\x02\u{466}\u{467}\x03\x02\x02\x02\u{467}\u{469}\x03\x02\x02\x02\u{468}\
	\u{466}\x03\x02\x02\x02\u{469}\u{46a}\x07\u{94}\x02\x02\u{46a}\u{46b}\x07\
	\u{b5}\x02\x02\u{46b}\u{46c}\x05\u{d2}\x6a\x02\u{46c}\u{d5}\x03\x02\x02\
	\x02\u{46d}\u{46e}\x05\u{d2}\x6a\x02\u{46e}\u{46f}\x05\u{d8}\x6d\x02\u{46f}\
	\u{470}\x05\u{d2}\x6a\x02\u{470}\u{47e}\x03\x02\x02\x02\u{471}\u{472}\x05\
	\u{d2}\x6a\x02\u{472}\u{473}\x07\x64\x02\x02\u{473}\u{474}\x05\u{d6}\x6c\
	\x02\u{474}\u{47e}\x03\x02\x02\x02\u{475}\u{476}\x05\u{d2}\x6a\x02\u{476}\
	\u{477}\x07\x65\x02\x02\u{477}\u{478}\x05\u{d6}\x6c\x02\u{478}\u{47e}\x03\
	\x02\x02\x02\u{479}\u{47a}\x07\u{bb}\x02\x02\u{47a}\u{47b}\x05\u{d6}\x6c\
	\x02\u{47b}\u{47c}\x07\u{bc}\x02\x02\u{47c}\u{47e}\x03\x02\x02\x02\u{47d}\
	\u{46d}\x03\x02\x02\x02\u{47d}\u{471}\x03\x02\x02\x02\u{47d}\u{475}\x03\
	\x02\x02\x02\u{47d}\u{479}\x03\x02\x02\x02\u{47e}\u{d7}\x03\x02\x02\x02\
	\u{47f}\u{480}\x09\x14\x02\x02\u{480}\u{d9}\x03\x02\x02\x02\u{481}\u{482}\
	\x09\x15\x02\x02\u{482}\u{db}\x03\x02\x02\x02\u{483}\u{484}\x07\u{b1}\x02\
	\x02\u{484}\u{48d}\x07\u{bb}\x02\x02\u{485}\u{48a}\x05\u{d2}\x6a\x02\u{486}\
	\u{487}\x07\u{b6}\x02\x02\u{487}\u{489}\x05\u{d2}\x6a\x02\u{488}\u{486}\
	\x03\x02\x02\x02\u{489}\u{48c}\x03\x02\x02\x02\u{48a}\u{488}\x03\x02\x02\
	\x02\u{48a}\u{48b}\x03\x02\x02\x02\u{48b}\u{48e}\x03\x02\x02\x02\u{48c}\
	\u{48a}\x03\x02\x02\x02\u{48d}\u{485}\x03\x02\x02\x02\u{48d}\u{48e}\x03\
	\x02\x02\x02\u{48e}\u{48f}\x03\x02\x02\x02\u{48f}\u{490}\x07\u{bc}\x02\x02\
	\u{490}\u{dd}\x03\x02\x02\x02\u{491}\u{496}\x07\u{b1}\x02\x02\u{492}\u{493}\
	\x07\u{b8}\x02\x02\u{493}\u{495}\x07\u{b1}\x02\x02\u{494}\u{492}\x03\x02\
	\x02\x02\u{495}\u{498}\x03\x02\x02\x02\u{496}\u{494}\x03\x02\x02\x02\u{496}\
	\u{497}\x03\x02\x02\x02\u{497}\u{df}\x03\x02\x02\x02\u{498}\u{496}\x03\x02\
	\x02\x02\u{499}\u{49e}\x05\u{de}\x70\x02\u{49a}\u{49b}\x07\u{b6}\x02\x02\
	\u{49b}\u{49d}\x05\u{de}\x70\x02\u{49c}\u{49a}\x03\x02\x02\x02\u{49d}\u{4a0}\
	\x03\x02\x02\x02\u{49e}\u{49c}\x03\x02\x02\x02\u{49e}\u{49f}\x03\x02\x02\
	\x02\u{49f}\u{e1}\x03\x02\x02\x02\u{4a0}\u{49e}\x03\x02\x02\x02\u{4a1}\u{4a2}\
	\x07\u{b9}\x02\x02\u{4a2}\u{4a7}\x05\u{de}\x70\x02\u{4a3}\u{4a4}\x07\u{b6}\
	\x02\x02\u{4a4}\u{4a6}\x05\u{de}\x70\x02\u{4a5}\u{4a3}\x03\x02\x02\x02\u{4a6}\
	\u{4a9}\x03\x02\x02\x02\u{4a7}\u{4a5}\x03\x02\x02\x02\u{4a7}\u{4a8}\x03\
	\x02\x02\x02\u{4a8}\u{4aa}\x03\x02\x02\x02\u{4a9}\u{4a7}\x03\x02\x02\x02\
	\u{4aa}\u{4ab}\x07\u{ba}\x02\x02\u{4ab}\u{4af}\x03\x02\x02\x02\u{4ac}\u{4ad}\
	\x07\u{b9}\x02\x02\u{4ad}\u{4af}\x07\u{ba}\x02\x02\u{4ae}\u{4a1}\x03\x02\
	\x02\x02\u{4ae}\u{4ac}\x03\x02\x02\x02\u{4af}\u{e3}\x03\x02\x02\x02\u{4b0}\
	\u{4b1}\x07\u{ad}\x02\x02\u{4b1}\u{4b4}\x05\u{e6}\x74\x02\u{4b2}\u{4b4}\
	\x07\u{af}\x02\x02\u{4b3}\u{4b0}\x03\x02\x02\x02\u{4b3}\u{4b2}\x03\x02\x02\
	\x02\u{4b4}\u{e5}\x03\x02\x02\x02\u{4b5}\u{4b6}\x09\x16\x02\x02\u{4b6}\u{e7}\
	\x03\x02\x02\x02\u{4b7}\u{4b8}\x07\u{ad}\x02\x02\u{4b8}\u{4b9}\x05\u{ea}\
	\x76\x02\u{4b9}\u{e9}\x03\x02\x02\x02\u{4ba}\u{4bb}\x09\x17\x02\x02\u{4bb}\
	\u{eb}\x03\x02\x02\x02\u{4bc}\u{4c1}\x07\u{b3}\x02\x02\u{4bd}\u{4c1}\x05\
	\u{ee}\x78\x02\u{4be}\u{4c1}\x07\u{b0}\x02\x02\u{4bf}\u{4c1}\x07\u{a9}\x02\
	\x02\u{4c0}\u{4bc}\x03\x02\x02\x02\u{4c0}\u{4bd}\x03\x02\x02\x02\u{4c0}\
	\u{4be}\x03\x02\x02\x02\u{4c0}\u{4bf}\x03\x02\x02\x02\u{4c1}\u{ed}\x03\x02\
	\x02\x02\u{4c2}\u{4c9}\x07\u{ad}\x02\x02\u{4c3}\u{4c9}\x07\u{ae}\x02\x02\
	\u{4c4}\u{4c5}\x07\u{c4}\x02\x02\u{4c5}\u{4c9}\x07\u{ad}\x02\x02\u{4c6}\
	\u{4c7}\x07\u{c4}\x02\x02\u{4c7}\u{4c9}\x07\u{ae}\x02\x02\u{4c8}\u{4c2}\
	\x03\x02\x02\x02\u{4c8}\u{4c3}\x03\x02\x02\x02\u{4c8}\u{4c4}\x03\x02\x02\
	\x02\u{4c8}\u{4c6}\x03\x02\x02\x02\u{4c9}\u{ef}\x03\x02\x02\x02\x7c\u{f3}\
	\u{f8}\u{fa}\u{104}\u{110}\u{113}\u{116}\u{119}\u{11c}\u{11f}\u{122}\u{125}\
	\u{128}\u{12d}\u{131}\u{134}\u{137}\u{13a}\u{13d}\u{140}\u{145}\u{149}\u{150}\
	\u{158}\u{165}\u{16d}\u{170}\u{173}\u{176}\u{186}\u{18b}\u{19b}\u{1af}\u{1b8}\
	\u{1d5}\u{1db}\u{1e4}\u{1e8}\u{1ee}\u{1fb}\u{203}\u{214}\u{21c}\u{227}\u{231}\
	\u{235}\u{239}\u{241}\u{245}\u{249}\u{251}\u{259}\u{26d}\u{276}\u{27a}\u{27f}\
	\u{286}\u{28c}\u{299}\u{29f}\u{2a4}\u{2aa}\u{2bd}\u{2cf}\u{2d1}\u{2df}\u{2e6}\
	\u{2ea}\u{2ed}\u{2f0}\u{2fb}\u{2fe}\u{305}\u{30a}\u{314}\u{31c}\u{31e}\u{32b}\
	\u{331}\u{341}\u{344}\u{34c}\u{355}\u{368}\u{36e}\u{377}\u{380}\u{386}\u{38f}\
	\u{39a}\u{3a6}\u{3ae}\u{3bc}\u{3c2}\u{3ce}\u{3d3}\u{3dc}\u{3e0}\u{3e7}\u{3eb}\
	\u{3f5}\u{3fb}\u{410}\u{417}\u{421}\u{42c}\u{432}\u{438}\u{444}\u{450}\u{458}\
	\u{466}\u{47d}\u{48a}\u{48d}\u{496}\u{49e}\u{4a7}\u{4ae}\u{4b3}\u{4c0}\u{4c8}";

