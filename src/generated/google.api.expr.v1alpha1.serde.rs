impl serde::Serialize for CheckedExpr {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reference_map.is_empty() {
            len += 1;
        }
        if !self.type_map.is_empty() {
            len += 1;
        }
        if self.source_info.is_some() {
            len += 1;
        }
        if !self.expr_version.is_empty() {
            len += 1;
        }
        if self.expr.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.CheckedExpr", len)?;
        if !self.reference_map.is_empty() {
            struct_ser.serialize_field("reference_map", &self.reference_map)?;
        }
        if !self.type_map.is_empty() {
            struct_ser.serialize_field("type_map", &self.type_map)?;
        }
        if let Some(v) = self.source_info.as_ref() {
            struct_ser.serialize_field("source_info", v)?;
        }
        if !self.expr_version.is_empty() {
            struct_ser.serialize_field("expr_version", &self.expr_version)?;
        }
        if let Some(v) = self.expr.as_ref() {
            struct_ser.serialize_field("expr", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckedExpr {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reference_map",
            "referenceMap",
            "type_map",
            "typeMap",
            "source_info",
            "sourceInfo",
            "expr_version",
            "exprVersion",
            "expr",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReferenceMap,
            TypeMap,
            SourceInfo,
            ExprVersion,
            Expr,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "referenceMap" | "reference_map" => Ok(GeneratedField::ReferenceMap),
                            "typeMap" | "type_map" => Ok(GeneratedField::TypeMap),
                            "sourceInfo" | "source_info" => Ok(GeneratedField::SourceInfo),
                            "exprVersion" | "expr_version" => Ok(GeneratedField::ExprVersion),
                            "expr" => Ok(GeneratedField::Expr),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckedExpr;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.CheckedExpr")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckedExpr, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reference_map__ = None;
                let mut type_map__ = None;
                let mut source_info__ = None;
                let mut expr_version__ = None;
                let mut expr__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReferenceMap => {
                            if reference_map__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceMap"));
                            }
                            reference_map__ = Some(
                                map_.next_value::<std::collections::HashMap<::pbjson::private::NumberDeserialize<i64>, _>>()?
                                    .into_iter().map(|(k,v)| (k.0, v)).collect()
                            );
                        }
                        GeneratedField::TypeMap => {
                            if type_map__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeMap"));
                            }
                            type_map__ = Some(
                                map_.next_value::<std::collections::HashMap<::pbjson::private::NumberDeserialize<i64>, _>>()?
                                    .into_iter().map(|(k,v)| (k.0, v)).collect()
                            );
                        }
                        GeneratedField::SourceInfo => {
                            if source_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceInfo"));
                            }
                            source_info__ = map_.next_value()?;
                        }
                        GeneratedField::ExprVersion => {
                            if expr_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exprVersion"));
                            }
                            expr_version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Expr => {
                            if expr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expr"));
                            }
                            expr__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CheckedExpr {
                    reference_map: reference_map__.unwrap_or_default(),
                    type_map: type_map__.unwrap_or_default(),
                    source_info: source_info__,
                    expr_version: expr_version__.unwrap_or_default(),
                    expr: expr__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.CheckedExpr", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Constant {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.constant_kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.Constant", len)?;
        if let Some(v) = self.constant_kind.as_ref() {
            match v {
                constant::ConstantKind::NullValue(v) => {
                    let v = super::super::super::protobuf::NullValue::try_from(*v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("null_value", &v)?;
                }
                constant::ConstantKind::BoolValue(v) => {
                    struct_ser.serialize_field("bool_value", v)?;
                }
                constant::ConstantKind::Int64Value(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("int64_value", ToString::to_string(&v).as_str())?;
                }
                constant::ConstantKind::Uint64Value(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("uint64_value", ToString::to_string(&v).as_str())?;
                }
                constant::ConstantKind::DoubleValue(v) => {
                    struct_ser.serialize_field("double_value", v)?;
                }
                constant::ConstantKind::StringValue(v) => {
                    struct_ser.serialize_field("string_value", v)?;
                }
                constant::ConstantKind::BytesValue(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("bytes_value", pbjson::private::base64::encode(&v).as_str())?;
                }
                constant::ConstantKind::DurationValue(v) => {
                    struct_ser.serialize_field("duration_value", v)?;
                }
                constant::ConstantKind::TimestampValue(v) => {
                    struct_ser.serialize_field("timestamp_value", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Constant {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "null_value",
            "nullValue",
            "bool_value",
            "boolValue",
            "int64_value",
            "int64Value",
            "uint64_value",
            "uint64Value",
            "double_value",
            "doubleValue",
            "string_value",
            "stringValue",
            "bytes_value",
            "bytesValue",
            "duration_value",
            "durationValue",
            "timestamp_value",
            "timestampValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NullValue,
            BoolValue,
            Int64Value,
            Uint64Value,
            DoubleValue,
            StringValue,
            BytesValue,
            DurationValue,
            TimestampValue,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "nullValue" | "null_value" => Ok(GeneratedField::NullValue),
                            "boolValue" | "bool_value" => Ok(GeneratedField::BoolValue),
                            "int64Value" | "int64_value" => Ok(GeneratedField::Int64Value),
                            "uint64Value" | "uint64_value" => Ok(GeneratedField::Uint64Value),
                            "doubleValue" | "double_value" => Ok(GeneratedField::DoubleValue),
                            "stringValue" | "string_value" => Ok(GeneratedField::StringValue),
                            "bytesValue" | "bytes_value" => Ok(GeneratedField::BytesValue),
                            "durationValue" | "duration_value" => Ok(GeneratedField::DurationValue),
                            "timestampValue" | "timestamp_value" => Ok(GeneratedField::TimestampValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Constant;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.Constant")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Constant, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut constant_kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NullValue => {
                            if constant_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nullValue"));
                            }
                            constant_kind__ = map_.next_value::<::std::option::Option<super::super::super::protobuf::NullValue>>()?.map(|x| constant::ConstantKind::NullValue(x as i32));
                        }
                        GeneratedField::BoolValue => {
                            if constant_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("boolValue"));
                            }
                            constant_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(constant::ConstantKind::BoolValue);
                        }
                        GeneratedField::Int64Value => {
                            if constant_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("int64Value"));
                            }
                            constant_kind__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| constant::ConstantKind::Int64Value(x.0));
                        }
                        GeneratedField::Uint64Value => {
                            if constant_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uint64Value"));
                            }
                            constant_kind__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| constant::ConstantKind::Uint64Value(x.0));
                        }
                        GeneratedField::DoubleValue => {
                            if constant_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("doubleValue"));
                            }
                            constant_kind__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| constant::ConstantKind::DoubleValue(x.0));
                        }
                        GeneratedField::StringValue => {
                            if constant_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringValue"));
                            }
                            constant_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(constant::ConstantKind::StringValue);
                        }
                        GeneratedField::BytesValue => {
                            if constant_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bytesValue"));
                            }
                            constant_kind__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| constant::ConstantKind::BytesValue(x.0));
                        }
                        GeneratedField::DurationValue => {
                            if constant_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("durationValue"));
                            }
                            constant_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(constant::ConstantKind::DurationValue)
;
                        }
                        GeneratedField::TimestampValue => {
                            if constant_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestampValue"));
                            }
                            constant_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(constant::ConstantKind::TimestampValue)
;
                        }
                    }
                }
                Ok(Constant {
                    constant_kind: constant_kind__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.Constant", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Decl {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.decl_kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.Decl", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.decl_kind.as_ref() {
            match v {
                decl::DeclKind::Ident(v) => {
                    struct_ser.serialize_field("ident", v)?;
                }
                decl::DeclKind::Function(v) => {
                    struct_ser.serialize_field("function", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Decl {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "ident",
            "function",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Ident,
            Function,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "ident" => Ok(GeneratedField::Ident),
                            "function" => Ok(GeneratedField::Function),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Decl;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.Decl")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Decl, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut decl_kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Ident => {
                            if decl_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ident"));
                            }
                            decl_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(decl::DeclKind::Ident)
;
                        }
                        GeneratedField::Function => {
                            if decl_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            decl_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(decl::DeclKind::Function)
;
                        }
                    }
                }
                Ok(Decl {
                    name: name__.unwrap_or_default(),
                    decl_kind: decl_kind__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.Decl", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for decl::FunctionDecl {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.overloads.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.Decl.FunctionDecl", len)?;
        if !self.overloads.is_empty() {
            struct_ser.serialize_field("overloads", &self.overloads)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for decl::FunctionDecl {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "overloads",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Overloads,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "overloads" => Ok(GeneratedField::Overloads),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = decl::FunctionDecl;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.Decl.FunctionDecl")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<decl::FunctionDecl, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut overloads__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Overloads => {
                            if overloads__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overloads"));
                            }
                            overloads__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(decl::FunctionDecl {
                    overloads: overloads__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.Decl.FunctionDecl", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for decl::function_decl::Overload {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.overload_id.is_empty() {
            len += 1;
        }
        if !self.params.is_empty() {
            len += 1;
        }
        if !self.type_params.is_empty() {
            len += 1;
        }
        if self.result_type.is_some() {
            len += 1;
        }
        if self.is_instance_function {
            len += 1;
        }
        if !self.doc.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.Decl.FunctionDecl.Overload", len)?;
        if !self.overload_id.is_empty() {
            struct_ser.serialize_field("overload_id", &self.overload_id)?;
        }
        if !self.params.is_empty() {
            struct_ser.serialize_field("params", &self.params)?;
        }
        if !self.type_params.is_empty() {
            struct_ser.serialize_field("type_params", &self.type_params)?;
        }
        if let Some(v) = self.result_type.as_ref() {
            struct_ser.serialize_field("result_type", v)?;
        }
        if self.is_instance_function {
            struct_ser.serialize_field("is_instance_function", &self.is_instance_function)?;
        }
        if !self.doc.is_empty() {
            struct_ser.serialize_field("doc", &self.doc)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for decl::function_decl::Overload {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "overload_id",
            "overloadId",
            "params",
            "type_params",
            "typeParams",
            "result_type",
            "resultType",
            "is_instance_function",
            "isInstanceFunction",
            "doc",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OverloadId,
            Params,
            TypeParams,
            ResultType,
            IsInstanceFunction,
            Doc,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "overloadId" | "overload_id" => Ok(GeneratedField::OverloadId),
                            "params" => Ok(GeneratedField::Params),
                            "typeParams" | "type_params" => Ok(GeneratedField::TypeParams),
                            "resultType" | "result_type" => Ok(GeneratedField::ResultType),
                            "isInstanceFunction" | "is_instance_function" => Ok(GeneratedField::IsInstanceFunction),
                            "doc" => Ok(GeneratedField::Doc),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = decl::function_decl::Overload;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.Decl.FunctionDecl.Overload")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<decl::function_decl::Overload, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut overload_id__ = None;
                let mut params__ = None;
                let mut type_params__ = None;
                let mut result_type__ = None;
                let mut is_instance_function__ = None;
                let mut doc__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OverloadId => {
                            if overload_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overloadId"));
                            }
                            overload_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypeParams => {
                            if type_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeParams"));
                            }
                            type_params__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResultType => {
                            if result_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resultType"));
                            }
                            result_type__ = map_.next_value()?;
                        }
                        GeneratedField::IsInstanceFunction => {
                            if is_instance_function__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isInstanceFunction"));
                            }
                            is_instance_function__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Doc => {
                            if doc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc"));
                            }
                            doc__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(decl::function_decl::Overload {
                    overload_id: overload_id__.unwrap_or_default(),
                    params: params__.unwrap_or_default(),
                    type_params: type_params__.unwrap_or_default(),
                    result_type: result_type__,
                    is_instance_function: is_instance_function__.unwrap_or_default(),
                    doc: doc__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.Decl.FunctionDecl.Overload", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for decl::IdentDecl {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        if !self.doc.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.Decl.IdentDecl", len)?;
        if let Some(v) = self.r#type.as_ref() {
            struct_ser.serialize_field("type", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        if !self.doc.is_empty() {
            struct_ser.serialize_field("doc", &self.doc)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for decl::IdentDecl {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "value",
            "doc",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Value,
            Doc,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
                            "value" => Ok(GeneratedField::Value),
                            "doc" => Ok(GeneratedField::Doc),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = decl::IdentDecl;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.Decl.IdentDecl")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<decl::IdentDecl, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut value__ = None;
                let mut doc__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = map_.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                        GeneratedField::Doc => {
                            if doc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc"));
                            }
                            doc__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(decl::IdentDecl {
                    r#type: r#type__,
                    value: value__,
                    doc: doc__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.Decl.IdentDecl", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Expr {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if self.expr_kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.Expr", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if let Some(v) = self.expr_kind.as_ref() {
            match v {
                expr::ExprKind::ConstExpr(v) => {
                    struct_ser.serialize_field("const_expr", v)?;
                }
                expr::ExprKind::IdentExpr(v) => {
                    struct_ser.serialize_field("ident_expr", v)?;
                }
                expr::ExprKind::SelectExpr(v) => {
                    struct_ser.serialize_field("select_expr", v)?;
                }
                expr::ExprKind::CallExpr(v) => {
                    struct_ser.serialize_field("call_expr", v)?;
                }
                expr::ExprKind::ListExpr(v) => {
                    struct_ser.serialize_field("list_expr", v)?;
                }
                expr::ExprKind::StructExpr(v) => {
                    struct_ser.serialize_field("struct_expr", v)?;
                }
                expr::ExprKind::ComprehensionExpr(v) => {
                    struct_ser.serialize_field("comprehension_expr", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Expr {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "const_expr",
            "constExpr",
            "ident_expr",
            "identExpr",
            "select_expr",
            "selectExpr",
            "call_expr",
            "callExpr",
            "list_expr",
            "listExpr",
            "struct_expr",
            "structExpr",
            "comprehension_expr",
            "comprehensionExpr",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ConstExpr,
            IdentExpr,
            SelectExpr,
            CallExpr,
            ListExpr,
            StructExpr,
            ComprehensionExpr,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "constExpr" | "const_expr" => Ok(GeneratedField::ConstExpr),
                            "identExpr" | "ident_expr" => Ok(GeneratedField::IdentExpr),
                            "selectExpr" | "select_expr" => Ok(GeneratedField::SelectExpr),
                            "callExpr" | "call_expr" => Ok(GeneratedField::CallExpr),
                            "listExpr" | "list_expr" => Ok(GeneratedField::ListExpr),
                            "structExpr" | "struct_expr" => Ok(GeneratedField::StructExpr),
                            "comprehensionExpr" | "comprehension_expr" => Ok(GeneratedField::ComprehensionExpr),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Expr;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.Expr")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Expr, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut expr_kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ConstExpr => {
                            if expr_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("constExpr"));
                            }
                            expr_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(expr::ExprKind::ConstExpr)
;
                        }
                        GeneratedField::IdentExpr => {
                            if expr_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identExpr"));
                            }
                            expr_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(expr::ExprKind::IdentExpr)
;
                        }
                        GeneratedField::SelectExpr => {
                            if expr_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selectExpr"));
                            }
                            expr_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(expr::ExprKind::SelectExpr)
;
                        }
                        GeneratedField::CallExpr => {
                            if expr_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("callExpr"));
                            }
                            expr_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(expr::ExprKind::CallExpr)
;
                        }
                        GeneratedField::ListExpr => {
                            if expr_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listExpr"));
                            }
                            expr_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(expr::ExprKind::ListExpr)
;
                        }
                        GeneratedField::StructExpr => {
                            if expr_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("structExpr"));
                            }
                            expr_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(expr::ExprKind::StructExpr)
;
                        }
                        GeneratedField::ComprehensionExpr => {
                            if expr_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comprehensionExpr"));
                            }
                            expr_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(expr::ExprKind::ComprehensionExpr)
;
                        }
                    }
                }
                Ok(Expr {
                    id: id__.unwrap_or_default(),
                    expr_kind: expr_kind__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.Expr", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for expr::Call {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.target.is_some() {
            len += 1;
        }
        if !self.function.is_empty() {
            len += 1;
        }
        if !self.args.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.Expr.Call", len)?;
        if let Some(v) = self.target.as_ref() {
            struct_ser.serialize_field("target", v)?;
        }
        if !self.function.is_empty() {
            struct_ser.serialize_field("function", &self.function)?;
        }
        if !self.args.is_empty() {
            struct_ser.serialize_field("args", &self.args)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for expr::Call {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "target",
            "function",
            "args",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Target,
            Function,
            Args,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "target" => Ok(GeneratedField::Target),
                            "function" => Ok(GeneratedField::Function),
                            "args" => Ok(GeneratedField::Args),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = expr::Call;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.Expr.Call")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<expr::Call, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut target__ = None;
                let mut function__ = None;
                let mut args__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Target => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            target__ = map_.next_value()?;
                        }
                        GeneratedField::Function => {
                            if function__.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            function__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Args => {
                            if args__.is_some() {
                                return Err(serde::de::Error::duplicate_field("args"));
                            }
                            args__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(expr::Call {
                    target: target__,
                    function: function__.unwrap_or_default(),
                    args: args__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.Expr.Call", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for expr::Comprehension {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.iter_var.is_empty() {
            len += 1;
        }
        if !self.iter_var2.is_empty() {
            len += 1;
        }
        if self.iter_range.is_some() {
            len += 1;
        }
        if !self.accu_var.is_empty() {
            len += 1;
        }
        if self.accu_init.is_some() {
            len += 1;
        }
        if self.loop_condition.is_some() {
            len += 1;
        }
        if self.loop_step.is_some() {
            len += 1;
        }
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.Expr.Comprehension", len)?;
        if !self.iter_var.is_empty() {
            struct_ser.serialize_field("iter_var", &self.iter_var)?;
        }
        if !self.iter_var2.is_empty() {
            struct_ser.serialize_field("iter_var2", &self.iter_var2)?;
        }
        if let Some(v) = self.iter_range.as_ref() {
            struct_ser.serialize_field("iter_range", v)?;
        }
        if !self.accu_var.is_empty() {
            struct_ser.serialize_field("accu_var", &self.accu_var)?;
        }
        if let Some(v) = self.accu_init.as_ref() {
            struct_ser.serialize_field("accu_init", v)?;
        }
        if let Some(v) = self.loop_condition.as_ref() {
            struct_ser.serialize_field("loop_condition", v)?;
        }
        if let Some(v) = self.loop_step.as_ref() {
            struct_ser.serialize_field("loop_step", v)?;
        }
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for expr::Comprehension {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "iter_var",
            "iterVar",
            "iter_var2",
            "iterVar2",
            "iter_range",
            "iterRange",
            "accu_var",
            "accuVar",
            "accu_init",
            "accuInit",
            "loop_condition",
            "loopCondition",
            "loop_step",
            "loopStep",
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IterVar,
            IterVar2,
            IterRange,
            AccuVar,
            AccuInit,
            LoopCondition,
            LoopStep,
            Result,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "iterVar" | "iter_var" => Ok(GeneratedField::IterVar),
                            "iterVar2" | "iter_var2" => Ok(GeneratedField::IterVar2),
                            "iterRange" | "iter_range" => Ok(GeneratedField::IterRange),
                            "accuVar" | "accu_var" => Ok(GeneratedField::AccuVar),
                            "accuInit" | "accu_init" => Ok(GeneratedField::AccuInit),
                            "loopCondition" | "loop_condition" => Ok(GeneratedField::LoopCondition),
                            "loopStep" | "loop_step" => Ok(GeneratedField::LoopStep),
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = expr::Comprehension;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.Expr.Comprehension")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<expr::Comprehension, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut iter_var__ = None;
                let mut iter_var2__ = None;
                let mut iter_range__ = None;
                let mut accu_var__ = None;
                let mut accu_init__ = None;
                let mut loop_condition__ = None;
                let mut loop_step__ = None;
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IterVar => {
                            if iter_var__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iterVar"));
                            }
                            iter_var__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IterVar2 => {
                            if iter_var2__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iterVar2"));
                            }
                            iter_var2__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IterRange => {
                            if iter_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iterRange"));
                            }
                            iter_range__ = map_.next_value()?;
                        }
                        GeneratedField::AccuVar => {
                            if accu_var__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accuVar"));
                            }
                            accu_var__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AccuInit => {
                            if accu_init__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accuInit"));
                            }
                            accu_init__ = map_.next_value()?;
                        }
                        GeneratedField::LoopCondition => {
                            if loop_condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loopCondition"));
                            }
                            loop_condition__ = map_.next_value()?;
                        }
                        GeneratedField::LoopStep => {
                            if loop_step__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loopStep"));
                            }
                            loop_step__ = map_.next_value()?;
                        }
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = map_.next_value()?;
                        }
                    }
                }
                Ok(expr::Comprehension {
                    iter_var: iter_var__.unwrap_or_default(),
                    iter_var2: iter_var2__.unwrap_or_default(),
                    iter_range: iter_range__,
                    accu_var: accu_var__.unwrap_or_default(),
                    accu_init: accu_init__,
                    loop_condition: loop_condition__,
                    loop_step: loop_step__,
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.Expr.Comprehension", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for expr::CreateList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.elements.is_empty() {
            len += 1;
        }
        if !self.optional_indices.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.Expr.CreateList", len)?;
        if !self.elements.is_empty() {
            struct_ser.serialize_field("elements", &self.elements)?;
        }
        if !self.optional_indices.is_empty() {
            struct_ser.serialize_field("optional_indices", &self.optional_indices)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for expr::CreateList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "elements",
            "optional_indices",
            "optionalIndices",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Elements,
            OptionalIndices,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "elements" => Ok(GeneratedField::Elements),
                            "optionalIndices" | "optional_indices" => Ok(GeneratedField::OptionalIndices),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = expr::CreateList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.Expr.CreateList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<expr::CreateList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut elements__ = None;
                let mut optional_indices__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Elements => {
                            if elements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("elements"));
                            }
                            elements__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OptionalIndices => {
                            if optional_indices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalIndices"));
                            }
                            optional_indices__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(expr::CreateList {
                    elements: elements__.unwrap_or_default(),
                    optional_indices: optional_indices__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.Expr.CreateList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for expr::CreateStruct {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message_name.is_empty() {
            len += 1;
        }
        if !self.entries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.Expr.CreateStruct", len)?;
        if !self.message_name.is_empty() {
            struct_ser.serialize_field("message_name", &self.message_name)?;
        }
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for expr::CreateStruct {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_name",
            "messageName",
            "entries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MessageName,
            Entries,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "messageName" | "message_name" => Ok(GeneratedField::MessageName),
                            "entries" => Ok(GeneratedField::Entries),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = expr::CreateStruct;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.Expr.CreateStruct")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<expr::CreateStruct, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_name__ = None;
                let mut entries__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessageName => {
                            if message_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageName"));
                            }
                            message_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(expr::CreateStruct {
                    message_name: message_name__.unwrap_or_default(),
                    entries: entries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.Expr.CreateStruct", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for expr::create_struct::Entry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        if self.optional_entry {
            len += 1;
        }
        if self.key_kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.Expr.CreateStruct.Entry", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        if self.optional_entry {
            struct_ser.serialize_field("optional_entry", &self.optional_entry)?;
        }
        if let Some(v) = self.key_kind.as_ref() {
            match v {
                expr::create_struct::entry::KeyKind::FieldKey(v) => {
                    struct_ser.serialize_field("field_key", v)?;
                }
                expr::create_struct::entry::KeyKind::MapKey(v) => {
                    struct_ser.serialize_field("map_key", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for expr::create_struct::Entry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "value",
            "optional_entry",
            "optionalEntry",
            "field_key",
            "fieldKey",
            "map_key",
            "mapKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Value,
            OptionalEntry,
            FieldKey,
            MapKey,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "value" => Ok(GeneratedField::Value),
                            "optionalEntry" | "optional_entry" => Ok(GeneratedField::OptionalEntry),
                            "fieldKey" | "field_key" => Ok(GeneratedField::FieldKey),
                            "mapKey" | "map_key" => Ok(GeneratedField::MapKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = expr::create_struct::Entry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.Expr.CreateStruct.Entry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<expr::create_struct::Entry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut value__ = None;
                let mut optional_entry__ = None;
                let mut key_kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                        GeneratedField::OptionalEntry => {
                            if optional_entry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalEntry"));
                            }
                            optional_entry__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FieldKey => {
                            if key_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldKey"));
                            }
                            key_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(expr::create_struct::entry::KeyKind::FieldKey);
                        }
                        GeneratedField::MapKey => {
                            if key_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mapKey"));
                            }
                            key_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(expr::create_struct::entry::KeyKind::MapKey)
;
                        }
                    }
                }
                Ok(expr::create_struct::Entry {
                    id: id__.unwrap_or_default(),
                    value: value__,
                    optional_entry: optional_entry__.unwrap_or_default(),
                    key_kind: key_kind__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.Expr.CreateStruct.Entry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for expr::Ident {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.Expr.Ident", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for expr::Ident {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = expr::Ident;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.Expr.Ident")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<expr::Ident, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(expr::Ident {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.Expr.Ident", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for expr::Select {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.operand.is_some() {
            len += 1;
        }
        if !self.field.is_empty() {
            len += 1;
        }
        if self.test_only {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.Expr.Select", len)?;
        if let Some(v) = self.operand.as_ref() {
            struct_ser.serialize_field("operand", v)?;
        }
        if !self.field.is_empty() {
            struct_ser.serialize_field("field", &self.field)?;
        }
        if self.test_only {
            struct_ser.serialize_field("test_only", &self.test_only)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for expr::Select {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operand",
            "field",
            "test_only",
            "testOnly",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Operand,
            Field,
            TestOnly,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "operand" => Ok(GeneratedField::Operand),
                            "field" => Ok(GeneratedField::Field),
                            "testOnly" | "test_only" => Ok(GeneratedField::TestOnly),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = expr::Select;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.Expr.Select")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<expr::Select, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operand__ = None;
                let mut field__ = None;
                let mut test_only__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Operand => {
                            if operand__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operand"));
                            }
                            operand__ = map_.next_value()?;
                        }
                        GeneratedField::Field => {
                            if field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            field__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TestOnly => {
                            if test_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testOnly"));
                            }
                            test_only__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(expr::Select {
                    operand: operand__,
                    field: field__.unwrap_or_default(),
                    test_only: test_only__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.Expr.Select", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ParsedExpr {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.expr.is_some() {
            len += 1;
        }
        if self.source_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.ParsedExpr", len)?;
        if let Some(v) = self.expr.as_ref() {
            struct_ser.serialize_field("expr", v)?;
        }
        if let Some(v) = self.source_info.as_ref() {
            struct_ser.serialize_field("source_info", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ParsedExpr {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "expr",
            "source_info",
            "sourceInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Expr,
            SourceInfo,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "expr" => Ok(GeneratedField::Expr),
                            "sourceInfo" | "source_info" => Ok(GeneratedField::SourceInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ParsedExpr;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.ParsedExpr")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ParsedExpr, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut expr__ = None;
                let mut source_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Expr => {
                            if expr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expr"));
                            }
                            expr__ = map_.next_value()?;
                        }
                        GeneratedField::SourceInfo => {
                            if source_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceInfo"));
                            }
                            source_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ParsedExpr {
                    expr: expr__,
                    source_info: source_info__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.ParsedExpr", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Reference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.overload_id.is_empty() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.Reference", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.overload_id.is_empty() {
            struct_ser.serialize_field("overload_id", &self.overload_id)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Reference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "overload_id",
            "overloadId",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            OverloadId,
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "overloadId" | "overload_id" => Ok(GeneratedField::OverloadId),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Reference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.Reference")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Reference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut overload_id__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OverloadId => {
                            if overload_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overloadId"));
                            }
                            overload_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Reference {
                    name: name__.unwrap_or_default(),
                    overload_id: overload_id__.unwrap_or_default(),
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.Reference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SourceInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.syntax_version.is_empty() {
            len += 1;
        }
        if !self.location.is_empty() {
            len += 1;
        }
        if !self.line_offsets.is_empty() {
            len += 1;
        }
        if !self.positions.is_empty() {
            len += 1;
        }
        if !self.macro_calls.is_empty() {
            len += 1;
        }
        if !self.extensions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.SourceInfo", len)?;
        if !self.syntax_version.is_empty() {
            struct_ser.serialize_field("syntax_version", &self.syntax_version)?;
        }
        if !self.location.is_empty() {
            struct_ser.serialize_field("location", &self.location)?;
        }
        if !self.line_offsets.is_empty() {
            struct_ser.serialize_field("line_offsets", &self.line_offsets)?;
        }
        if !self.positions.is_empty() {
            struct_ser.serialize_field("positions", &self.positions)?;
        }
        if !self.macro_calls.is_empty() {
            struct_ser.serialize_field("macro_calls", &self.macro_calls)?;
        }
        if !self.extensions.is_empty() {
            struct_ser.serialize_field("extensions", &self.extensions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SourceInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "syntax_version",
            "syntaxVersion",
            "location",
            "line_offsets",
            "lineOffsets",
            "positions",
            "macro_calls",
            "macroCalls",
            "extensions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SyntaxVersion,
            Location,
            LineOffsets,
            Positions,
            MacroCalls,
            Extensions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "syntaxVersion" | "syntax_version" => Ok(GeneratedField::SyntaxVersion),
                            "location" => Ok(GeneratedField::Location),
                            "lineOffsets" | "line_offsets" => Ok(GeneratedField::LineOffsets),
                            "positions" => Ok(GeneratedField::Positions),
                            "macroCalls" | "macro_calls" => Ok(GeneratedField::MacroCalls),
                            "extensions" => Ok(GeneratedField::Extensions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SourceInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.SourceInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SourceInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut syntax_version__ = None;
                let mut location__ = None;
                let mut line_offsets__ = None;
                let mut positions__ = None;
                let mut macro_calls__ = None;
                let mut extensions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SyntaxVersion => {
                            if syntax_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("syntaxVersion"));
                            }
                            syntax_version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Location => {
                            if location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            location__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LineOffsets => {
                            if line_offsets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lineOffsets"));
                            }
                            line_offsets__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Positions => {
                            if positions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("positions"));
                            }
                            positions__ = Some(
                                map_.next_value::<std::collections::HashMap<::pbjson::private::NumberDeserialize<i64>, ::pbjson::private::NumberDeserialize<i32>>>()?
                                    .into_iter().map(|(k,v)| (k.0, v.0)).collect()
                            );
                        }
                        GeneratedField::MacroCalls => {
                            if macro_calls__.is_some() {
                                return Err(serde::de::Error::duplicate_field("macroCalls"));
                            }
                            macro_calls__ = Some(
                                map_.next_value::<std::collections::HashMap<::pbjson::private::NumberDeserialize<i64>, _>>()?
                                    .into_iter().map(|(k,v)| (k.0, v)).collect()
                            );
                        }
                        GeneratedField::Extensions => {
                            if extensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensions"));
                            }
                            extensions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SourceInfo {
                    syntax_version: syntax_version__.unwrap_or_default(),
                    location: location__.unwrap_or_default(),
                    line_offsets: line_offsets__.unwrap_or_default(),
                    positions: positions__.unwrap_or_default(),
                    macro_calls: macro_calls__.unwrap_or_default(),
                    extensions: extensions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.SourceInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for source_info::Extension {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.affected_components.is_empty() {
            len += 1;
        }
        if self.version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.SourceInfo.Extension", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.affected_components.is_empty() {
            let v = self.affected_components.iter().cloned().map(|v| {
                source_info::extension::Component::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("affected_components", &v)?;
        }
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for source_info::Extension {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "affected_components",
            "affectedComponents",
            "version",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            AffectedComponents,
            Version,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "affectedComponents" | "affected_components" => Ok(GeneratedField::AffectedComponents),
                            "version" => Ok(GeneratedField::Version),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = source_info::Extension;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.SourceInfo.Extension")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<source_info::Extension, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut affected_components__ = None;
                let mut version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AffectedComponents => {
                            if affected_components__.is_some() {
                                return Err(serde::de::Error::duplicate_field("affectedComponents"));
                            }
                            affected_components__ = Some(map_.next_value::<Vec<source_info::extension::Component>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = map_.next_value()?;
                        }
                    }
                }
                Ok(source_info::Extension {
                    id: id__.unwrap_or_default(),
                    affected_components: affected_components__.unwrap_or_default(),
                    version: version__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.SourceInfo.Extension", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for source_info::extension::Component {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "COMPONENT_UNSPECIFIED",
            Self::Parser => "COMPONENT_PARSER",
            Self::TypeChecker => "COMPONENT_TYPE_CHECKER",
            Self::Runtime => "COMPONENT_RUNTIME",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for source_info::extension::Component {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "COMPONENT_UNSPECIFIED",
            "COMPONENT_PARSER",
            "COMPONENT_TYPE_CHECKER",
            "COMPONENT_RUNTIME",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = source_info::extension::Component;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "COMPONENT_UNSPECIFIED" => Ok(source_info::extension::Component::Unspecified),
                    "COMPONENT_PARSER" => Ok(source_info::extension::Component::Parser),
                    "COMPONENT_TYPE_CHECKER" => Ok(source_info::extension::Component::TypeChecker),
                    "COMPONENT_RUNTIME" => Ok(source_info::extension::Component::Runtime),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for source_info::extension::Version {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.major != 0 {
            len += 1;
        }
        if self.minor != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.SourceInfo.Extension.Version", len)?;
        if self.major != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("major", ToString::to_string(&self.major).as_str())?;
        }
        if self.minor != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("minor", ToString::to_string(&self.minor).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for source_info::extension::Version {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "major",
            "minor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Major,
            Minor,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "major" => Ok(GeneratedField::Major),
                            "minor" => Ok(GeneratedField::Minor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = source_info::extension::Version;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.SourceInfo.Extension.Version")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<source_info::extension::Version, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut major__ = None;
                let mut minor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Major => {
                            if major__.is_some() {
                                return Err(serde::de::Error::duplicate_field("major"));
                            }
                            major__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Minor => {
                            if minor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minor"));
                            }
                            minor__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(source_info::extension::Version {
                    major: major__.unwrap_or_default(),
                    minor: minor__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.SourceInfo.Extension.Version", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SourcePosition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.location.is_empty() {
            len += 1;
        }
        if self.offset != 0 {
            len += 1;
        }
        if self.line != 0 {
            len += 1;
        }
        if self.column != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.SourcePosition", len)?;
        if !self.location.is_empty() {
            struct_ser.serialize_field("location", &self.location)?;
        }
        if self.offset != 0 {
            struct_ser.serialize_field("offset", &self.offset)?;
        }
        if self.line != 0 {
            struct_ser.serialize_field("line", &self.line)?;
        }
        if self.column != 0 {
            struct_ser.serialize_field("column", &self.column)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SourcePosition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "location",
            "offset",
            "line",
            "column",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Location,
            Offset,
            Line,
            Column,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "location" => Ok(GeneratedField::Location),
                            "offset" => Ok(GeneratedField::Offset),
                            "line" => Ok(GeneratedField::Line),
                            "column" => Ok(GeneratedField::Column),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SourcePosition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.SourcePosition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SourcePosition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut location__ = None;
                let mut offset__ = None;
                let mut line__ = None;
                let mut column__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Location => {
                            if location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            location__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Offset => {
                            if offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            offset__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Line => {
                            if line__.is_some() {
                                return Err(serde::de::Error::duplicate_field("line"));
                            }
                            line__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Column => {
                            if column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("column"));
                            }
                            column__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SourcePosition {
                    location: location__.unwrap_or_default(),
                    offset: offset__.unwrap_or_default(),
                    line: line__.unwrap_or_default(),
                    column: column__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.SourcePosition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Type {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.type_kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.Type", len)?;
        if let Some(v) = self.type_kind.as_ref() {
            match v {
                r#type::TypeKind::Dyn(v) => {
                    struct_ser.serialize_field("dyn", v)?;
                }
                r#type::TypeKind::Null(v) => {
                    let v = super::super::super::protobuf::NullValue::try_from(*v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("null", &v)?;
                }
                r#type::TypeKind::Primitive(v) => {
                    let v = r#type::PrimitiveType::try_from(*v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("primitive", &v)?;
                }
                r#type::TypeKind::Wrapper(v) => {
                    let v = r#type::PrimitiveType::try_from(*v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("wrapper", &v)?;
                }
                r#type::TypeKind::WellKnown(v) => {
                    let v = r#type::WellKnownType::try_from(*v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("well_known", &v)?;
                }
                r#type::TypeKind::ListType(v) => {
                    struct_ser.serialize_field("list_type", v)?;
                }
                r#type::TypeKind::MapType(v) => {
                    struct_ser.serialize_field("map_type", v)?;
                }
                r#type::TypeKind::Function(v) => {
                    struct_ser.serialize_field("function", v)?;
                }
                r#type::TypeKind::MessageType(v) => {
                    struct_ser.serialize_field("message_type", v)?;
                }
                r#type::TypeKind::TypeParam(v) => {
                    struct_ser.serialize_field("type_param", v)?;
                }
                r#type::TypeKind::Type(v) => {
                    struct_ser.serialize_field("type", v)?;
                }
                r#type::TypeKind::Error(v) => {
                    struct_ser.serialize_field("error", v)?;
                }
                r#type::TypeKind::AbstractType(v) => {
                    struct_ser.serialize_field("abstract_type", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Type {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dyn",
            "null",
            "primitive",
            "wrapper",
            "well_known",
            "wellKnown",
            "list_type",
            "listType",
            "map_type",
            "mapType",
            "function",
            "message_type",
            "messageType",
            "type_param",
            "typeParam",
            "type",
            "error",
            "abstract_type",
            "abstractType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Dyn,
            Null,
            Primitive,
            Wrapper,
            WellKnown,
            ListType,
            MapType,
            Function,
            MessageType,
            TypeParam,
            Type,
            Error,
            AbstractType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "dyn" => Ok(GeneratedField::Dyn),
                            "null" => Ok(GeneratedField::Null),
                            "primitive" => Ok(GeneratedField::Primitive),
                            "wrapper" => Ok(GeneratedField::Wrapper),
                            "wellKnown" | "well_known" => Ok(GeneratedField::WellKnown),
                            "listType" | "list_type" => Ok(GeneratedField::ListType),
                            "mapType" | "map_type" => Ok(GeneratedField::MapType),
                            "function" => Ok(GeneratedField::Function),
                            "messageType" | "message_type" => Ok(GeneratedField::MessageType),
                            "typeParam" | "type_param" => Ok(GeneratedField::TypeParam),
                            "type" => Ok(GeneratedField::Type),
                            "error" => Ok(GeneratedField::Error),
                            "abstractType" | "abstract_type" => Ok(GeneratedField::AbstractType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Type;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.Type")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Type, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut type_kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Dyn => {
                            if type_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dyn"));
                            }
                            type_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::TypeKind::Dyn)
;
                        }
                        GeneratedField::Null => {
                            if type_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("null"));
                            }
                            type_kind__ = map_.next_value::<::std::option::Option<super::super::super::protobuf::NullValue>>()?.map(|x| r#type::TypeKind::Null(x as i32));
                        }
                        GeneratedField::Primitive => {
                            if type_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("primitive"));
                            }
                            type_kind__ = map_.next_value::<::std::option::Option<r#type::PrimitiveType>>()?.map(|x| r#type::TypeKind::Primitive(x as i32));
                        }
                        GeneratedField::Wrapper => {
                            if type_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wrapper"));
                            }
                            type_kind__ = map_.next_value::<::std::option::Option<r#type::PrimitiveType>>()?.map(|x| r#type::TypeKind::Wrapper(x as i32));
                        }
                        GeneratedField::WellKnown => {
                            if type_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wellKnown"));
                            }
                            type_kind__ = map_.next_value::<::std::option::Option<r#type::WellKnownType>>()?.map(|x| r#type::TypeKind::WellKnown(x as i32));
                        }
                        GeneratedField::ListType => {
                            if type_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listType"));
                            }
                            type_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::TypeKind::ListType)
;
                        }
                        GeneratedField::MapType => {
                            if type_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mapType"));
                            }
                            type_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::TypeKind::MapType)
;
                        }
                        GeneratedField::Function => {
                            if type_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            type_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::TypeKind::Function)
;
                        }
                        GeneratedField::MessageType => {
                            if type_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageType"));
                            }
                            type_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::TypeKind::MessageType);
                        }
                        GeneratedField::TypeParam => {
                            if type_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeParam"));
                            }
                            type_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::TypeKind::TypeParam);
                        }
                        GeneratedField::Type => {
                            if type_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            type_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::TypeKind::Type)
;
                        }
                        GeneratedField::Error => {
                            if type_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            type_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::TypeKind::Error)
;
                        }
                        GeneratedField::AbstractType => {
                            if type_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("abstractType"));
                            }
                            type_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(r#type::TypeKind::AbstractType)
;
                        }
                    }
                }
                Ok(Type {
                    type_kind: type_kind__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.Type", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::AbstractType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.parameter_types.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.Type.AbstractType", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.parameter_types.is_empty() {
            struct_ser.serialize_field("parameter_types", &self.parameter_types)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::AbstractType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "parameter_types",
            "parameterTypes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ParameterTypes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "parameterTypes" | "parameter_types" => Ok(GeneratedField::ParameterTypes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = r#type::AbstractType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.Type.AbstractType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::AbstractType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut parameter_types__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ParameterTypes => {
                            if parameter_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameterTypes"));
                            }
                            parameter_types__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(r#type::AbstractType {
                    name: name__.unwrap_or_default(),
                    parameter_types: parameter_types__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.Type.AbstractType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::FunctionType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.result_type.is_some() {
            len += 1;
        }
        if !self.arg_types.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.Type.FunctionType", len)?;
        if let Some(v) = self.result_type.as_ref() {
            struct_ser.serialize_field("result_type", v)?;
        }
        if !self.arg_types.is_empty() {
            struct_ser.serialize_field("arg_types", &self.arg_types)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::FunctionType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "result_type",
            "resultType",
            "arg_types",
            "argTypes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResultType,
            ArgTypes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "resultType" | "result_type" => Ok(GeneratedField::ResultType),
                            "argTypes" | "arg_types" => Ok(GeneratedField::ArgTypes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = r#type::FunctionType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.Type.FunctionType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::FunctionType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result_type__ = None;
                let mut arg_types__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResultType => {
                            if result_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resultType"));
                            }
                            result_type__ = map_.next_value()?;
                        }
                        GeneratedField::ArgTypes => {
                            if arg_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("argTypes"));
                            }
                            arg_types__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(r#type::FunctionType {
                    result_type: result_type__,
                    arg_types: arg_types__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.Type.FunctionType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::ListType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.elem_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.Type.ListType", len)?;
        if let Some(v) = self.elem_type.as_ref() {
            struct_ser.serialize_field("elem_type", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::ListType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "elem_type",
            "elemType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ElemType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "elemType" | "elem_type" => Ok(GeneratedField::ElemType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = r#type::ListType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.Type.ListType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::ListType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut elem_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ElemType => {
                            if elem_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("elemType"));
                            }
                            elem_type__ = map_.next_value()?;
                        }
                    }
                }
                Ok(r#type::ListType {
                    elem_type: elem_type__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.Type.ListType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::MapType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key_type.is_some() {
            len += 1;
        }
        if self.value_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.api.expr.v1alpha1.Type.MapType", len)?;
        if let Some(v) = self.key_type.as_ref() {
            struct_ser.serialize_field("key_type", v)?;
        }
        if let Some(v) = self.value_type.as_ref() {
            struct_ser.serialize_field("value_type", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for r#type::MapType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_type",
            "keyType",
            "value_type",
            "valueType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyType,
            ValueType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "keyType" | "key_type" => Ok(GeneratedField::KeyType),
                            "valueType" | "value_type" => Ok(GeneratedField::ValueType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = r#type::MapType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.api.expr.v1alpha1.Type.MapType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<r#type::MapType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_type__ = None;
                let mut value_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KeyType => {
                            if key_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyType"));
                            }
                            key_type__ = map_.next_value()?;
                        }
                        GeneratedField::ValueType => {
                            if value_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueType"));
                            }
                            value_type__ = map_.next_value()?;
                        }
                    }
                }
                Ok(r#type::MapType {
                    key_type: key_type__,
                    value_type: value_type__,
                })
            }
        }
        deserializer.deserialize_struct("google.api.expr.v1alpha1.Type.MapType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::PrimitiveType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PRIMITIVE_TYPE_UNSPECIFIED",
            Self::Bool => "BOOL",
            Self::Int64 => "INT64",
            Self::Uint64 => "UINT64",
            Self::Double => "DOUBLE",
            Self::String => "STRING",
            Self::Bytes => "BYTES",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for r#type::PrimitiveType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PRIMITIVE_TYPE_UNSPECIFIED",
            "BOOL",
            "INT64",
            "UINT64",
            "DOUBLE",
            "STRING",
            "BYTES",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = r#type::PrimitiveType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PRIMITIVE_TYPE_UNSPECIFIED" => Ok(r#type::PrimitiveType::Unspecified),
                    "BOOL" => Ok(r#type::PrimitiveType::Bool),
                    "INT64" => Ok(r#type::PrimitiveType::Int64),
                    "UINT64" => Ok(r#type::PrimitiveType::Uint64),
                    "DOUBLE" => Ok(r#type::PrimitiveType::Double),
                    "STRING" => Ok(r#type::PrimitiveType::String),
                    "BYTES" => Ok(r#type::PrimitiveType::Bytes),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for r#type::WellKnownType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "WELL_KNOWN_TYPE_UNSPECIFIED",
            Self::Any => "ANY",
            Self::Timestamp => "TIMESTAMP",
            Self::Duration => "DURATION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for r#type::WellKnownType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "WELL_KNOWN_TYPE_UNSPECIFIED",
            "ANY",
            "TIMESTAMP",
            "DURATION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = r#type::WellKnownType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "WELL_KNOWN_TYPE_UNSPECIFIED" => Ok(r#type::WellKnownType::Unspecified),
                    "ANY" => Ok(r#type::WellKnownType::Any),
                    "TIMESTAMP" => Ok(r#type::WellKnownType::Timestamp),
                    "DURATION" => Ok(r#type::WellKnownType::Duration),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
