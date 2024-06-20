use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::*,
        visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
    },
    plugin::{
        metadata::TransformPluginMetadataContextKind, plugin_transform,
        proxies::TransformPluginProgramMetadata,
    },
};

static MODULE_NAME: &str = "@linaria/react";
static STYLED_TAGGED_TEMPLATE: &str = "styled";
static NAME_PROP: &str = "name";
static CLASS_PROP: &str = "class";

pub struct TransformVisitor {
    pub file_name: String,
    pub import_name: Option<Ident>,
}

impl VisitMut for TransformVisitor {
    fn visit_mut_var_declarator(&mut self, n: &mut VarDeclarator) {
        n.visit_mut_children_with(self);

        let Some(name) = n.name.as_ident() else {
            return;
        };
        let Some(member_expr) = n
            .init
            .as_ref()
            .and_then(|f| f.as_tagged_tpl())
            .and_then(|f| f.tag.as_member())
        else {
            return;
        };
        let Some(object) = member_expr.obj.as_ident() else {
            return;
        };
        let Some(property) = member_expr.prop.as_ident() else {
            return;
        };

        if object.sym != STYLED_TAGGED_TEMPLATE {
            return;
        }

        n.init = Some(Box::new(Expr::Call(CallExpr {
            args: vec![Expr::Object(ObjectLit {
                span: DUMMY_SP,
                props: vec![
                    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                        key: PropName::Ident(Ident::new(NAME_PROP.into(), DUMMY_SP)),
                        value: Box::new(Expr::Lit(Lit::Str(Str {
                            value: name.sym.clone(),
                            span: DUMMY_SP,
                            raw: None,
                        }))),
                    }))),
                    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                        key: PropName::Ident(Ident::new(CLASS_PROP.into(), DUMMY_SP)),
                        value: Box::new(Expr::Lit(Lit::Str(Str {
                            value: self.file_name.clone().into(),
                            span: DUMMY_SP,
                            raw: None,
                        }))),
                    }))),
                ],
            })
            .into()],
            span: DUMMY_SP,
            type_args: None,
            callee: Callee::Expr(Box::new(Expr::Call(CallExpr {
                args: vec![Expr::Lit(Lit::Str(Str {
                    value: property.sym.clone(),
                    span: DUMMY_SP,
                    raw: None,
                }))
                .into()],
                span: DUMMY_SP,
                type_args: None,
                callee: Callee::Expr(self.import_name.clone().unwrap().into()),
            }))),
        })));
    }

    fn visit_mut_module_decl(&mut self, n: &mut ModuleDecl) {
        n.visit_mut_children_with(self);

        match n {
            ModuleDecl::Import(import_stmt) => {
                if import_stmt.src.value == MODULE_NAME {
                    self.import_name = import_stmt.specifiers.iter().find_map(|specifier| {
                        if let ImportSpecifier::Named(named_import) = specifier {
                            Some(named_import.local.clone())
                        } else {
                            None
                        }
                    });
                }
            }
            _ => {}
        }
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    let file_name = _metadata
        .get_context(&TransformPluginMetadataContextKind::Filename)
        .unwrap_or_default();

    program.fold_with(&mut as_folder(TransformVisitor {
        file_name,
        import_name: None,
    }))
}
