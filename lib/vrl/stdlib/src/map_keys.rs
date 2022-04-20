use vrl::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct MapKeys;

impl Function for MapKeys {
    fn identifier(&self) -> &'static str {
        "map_keys"
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[
            Parameter {
                keyword: "value",
                kind: kind::OBJECT,
                required: true,
            },
            Parameter {
                keyword: "recursive",
                kind: kind::BOOLEAN,
                required: false,
            },
        ]
    }

    fn examples(&self) -> &'static [Example] {
        &[
            Example {
                title: "map object keys",
                source: r#"map_keys({ "a": 1, "b": 2 }) -> |key| { upcase(key) }"#,
                result: Ok(r#"{ "A": 1, "B": 2 }"#),
            },
            Example {
                title: "recursively map object keys",
                source: r#"map_keys({ "a": 1, "b": [{ "c": 2 }, { "d": 3 }], "e": { "f": 4 } }, recursive: true) -> |key| { upcase(key) }"#,
                result: Ok(r#"{ "A": 1, "B": [{ "C": 2 }, { "D": 3 }], "E": { "F": 4 } }"#),
            },
        ]
    }

    fn compile(
        &self,
        _state: (&mut state::LocalEnv, &mut state::ExternalEnv),
        _ctx: &mut FunctionCompileContext,
        mut arguments: ArgumentList,
    ) -> Compiled {
        let value = arguments.required("value");
        let recursive = arguments.optional("recursive");
        let closure = arguments.required_closure()?;

        Ok(Box::new(MapKeysFn {
            value,
            closure,
            recursive,
        }))
    }

    fn closure(&self) -> Option<closure::Definition> {
        let object = closure::Input {
            parameter_keyword: "value",
            kind: Kind::object(Collection::any()),
            variables: vec![closure::Variable {
                kind: Kind::bytes(),
            }],
            output: closure::Output::Scalar {
                kind: Kind::bytes(),
            },
            example: Example {
                title: "map object keys",
                source: r#"map_keys({ "one" : 1, "two": 2 }) -> |key| { upcase(key) }"#,
                result: Ok(r#"{ "ONE": 1, "TWO": 2 }"#),
            },
        };

        Some(closure::Definition {
            inputs: vec![object],
        })
    }

    fn call_by_vm(&self, _ctx: &mut Context, _args: &mut VmArgumentList) -> Result<Value> {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct MapKeysFn {
    value: Box<dyn Expression>,
    recursive: Option<Box<dyn Expression>>,
    closure: FunctionClosure,
}

impl Expression for MapKeysFn {
    fn resolve(&self, ctx: &mut Context) -> Result<Value> {
        let recursive = match &self.recursive {
            None => false,
            Some(expr) => expr.resolve(ctx)?.try_boolean()?,
        };

        let value = self.value.resolve(ctx)?;
        let mut iter = value.into_iter(recursive);

        for item in iter.by_ref() {
            if let IterItem::KeyValue(key, _) = item {
                self.closure.map_key(ctx, key)?;
            }
        }

        Ok(iter.into())
    }

    fn type_def(&self, ctx: (&state::LocalEnv, &state::ExternalEnv)) -> TypeDef {
        let type_def = self.closure.type_def(ctx);

        TypeDef::object(Collection::from_unknown(type_def.kind().clone()))
            .with_fallibility(type_def.is_fallible())
    }
}
