impl serde::Serialize for BufferFactoryConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.minimum_account_to_track_power_of_two != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.overload.v3.BufferFactoryConfig", len)?;
        if self.minimum_account_to_track_power_of_two != 0 {
            struct_ser.serialize_field("minimum_account_to_track_power_of_two", &self.minimum_account_to_track_power_of_two)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BufferFactoryConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "minimum_account_to_track_power_of_two",
            "minimumAccountToTrackPowerOfTwo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinimumAccountToTrackPowerOfTwo,
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
                            "minimumAccountToTrackPowerOfTwo" | "minimum_account_to_track_power_of_two" => Ok(GeneratedField::MinimumAccountToTrackPowerOfTwo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BufferFactoryConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.overload.v3.BufferFactoryConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BufferFactoryConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut minimum_account_to_track_power_of_two__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MinimumAccountToTrackPowerOfTwo => {
                            if minimum_account_to_track_power_of_two__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minimumAccountToTrackPowerOfTwo"));
                            }
                            minimum_account_to_track_power_of_two__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(BufferFactoryConfig {
                    minimum_account_to_track_power_of_two: minimum_account_to_track_power_of_two__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.overload.v3.BufferFactoryConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoadShedPoint {
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
        if !self.triggers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.overload.v3.LoadShedPoint", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.triggers.is_empty() {
            struct_ser.serialize_field("triggers", &self.triggers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoadShedPoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "triggers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Triggers,
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
                            "triggers" => Ok(GeneratedField::Triggers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LoadShedPoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.overload.v3.LoadShedPoint")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoadShedPoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut triggers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Triggers => {
                            if triggers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("triggers"));
                            }
                            triggers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LoadShedPoint {
                    name: name__.unwrap_or_default(),
                    triggers: triggers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.overload.v3.LoadShedPoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OverloadAction {
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
        if !self.triggers.is_empty() {
            len += 1;
        }
        if self.typed_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.overload.v3.OverloadAction", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.triggers.is_empty() {
            struct_ser.serialize_field("triggers", &self.triggers)?;
        }
        if let Some(v) = self.typed_config.as_ref() {
            struct_ser.serialize_field("typed_config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OverloadAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "triggers",
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Triggers,
            TypedConfig,
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
                            "triggers" => Ok(GeneratedField::Triggers),
                            "typedConfig" | "typed_config" => Ok(GeneratedField::TypedConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OverloadAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.overload.v3.OverloadAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OverloadAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut triggers__ = None;
                let mut typed_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Triggers => {
                            if triggers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("triggers"));
                            }
                            triggers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypedConfig => {
                            if typed_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            typed_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OverloadAction {
                    name: name__.unwrap_or_default(),
                    triggers: triggers__.unwrap_or_default(),
                    typed_config: typed_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.overload.v3.OverloadAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OverloadManager {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.refresh_interval.is_some() {
            len += 1;
        }
        if !self.resource_monitors.is_empty() {
            len += 1;
        }
        if !self.actions.is_empty() {
            len += 1;
        }
        if !self.loadshed_points.is_empty() {
            len += 1;
        }
        if self.buffer_factory_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.overload.v3.OverloadManager", len)?;
        if let Some(v) = self.refresh_interval.as_ref() {
            struct_ser.serialize_field("refresh_interval", v)?;
        }
        if !self.resource_monitors.is_empty() {
            struct_ser.serialize_field("resource_monitors", &self.resource_monitors)?;
        }
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        if !self.loadshed_points.is_empty() {
            struct_ser.serialize_field("loadshed_points", &self.loadshed_points)?;
        }
        if let Some(v) = self.buffer_factory_config.as_ref() {
            struct_ser.serialize_field("buffer_factory_config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OverloadManager {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "refresh_interval",
            "refreshInterval",
            "resource_monitors",
            "resourceMonitors",
            "actions",
            "loadshed_points",
            "loadshedPoints",
            "buffer_factory_config",
            "bufferFactoryConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RefreshInterval,
            ResourceMonitors,
            Actions,
            LoadshedPoints,
            BufferFactoryConfig,
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
                            "refreshInterval" | "refresh_interval" => Ok(GeneratedField::RefreshInterval),
                            "resourceMonitors" | "resource_monitors" => Ok(GeneratedField::ResourceMonitors),
                            "actions" => Ok(GeneratedField::Actions),
                            "loadshedPoints" | "loadshed_points" => Ok(GeneratedField::LoadshedPoints),
                            "bufferFactoryConfig" | "buffer_factory_config" => Ok(GeneratedField::BufferFactoryConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OverloadManager;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.overload.v3.OverloadManager")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OverloadManager, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut refresh_interval__ = None;
                let mut resource_monitors__ = None;
                let mut actions__ = None;
                let mut loadshed_points__ = None;
                let mut buffer_factory_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RefreshInterval => {
                            if refresh_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshInterval"));
                            }
                            refresh_interval__ = map_.next_value()?;
                        }
                        GeneratedField::ResourceMonitors => {
                            if resource_monitors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceMonitors"));
                            }
                            resource_monitors__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoadshedPoints => {
                            if loadshed_points__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadshedPoints"));
                            }
                            loadshed_points__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BufferFactoryConfig => {
                            if buffer_factory_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bufferFactoryConfig"));
                            }
                            buffer_factory_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OverloadManager {
                    refresh_interval: refresh_interval__,
                    resource_monitors: resource_monitors__.unwrap_or_default(),
                    actions: actions__.unwrap_or_default(),
                    loadshed_points: loadshed_points__.unwrap_or_default(),
                    buffer_factory_config: buffer_factory_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.overload.v3.OverloadManager", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceMonitor {
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
        if self.config_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.overload.v3.ResourceMonitor", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                resource_monitor::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typed_config", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceMonitor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            TypedConfig,
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
                            "typedConfig" | "typed_config" => Ok(GeneratedField::TypedConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceMonitor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.overload.v3.ResourceMonitor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceMonitor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut config_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypedConfig => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            config_type__ = map_.next_value::<::std::option::Option<_>>()?.map(resource_monitor::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(ResourceMonitor {
                    name: name__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.overload.v3.ResourceMonitor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ScaleTimersOverloadActionConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.timer_scale_factors.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.overload.v3.ScaleTimersOverloadActionConfig", len)?;
        if !self.timer_scale_factors.is_empty() {
            struct_ser.serialize_field("timer_scale_factors", &self.timer_scale_factors)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ScaleTimersOverloadActionConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timer_scale_factors",
            "timerScaleFactors",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TimerScaleFactors,
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
                            "timerScaleFactors" | "timer_scale_factors" => Ok(GeneratedField::TimerScaleFactors),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScaleTimersOverloadActionConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.overload.v3.ScaleTimersOverloadActionConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ScaleTimersOverloadActionConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timer_scale_factors__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TimerScaleFactors => {
                            if timer_scale_factors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timerScaleFactors"));
                            }
                            timer_scale_factors__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ScaleTimersOverloadActionConfig {
                    timer_scale_factors: timer_scale_factors__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.overload.v3.ScaleTimersOverloadActionConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for scale_timers_overload_action_config::ScaleTimer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.timer != 0 {
            len += 1;
        }
        if self.overload_adjust.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.overload.v3.ScaleTimersOverloadActionConfig.ScaleTimer", len)?;
        if self.timer != 0 {
            let v = scale_timers_overload_action_config::TimerType::try_from(self.timer)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.timer)))?;
            struct_ser.serialize_field("timer", &v)?;
        }
        if let Some(v) = self.overload_adjust.as_ref() {
            match v {
                scale_timers_overload_action_config::scale_timer::OverloadAdjust::MinTimeout(v) => {
                    struct_ser.serialize_field("min_timeout", v)?;
                }
                scale_timers_overload_action_config::scale_timer::OverloadAdjust::MinScale(v) => {
                    struct_ser.serialize_field("min_scale", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for scale_timers_overload_action_config::ScaleTimer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timer",
            "min_timeout",
            "minTimeout",
            "min_scale",
            "minScale",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timer,
            MinTimeout,
            MinScale,
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
                            "timer" => Ok(GeneratedField::Timer),
                            "minTimeout" | "min_timeout" => Ok(GeneratedField::MinTimeout),
                            "minScale" | "min_scale" => Ok(GeneratedField::MinScale),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = scale_timers_overload_action_config::ScaleTimer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.overload.v3.ScaleTimersOverloadActionConfig.ScaleTimer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<scale_timers_overload_action_config::ScaleTimer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timer__ = None;
                let mut overload_adjust__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Timer => {
                            if timer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timer"));
                            }
                            timer__ = Some(map_.next_value::<scale_timers_overload_action_config::TimerType>()? as i32);
                        }
                        GeneratedField::MinTimeout => {
                            if overload_adjust__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minTimeout"));
                            }
                            overload_adjust__ = map_.next_value::<::std::option::Option<_>>()?.map(scale_timers_overload_action_config::scale_timer::OverloadAdjust::MinTimeout)
;
                        }
                        GeneratedField::MinScale => {
                            if overload_adjust__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minScale"));
                            }
                            overload_adjust__ = map_.next_value::<::std::option::Option<_>>()?.map(scale_timers_overload_action_config::scale_timer::OverloadAdjust::MinScale)
;
                        }
                    }
                }
                Ok(scale_timers_overload_action_config::ScaleTimer {
                    timer: timer__.unwrap_or_default(),
                    overload_adjust: overload_adjust__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.overload.v3.ScaleTimersOverloadActionConfig.ScaleTimer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for scale_timers_overload_action_config::TimerType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UNSPECIFIED",
            Self::HttpDownstreamConnectionIdle => "HTTP_DOWNSTREAM_CONNECTION_IDLE",
            Self::HttpDownstreamStreamIdle => "HTTP_DOWNSTREAM_STREAM_IDLE",
            Self::TransportSocketConnect => "TRANSPORT_SOCKET_CONNECT",
            Self::HttpDownstreamConnectionMax => "HTTP_DOWNSTREAM_CONNECTION_MAX",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for scale_timers_overload_action_config::TimerType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNSPECIFIED",
            "HTTP_DOWNSTREAM_CONNECTION_IDLE",
            "HTTP_DOWNSTREAM_STREAM_IDLE",
            "TRANSPORT_SOCKET_CONNECT",
            "HTTP_DOWNSTREAM_CONNECTION_MAX",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = scale_timers_overload_action_config::TimerType;

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
                    "UNSPECIFIED" => Ok(scale_timers_overload_action_config::TimerType::Unspecified),
                    "HTTP_DOWNSTREAM_CONNECTION_IDLE" => Ok(scale_timers_overload_action_config::TimerType::HttpDownstreamConnectionIdle),
                    "HTTP_DOWNSTREAM_STREAM_IDLE" => Ok(scale_timers_overload_action_config::TimerType::HttpDownstreamStreamIdle),
                    "TRANSPORT_SOCKET_CONNECT" => Ok(scale_timers_overload_action_config::TimerType::TransportSocketConnect),
                    "HTTP_DOWNSTREAM_CONNECTION_MAX" => Ok(scale_timers_overload_action_config::TimerType::HttpDownstreamConnectionMax),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ScaledTrigger {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.scaling_threshold != 0. {
            len += 1;
        }
        if self.saturation_threshold != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.overload.v3.ScaledTrigger", len)?;
        if self.scaling_threshold != 0. {
            struct_ser.serialize_field("scaling_threshold", &self.scaling_threshold)?;
        }
        if self.saturation_threshold != 0. {
            struct_ser.serialize_field("saturation_threshold", &self.saturation_threshold)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ScaledTrigger {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "scaling_threshold",
            "scalingThreshold",
            "saturation_threshold",
            "saturationThreshold",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ScalingThreshold,
            SaturationThreshold,
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
                            "scalingThreshold" | "scaling_threshold" => Ok(GeneratedField::ScalingThreshold),
                            "saturationThreshold" | "saturation_threshold" => Ok(GeneratedField::SaturationThreshold),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScaledTrigger;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.overload.v3.ScaledTrigger")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ScaledTrigger, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut scaling_threshold__ = None;
                let mut saturation_threshold__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ScalingThreshold => {
                            if scaling_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scalingThreshold"));
                            }
                            scaling_threshold__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SaturationThreshold => {
                            if saturation_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("saturationThreshold"));
                            }
                            saturation_threshold__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ScaledTrigger {
                    scaling_threshold: scaling_threshold__.unwrap_or_default(),
                    saturation_threshold: saturation_threshold__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.overload.v3.ScaledTrigger", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ThresholdTrigger {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.overload.v3.ThresholdTrigger", len)?;
        if self.value != 0. {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ThresholdTrigger {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = ThresholdTrigger;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.overload.v3.ThresholdTrigger")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ThresholdTrigger, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ThresholdTrigger {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.overload.v3.ThresholdTrigger", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Trigger {
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
        if self.trigger_oneof.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.overload.v3.Trigger", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.trigger_oneof.as_ref() {
            match v {
                trigger::TriggerOneof::Threshold(v) => {
                    struct_ser.serialize_field("threshold", v)?;
                }
                trigger::TriggerOneof::Scaled(v) => {
                    struct_ser.serialize_field("scaled", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Trigger {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "threshold",
            "scaled",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Threshold,
            Scaled,
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
                            "threshold" => Ok(GeneratedField::Threshold),
                            "scaled" => Ok(GeneratedField::Scaled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Trigger;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.overload.v3.Trigger")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Trigger, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut trigger_oneof__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Threshold => {
                            if trigger_oneof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("threshold"));
                            }
                            trigger_oneof__ = map_.next_value::<::std::option::Option<_>>()?.map(trigger::TriggerOneof::Threshold)
;
                        }
                        GeneratedField::Scaled => {
                            if trigger_oneof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scaled"));
                            }
                            trigger_oneof__ = map_.next_value::<::std::option::Option<_>>()?.map(trigger::TriggerOneof::Scaled)
;
                        }
                    }
                }
                Ok(Trigger {
                    name: name__.unwrap_or_default(),
                    trigger_oneof: trigger_oneof__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.overload.v3.Trigger", FIELDS, GeneratedVisitor)
    }
}
