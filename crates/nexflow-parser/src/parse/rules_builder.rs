// NexCore -- Nexflow Parser: RulesDSL Builder
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Builds a typed `RulesProgram` AST from RulesDSL source text.

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::ParseTree;
use antlr_rust::InputStream;

use crate::ast::common::*;
use crate::ast::rules::*;
use crate::generated::rulesdsllexer::RulesDSLLexer;
use crate::generated::rulesdslparser::*;
use crate::parse::helpers::{rules_text as terminal_text, unquote};

/// Parse a `.rules` source string into a typed `RulesProgram`.
pub fn parse_rules(source: &str) -> Result<RulesProgram, String> {
    let input = InputStream::new(source);
    let lexer = RulesDSLLexer::new(input);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = RulesDSLParser::new(token_stream);

    let tree = parser
        .program()
        .map_err(|e| format!("Parse error: {e:?}"))?;

    build_program(&*tree)
}

fn build_program(ctx: &ProgramContext<'_>) -> Result<RulesProgram, String> {
    let imports: Vec<ImportPath> = ctx
        .importStatement_all()
        .iter()
        .filter_map(|s| {
            s.importPath().map(|p| ImportPath {
                raw: p.get_text(),
            })
        })
        .collect();

    let services: Vec<ServiceDecl> = ctx
        .servicesBlock()
        .map(|sb| {
            sb.serviceDecl_all()
                .iter()
                .map(|sd| build_service_decl(sd))
                .collect()
        })
        .unwrap_or_default();

    let actions: Vec<ActionDecl> = ctx
        .actionsBlock()
        .map(|ab| {
            ab.actionDecl_all()
                .iter()
                .map(|ad| build_action_decl(ad))
                .collect()
        })
        .unwrap_or_default();

    let decision_tables: Vec<DecisionTableDef> = ctx
        .decisionTableDef_all()
        .iter()
        .map(|dt| build_decision_table(dt))
        .collect();

    let procedural_rules: Vec<ProceduralRuleDef> = ctx
        .proceduralRuleDef_all()
        .iter()
        .map(|pr| build_procedural_rule(pr))
        .collect();

    Ok(RulesProgram {
        imports,
        services,
        actions,
        decision_tables,
        procedural_rules,
    })
}

// -- Service declarations --

fn build_service_decl(ctx: &ServiceDeclContext<'_>) -> ServiceDecl {
    let name = ctx
        .serviceName()
        .map(|sn| sn.get_text())
        .unwrap_or_default();
    let class_name = ctx
        .serviceClassName()
        .map(|cn| cn.get_text())
        .unwrap_or_default();
    let method_name = ctx
        .serviceMethodName()
        .map(|mn| mn.get_text())
        .unwrap_or_default();
    let return_type = ctx
        .serviceReturnType()
        .map(|rt| rt.get_text())
        .unwrap_or_default();

    // Service type: sync, async, cached(duration)
    let service_type = ctx
        .serviceType()
        .map(|st| {
            let text = st.get_text();
            if text.starts_with("cached") {
                let dur = st.duration().map(|d| d.get_text()).unwrap_or_default();
                ServiceType::Cached(dur)
            } else if text == "async" {
                ServiceType::Async
            } else {
                ServiceType::Sync
            }
        })
        .unwrap_or(ServiceType::Sync);

    let params: Vec<ServiceParam> = ctx
        .serviceParamList()
        .map(|spl| {
            spl.serviceParam_all()
                .iter()
                .map(|sp| ServiceParam {
                    name: sp.IDENTIFIER().map(|id| terminal_text(&*id)).unwrap_or_default(),
                    param_type: sp.paramType().map(|pt| pt.get_text()).unwrap_or_default(),
                })
                .collect()
        })
        .unwrap_or_default();

    // Service options: timeout, fallback, retry
    let mut timeout = None;
    let mut fallback = None;
    let mut retry = None;

    if let Some(opts) = ctx.serviceOptions() {
        for opt in opts.serviceOption_all() {
            let text = opt.get_text();
            if text.starts_with("timeout") {
                timeout = opt.duration().map(|d| d.get_text());
            } else if text.starts_with("fallback") {
                fallback = opt.literal().map(|l| l.get_text());
            } else if text.starts_with("retry") {
                retry = opt.INTEGER().and_then(|n| terminal_text(&*n).parse().ok());
            }
        }
    }

    ServiceDecl {
        name,
        service_type,
        class_name,
        method_name,
        params,
        return_type,
        timeout,
        fallback,
        retry,
    }
}

// -- Action declarations --

fn build_action_decl(ctx: &ActionDeclContext<'_>) -> ActionDecl {
    let name = ctx
        .actionDeclName()
        .map(|adn| adn.get_text())
        .unwrap_or_default();

    let params: Vec<ServiceParam> = ctx
        .actionParamList()
        .map(|apl| {
            apl.actionParam_all()
                .iter()
                .map(|ap| ServiceParam {
                    name: ap.IDENTIFIER().map(|id| terminal_text(&*id)).unwrap_or_default(),
                    param_type: ap.paramType().map(|pt| pt.get_text()).unwrap_or_default(),
                })
                .collect()
        })
        .unwrap_or_default();

    // Action target: emit/state/audit/call
    let target = ctx
        .actionTarget()
        .map(|at| build_action_target(&*at))
        .unwrap_or(ActionTarget::Audit);

    ActionDecl {
        name,
        params,
        target,
    }
}

fn build_action_target(ctx: &ActionTargetContext<'_>) -> ActionTarget {
    if let Some(et) = ctx.emitTarget() {
        let stream = et.IDENTIFIER().map(|id| terminal_text(&*id)).unwrap_or_default();
        return ActionTarget::Emit { stream };
    }
    if let Some(st) = ctx.stateTarget() {
        let state_name = st.IDENTIFIER().map(|id| terminal_text(&*id)).unwrap_or_default();
        let (operation, arg) = st
            .stateOperation()
            .map(|so| {
                let op = so.IDENTIFIER().map(|id| terminal_text(&*id)).unwrap_or_default();
                let arg = so.stateOperationArg().map(|soa| {
                    let t = soa.get_text();
                    unquote(&t)
                });
                (op, arg)
            })
            .unwrap_or_default();
        return ActionTarget::State {
            state_name,
            operation,
            arg,
        };
    }
    if ctx.auditTarget().is_some() {
        return ActionTarget::Audit;
    }
    if let Some(ct) = ctx.callTarget() {
        let ids: Vec<_> = ct.IDENTIFIER_all();
        let class = ids.first().map(|id| terminal_text(id)).unwrap_or_default();
        let method = ids.get(1).map(|id| terminal_text(id)).unwrap_or_default();
        return ActionTarget::Call { class, method };
    }
    ActionTarget::Audit
}

// -- Decision tables --

fn build_decision_table(ctx: &DecisionTableDefContext<'_>) -> DecisionTableDef {
    let name = ctx
        .tableName()
        .map(|tn| tn.get_text())
        .unwrap_or_default();

    let hit_policy = ctx.hitPolicyDecl().and_then(|hpd| {
        hpd.hitPolicyType().map(|hpt| {
            let text = hpt.get_text();
            match text.as_str() {
                "first_match" => HitPolicy::FirstMatch,
                "single_hit" => HitPolicy::SingleHit,
                "multi_hit" => HitPolicy::MultiHit,
                "collect_all" => HitPolicy::CollectAll,
                _ => HitPolicy::FirstMatch,
            }
        })
    });

    let description = ctx
        .descriptionDecl()
        .and_then(|dd| dd.stringLiteral())
        .map(|sl| unquote(&sl.get_text()));

    let version = ctx
        .versionDecl()
        .and_then(|vd| vd.VERSION_NUMBER())
        .map(|v| terminal_text(&*v));

    let inputs: Vec<InputParam> = ctx
        .givenBlock()
        .map(|gb| {
            gb.inputParam_all()
                .iter()
                .map(|ip| InputParam {
                    name: ip.paramName().map(|pn| pn.get_text()).unwrap_or_default(),
                    param_type: ip.paramType().map(|pt| pt.get_text()).unwrap_or_default(),
                })
                .collect()
        })
        .unwrap_or_default();

    let (columns, rows) = ctx
        .decideBlock()
        .and_then(|db| db.tableMatrix())
        .map(|tm| build_table_matrix(&*tm))
        .unwrap_or_default();

    let returns: Vec<ReturnParam> = ctx
        .returnSpec()
        .map(|rs| {
            rs.returnParam_all()
                .iter()
                .map(|rp| ReturnParam {
                    name: rp.paramName().map(|pn| pn.get_text()).unwrap_or_default(),
                    param_type: rp.paramType().map(|pt| pt.get_text()).unwrap_or_default(),
                })
                .collect()
        })
        .unwrap_or_default();

    let execute = ctx
        .executeSpec()
        .and_then(|es| es.executeType())
        .map(|et| et.get_text());

    let post_calculate: Vec<PostCalculateStmt> = ctx
        .postCalculateBlock()
        .map(|pcb| {
            pcb.postCalculateStatement_all()
                .iter()
                .map(|pcs| {
                    if let Some(ls) = pcs.letStatement() {
                        PostCalculateStmt {
                            name: ls.IDENTIFIER().map(|id| terminal_text(&*id)).unwrap_or_default(),
                            expression: ls.valueExpr().map(|ve| ve.get_text()).unwrap_or_default(),
                            is_let: true,
                        }
                    } else if let Some(as_stmt) = pcs.assignmentStatement() {
                        let name = as_stmt.IDENTIFIER().map(|id| terminal_text(&*id)).unwrap_or_default();
                        let expression = as_stmt
                            .valueExpr()
                            .map(|ve| ve.get_text())
                            .or_else(|| as_stmt.whenExpression().map(|we| we.get_text()))
                            .unwrap_or_default();
                        PostCalculateStmt {
                            name,
                            expression,
                            is_let: false,
                        }
                    } else {
                        let text = pcs.get_text();
                        let parts: Vec<&str> = text.splitn(2, '=').collect();
                        PostCalculateStmt {
                            name: parts.first().unwrap_or(&"").trim().to_string(),
                            expression: parts.get(1).unwrap_or(&"").trim().to_string(),
                            is_let: false,
                        }
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    let aggregate: Vec<AggregateStmt> = ctx
        .aggregateBlock()
        .map(|ab| {
            ab.aggregateStatement_all()
                .iter()
                .map(|as_stmt| {
                    let name = as_stmt
                        .IDENTIFIER()
                        .map(|id| terminal_text(&*id))
                        .unwrap_or_default();
                    let expression = as_stmt
                        .valueExpr()
                        .map(|ve| ve.get_text())
                        .unwrap_or_default();
                    AggregateStmt { name, expression }
                })
                .collect()
        })
        .unwrap_or_default();

    DecisionTableDef {
        name,
        hit_policy,
        description,
        version,
        inputs,
        columns,
        rows,
        returns,
        execute,
        post_calculate,
        aggregate,
    }
}

fn build_table_matrix(ctx: &TableMatrixContext<'_>) -> (Vec<String>, Vec<TableRow>) {
    let columns: Vec<String> = ctx
        .tableHeader()
        .map(|th| {
            th.columnHeader_all()
                .iter()
                .map(|ch| {
                    ch.columnName()
                        .map(|cn| cn.get_text())
                        .unwrap_or_default()
                })
                .collect()
        })
        .unwrap_or_default();

    let rows: Vec<TableRow> = ctx
        .tableRow_all()
        .iter()
        .map(|tr| {
            let priority = tr
                .priorityCell()
                .and_then(|pc| pc.INTEGER())
                .and_then(|n| terminal_text(&*n).parse().ok());
            let cells: Vec<CellContent> = tr
                .cell_all()
                .iter()
                .map(|c| {
                    c.cellContent()
                        .map(|cc| build_cell_content(&*cc))
                        .unwrap_or(CellContent::Wildcard)
                })
                .collect();
            TableRow { priority, cells }
        })
        .collect();

    (columns, rows)
}

fn build_cell_content(ctx: &CellContentContext<'_>) -> CellContent {
    if let Some(cond) = ctx.condition() {
        return build_condition(&*cond);
    }
    if let Some(act) = ctx.action() {
        return build_action(&*act);
    }
    CellContent::Wildcard
}

fn build_condition(ctx: &ConditionContext<'_>) -> CellContent {
    // Wildcard: *
    if ctx.STAR().is_some() {
        return CellContent::Wildcard;
    }
    // Exact match
    if let Some(em) = ctx.exactMatch() {
        return CellContent::Condition(ConditionExpr::ExactMatch(em.get_text()));
    }
    // Range: numberLiteral TO numberLiteral
    if let Some(rc) = ctx.rangeCondition() {
        let nums: Vec<_> = rc.numberLiteral_all();
        let money: Vec<_> = rc.MONEY_LITERAL_all();
        let from = nums.first().map(|n| n.get_text())
            .or_else(|| money.first().map(|m| terminal_text(m)))
            .unwrap_or_default();
        let to = nums.get(1).map(|n| n.get_text())
            .or_else(|| money.get(1).map(|m| terminal_text(m)))
            .unwrap_or_default();
        return CellContent::Condition(ConditionExpr::Range { from, to });
    }
    // Set: in (a, b, c)
    if let Some(sc) = ctx.setCondition() {
        let negated = sc.NOT().is_some();
        let values: Vec<String> = sc
            .valueList()
            .map(|vl| {
                vl.valueExpr_all()
                    .iter()
                    .map(|ve| ve.get_text())
                    .collect()
            })
            .unwrap_or_default();
        return CellContent::Condition(ConditionExpr::InSet { values, negated });
    }
    // Pattern: matches/starts_with/ends_with/contains
    if let Some(pc) = ctx.patternCondition() {
        let text = pc.get_text();
        let kind = if text.starts_with("matches") {
            "matches"
        } else if text.starts_with("starts_with") {
            "starts_with"
        } else if text.starts_with("ends_with") {
            "ends_with"
        } else {
            "contains"
        };
        let pattern = pc
            .stringLiteral()
            .map(|sl| unquote(&sl.get_text()))
            .unwrap_or_default();
        return CellContent::Condition(ConditionExpr::Pattern {
            kind: kind.to_string(),
            pattern,
        });
    }
    // Null check
    if let Some(nc) = ctx.nullCondition() {
        let text = nc.get_text();
        let is_null = !text.contains("not");
        return CellContent::Condition(ConditionExpr::NullCheck { is_null });
    }
    // Comparison: operator + value
    if let Some(cc) = ctx.comparisonCondition() {
        let operator = cc
            .comparisonOp()
            .map(|co| co.get_text())
            .unwrap_or_default();
        let value = cc
            .valueExpr()
            .map(|ve| ve.get_text())
            .unwrap_or_default();
        return CellContent::Condition(ConditionExpr::Comparison { operator, value });
    }
    // Expression condition
    if let Some(ec) = ctx.expressionCondition() {
        let expr = ec
            .booleanExpr()
            .map(|be| be.get_text())
            .unwrap_or_default();
        return CellContent::Condition(ConditionExpr::Expression(expr));
    }
    // Marker state
    if let Some(mc) = ctx.markerStateCondition() {
        let ids: Vec<_> = mc.IDENTIFIER_all();
        let marker = ids.first().map(|id| terminal_text(id)).unwrap_or_default();
        let state = if mc.FIRED().is_some() {
            "fired".to_string()
        } else if mc.PENDING().is_some() {
            "pending".to_string()
        } else {
            "between".to_string()
        };
        return CellContent::Condition(ConditionExpr::MarkerState { marker, state });
    }

    CellContent::Wildcard
}

fn build_action(ctx: &ActionContext<'_>) -> CellContent {
    // Wildcard: *
    if ctx.STAR().is_some() {
        return CellContent::Wildcard;
    }
    // Assign: literal
    if let Some(aa) = ctx.assignAction() {
        return CellContent::Action(ActionExpr::Assign(aa.get_text()));
    }
    // Calculate: arithmetic expression
    if let Some(ca) = ctx.calculateAction() {
        return CellContent::Action(ActionExpr::Calculate(ca.get_text()));
    }
    // Lookup
    if let Some(la) = ctx.lookupAction() {
        let table = la.IDENTIFIER().map(|id| terminal_text(&*id)).unwrap_or_default();
        let args: Vec<String> = la
            .valueExpr_all()
            .iter()
            .map(|ve| ve.get_text())
            .collect();
        // Check for default
        let default = if la.DEFAULT().is_some() {
            args.last().cloned()
        } else {
            None
        };
        return CellContent::Action(ActionExpr::Lookup {
            table,
            args,
            default,
        });
    }
    // Call: function(args)
    if let Some(ca) = ctx.callAction() {
        let function = ca
            .IDENTIFIER()
            .map(|id| terminal_text(&*id))
            .unwrap_or_default();
        let args: Vec<String> = ca
            .actionArg_all()
            .iter()
            .map(|aa| aa.get_text())
            .collect();
        return CellContent::Action(ActionExpr::Call { function, args });
    }
    // Emit
    if let Some(ea) = ctx.emitAction() {
        let target = ea.IDENTIFIER().map(|id| terminal_text(&*id)).unwrap_or_default();
        return CellContent::Action(ActionExpr::Emit { target });
    }

    CellContent::Wildcard
}

// -- Procedural rules --

fn build_procedural_rule(ctx: &ProceduralRuleDefContext<'_>) -> ProceduralRuleDef {
    let name = ctx
        .ruleName()
        .map(|rn| {
            let text = rn.get_text();
            unquote(&text)
        })
        .unwrap_or_default();

    let description = ctx
        .descriptionDecl()
        .and_then(|dd| dd.stringLiteral())
        .map(|sl| unquote(&sl.get_text()));

    let body: Vec<BlockItem> = ctx
        .blockItem_all()
        .iter()
        .map(|bi| build_block_item(bi))
        .collect();

    ProceduralRuleDef {
        name,
        description,
        body,
    }
}

fn build_block_item(ctx: &BlockItemContext<'_>) -> BlockItem {
    if let Some(rs) = ctx.ruleStep() {
        return build_rule_step(&*rs);
    }
    if let Some(ss) = ctx.setStatement() {
        let name = ss
            .IDENTIFIER()
            .map(|id| terminal_text(&*id))
            .unwrap_or_default();
        let expression = ss
            .valueExpr()
            .map(|ve| ve.get_text())
            .unwrap_or_default();
        return BlockItem::Set { name, expression };
    }
    if let Some(ls) = ctx.letStatement() {
        let name = ls
            .IDENTIFIER()
            .map(|id| terminal_text(&*id))
            .unwrap_or_default();
        let expression = ls
            .valueExpr()
            .map(|ve| ve.get_text())
            .unwrap_or_default();
        return BlockItem::Let { name, expression };
    }
    if let Some(as_seq) = ctx.actionSequence() {
        // Parse action calls from the sequence
        let calls: Vec<_> = as_seq.actionCall_all();
        if let Some(first) = calls.first() {
            let name = first
                .IDENTIFIER()
                .map(|id| terminal_text(&*id))
                .or_else(|| first.DQUOTED_STRING().map(|s| unquote(&terminal_text(&*s))))
                .unwrap_or_default();
            let args: Vec<String> = first
                .parameterList()
                .map(|pl| {
                    pl.parameter_all()
                        .iter()
                        .map(|p| p.get_text())
                        .collect()
                })
                .unwrap_or_default();
            return BlockItem::ActionCall { name, args };
        }
        return BlockItem::ActionCall {
            name: as_seq.get_text(),
            args: Vec::new(),
        };
    }
    if ctx.returnStatement().is_some() {
        return BlockItem::Return;
    }
    BlockItem::ActionCall {
        name: ctx.get_text(),
        args: Vec::new(),
    }
}

fn build_rule_step(ctx: &RuleStepContext<'_>) -> BlockItem {
    let bool_exprs: Vec<_> = ctx.booleanExpr_all();
    let blocks: Vec<_> = ctx.block_all();

    let condition = bool_exprs
        .first()
        .map(|be| be.get_text())
        .unwrap_or_default();

    let then_block: Vec<BlockItem> = blocks
        .first()
        .map(|b| {
            b.blockItem_all()
                .iter()
                .map(|bi| build_block_item(bi))
                .collect()
        })
        .unwrap_or_default();

    // elseif blocks: pairs of (condition, block) starting from index 1
    let mut elseif_blocks: Vec<(String, Vec<BlockItem>)> = Vec::new();
    for i in 1..bool_exprs.len() {
        let cond = bool_exprs[i].get_text();
        let block_items: Vec<BlockItem> = blocks
            .get(i)
            .map(|b| {
                b.blockItem_all()
                    .iter()
                    .map(|bi| build_block_item(bi))
                    .collect()
            })
            .unwrap_or_default();
        elseif_blocks.push((cond, block_items));
    }

    // else block is the last block if there are more blocks than conditions
    let else_block: Vec<BlockItem> = if blocks.len() > bool_exprs.len() {
        blocks
            .last()
            .map(|b| {
                b.blockItem_all()
                    .iter()
                    .map(|bi| build_block_item(bi))
                    .collect()
            })
            .unwrap_or_default()
    } else {
        Vec::new()
    };

    BlockItem::IfThenElse {
        condition,
        then_block,
        elseif_blocks,
        else_block,
    }
}
