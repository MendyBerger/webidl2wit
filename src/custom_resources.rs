use crate::translations::State;

pub(super) enum TypedArrayKind {
    UInt8,
    UInt8Clamped,
    UInt16,
    UInt32,
    Int8,
    Int16,
    Int32,
    Float32,
    Float64,
}
impl TypedArrayKind {
    fn to_wit_encoder_type(&self) -> wit_encoder::Type {
        match self {
            TypedArrayKind::UInt8 => wit_encoder::Type::U8,
            TypedArrayKind::UInt8Clamped => wit_encoder::Type::U8,
            TypedArrayKind::UInt16 => wit_encoder::Type::U16,
            TypedArrayKind::UInt32 => wit_encoder::Type::U32,
            TypedArrayKind::Int8 => wit_encoder::Type::S8,
            TypedArrayKind::Int16 => wit_encoder::Type::S16,
            TypedArrayKind::Int32 => wit_encoder::Type::S32,
            TypedArrayKind::Float32 => wit_encoder::Type::F32,
            TypedArrayKind::Float64 => wit_encoder::Type::F64,
        }
    }
    fn prefix_name(&self) -> &'static str {
        match self {
            TypedArrayKind::UInt8 => "uint8",
            TypedArrayKind::UInt8Clamped => "uint8-clamped",
            TypedArrayKind::UInt16 => "uint16",
            TypedArrayKind::UInt32 => "uint32",
            TypedArrayKind::Int8 => "int8",
            TypedArrayKind::Int16 => "int16",
            TypedArrayKind::Int32 => "int32",
            TypedArrayKind::Float32 => "float32",
            TypedArrayKind::Float64 => "float64",
        }
    }
}

impl<'a> State<'a> {
    pub(super) fn add_array_buffer<'b>(&mut self) -> anyhow::Result<wit_encoder::Ident> {
        let buffer_name = wit_encoder::Ident::new("array-buffer");
        if !self.type_def_exists(&buffer_name) {
            let constructor_options_name =
                wit_encoder::Ident::new(format!("array-buffer-constructor-options"));
            let constructor_options = wit_encoder::TypeDef::record(
                constructor_options_name.clone(),
                [("max-byte-length", wit_encoder::Type::U32)],
            );
            self.interface
                .items_mut()
                .push(wit_encoder::InterfaceItem::TypeDef(constructor_options));

            let array = wit_encoder::TypeDef::resource(
                buffer_name.clone(),
                [
                    {
                        let mut func = wit_encoder::ResourceFunc::constructor();
                        func.params(wit_encoder::Params::from_iter([
                            ("length", wit_encoder::Type::U32),
                            (
                                "options",
                                wit_encoder::Type::option(wit_encoder::Type::named(
                                    constructor_options_name.clone(),
                                )),
                            ),
                        ]));
                        func
                    },
                    {
                        let mut func = wit_encoder::ResourceFunc::method("byte-length");
                        func.results(wit_encoder::Results::anon(wit_encoder::Type::U32));
                        func
                    },
                    {
                        let mut func = wit_encoder::ResourceFunc::method("slice");
                        func.params(wit_encoder::Params::from_iter([
                            ("begin", wit_encoder::Type::U32),
                            ("end", wit_encoder::Type::option(wit_encoder::Type::U32)),
                        ]));
                        func.results(wit_encoder::Results::anon(wit_encoder::Type::named(
                            "array-buffer",
                        )));
                        func
                    },
                    // still missing:
                    // - isView
                    // - detached
                    // - maxByteLength
                    // - resizable
                    // - resize
                    // - transfer
                    // - transferToFixedLength
                ],
            );
            self.interface
                .items_mut()
                .push(wit_encoder::InterfaceItem::TypeDef(array));
        }
        Ok(buffer_name)
    }

    pub(super) fn add_typed_array<'b>(
        &mut self,
        kind: TypedArrayKind,
    ) -> anyhow::Result<wit_encoder::Ident> {
        let type_ = kind.to_wit_encoder_type();
        let array_name = wit_encoder::Ident::new(format!("{}-array", kind.prefix_name()));
        if !self.type_def_exists(&array_name) {
            let constructor_options_name =
                wit_encoder::Ident::new(format!("{array_name}-constructor-options"));
            let constructor_options = wit_encoder::TypeDef::variant(
                constructor_options_name.clone(),
                [
                    wit_encoder::VariantCase::value(
                        array_name.clone(),
                        wit_encoder::Type::named(array_name.clone()),
                    ),
                    wit_encoder::VariantCase::value("length", wit_encoder::Type::U32),
                    wit_encoder::VariantCase::value(
                        "array-buffer",
                        wit_encoder::Type::tuple([
                            wit_encoder::Type::named("array-buffer"),
                            wit_encoder::Type::option(wit_encoder::Type::U32),
                            wit_encoder::Type::option(wit_encoder::Type::U32),
                        ]),
                    ),
                    wit_encoder::VariantCase::value(
                        "shared-array-buffer",
                        wit_encoder::Type::tuple([
                            wit_encoder::Type::named("shared-array-buffer"),
                            wit_encoder::Type::option(wit_encoder::Type::U32),
                            wit_encoder::Type::option(wit_encoder::Type::U32),
                        ]),
                    ),
                ],
            );
            self.interface
                .items_mut()
                .push(wit_encoder::InterfaceItem::TypeDef(constructor_options));

            let set_src_name = wit_encoder::Ident::new(format!("{array_name}-set-src"));
            let set_src = wit_encoder::TypeDef::variant(
                set_src_name.clone(),
                [
                    wit_encoder::VariantCase::value("list", wit_encoder::Type::list(type_.clone())),
                    wit_encoder::VariantCase::value(
                        array_name.clone(),
                        wit_encoder::Type::named(array_name.clone()),
                    ),
                ],
            );
            self.interface
                .items_mut()
                .push(wit_encoder::InterfaceItem::TypeDef(set_src));

            let array = wit_encoder::TypeDef::resource(
                array_name.clone(),
                [
                    {
                        let mut func = wit_encoder::ResourceFunc::constructor();
                        func.params((
                            "options",
                            wit_encoder::Type::option(wit_encoder::Type::named(
                                constructor_options_name.clone(),
                            )),
                        ));
                        func
                    },
                    {
                        let mut func = wit_encoder::ResourceFunc::method("fill");
                        func.params(wit_encoder::Params::from_iter([
                            ("value", type_.clone()),
                            ("start", wit_encoder::Type::option(wit_encoder::Type::U32)),
                            ("end", wit_encoder::Type::option(wit_encoder::Type::U32)),
                        ]));
                        func.results(wit_encoder::Results::anon(wit_encoder::Type::named(
                            array_name.clone(),
                        )));
                        func
                    },
                    {
                        let mut func = wit_encoder::ResourceFunc::method("buffer");
                        func.results(wit_encoder::Results::anon(wit_encoder::Type::named(
                            "array-buffer",
                        )));
                        func
                    },
                    {
                        let mut func = wit_encoder::ResourceFunc::method("length");
                        func.results(wit_encoder::Results::anon(wit_encoder::Type::U32));
                        func
                    },
                    {
                        let mut func = wit_encoder::ResourceFunc::method("byte-offset");
                        func.results(wit_encoder::Results::anon(wit_encoder::Type::U32));
                        func
                    },
                    {
                        let mut func = wit_encoder::ResourceFunc::method("byte-length");
                        func.results(wit_encoder::Results::anon(wit_encoder::Type::U32));
                        func
                    },
                    {
                        let mut func = wit_encoder::ResourceFunc::method("at");
                        func.params(("index", wit_encoder::Type::S32));
                        func.results(wit_encoder::Results::anon(wit_encoder::Type::option(
                            type_.clone(),
                        )));
                        func
                    },
                    {
                        let mut func = wit_encoder::ResourceFunc::method("copy-within");
                        func.params(wit_encoder::Params::from_iter([
                            ("target", wit_encoder::Type::U32),
                            ("start", wit_encoder::Type::U32),
                            ("end", wit_encoder::Type::option(wit_encoder::Type::U32)),
                        ]));
                        func.results(wit_encoder::Results::anon(wit_encoder::Type::named(
                            array_name.clone(),
                        )));
                        func
                    },
                    {
                        let mut func = wit_encoder::ResourceFunc::method("set");
                        func.params(wit_encoder::Params::from_iter([
                            ("src", wit_encoder::Type::named(set_src_name.clone())),
                            ("offset", wit_encoder::Type::U32),
                        ]));
                        func
                    },
                    {
                        let mut func = wit_encoder::ResourceFunc::method("slice");
                        func.params(wit_encoder::Params::from_iter([
                            ("begin", wit_encoder::Type::U32),
                            ("end", wit_encoder::Type::U32),
                        ]));
                        func.results(wit_encoder::Results::anon(wit_encoder::Type::named(
                            array_name.clone(),
                        )));
                        func
                    },
                    {
                        let mut func = wit_encoder::ResourceFunc::method("subarray");
                        func.params(wit_encoder::Params::from_iter([
                            ("begin", wit_encoder::Type::U32),
                            ("end", wit_encoder::Type::U32),
                        ]));
                        func.results(wit_encoder::Results::anon(wit_encoder::Type::named(
                            array_name.clone(),
                        )));
                        func
                    },
                    {
                        let mut func = wit_encoder::ResourceFunc::method("values");
                        func.results(wit_encoder::Results::anon(wit_encoder::Type::list(
                            type_.clone(),
                        )));
                        func
                    },
                    {
                        let mut func = wit_encoder::ResourceFunc::method("get-index");
                        func.params(("index", wit_encoder::Type::U32));
                        func.results(wit_encoder::Results::anon(type_.clone()));
                        func
                    },
                    {
                        let mut func = wit_encoder::ResourceFunc::method("set-index");
                        func.params(wit_encoder::Params::from_iter([
                            ("index", wit_encoder::Type::U32),
                            ("value", type_.clone()),
                        ]));
                        func
                    },
                    // still missing:
                    // - entries
                    // - every
                    // - filter
                    // - find
                    // - findIndex
                    // - findLast
                    // - findLastIndex
                    // - includes
                    // - indexOf
                    // - join
                    // - keys
                    // - lastIndexOf
                    // - map
                    // - reduce
                    // - reduceRight
                    // - some
                    // - sort
                    // - toReversed
                    // - toSorted
                    // - with
                    // - forEach
                ],
            );
            self.interface
                .items_mut()
                .push(wit_encoder::InterfaceItem::TypeDef(array));
        }
        Ok(array_name)
    }

    pub(super) fn add_record<'b>(
        &mut self,
        record: &weedle::types::RecordType<'b>,
    ) -> anyhow::Result<wit_encoder::Ident> {
        let value = self.wi2w_type(&record.generics.body.2, false)?;

        let record_name = wit_encoder::Ident::new(format!("record-{value}"));
        if !self.type_def_exists(&record_name) {
            let set = wit_encoder::TypeDef::resource(
                record_name.clone(),
                [
                    { wit_encoder::ResourceFunc::constructor() },
                    {
                        let mut func = wit_encoder::ResourceFunc::method("add");
                        func.params(wit_encoder::Params::from_iter([
                            ("key", wit_encoder::Type::String),
                            ("value", value.clone()),
                        ]));
                        func
                    },
                    {
                        let mut func = wit_encoder::ResourceFunc::method("get");
                        func.params(("key", wit_encoder::Type::String));
                        func.results(wit_encoder::Results::anon(value.clone()));
                        func
                    },
                    {
                        let mut func = wit_encoder::ResourceFunc::method("has");
                        func.params(("key", wit_encoder::Type::String));
                        func.results(wit_encoder::Results::anon(wit_encoder::Type::Bool));
                        func
                    },
                    {
                        let mut func = wit_encoder::ResourceFunc::method("remove");
                        func.params(("key", wit_encoder::Type::String));
                        func
                    },
                    {
                        let mut func = wit_encoder::ResourceFunc::method("keys");
                        func.results(wit_encoder::Results::anon(wit_encoder::Type::list(
                            wit_encoder::Type::String,
                        )));
                        func
                    },
                    {
                        let mut func = wit_encoder::ResourceFunc::method("values");
                        func.results(wit_encoder::Results::anon(wit_encoder::Type::list(
                            value.clone(),
                        )));
                        func
                    },
                    {
                        let mut func = wit_encoder::ResourceFunc::method("entries");
                        func.results(wit_encoder::Results::anon(wit_encoder::Type::tuple([
                            wit_encoder::Type::String,
                            value.clone(),
                        ])));
                        func
                    },
                ],
            );
            self.interface
                .items_mut()
                .push(wit_encoder::InterfaceItem::TypeDef(set));
        }
        Ok(record_name)
    }

    fn type_def_exists(&self, name: &wit_encoder::Ident) -> bool {
        self.interface.items().iter().any(|item| match item {
            wit_encoder::InterfaceItem::TypeDef(td) => td.name() == name,
            wit_encoder::InterfaceItem::Function(_) => false,
        })
    }
}
