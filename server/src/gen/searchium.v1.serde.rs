// @generated
impl serde::Serialize for DirectoryDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        if self.directory_count != 0 {
            len += 1;
        }
        if self.file_count != 0 {
            len += 1;
        }
        if self.searchable_files_count != 0 {
            len += 1;
        }
        if self.searchable_files_byte_length != 0 {
            len += 1;
        }
        if self.binary_files_count != 0 {
            len += 1;
        }
        if self.binary_files_byte_length != 0 {
            len += 1;
        }
        if !self.searchable_files_by_extension_details.is_empty() {
            len += 1;
        }
        if !self.large_searchable_file_details.is_empty() {
            len += 1;
        }
        if !self.binary_files_by_extension_details.is_empty() {
            len += 1;
        }
        if !self.large_binary_files_details.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.DirectoryDetails", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if self.directory_count != 0 {
            struct_ser.serialize_field("directoryCount", ToString::to_string(&self.directory_count).as_str())?;
        }
        if self.file_count != 0 {
            struct_ser.serialize_field("fileCount", ToString::to_string(&self.file_count).as_str())?;
        }
        if self.searchable_files_count != 0 {
            struct_ser.serialize_field("searchableFilesCount", ToString::to_string(&self.searchable_files_count).as_str())?;
        }
        if self.searchable_files_byte_length != 0 {
            struct_ser.serialize_field("searchableFilesByteLength", ToString::to_string(&self.searchable_files_byte_length).as_str())?;
        }
        if self.binary_files_count != 0 {
            struct_ser.serialize_field("binaryFilesCount", ToString::to_string(&self.binary_files_count).as_str())?;
        }
        if self.binary_files_byte_length != 0 {
            struct_ser.serialize_field("binaryFilesByteLength", ToString::to_string(&self.binary_files_byte_length).as_str())?;
        }
        if !self.searchable_files_by_extension_details.is_empty() {
            struct_ser.serialize_field("searchableFilesByExtensionDetails", &self.searchable_files_by_extension_details)?;
        }
        if !self.large_searchable_file_details.is_empty() {
            struct_ser.serialize_field("largeSearchableFileDetails", &self.large_searchable_file_details)?;
        }
        if !self.binary_files_by_extension_details.is_empty() {
            struct_ser.serialize_field("binaryFilesByExtensionDetails", &self.binary_files_by_extension_details)?;
        }
        if !self.large_binary_files_details.is_empty() {
            struct_ser.serialize_field("largeBinaryFilesDetails", &self.large_binary_files_details)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DirectoryDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "directory_count",
            "directoryCount",
            "file_count",
            "fileCount",
            "searchable_files_count",
            "searchableFilesCount",
            "searchable_files_byte_length",
            "searchableFilesByteLength",
            "binary_files_count",
            "binaryFilesCount",
            "binary_files_byte_length",
            "binaryFilesByteLength",
            "searchable_files_by_extension_details",
            "searchableFilesByExtensionDetails",
            "large_searchable_file_details",
            "largeSearchableFileDetails",
            "binary_files_by_extension_details",
            "binaryFilesByExtensionDetails",
            "large_binary_files_details",
            "largeBinaryFilesDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            DirectoryCount,
            FileCount,
            SearchableFilesCount,
            SearchableFilesByteLength,
            BinaryFilesCount,
            BinaryFilesByteLength,
            SearchableFilesByExtensionDetails,
            LargeSearchableFileDetails,
            BinaryFilesByExtensionDetails,
            LargeBinaryFilesDetails,
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
                            "path" => Ok(GeneratedField::Path),
                            "directoryCount" | "directory_count" => Ok(GeneratedField::DirectoryCount),
                            "fileCount" | "file_count" => Ok(GeneratedField::FileCount),
                            "searchableFilesCount" | "searchable_files_count" => Ok(GeneratedField::SearchableFilesCount),
                            "searchableFilesByteLength" | "searchable_files_byte_length" => Ok(GeneratedField::SearchableFilesByteLength),
                            "binaryFilesCount" | "binary_files_count" => Ok(GeneratedField::BinaryFilesCount),
                            "binaryFilesByteLength" | "binary_files_byte_length" => Ok(GeneratedField::BinaryFilesByteLength),
                            "searchableFilesByExtensionDetails" | "searchable_files_by_extension_details" => Ok(GeneratedField::SearchableFilesByExtensionDetails),
                            "largeSearchableFileDetails" | "large_searchable_file_details" => Ok(GeneratedField::LargeSearchableFileDetails),
                            "binaryFilesByExtensionDetails" | "binary_files_by_extension_details" => Ok(GeneratedField::BinaryFilesByExtensionDetails),
                            "largeBinaryFilesDetails" | "large_binary_files_details" => Ok(GeneratedField::LargeBinaryFilesDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DirectoryDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.DirectoryDetails")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DirectoryDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut directory_count__ = None;
                let mut file_count__ = None;
                let mut searchable_files_count__ = None;
                let mut searchable_files_byte_length__ = None;
                let mut binary_files_count__ = None;
                let mut binary_files_byte_length__ = None;
                let mut searchable_files_by_extension_details__ = None;
                let mut large_searchable_file_details__ = None;
                let mut binary_files_by_extension_details__ = None;
                let mut large_binary_files_details__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map.next_value()?);
                        }
                        GeneratedField::DirectoryCount => {
                            if directory_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directoryCount"));
                            }
                            directory_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FileCount => {
                            if file_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileCount"));
                            }
                            file_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SearchableFilesCount => {
                            if searchable_files_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchableFilesCount"));
                            }
                            searchable_files_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SearchableFilesByteLength => {
                            if searchable_files_byte_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchableFilesByteLength"));
                            }
                            searchable_files_byte_length__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BinaryFilesCount => {
                            if binary_files_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("binaryFilesCount"));
                            }
                            binary_files_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BinaryFilesByteLength => {
                            if binary_files_byte_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("binaryFilesByteLength"));
                            }
                            binary_files_byte_length__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SearchableFilesByExtensionDetails => {
                            if searchable_files_by_extension_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchableFilesByExtensionDetails"));
                            }
                            searchable_files_by_extension_details__ = Some(map.next_value()?);
                        }
                        GeneratedField::LargeSearchableFileDetails => {
                            if large_searchable_file_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("largeSearchableFileDetails"));
                            }
                            large_searchable_file_details__ = Some(map.next_value()?);
                        }
                        GeneratedField::BinaryFilesByExtensionDetails => {
                            if binary_files_by_extension_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("binaryFilesByExtensionDetails"));
                            }
                            binary_files_by_extension_details__ = Some(map.next_value()?);
                        }
                        GeneratedField::LargeBinaryFilesDetails => {
                            if large_binary_files_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("largeBinaryFilesDetails"));
                            }
                            large_binary_files_details__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DirectoryDetails {
                    path: path__.unwrap_or_default(),
                    directory_count: directory_count__.unwrap_or_default(),
                    file_count: file_count__.unwrap_or_default(),
                    searchable_files_count: searchable_files_count__.unwrap_or_default(),
                    searchable_files_byte_length: searchable_files_byte_length__.unwrap_or_default(),
                    binary_files_count: binary_files_count__.unwrap_or_default(),
                    binary_files_byte_length: binary_files_byte_length__.unwrap_or_default(),
                    searchable_files_by_extension_details: searchable_files_by_extension_details__.unwrap_or_default(),
                    large_searchable_file_details: large_searchable_file_details__.unwrap_or_default(),
                    binary_files_by_extension_details: binary_files_by_extension_details__.unwrap_or_default(),
                    large_binary_files_details: large_binary_files_details__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.DirectoryDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DirectoryEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.DirectoryEntry", len)?;
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DirectoryEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = DirectoryEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.DirectoryEntry")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DirectoryEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entries__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DirectoryEntry {
                    entries: entries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.DirectoryEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DoneResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.info.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.DoneResponse", len)?;
        if !self.info.is_empty() {
            struct_ser.serialize_field("info", &self.info)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DoneResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "info",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Info,
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
                            "info" => Ok(GeneratedField::Info),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DoneResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.DoneResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DoneResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut info__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Info => {
                            if info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("info"));
                            }
                            info__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DoneResponse {
                    info: info__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.DoneResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ErrorResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message.is_empty() {
            len += 1;
        }
        if !self.full_type_name.is_empty() {
            len += 1;
        }
        if !self.stack_trace.is_empty() {
            len += 1;
        }
        if self.inner_error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.ErrorResponse", len)?;
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if !self.full_type_name.is_empty() {
            struct_ser.serialize_field("fullTypeName", &self.full_type_name)?;
        }
        if !self.stack_trace.is_empty() {
            struct_ser.serialize_field("stackTrace", &self.stack_trace)?;
        }
        if let Some(v) = self.inner_error.as_ref() {
            struct_ser.serialize_field("innerError", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ErrorResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message",
            "full_type_name",
            "fullTypeName",
            "stack_trace",
            "stackTrace",
            "inner_error",
            "innerError",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Message,
            FullTypeName,
            StackTrace,
            InnerError,
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
                            "message" => Ok(GeneratedField::Message),
                            "fullTypeName" | "full_type_name" => Ok(GeneratedField::FullTypeName),
                            "stackTrace" | "stack_trace" => Ok(GeneratedField::StackTrace),
                            "innerError" | "inner_error" => Ok(GeneratedField::InnerError),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ErrorResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.ErrorResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ErrorResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message__ = None;
                let mut full_type_name__ = None;
                let mut stack_trace__ = None;
                let mut inner_error__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map.next_value()?);
                        }
                        GeneratedField::FullTypeName => {
                            if full_type_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullTypeName"));
                            }
                            full_type_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::StackTrace => {
                            if stack_trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stackTrace"));
                            }
                            stack_trace__ = Some(map.next_value()?);
                        }
                        GeneratedField::InnerError => {
                            if inner_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("innerError"));
                            }
                            inner_error__ = map.next_value()?;
                        }
                    }
                }
                Ok(ErrorResponse {
                    message: message__.unwrap_or_default(),
                    full_type_name: full_type_name__.unwrap_or_default(),
                    stack_trace: stack_trace__.unwrap_or_default(),
                    inner_error: inner_error__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.ErrorResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileByExtensionDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file_extension.is_empty() {
            len += 1;
        }
        if self.file_count != 0 {
            len += 1;
        }
        if self.file_byte_length != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.FileByExtensionDetails", len)?;
        if !self.file_extension.is_empty() {
            struct_ser.serialize_field("fileExtension", &self.file_extension)?;
        }
        if self.file_count != 0 {
            struct_ser.serialize_field("fileCount", ToString::to_string(&self.file_count).as_str())?;
        }
        if self.file_byte_length != 0 {
            struct_ser.serialize_field("fileByteLength", ToString::to_string(&self.file_byte_length).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileByExtensionDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_extension",
            "fileExtension",
            "file_count",
            "fileCount",
            "file_byte_length",
            "fileByteLength",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FileExtension,
            FileCount,
            FileByteLength,
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
                            "fileExtension" | "file_extension" => Ok(GeneratedField::FileExtension),
                            "fileCount" | "file_count" => Ok(GeneratedField::FileCount),
                            "fileByteLength" | "file_byte_length" => Ok(GeneratedField::FileByteLength),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileByExtensionDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.FileByExtensionDetails")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FileByExtensionDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_extension__ = None;
                let mut file_count__ = None;
                let mut file_byte_length__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FileExtension => {
                            if file_extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileExtension"));
                            }
                            file_extension__ = Some(map.next_value()?);
                        }
                        GeneratedField::FileCount => {
                            if file_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileCount"));
                            }
                            file_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FileByteLength => {
                            if file_byte_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileByteLength"));
                            }
                            file_byte_length__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(FileByExtensionDetails {
                    file_extension: file_extension__.unwrap_or_default(),
                    file_count: file_count__.unwrap_or_default(),
                    file_byte_length: file_byte_length__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.FileByExtensionDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("searchium.v1.FileEntry", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.FileEntry")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FileEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(FileEntry {
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.FileEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileExtract {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.text.is_empty() {
            len += 1;
        }
        if self.offset != 0 {
            len += 1;
        }
        if self.length != 0 {
            len += 1;
        }
        if self.line_number != 0 {
            len += 1;
        }
        if self.column_number != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.FileExtract", len)?;
        if !self.text.is_empty() {
            struct_ser.serialize_field("text", &self.text)?;
        }
        if self.offset != 0 {
            struct_ser.serialize_field("offset", &self.offset)?;
        }
        if self.length != 0 {
            struct_ser.serialize_field("length", &self.length)?;
        }
        if self.line_number != 0 {
            struct_ser.serialize_field("lineNumber", &self.line_number)?;
        }
        if self.column_number != 0 {
            struct_ser.serialize_field("columnNumber", &self.column_number)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileExtract {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "text",
            "offset",
            "length",
            "line_number",
            "lineNumber",
            "column_number",
            "columnNumber",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Text,
            Offset,
            Length,
            LineNumber,
            ColumnNumber,
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
                            "text" => Ok(GeneratedField::Text),
                            "offset" => Ok(GeneratedField::Offset),
                            "length" => Ok(GeneratedField::Length),
                            "lineNumber" | "line_number" => Ok(GeneratedField::LineNumber),
                            "columnNumber" | "column_number" => Ok(GeneratedField::ColumnNumber),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileExtract;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.FileExtract")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FileExtract, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut text__ = None;
                let mut offset__ = None;
                let mut length__ = None;
                let mut line_number__ = None;
                let mut column_number__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Text => {
                            if text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            text__ = Some(map.next_value()?);
                        }
                        GeneratedField::Offset => {
                            if offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            offset__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Length => {
                            if length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("length"));
                            }
                            length__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LineNumber => {
                            if line_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lineNumber"));
                            }
                            line_number__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ColumnNumber => {
                            if column_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnNumber"));
                            }
                            column_number__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(FileExtract {
                    text: text__.unwrap_or_default(),
                    offset: offset__.unwrap_or_default(),
                    length: length__.unwrap_or_default(),
                    line_number: line_number__.unwrap_or_default(),
                    column_number: column_number__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.FileExtract", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FilePositionSpan {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.position != 0 {
            len += 1;
        }
        if self.length != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.FilePositionSpan", len)?;
        if self.position != 0 {
            struct_ser.serialize_field("position", &self.position)?;
        }
        if self.length != 0 {
            struct_ser.serialize_field("length", &self.length)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilePositionSpan {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "position",
            "length",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Position,
            Length,
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
                            "position" => Ok(GeneratedField::Position),
                            "length" => Ok(GeneratedField::Length),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FilePositionSpan;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.FilePositionSpan")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FilePositionSpan, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut position__ = None;
                let mut length__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Position => {
                            if position__.is_some() {
                                return Err(serde::de::Error::duplicate_field("position"));
                            }
                            position__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Length => {
                            if length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("length"));
                            }
                            length__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(FilePositionSpan {
                    position: position__.unwrap_or_default(),
                    length: length__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.FilePositionSpan", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FilePositionsData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.positions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.FilePositionsData", len)?;
        if !self.positions.is_empty() {
            struct_ser.serialize_field("positions", &self.positions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilePositionsData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "positions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Positions,
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
                            "positions" => Ok(GeneratedField::Positions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FilePositionsData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.FilePositionsData")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FilePositionsData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut positions__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Positions => {
                            if positions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("positions"));
                            }
                            positions__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FilePositionsData {
                    positions: positions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.FilePositionsData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileSystemEntry {
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
        if self.data.is_some() {
            len += 1;
        }
        if self.subtype.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.FileSystemEntry", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        if let Some(v) = self.subtype.as_ref() {
            match v {
                file_system_entry::Subtype::FileEntry(v) => {
                    struct_ser.serialize_field("fileEntry", v)?;
                }
                file_system_entry::Subtype::DirectoryEntry(v) => {
                    struct_ser.serialize_field("directoryEntry", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileSystemEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "data",
            "file_entry",
            "fileEntry",
            "directory_entry",
            "directoryEntry",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Data,
            FileEntry,
            DirectoryEntry,
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
                            "data" => Ok(GeneratedField::Data),
                            "fileEntry" | "file_entry" => Ok(GeneratedField::FileEntry),
                            "directoryEntry" | "directory_entry" => Ok(GeneratedField::DirectoryEntry),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileSystemEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.FileSystemEntry")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FileSystemEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut data__ = None;
                let mut subtype__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = map.next_value()?;
                        }
                        GeneratedField::FileEntry => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileEntry"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(file_system_entry::Subtype::FileEntry)
;
                        }
                        GeneratedField::DirectoryEntry => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directoryEntry"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(file_system_entry::Subtype::DirectoryEntry)
;
                        }
                    }
                }
                Ok(FileSystemEntry {
                    name: name__.unwrap_or_default(),
                    data: data__,
                    subtype: subtype__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.FileSystemEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileSystemEntryData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.file_positions_data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.FileSystemEntryData", len)?;
        if let Some(v) = self.file_positions_data.as_ref() {
            struct_ser.serialize_field("filePositionsData", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileSystemEntryData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_positions_data",
            "filePositionsData",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FilePositionsData,
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
                            "filePositionsData" | "file_positions_data" => Ok(GeneratedField::FilePositionsData),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileSystemEntryData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.FileSystemEntryData")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FileSystemEntryData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_positions_data__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FilePositionsData => {
                            if file_positions_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filePositionsData"));
                            }
                            file_positions_data__ = map.next_value()?;
                        }
                    }
                }
                Ok(FileSystemEntryData {
                    file_positions_data: file_positions_data__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.FileSystemEntryData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileSystemScanFinished {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.old_version != 0 {
            len += 1;
        }
        if self.new_version != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.FileSystemScanFinished", len)?;
        if self.old_version != 0 {
            struct_ser.serialize_field("oldVersion", &self.old_version)?;
        }
        if self.new_version != 0 {
            struct_ser.serialize_field("newVersion", &self.new_version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileSystemScanFinished {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "old_version",
            "oldVersion",
            "new_version",
            "newVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OldVersion,
            NewVersion,
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
                            "oldVersion" | "old_version" => Ok(GeneratedField::OldVersion),
                            "newVersion" | "new_version" => Ok(GeneratedField::NewVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileSystemScanFinished;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.FileSystemScanFinished")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FileSystemScanFinished, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut old_version__ = None;
                let mut new_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::OldVersion => {
                            if old_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oldVersion"));
                            }
                            old_version__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NewVersion => {
                            if new_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newVersion"));
                            }
                            new_version__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(FileSystemScanFinished {
                    old_version: old_version__.unwrap_or_default(),
                    new_version: new_version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.FileSystemScanFinished", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileSystemScanStarted {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("searchium.v1.FileSystemScanStarted", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileSystemScanStarted {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileSystemScanStarted;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.FileSystemScanStarted")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FileSystemScanStarted, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(FileSystemScanStarted {
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.FileSystemScanStarted", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileSystemTree {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if !self.projects.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.FileSystemTree", len)?;
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.projects.is_empty() {
            struct_ser.serialize_field("projects", &self.projects)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileSystemTree {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "projects",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            Projects,
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
                            "version" => Ok(GeneratedField::Version),
                            "projects" => Ok(GeneratedField::Projects),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileSystemTree;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.FileSystemTree")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FileSystemTree, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut projects__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Projects => {
                            if projects__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projects"));
                            }
                            projects__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FileSystemTree {
                    version: version__.unwrap_or_default(),
                    projects: projects__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.FileSystemTree", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileSystemTreeObsolete {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if self.root.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.FileSystemTree_Obsolete", len)?;
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if let Some(v) = self.root.as_ref() {
            struct_ser.serialize_field("root", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileSystemTreeObsolete {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "root",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            Root,
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
                            "version" => Ok(GeneratedField::Version),
                            "root" => Ok(GeneratedField::Root),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileSystemTreeObsolete;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.FileSystemTree_Obsolete")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FileSystemTreeObsolete, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut root__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Root => {
                            if root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("root"));
                            }
                            root__ = map.next_value()?;
                        }
                    }
                }
                Ok(FileSystemTreeObsolete {
                    version: version__.unwrap_or_default(),
                    root: root__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.FileSystemTree_Obsolete", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDatabaseDetailsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_files_by_extension_details_count != 0 {
            len += 1;
        }
        if self.max_large_files_details_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.GetDatabaseDetailsRequest", len)?;
        if self.max_files_by_extension_details_count != 0 {
            struct_ser.serialize_field("maxFilesByExtensionDetailsCount", &self.max_files_by_extension_details_count)?;
        }
        if self.max_large_files_details_count != 0 {
            struct_ser.serialize_field("maxLargeFilesDetailsCount", &self.max_large_files_details_count)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDatabaseDetailsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_files_by_extension_details_count",
            "maxFilesByExtensionDetailsCount",
            "max_large_files_details_count",
            "maxLargeFilesDetailsCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxFilesByExtensionDetailsCount,
            MaxLargeFilesDetailsCount,
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
                            "maxFilesByExtensionDetailsCount" | "max_files_by_extension_details_count" => Ok(GeneratedField::MaxFilesByExtensionDetailsCount),
                            "maxLargeFilesDetailsCount" | "max_large_files_details_count" => Ok(GeneratedField::MaxLargeFilesDetailsCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDatabaseDetailsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.GetDatabaseDetailsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetDatabaseDetailsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_files_by_extension_details_count__ = None;
                let mut max_large_files_details_count__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MaxFilesByExtensionDetailsCount => {
                            if max_files_by_extension_details_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxFilesByExtensionDetailsCount"));
                            }
                            max_files_by_extension_details_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxLargeFilesDetailsCount => {
                            if max_large_files_details_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxLargeFilesDetailsCount"));
                            }
                            max_large_files_details_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetDatabaseDetailsRequest {
                    max_files_by_extension_details_count: max_files_by_extension_details_count__.unwrap_or_default(),
                    max_large_files_details_count: max_large_files_details_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.GetDatabaseDetailsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDatabaseDetailsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_details.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.GetDatabaseDetailsResponse", len)?;
        if !self.project_details.is_empty() {
            struct_ser.serialize_field("projectDetails", &self.project_details)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDatabaseDetailsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_details",
            "projectDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectDetails,
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
                            "projectDetails" | "project_details" => Ok(GeneratedField::ProjectDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDatabaseDetailsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.GetDatabaseDetailsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetDatabaseDetailsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_details__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProjectDetails => {
                            if project_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectDetails"));
                            }
                            project_details__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetDatabaseDetailsResponse {
                    project_details: project_details__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.GetDatabaseDetailsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDatabaseStatisticsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.force_garbage_collection {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.GetDatabaseStatisticsRequest", len)?;
        if self.force_garbage_collection {
            struct_ser.serialize_field("forceGarbageCollection", &self.force_garbage_collection)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDatabaseStatisticsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "force_garbage_collection",
            "forceGarbageCollection",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ForceGarbageCollection,
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
                            "forceGarbageCollection" | "force_garbage_collection" => Ok(GeneratedField::ForceGarbageCollection),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDatabaseStatisticsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.GetDatabaseStatisticsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetDatabaseStatisticsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut force_garbage_collection__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ForceGarbageCollection => {
                            if force_garbage_collection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forceGarbageCollection"));
                            }
                            force_garbage_collection__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetDatabaseStatisticsRequest {
                    force_garbage_collection: force_garbage_collection__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.GetDatabaseStatisticsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDatabaseStatisticsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.project_count != 0 {
            len += 1;
        }
        if self.file_count != 0 {
            len += 1;
        }
        if self.searchable_file_count != 0 {
            len += 1;
        }
        if self.server_native_memory_usage != 0 {
            len += 1;
        }
        if self.server_gc_memory_usage != 0 {
            len += 1;
        }
        if self.index_last_updated_utc.is_some() {
            len += 1;
        }
        if self.server_status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.GetDatabaseStatisticsResponse", len)?;
        if self.project_count != 0 {
            struct_ser.serialize_field("projectCount", &self.project_count)?;
        }
        if self.file_count != 0 {
            struct_ser.serialize_field("fileCount", ToString::to_string(&self.file_count).as_str())?;
        }
        if self.searchable_file_count != 0 {
            struct_ser.serialize_field("searchableFileCount", ToString::to_string(&self.searchable_file_count).as_str())?;
        }
        if self.server_native_memory_usage != 0 {
            struct_ser.serialize_field("serverNativeMemoryUsage", ToString::to_string(&self.server_native_memory_usage).as_str())?;
        }
        if self.server_gc_memory_usage != 0 {
            struct_ser.serialize_field("serverGcMemoryUsage", ToString::to_string(&self.server_gc_memory_usage).as_str())?;
        }
        if let Some(v) = self.index_last_updated_utc.as_ref() {
            struct_ser.serialize_field("indexLastUpdatedUtc", v)?;
        }
        if self.server_status != 0 {
            let v = IndexingServerStatus::from_i32(self.server_status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.server_status)))?;
            struct_ser.serialize_field("serverStatus", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDatabaseStatisticsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_count",
            "projectCount",
            "file_count",
            "fileCount",
            "searchable_file_count",
            "searchableFileCount",
            "server_native_memory_usage",
            "serverNativeMemoryUsage",
            "server_gc_memory_usage",
            "serverGcMemoryUsage",
            "index_last_updated_utc",
            "indexLastUpdatedUtc",
            "server_status",
            "serverStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectCount,
            FileCount,
            SearchableFileCount,
            ServerNativeMemoryUsage,
            ServerGcMemoryUsage,
            IndexLastUpdatedUtc,
            ServerStatus,
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
                            "projectCount" | "project_count" => Ok(GeneratedField::ProjectCount),
                            "fileCount" | "file_count" => Ok(GeneratedField::FileCount),
                            "searchableFileCount" | "searchable_file_count" => Ok(GeneratedField::SearchableFileCount),
                            "serverNativeMemoryUsage" | "server_native_memory_usage" => Ok(GeneratedField::ServerNativeMemoryUsage),
                            "serverGcMemoryUsage" | "server_gc_memory_usage" => Ok(GeneratedField::ServerGcMemoryUsage),
                            "indexLastUpdatedUtc" | "index_last_updated_utc" => Ok(GeneratedField::IndexLastUpdatedUtc),
                            "serverStatus" | "server_status" => Ok(GeneratedField::ServerStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDatabaseStatisticsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.GetDatabaseStatisticsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetDatabaseStatisticsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_count__ = None;
                let mut file_count__ = None;
                let mut searchable_file_count__ = None;
                let mut server_native_memory_usage__ = None;
                let mut server_gc_memory_usage__ = None;
                let mut index_last_updated_utc__ = None;
                let mut server_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProjectCount => {
                            if project_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectCount"));
                            }
                            project_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FileCount => {
                            if file_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileCount"));
                            }
                            file_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SearchableFileCount => {
                            if searchable_file_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchableFileCount"));
                            }
                            searchable_file_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ServerNativeMemoryUsage => {
                            if server_native_memory_usage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverNativeMemoryUsage"));
                            }
                            server_native_memory_usage__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ServerGcMemoryUsage => {
                            if server_gc_memory_usage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverGcMemoryUsage"));
                            }
                            server_gc_memory_usage__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IndexLastUpdatedUtc => {
                            if index_last_updated_utc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("indexLastUpdatedUtc"));
                            }
                            index_last_updated_utc__ = map.next_value()?;
                        }
                        GeneratedField::ServerStatus => {
                            if server_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverStatus"));
                            }
                            server_status__ = Some(map.next_value::<IndexingServerStatus>()? as i32);
                        }
                    }
                }
                Ok(GetDatabaseStatisticsResponse {
                    project_count: project_count__.unwrap_or_default(),
                    file_count: file_count__.unwrap_or_default(),
                    searchable_file_count: searchable_file_count__.unwrap_or_default(),
                    server_native_memory_usage: server_native_memory_usage__.unwrap_or_default(),
                    server_gc_memory_usage: server_gc_memory_usage__.unwrap_or_default(),
                    index_last_updated_utc: index_last_updated_utc__,
                    server_status: server_status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.GetDatabaseStatisticsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDirectoryDetailsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        if self.max_files_by_extension_details_count != 0 {
            len += 1;
        }
        if self.max_large_files_details_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.GetDirectoryDetailsRequest", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if self.max_files_by_extension_details_count != 0 {
            struct_ser.serialize_field("maxFilesByExtensionDetailsCount", &self.max_files_by_extension_details_count)?;
        }
        if self.max_large_files_details_count != 0 {
            struct_ser.serialize_field("maxLargeFilesDetailsCount", &self.max_large_files_details_count)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDirectoryDetailsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "max_files_by_extension_details_count",
            "maxFilesByExtensionDetailsCount",
            "max_large_files_details_count",
            "maxLargeFilesDetailsCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            MaxFilesByExtensionDetailsCount,
            MaxLargeFilesDetailsCount,
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
                            "path" => Ok(GeneratedField::Path),
                            "maxFilesByExtensionDetailsCount" | "max_files_by_extension_details_count" => Ok(GeneratedField::MaxFilesByExtensionDetailsCount),
                            "maxLargeFilesDetailsCount" | "max_large_files_details_count" => Ok(GeneratedField::MaxLargeFilesDetailsCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDirectoryDetailsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.GetDirectoryDetailsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetDirectoryDetailsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut max_files_by_extension_details_count__ = None;
                let mut max_large_files_details_count__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxFilesByExtensionDetailsCount => {
                            if max_files_by_extension_details_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxFilesByExtensionDetailsCount"));
                            }
                            max_files_by_extension_details_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxLargeFilesDetailsCount => {
                            if max_large_files_details_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxLargeFilesDetailsCount"));
                            }
                            max_large_files_details_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetDirectoryDetailsRequest {
                    path: path__.unwrap_or_default(),
                    max_files_by_extension_details_count: max_files_by_extension_details_count__.unwrap_or_default(),
                    max_large_files_details_count: max_large_files_details_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.GetDirectoryDetailsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDirectoryDetailsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.directory_details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.GetDirectoryDetailsResponse", len)?;
        if let Some(v) = self.directory_details.as_ref() {
            struct_ser.serialize_field("directoryDetails", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDirectoryDetailsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "directory_details",
            "directoryDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DirectoryDetails,
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
                            "directoryDetails" | "directory_details" => Ok(GeneratedField::DirectoryDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDirectoryDetailsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.GetDirectoryDetailsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetDirectoryDetailsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut directory_details__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DirectoryDetails => {
                            if directory_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directoryDetails"));
                            }
                            directory_details__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetDirectoryDetailsResponse {
                    directory_details: directory_details__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.GetDirectoryDetailsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDirectoryEntriesMultipleRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_path.is_empty() {
            len += 1;
        }
        if !self.relative_path_list.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.GetDirectoryEntriesMultipleRequest", len)?;
        if !self.project_path.is_empty() {
            struct_ser.serialize_field("projectPath", &self.project_path)?;
        }
        if !self.relative_path_list.is_empty() {
            struct_ser.serialize_field("relativePathList", &self.relative_path_list)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDirectoryEntriesMultipleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_path",
            "projectPath",
            "relative_path_list",
            "relativePathList",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectPath,
            RelativePathList,
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
                            "projectPath" | "project_path" => Ok(GeneratedField::ProjectPath),
                            "relativePathList" | "relative_path_list" => Ok(GeneratedField::RelativePathList),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDirectoryEntriesMultipleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.GetDirectoryEntriesMultipleRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetDirectoryEntriesMultipleRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_path__ = None;
                let mut relative_path_list__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProjectPath => {
                            if project_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectPath"));
                            }
                            project_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::RelativePathList => {
                            if relative_path_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relativePathList"));
                            }
                            relative_path_list__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetDirectoryEntriesMultipleRequest {
                    project_path: project_path__.unwrap_or_default(),
                    relative_path_list: relative_path_list__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.GetDirectoryEntriesMultipleRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDirectoryEntriesMultipleResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.directory_entries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.GetDirectoryEntriesMultipleResponse", len)?;
        if !self.directory_entries.is_empty() {
            struct_ser.serialize_field("directoryEntries", &self.directory_entries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDirectoryEntriesMultipleResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "directory_entries",
            "directoryEntries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DirectoryEntries,
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
                            "directoryEntries" | "directory_entries" => Ok(GeneratedField::DirectoryEntries),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDirectoryEntriesMultipleResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.GetDirectoryEntriesMultipleResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetDirectoryEntriesMultipleResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut directory_entries__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DirectoryEntries => {
                            if directory_entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directoryEntries"));
                            }
                            directory_entries__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetDirectoryEntriesMultipleResponse {
                    directory_entries: directory_entries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.GetDirectoryEntriesMultipleResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDirectoryEntriesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_path.is_empty() {
            len += 1;
        }
        if !self.directory_relative_path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.GetDirectoryEntriesRequest", len)?;
        if !self.project_path.is_empty() {
            struct_ser.serialize_field("projectPath", &self.project_path)?;
        }
        if !self.directory_relative_path.is_empty() {
            struct_ser.serialize_field("directoryRelativePath", &self.directory_relative_path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDirectoryEntriesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_path",
            "projectPath",
            "directory_relative_path",
            "directoryRelativePath",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectPath,
            DirectoryRelativePath,
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
                            "projectPath" | "project_path" => Ok(GeneratedField::ProjectPath),
                            "directoryRelativePath" | "directory_relative_path" => Ok(GeneratedField::DirectoryRelativePath),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDirectoryEntriesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.GetDirectoryEntriesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetDirectoryEntriesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_path__ = None;
                let mut directory_relative_path__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProjectPath => {
                            if project_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectPath"));
                            }
                            project_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::DirectoryRelativePath => {
                            if directory_relative_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directoryRelativePath"));
                            }
                            directory_relative_path__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetDirectoryEntriesRequest {
                    project_path: project_path__.unwrap_or_default(),
                    directory_relative_path: directory_relative_path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.GetDirectoryEntriesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDirectoryEntriesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.directory_entry.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.GetDirectoryEntriesResponse", len)?;
        if let Some(v) = self.directory_entry.as_ref() {
            struct_ser.serialize_field("directoryEntry", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDirectoryEntriesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "directory_entry",
            "directoryEntry",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DirectoryEntry,
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
                            "directoryEntry" | "directory_entry" => Ok(GeneratedField::DirectoryEntry),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDirectoryEntriesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.GetDirectoryEntriesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetDirectoryEntriesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut directory_entry__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DirectoryEntry => {
                            if directory_entry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directoryEntry"));
                            }
                            directory_entry__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetDirectoryEntriesResponse {
                    directory_entry: directory_entry__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.GetDirectoryEntriesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDirectoryStatisticsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.directory_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.GetDirectoryStatisticsRequest", len)?;
        if !self.directory_name.is_empty() {
            struct_ser.serialize_field("directoryName", &self.directory_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDirectoryStatisticsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "directory_name",
            "directoryName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DirectoryName,
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
                            "directoryName" | "directory_name" => Ok(GeneratedField::DirectoryName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDirectoryStatisticsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.GetDirectoryStatisticsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetDirectoryStatisticsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut directory_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DirectoryName => {
                            if directory_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directoryName"));
                            }
                            directory_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetDirectoryStatisticsRequest {
                    directory_name: directory_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.GetDirectoryStatisticsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDirectoryStatisticsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.directory_count != 0 {
            len += 1;
        }
        if self.file_count != 0 {
            len += 1;
        }
        if self.indexed_file_count != 0 {
            len += 1;
        }
        if self.total_indexed_file_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.GetDirectoryStatisticsResponse", len)?;
        if self.directory_count != 0 {
            struct_ser.serialize_field("directoryCount", &self.directory_count)?;
        }
        if self.file_count != 0 {
            struct_ser.serialize_field("fileCount", &self.file_count)?;
        }
        if self.indexed_file_count != 0 {
            struct_ser.serialize_field("indexedFileCount", &self.indexed_file_count)?;
        }
        if self.total_indexed_file_size != 0 {
            struct_ser.serialize_field("totalIndexedFileSize", ToString::to_string(&self.total_indexed_file_size).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDirectoryStatisticsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "directory_count",
            "directoryCount",
            "file_count",
            "fileCount",
            "indexed_file_count",
            "indexedFileCount",
            "total_indexed_file_size",
            "totalIndexedFileSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DirectoryCount,
            FileCount,
            IndexedFileCount,
            TotalIndexedFileSize,
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
                            "directoryCount" | "directory_count" => Ok(GeneratedField::DirectoryCount),
                            "fileCount" | "file_count" => Ok(GeneratedField::FileCount),
                            "indexedFileCount" | "indexed_file_count" => Ok(GeneratedField::IndexedFileCount),
                            "totalIndexedFileSize" | "total_indexed_file_size" => Ok(GeneratedField::TotalIndexedFileSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDirectoryStatisticsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.GetDirectoryStatisticsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetDirectoryStatisticsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut directory_count__ = None;
                let mut file_count__ = None;
                let mut indexed_file_count__ = None;
                let mut total_indexed_file_size__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DirectoryCount => {
                            if directory_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directoryCount"));
                            }
                            directory_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FileCount => {
                            if file_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileCount"));
                            }
                            file_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IndexedFileCount => {
                            if indexed_file_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("indexedFileCount"));
                            }
                            indexed_file_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalIndexedFileSize => {
                            if total_indexed_file_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalIndexedFileSize"));
                            }
                            total_indexed_file_size__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetDirectoryStatisticsResponse {
                    directory_count: directory_count__.unwrap_or_default(),
                    file_count: file_count__.unwrap_or_default(),
                    indexed_file_count: indexed_file_count__.unwrap_or_default(),
                    total_indexed_file_size: total_indexed_file_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.GetDirectoryStatisticsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFileExtractsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file_name.is_empty() {
            len += 1;
        }
        if !self.positions.is_empty() {
            len += 1;
        }
        if self.max_extract_length != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.GetFileExtractsRequest", len)?;
        if !self.file_name.is_empty() {
            struct_ser.serialize_field("fileName", &self.file_name)?;
        }
        if !self.positions.is_empty() {
            struct_ser.serialize_field("positions", &self.positions)?;
        }
        if self.max_extract_length != 0 {
            struct_ser.serialize_field("maxExtractLength", &self.max_extract_length)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFileExtractsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_name",
            "fileName",
            "positions",
            "max_extract_length",
            "maxExtractLength",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FileName,
            Positions,
            MaxExtractLength,
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
                            "fileName" | "file_name" => Ok(GeneratedField::FileName),
                            "positions" => Ok(GeneratedField::Positions),
                            "maxExtractLength" | "max_extract_length" => Ok(GeneratedField::MaxExtractLength),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFileExtractsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.GetFileExtractsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetFileExtractsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_name__ = None;
                let mut positions__ = None;
                let mut max_extract_length__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FileName => {
                            if file_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileName"));
                            }
                            file_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Positions => {
                            if positions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("positions"));
                            }
                            positions__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxExtractLength => {
                            if max_extract_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxExtractLength"));
                            }
                            max_extract_length__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetFileExtractsRequest {
                    file_name: file_name__.unwrap_or_default(),
                    positions: positions__.unwrap_or_default(),
                    max_extract_length: max_extract_length__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.GetFileExtractsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFileExtractsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file_name.is_empty() {
            len += 1;
        }
        if !self.file_extracts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.GetFileExtractsResponse", len)?;
        if !self.file_name.is_empty() {
            struct_ser.serialize_field("fileName", &self.file_name)?;
        }
        if !self.file_extracts.is_empty() {
            struct_ser.serialize_field("fileExtracts", &self.file_extracts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFileExtractsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_name",
            "fileName",
            "file_extracts",
            "fileExtracts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FileName,
            FileExtracts,
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
                            "fileName" | "file_name" => Ok(GeneratedField::FileName),
                            "fileExtracts" | "file_extracts" => Ok(GeneratedField::FileExtracts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFileExtractsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.GetFileExtractsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetFileExtractsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_name__ = None;
                let mut file_extracts__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FileName => {
                            if file_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileName"));
                            }
                            file_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::FileExtracts => {
                            if file_extracts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileExtracts"));
                            }
                            file_extracts__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetFileExtractsResponse {
                    file_name: file_name__.unwrap_or_default(),
                    file_extracts: file_extracts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.GetFileExtractsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFileSystemRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.known_version != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.GetFileSystemRequest", len)?;
        if self.known_version != 0 {
            struct_ser.serialize_field("knownVersion", &self.known_version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFileSystemRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "known_version",
            "knownVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KnownVersion,
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
                            "knownVersion" | "known_version" => Ok(GeneratedField::KnownVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFileSystemRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.GetFileSystemRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetFileSystemRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut known_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::KnownVersion => {
                            if known_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("knownVersion"));
                            }
                            known_version__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetFileSystemRequest {
                    known_version: known_version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.GetFileSystemRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFileSystemResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.file_system_tree_obsolete.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.GetFileSystemResponse", len)?;
        if let Some(v) = self.file_system_tree_obsolete.as_ref() {
            struct_ser.serialize_field("fileSystemTreeObsolete", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFileSystemResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_system_tree_obsolete",
            "fileSystemTreeObsolete",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FileSystemTreeObsolete,
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
                            "fileSystemTreeObsolete" | "file_system_tree_obsolete" => Ok(GeneratedField::FileSystemTreeObsolete),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFileSystemResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.GetFileSystemResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetFileSystemResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_system_tree_obsolete__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FileSystemTreeObsolete => {
                            if file_system_tree_obsolete__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileSystemTreeObsolete"));
                            }
                            file_system_tree_obsolete__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetFileSystemResponse {
                    file_system_tree_obsolete: file_system_tree_obsolete__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.GetFileSystemResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFileSystemTreeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.file_system_tree.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.GetFileSystemTreeResponse", len)?;
        if let Some(v) = self.file_system_tree.as_ref() {
            struct_ser.serialize_field("fileSystemTree", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFileSystemTreeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_system_tree",
            "fileSystemTree",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FileSystemTree,
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
                            "fileSystemTree" | "file_system_tree" => Ok(GeneratedField::FileSystemTree),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFileSystemTreeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.GetFileSystemTreeResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetFileSystemTreeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_system_tree__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FileSystemTree => {
                            if file_system_tree__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileSystemTree"));
                            }
                            file_system_tree__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetFileSystemTreeResponse {
                    file_system_tree: file_system_tree__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.GetFileSystemTreeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFileSystemVersionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("searchium.v1.GetFileSystemVersionRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFileSystemVersionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFileSystemVersionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.GetFileSystemVersionRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetFileSystemVersionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetFileSystemVersionRequest {
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.GetFileSystemVersionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFileSystemVersionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.GetFileSystemVersionResponse", len)?;
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFileSystemVersionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = GetFileSystemVersionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.GetFileSystemVersionResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetFileSystemVersionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetFileSystemVersionResponse {
                    version: version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.GetFileSystemVersionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFilesystemTreeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.known_version != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.GetFilesystemTreeRequest", len)?;
        if self.known_version != 0 {
            struct_ser.serialize_field("knownVersion", &self.known_version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFilesystemTreeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "known_version",
            "knownVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KnownVersion,
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
                            "knownVersion" | "known_version" => Ok(GeneratedField::KnownVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFilesystemTreeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.GetFilesystemTreeRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetFilesystemTreeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut known_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::KnownVersion => {
                            if known_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("knownVersion"));
                            }
                            known_version__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetFilesystemTreeRequest {
                    known_version: known_version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.GetFilesystemTreeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetProjectDetailsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_path.is_empty() {
            len += 1;
        }
        if self.max_files_by_extension_details_count != 0 {
            len += 1;
        }
        if self.max_large_files_details_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.GetProjectDetailsRequest", len)?;
        if !self.project_path.is_empty() {
            struct_ser.serialize_field("projectPath", &self.project_path)?;
        }
        if self.max_files_by_extension_details_count != 0 {
            struct_ser.serialize_field("maxFilesByExtensionDetailsCount", &self.max_files_by_extension_details_count)?;
        }
        if self.max_large_files_details_count != 0 {
            struct_ser.serialize_field("maxLargeFilesDetailsCount", &self.max_large_files_details_count)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetProjectDetailsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_path",
            "projectPath",
            "max_files_by_extension_details_count",
            "maxFilesByExtensionDetailsCount",
            "max_large_files_details_count",
            "maxLargeFilesDetailsCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectPath,
            MaxFilesByExtensionDetailsCount,
            MaxLargeFilesDetailsCount,
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
                            "projectPath" | "project_path" => Ok(GeneratedField::ProjectPath),
                            "maxFilesByExtensionDetailsCount" | "max_files_by_extension_details_count" => Ok(GeneratedField::MaxFilesByExtensionDetailsCount),
                            "maxLargeFilesDetailsCount" | "max_large_files_details_count" => Ok(GeneratedField::MaxLargeFilesDetailsCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetProjectDetailsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.GetProjectDetailsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetProjectDetailsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_path__ = None;
                let mut max_files_by_extension_details_count__ = None;
                let mut max_large_files_details_count__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProjectPath => {
                            if project_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectPath"));
                            }
                            project_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxFilesByExtensionDetailsCount => {
                            if max_files_by_extension_details_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxFilesByExtensionDetailsCount"));
                            }
                            max_files_by_extension_details_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxLargeFilesDetailsCount => {
                            if max_large_files_details_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxLargeFilesDetailsCount"));
                            }
                            max_large_files_details_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetProjectDetailsRequest {
                    project_path: project_path__.unwrap_or_default(),
                    max_files_by_extension_details_count: max_files_by_extension_details_count__.unwrap_or_default(),
                    max_large_files_details_count: max_large_files_details_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.GetProjectDetailsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetProjectDetailsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.project_details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.GetProjectDetailsResponse", len)?;
        if let Some(v) = self.project_details.as_ref() {
            struct_ser.serialize_field("projectDetails", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetProjectDetailsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_details",
            "projectDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectDetails,
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
                            "projectDetails" | "project_details" => Ok(GeneratedField::ProjectDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetProjectDetailsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.GetProjectDetailsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetProjectDetailsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_details__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProjectDetails => {
                            if project_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectDetails"));
                            }
                            project_details__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetProjectDetailsResponse {
                    project_details: project_details__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.GetProjectDetailsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IndexingServerStateChangedEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.server_status != 0 {
            len += 1;
        }
        if self.last_index_updated_utc.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.IndexingServerStateChangedEvent", len)?;
        if self.server_status != 0 {
            let v = IndexingServerStatus::from_i32(self.server_status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.server_status)))?;
            struct_ser.serialize_field("serverStatus", &v)?;
        }
        if let Some(v) = self.last_index_updated_utc.as_ref() {
            struct_ser.serialize_field("lastIndexUpdatedUtc", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IndexingServerStateChangedEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "server_status",
            "serverStatus",
            "last_index_updated_utc",
            "lastIndexUpdatedUtc",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServerStatus,
            LastIndexUpdatedUtc,
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
                            "serverStatus" | "server_status" => Ok(GeneratedField::ServerStatus),
                            "lastIndexUpdatedUtc" | "last_index_updated_utc" => Ok(GeneratedField::LastIndexUpdatedUtc),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IndexingServerStateChangedEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.IndexingServerStateChangedEvent")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IndexingServerStateChangedEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut server_status__ = None;
                let mut last_index_updated_utc__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ServerStatus => {
                            if server_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverStatus"));
                            }
                            server_status__ = Some(map.next_value::<IndexingServerStatus>()? as i32);
                        }
                        GeneratedField::LastIndexUpdatedUtc => {
                            if last_index_updated_utc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastIndexUpdatedUtc"));
                            }
                            last_index_updated_utc__ = map.next_value()?;
                        }
                    }
                }
                Ok(IndexingServerStateChangedEvent {
                    server_status: server_status__.unwrap_or_default(),
                    last_index_updated_utc: last_index_updated_utc__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.IndexingServerStateChangedEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IndexingServerStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Idle => "INDEXING_SERVER_STATUS_IDLE",
            Self::Paused => "INDEXING_SERVER_STATUS_PAUSED",
            Self::Yield => "INDEXING_SERVER_STATUS_YIELD",
            Self::Busy => "INDEXING_SERVER_STATUS_BUSY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for IndexingServerStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "INDEXING_SERVER_STATUS_IDLE",
            "INDEXING_SERVER_STATUS_PAUSED",
            "INDEXING_SERVER_STATUS_YIELD",
            "INDEXING_SERVER_STATUS_BUSY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IndexingServerStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(IndexingServerStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(IndexingServerStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "INDEXING_SERVER_STATUS_IDLE" => Ok(IndexingServerStatus::Idle),
                    "INDEXING_SERVER_STATUS_PAUSED" => Ok(IndexingServerStatus::Paused),
                    "INDEXING_SERVER_STATUS_YIELD" => Ok(IndexingServerStatus::Yield),
                    "INDEXING_SERVER_STATUS_BUSY" => Ok(IndexingServerStatus::Busy),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for IpcEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("searchium.v1.IpcEvent", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IpcEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IpcEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.IpcEvent")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IpcEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(IpcEvent {
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.IpcEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IpcMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request_id != 0 {
            len += 1;
        }
        if !self.protocol.is_empty() {
            len += 1;
        }
        if self.data.is_some() {
            len += 1;
        }
        if self.request_response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.IpcMessage", len)?;
        if self.request_id != 0 {
            struct_ser.serialize_field("requestId", ToString::to_string(&self.request_id).as_str())?;
        }
        if !self.protocol.is_empty() {
            struct_ser.serialize_field("protocol", &self.protocol)?;
        }
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        if let Some(v) = self.request_response.as_ref() {
            match v {
                ipc_message::RequestResponse::Request(v) => {
                    struct_ser.serialize_field("request", v)?;
                }
                ipc_message::RequestResponse::Response(v) => {
                    struct_ser.serialize_field("response", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IpcMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_id",
            "requestId",
            "protocol",
            "data",
            "request",
            "response",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestId,
            Protocol,
            Data,
            Request,
            Response,
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
                            "requestId" | "request_id" => Ok(GeneratedField::RequestId),
                            "protocol" => Ok(GeneratedField::Protocol),
                            "data" => Ok(GeneratedField::Data),
                            "request" => Ok(GeneratedField::Request),
                            "response" => Ok(GeneratedField::Response),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IpcMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.IpcMessage")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IpcMessage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_id__ = None;
                let mut protocol__ = None;
                let mut data__ = None;
                let mut request_response__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RequestId => {
                            if request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestId"));
                            }
                            request_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = Some(map.next_value()?);
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = map.next_value()?;
                        }
                        GeneratedField::Request => {
                            if request_response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            request_response__ = map.next_value::<::std::option::Option<_>>()?.map(ipc_message::RequestResponse::Request)
;
                        }
                        GeneratedField::Response => {
                            if request_response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            request_response__ = map.next_value::<::std::option::Option<_>>()?.map(ipc_message::RequestResponse::Response)
;
                        }
                    }
                }
                Ok(IpcMessage {
                    request_id: request_id__.unwrap_or_default(),
                    protocol: protocol__.unwrap_or_default(),
                    data: data__,
                    request_response: request_response__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.IpcMessage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IpcMessageData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subtype.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.IpcMessageData", len)?;
        if let Some(v) = self.subtype.as_ref() {
            match v {
                ipc_message_data::Subtype::IpcStringData(v) => {
                    struct_ser.serialize_field("ipcStringData", v)?;
                }
                ipc_message_data::Subtype::TypedMessage(v) => {
                    struct_ser.serialize_field("typedMessage", v)?;
                }
                ipc_message_data::Subtype::ErrorResponse(v) => {
                    struct_ser.serialize_field("errorResponse", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IpcMessageData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ipc_string_data",
            "ipcStringData",
            "typed_message",
            "typedMessage",
            "error_response",
            "errorResponse",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IpcStringData,
            TypedMessage,
            ErrorResponse,
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
                            "ipcStringData" | "ipc_string_data" => Ok(GeneratedField::IpcStringData),
                            "typedMessage" | "typed_message" => Ok(GeneratedField::TypedMessage),
                            "errorResponse" | "error_response" => Ok(GeneratedField::ErrorResponse),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IpcMessageData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.IpcMessageData")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IpcMessageData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subtype__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IpcStringData => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipcStringData"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(ipc_message_data::Subtype::IpcStringData)
;
                        }
                        GeneratedField::TypedMessage => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedMessage"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(ipc_message_data::Subtype::TypedMessage)
;
                        }
                        GeneratedField::ErrorResponse => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorResponse"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(ipc_message_data::Subtype::ErrorResponse)
;
                        }
                    }
                }
                Ok(IpcMessageData {
                    subtype: subtype__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.IpcMessageData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IpcRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.run_on_sequential_queue {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.IpcRequest", len)?;
        if self.run_on_sequential_queue {
            struct_ser.serialize_field("runOnSequentialQueue", &self.run_on_sequential_queue)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IpcRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run_on_sequential_queue",
            "runOnSequentialQueue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RunOnSequentialQueue,
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
                            "runOnSequentialQueue" | "run_on_sequential_queue" => Ok(GeneratedField::RunOnSequentialQueue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IpcRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.IpcRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IpcRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run_on_sequential_queue__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RunOnSequentialQueue => {
                            if run_on_sequential_queue__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runOnSequentialQueue"));
                            }
                            run_on_sequential_queue__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(IpcRequest {
                    run_on_sequential_queue: run_on_sequential_queue__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.IpcRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IpcResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ipc_event.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.IpcResponse", len)?;
        if let Some(v) = self.ipc_event.as_ref() {
            struct_ser.serialize_field("ipcEvent", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IpcResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ipc_event",
            "ipcEvent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IpcEvent,
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
                            "ipcEvent" | "ipc_event" => Ok(GeneratedField::IpcEvent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IpcResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.IpcResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IpcResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ipc_event__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IpcEvent => {
                            if ipc_event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipcEvent"));
                            }
                            ipc_event__ = map.next_value()?;
                        }
                    }
                }
                Ok(IpcResponse {
                    ipc_event: ipc_event__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.IpcResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IpcStringData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.IpcStringData", len)?;
        if !self.text.is_empty() {
            struct_ser.serialize_field("text", &self.text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IpcStringData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "text",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Text,
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
                            "text" => Ok(GeneratedField::Text),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IpcStringData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.IpcStringData")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IpcStringData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut text__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Text => {
                            if text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            text__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(IpcStringData {
                    text: text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.IpcStringData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LargeFileDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.relative_path.is_empty() {
            len += 1;
        }
        if self.byte_length != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.LargeFileDetails", len)?;
        if !self.relative_path.is_empty() {
            struct_ser.serialize_field("relativePath", &self.relative_path)?;
        }
        if self.byte_length != 0 {
            struct_ser.serialize_field("byteLength", ToString::to_string(&self.byte_length).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LargeFileDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "relative_path",
            "relativePath",
            "byte_length",
            "byteLength",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RelativePath,
            ByteLength,
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
                            "relativePath" | "relative_path" => Ok(GeneratedField::RelativePath),
                            "byteLength" | "byte_length" => Ok(GeneratedField::ByteLength),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LargeFileDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.LargeFileDetails")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LargeFileDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut relative_path__ = None;
                let mut byte_length__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RelativePath => {
                            if relative_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relativePath"));
                            }
                            relative_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::ByteLength => {
                            if byte_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("byteLength"));
                            }
                            byte_length__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(LargeFileDetails {
                    relative_path: relative_path__.unwrap_or_default(),
                    byte_length: byte_length__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.LargeFileDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OptionalDirectoryEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.has_value {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.OptionalDirectoryEntry", len)?;
        if self.has_value {
            struct_ser.serialize_field("hasValue", &self.has_value)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OptionalDirectoryEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "has_value",
            "hasValue",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HasValue,
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
                            "hasValue" | "has_value" => Ok(GeneratedField::HasValue),
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
            type Value = OptionalDirectoryEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.OptionalDirectoryEntry")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OptionalDirectoryEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut has_value__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HasValue => {
                            if has_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hasValue"));
                            }
                            has_value__ = Some(map.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                    }
                }
                Ok(OptionalDirectoryEntry {
                    has_value: has_value__.unwrap_or_default(),
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.OptionalDirectoryEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PairedTypedEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.operation_id != 0 {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        if self.subtype.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.PairedTypedEvent", len)?;
        if self.operation_id != 0 {
            struct_ser.serialize_field("operationId", ToString::to_string(&self.operation_id).as_str())?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if let Some(v) = self.subtype.as_ref() {
            match v {
                paired_typed_event::Subtype::FileSystemScanStarted(v) => {
                    struct_ser.serialize_field("fileSystemScanStarted", v)?;
                }
                paired_typed_event::Subtype::FileSystemScanFinished(v) => {
                    struct_ser.serialize_field("fileSystemScanFinished", v)?;
                }
                paired_typed_event::Subtype::SearchEngineFilesLoading(v) => {
                    struct_ser.serialize_field("searchEngineFilesLoading", v)?;
                }
                paired_typed_event::Subtype::SearchEngineFilesLoadingProgress(v) => {
                    struct_ser.serialize_field("searchEngineFilesLoadingProgress", v)?;
                }
                paired_typed_event::Subtype::SearchEngineFilesLoaded(v) => {
                    struct_ser.serialize_field("searchEngineFilesLoaded", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PairedTypedEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operation_id",
            "operationId",
            "error",
            "file_system_scan_started",
            "fileSystemScanStarted",
            "file_system_scan_finished",
            "fileSystemScanFinished",
            "search_engine_files_loading",
            "searchEngineFilesLoading",
            "search_engine_files_loading_progress",
            "searchEngineFilesLoadingProgress",
            "search_engine_files_loaded",
            "searchEngineFilesLoaded",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OperationId,
            Error,
            FileSystemScanStarted,
            FileSystemScanFinished,
            SearchEngineFilesLoading,
            SearchEngineFilesLoadingProgress,
            SearchEngineFilesLoaded,
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
                            "operationId" | "operation_id" => Ok(GeneratedField::OperationId),
                            "error" => Ok(GeneratedField::Error),
                            "fileSystemScanStarted" | "file_system_scan_started" => Ok(GeneratedField::FileSystemScanStarted),
                            "fileSystemScanFinished" | "file_system_scan_finished" => Ok(GeneratedField::FileSystemScanFinished),
                            "searchEngineFilesLoading" | "search_engine_files_loading" => Ok(GeneratedField::SearchEngineFilesLoading),
                            "searchEngineFilesLoadingProgress" | "search_engine_files_loading_progress" => Ok(GeneratedField::SearchEngineFilesLoadingProgress),
                            "searchEngineFilesLoaded" | "search_engine_files_loaded" => Ok(GeneratedField::SearchEngineFilesLoaded),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PairedTypedEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.PairedTypedEvent")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PairedTypedEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operation_id__ = None;
                let mut error__ = None;
                let mut subtype__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::OperationId => {
                            if operation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operationId"));
                            }
                            operation_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::FileSystemScanStarted => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileSystemScanStarted"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(paired_typed_event::Subtype::FileSystemScanStarted)
;
                        }
                        GeneratedField::FileSystemScanFinished => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileSystemScanFinished"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(paired_typed_event::Subtype::FileSystemScanFinished)
;
                        }
                        GeneratedField::SearchEngineFilesLoading => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchEngineFilesLoading"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(paired_typed_event::Subtype::SearchEngineFilesLoading)
;
                        }
                        GeneratedField::SearchEngineFilesLoadingProgress => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchEngineFilesLoadingProgress"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(paired_typed_event::Subtype::SearchEngineFilesLoadingProgress)
;
                        }
                        GeneratedField::SearchEngineFilesLoaded => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchEngineFilesLoaded"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(paired_typed_event::Subtype::SearchEngineFilesLoaded)
;
                        }
                    }
                }
                Ok(PairedTypedEvent {
                    operation_id: operation_id__.unwrap_or_default(),
                    error: error__,
                    subtype: subtype__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.PairedTypedEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PauseIndexingRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("searchium.v1.PauseIndexingRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PauseIndexingRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PauseIndexingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.PauseIndexingRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PauseIndexingRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PauseIndexingRequest {
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.PauseIndexingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PauseResumeIndexingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("searchium.v1.PauseResumeIndexingResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PauseResumeIndexingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PauseResumeIndexingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.PauseResumeIndexingResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PauseResumeIndexingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PauseResumeIndexingResponse {
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.PauseResumeIndexingResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProgressReportEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.display_text.is_empty() {
            len += 1;
        }
        if self.completed != 0 {
            len += 1;
        }
        if self.total != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.ProgressReportEvent", len)?;
        if !self.display_text.is_empty() {
            struct_ser.serialize_field("displayText", &self.display_text)?;
        }
        if self.completed != 0 {
            struct_ser.serialize_field("completed", &self.completed)?;
        }
        if self.total != 0 {
            struct_ser.serialize_field("total", &self.total)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProgressReportEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "display_text",
            "displayText",
            "completed",
            "total",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DisplayText,
            Completed,
            Total,
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
                            "displayText" | "display_text" => Ok(GeneratedField::DisplayText),
                            "completed" => Ok(GeneratedField::Completed),
                            "total" => Ok(GeneratedField::Total),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProgressReportEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.ProgressReportEvent")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProgressReportEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut display_text__ = None;
                let mut completed__ = None;
                let mut total__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DisplayText => {
                            if display_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayText"));
                            }
                            display_text__ = Some(map.next_value()?);
                        }
                        GeneratedField::Completed => {
                            if completed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("completed"));
                            }
                            completed__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ProgressReportEvent {
                    display_text: display_text__.unwrap_or_default(),
                    completed: completed__.unwrap_or_default(),
                    total: total__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.ProgressReportEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectConfigurationDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ignore_paths_section.is_some() {
            len += 1;
        }
        if self.ignore_searchable_files_section.is_some() {
            len += 1;
        }
        if self.include_searchable_files_section.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.ProjectConfigurationDetails", len)?;
        if let Some(v) = self.ignore_paths_section.as_ref() {
            struct_ser.serialize_field("ignorePathsSection", v)?;
        }
        if let Some(v) = self.ignore_searchable_files_section.as_ref() {
            struct_ser.serialize_field("ignoreSearchableFilesSection", v)?;
        }
        if let Some(v) = self.include_searchable_files_section.as_ref() {
            struct_ser.serialize_field("includeSearchableFilesSection", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProjectConfigurationDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ignore_paths_section",
            "ignorePathsSection",
            "ignore_searchable_files_section",
            "ignoreSearchableFilesSection",
            "include_searchable_files_section",
            "includeSearchableFilesSection",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IgnorePathsSection,
            IgnoreSearchableFilesSection,
            IncludeSearchableFilesSection,
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
                            "ignorePathsSection" | "ignore_paths_section" => Ok(GeneratedField::IgnorePathsSection),
                            "ignoreSearchableFilesSection" | "ignore_searchable_files_section" => Ok(GeneratedField::IgnoreSearchableFilesSection),
                            "includeSearchableFilesSection" | "include_searchable_files_section" => Ok(GeneratedField::IncludeSearchableFilesSection),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProjectConfigurationDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.ProjectConfigurationDetails")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProjectConfigurationDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ignore_paths_section__ = None;
                let mut ignore_searchable_files_section__ = None;
                let mut include_searchable_files_section__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IgnorePathsSection => {
                            if ignore_paths_section__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignorePathsSection"));
                            }
                            ignore_paths_section__ = map.next_value()?;
                        }
                        GeneratedField::IgnoreSearchableFilesSection => {
                            if ignore_searchable_files_section__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreSearchableFilesSection"));
                            }
                            ignore_searchable_files_section__ = map.next_value()?;
                        }
                        GeneratedField::IncludeSearchableFilesSection => {
                            if include_searchable_files_section__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeSearchableFilesSection"));
                            }
                            include_searchable_files_section__ = map.next_value()?;
                        }
                    }
                }
                Ok(ProjectConfigurationDetails {
                    ignore_paths_section: ignore_paths_section__,
                    ignore_searchable_files_section: ignore_searchable_files_section__,
                    include_searchable_files_section: include_searchable_files_section__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.ProjectConfigurationDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectConfigurationSectionDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.containing_file_path.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.contents.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.ProjectConfigurationSectionDetails", len)?;
        if !self.containing_file_path.is_empty() {
            struct_ser.serialize_field("containingFilePath", &self.containing_file_path)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.contents.is_empty() {
            struct_ser.serialize_field("contents", &self.contents)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProjectConfigurationSectionDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "containing_file_path",
            "containingFilePath",
            "name",
            "contents",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContainingFilePath,
            Name,
            Contents,
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
                            "containingFilePath" | "containing_file_path" => Ok(GeneratedField::ContainingFilePath),
                            "name" => Ok(GeneratedField::Name),
                            "contents" => Ok(GeneratedField::Contents),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProjectConfigurationSectionDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.ProjectConfigurationSectionDetails")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProjectConfigurationSectionDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut containing_file_path__ = None;
                let mut name__ = None;
                let mut contents__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ContainingFilePath => {
                            if containing_file_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("containingFilePath"));
                            }
                            containing_file_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Contents => {
                            if contents__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contents"));
                            }
                            contents__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ProjectConfigurationSectionDetails {
                    containing_file_path: containing_file_path__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    contents: contents__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.ProjectConfigurationSectionDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.root_path.is_empty() {
            len += 1;
        }
        if self.directory_details.is_some() {
            len += 1;
        }
        if self.project_configuration_details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.ProjectDetails", len)?;
        if !self.root_path.is_empty() {
            struct_ser.serialize_field("rootPath", &self.root_path)?;
        }
        if let Some(v) = self.directory_details.as_ref() {
            struct_ser.serialize_field("directoryDetails", v)?;
        }
        if let Some(v) = self.project_configuration_details.as_ref() {
            struct_ser.serialize_field("projectConfigurationDetails", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProjectDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "root_path",
            "rootPath",
            "directory_details",
            "directoryDetails",
            "project_configuration_details",
            "projectConfigurationDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RootPath,
            DirectoryDetails,
            ProjectConfigurationDetails,
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
                            "rootPath" | "root_path" => Ok(GeneratedField::RootPath),
                            "directoryDetails" | "directory_details" => Ok(GeneratedField::DirectoryDetails),
                            "projectConfigurationDetails" | "project_configuration_details" => Ok(GeneratedField::ProjectConfigurationDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProjectDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.ProjectDetails")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProjectDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut root_path__ = None;
                let mut directory_details__ = None;
                let mut project_configuration_details__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RootPath => {
                            if root_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rootPath"));
                            }
                            root_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::DirectoryDetails => {
                            if directory_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directoryDetails"));
                            }
                            directory_details__ = map.next_value()?;
                        }
                        GeneratedField::ProjectConfigurationDetails => {
                            if project_configuration_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectConfigurationDetails"));
                            }
                            project_configuration_details__ = map.next_value()?;
                        }
                    }
                }
                Ok(ProjectDetails {
                    root_path: root_path__.unwrap_or_default(),
                    directory_details: directory_details__,
                    project_configuration_details: project_configuration_details__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.ProjectDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.root_path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.ProjectEntry", len)?;
        if !self.root_path.is_empty() {
            struct_ser.serialize_field("rootPath", &self.root_path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProjectEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "root_path",
            "rootPath",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RootPath,
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
                            "rootPath" | "root_path" => Ok(GeneratedField::RootPath),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProjectEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.ProjectEntry")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProjectEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut root_path__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RootPath => {
                            if root_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rootPath"));
                            }
                            root_path__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ProjectEntry {
                    root_path: root_path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.ProjectEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RefreshFileSystemTreeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("searchium.v1.RefreshFileSystemTreeRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RefreshFileSystemTreeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RefreshFileSystemTreeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.RefreshFileSystemTreeRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RefreshFileSystemTreeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RefreshFileSystemTreeRequest {
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.RefreshFileSystemTreeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RefreshFileSystemTreeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("searchium.v1.RefreshFileSystemTreeResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RefreshFileSystemTreeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RefreshFileSystemTreeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.RefreshFileSystemTreeResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RefreshFileSystemTreeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RefreshFileSystemTreeResponse {
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.RefreshFileSystemTreeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegisterFileRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.RegisterFileRequest", len)?;
        if !self.file_name.is_empty() {
            struct_ser.serialize_field("fileName", &self.file_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisterFileRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_name",
            "fileName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FileName,
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
                            "fileName" | "file_name" => Ok(GeneratedField::FileName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegisterFileRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.RegisterFileRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RegisterFileRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FileName => {
                            if file_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileName"));
                            }
                            file_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RegisterFileRequest {
                    file_name: file_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.RegisterFileRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResumeIndexingRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("searchium.v1.ResumeIndexingRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResumeIndexingRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResumeIndexingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.ResumeIndexingRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ResumeIndexingRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ResumeIndexingRequest {
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.ResumeIndexingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchCodeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.search_params.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.SearchCodeRequest", len)?;
        if let Some(v) = self.search_params.as_ref() {
            struct_ser.serialize_field("searchParams", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchCodeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "search_params",
            "searchParams",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SearchParams,
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
                            "searchParams" | "search_params" => Ok(GeneratedField::SearchParams),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchCodeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.SearchCodeRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SearchCodeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut search_params__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SearchParams => {
                            if search_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchParams"));
                            }
                            search_params__ = map.next_value()?;
                        }
                    }
                }
                Ok(SearchCodeRequest {
                    search_params: search_params__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.SearchCodeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchCodeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.search_results.is_some() {
            len += 1;
        }
        if self.hit_count != 0 {
            len += 1;
        }
        if self.searched_file_count != 0 {
            len += 1;
        }
        if self.total_file_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.SearchCodeResponse", len)?;
        if let Some(v) = self.search_results.as_ref() {
            struct_ser.serialize_field("searchResults", v)?;
        }
        if self.hit_count != 0 {
            struct_ser.serialize_field("hitCount", ToString::to_string(&self.hit_count).as_str())?;
        }
        if self.searched_file_count != 0 {
            struct_ser.serialize_field("searchedFileCount", ToString::to_string(&self.searched_file_count).as_str())?;
        }
        if self.total_file_count != 0 {
            struct_ser.serialize_field("totalFileCount", ToString::to_string(&self.total_file_count).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchCodeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "search_results",
            "searchResults",
            "hit_count",
            "hitCount",
            "searched_file_count",
            "searchedFileCount",
            "total_file_count",
            "totalFileCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SearchResults,
            HitCount,
            SearchedFileCount,
            TotalFileCount,
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
                            "searchResults" | "search_results" => Ok(GeneratedField::SearchResults),
                            "hitCount" | "hit_count" => Ok(GeneratedField::HitCount),
                            "searchedFileCount" | "searched_file_count" => Ok(GeneratedField::SearchedFileCount),
                            "totalFileCount" | "total_file_count" => Ok(GeneratedField::TotalFileCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchCodeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.SearchCodeResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SearchCodeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut search_results__ = None;
                let mut hit_count__ = None;
                let mut searched_file_count__ = None;
                let mut total_file_count__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SearchResults => {
                            if search_results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchResults"));
                            }
                            search_results__ = map.next_value()?;
                        }
                        GeneratedField::HitCount => {
                            if hit_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hitCount"));
                            }
                            hit_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SearchedFileCount => {
                            if searched_file_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchedFileCount"));
                            }
                            searched_file_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalFileCount => {
                            if total_file_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalFileCount"));
                            }
                            total_file_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SearchCodeResponse {
                    search_results: search_results__,
                    hit_count: hit_count__.unwrap_or_default(),
                    searched_file_count: searched_file_count__.unwrap_or_default(),
                    total_file_count: total_file_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.SearchCodeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchEngineFilesLoaded {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tree_version != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.SearchEngineFilesLoaded", len)?;
        if self.tree_version != 0 {
            struct_ser.serialize_field("treeVersion", ToString::to_string(&self.tree_version).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchEngineFilesLoaded {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tree_version",
            "treeVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TreeVersion,
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
                            "treeVersion" | "tree_version" => Ok(GeneratedField::TreeVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchEngineFilesLoaded;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.SearchEngineFilesLoaded")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SearchEngineFilesLoaded, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tree_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TreeVersion => {
                            if tree_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("treeVersion"));
                            }
                            tree_version__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SearchEngineFilesLoaded {
                    tree_version: tree_version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.SearchEngineFilesLoaded", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchEngineFilesLoading {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("searchium.v1.SearchEngineFilesLoading", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchEngineFilesLoading {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchEngineFilesLoading;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.SearchEngineFilesLoading")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SearchEngineFilesLoading, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(SearchEngineFilesLoading {
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.SearchEngineFilesLoading", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchEngineFilesLoadingProgress {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("searchium.v1.SearchEngineFilesLoadingProgress", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchEngineFilesLoadingProgress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchEngineFilesLoadingProgress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.SearchEngineFilesLoadingProgress")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SearchEngineFilesLoadingProgress, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(SearchEngineFilesLoadingProgress {
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.SearchEngineFilesLoadingProgress", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchFilePathsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.search_params.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.SearchFilePathsRequest", len)?;
        if let Some(v) = self.search_params.as_ref() {
            struct_ser.serialize_field("searchParams", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchFilePathsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "search_params",
            "searchParams",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SearchParams,
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
                            "searchParams" | "search_params" => Ok(GeneratedField::SearchParams),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchFilePathsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.SearchFilePathsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SearchFilePathsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut search_params__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SearchParams => {
                            if search_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchParams"));
                            }
                            search_params__ = map.next_value()?;
                        }
                    }
                }
                Ok(SearchFilePathsRequest {
                    search_params: search_params__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.SearchFilePathsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchFilePathsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.search_result.is_some() {
            len += 1;
        }
        if self.hit_count != 0 {
            len += 1;
        }
        if self.total_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.SearchFilePathsResponse", len)?;
        if let Some(v) = self.search_result.as_ref() {
            struct_ser.serialize_field("searchResult", v)?;
        }
        if self.hit_count != 0 {
            struct_ser.serialize_field("hitCount", ToString::to_string(&self.hit_count).as_str())?;
        }
        if self.total_count != 0 {
            struct_ser.serialize_field("totalCount", ToString::to_string(&self.total_count).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchFilePathsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "search_result",
            "searchResult",
            "hit_count",
            "hitCount",
            "total_count",
            "totalCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SearchResult,
            HitCount,
            TotalCount,
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
                            "searchResult" | "search_result" => Ok(GeneratedField::SearchResult),
                            "hitCount" | "hit_count" => Ok(GeneratedField::HitCount),
                            "totalCount" | "total_count" => Ok(GeneratedField::TotalCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchFilePathsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.SearchFilePathsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SearchFilePathsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut search_result__ = None;
                let mut hit_count__ = None;
                let mut total_count__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SearchResult => {
                            if search_result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchResult"));
                            }
                            search_result__ = map.next_value()?;
                        }
                        GeneratedField::HitCount => {
                            if hit_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hitCount"));
                            }
                            hit_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalCount => {
                            if total_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalCount"));
                            }
                            total_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SearchFilePathsResponse {
                    search_result: search_result__,
                    hit_count: hit_count__.unwrap_or_default(),
                    total_count: total_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.SearchFilePathsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.search_string.is_empty() {
            len += 1;
        }
        if !self.file_path_pattern.is_empty() {
            len += 1;
        }
        if self.max_results != 0 {
            len += 1;
        }
        if self.match_case {
            len += 1;
        }
        if self.match_whole_word {
            len += 1;
        }
        if self.include_sym_links {
            len += 1;
        }
        if self.regex {
            len += 1;
        }
        if self.use_re2_engine {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.SearchParams", len)?;
        if !self.search_string.is_empty() {
            struct_ser.serialize_field("searchString", &self.search_string)?;
        }
        if !self.file_path_pattern.is_empty() {
            struct_ser.serialize_field("filePathPattern", &self.file_path_pattern)?;
        }
        if self.max_results != 0 {
            struct_ser.serialize_field("maxResults", &self.max_results)?;
        }
        if self.match_case {
            struct_ser.serialize_field("matchCase", &self.match_case)?;
        }
        if self.match_whole_word {
            struct_ser.serialize_field("matchWholeWord", &self.match_whole_word)?;
        }
        if self.include_sym_links {
            struct_ser.serialize_field("includeSymLinks", &self.include_sym_links)?;
        }
        if self.regex {
            struct_ser.serialize_field("regex", &self.regex)?;
        }
        if self.use_re2_engine {
            struct_ser.serialize_field("useRe2Engine", &self.use_re2_engine)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "search_string",
            "searchString",
            "file_path_pattern",
            "filePathPattern",
            "max_results",
            "maxResults",
            "match_case",
            "matchCase",
            "match_whole_word",
            "matchWholeWord",
            "include_sym_links",
            "includeSymLinks",
            "regex",
            "use_re2_engine",
            "useRe2Engine",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SearchString,
            FilePathPattern,
            MaxResults,
            MatchCase,
            MatchWholeWord,
            IncludeSymLinks,
            Regex,
            UseRe2Engine,
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
                            "searchString" | "search_string" => Ok(GeneratedField::SearchString),
                            "filePathPattern" | "file_path_pattern" => Ok(GeneratedField::FilePathPattern),
                            "maxResults" | "max_results" => Ok(GeneratedField::MaxResults),
                            "matchCase" | "match_case" => Ok(GeneratedField::MatchCase),
                            "matchWholeWord" | "match_whole_word" => Ok(GeneratedField::MatchWholeWord),
                            "includeSymLinks" | "include_sym_links" => Ok(GeneratedField::IncludeSymLinks),
                            "regex" => Ok(GeneratedField::Regex),
                            "useRe2Engine" | "use_re2_engine" => Ok(GeneratedField::UseRe2Engine),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.SearchParams")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SearchParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut search_string__ = None;
                let mut file_path_pattern__ = None;
                let mut max_results__ = None;
                let mut match_case__ = None;
                let mut match_whole_word__ = None;
                let mut include_sym_links__ = None;
                let mut regex__ = None;
                let mut use_re2_engine__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SearchString => {
                            if search_string__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchString"));
                            }
                            search_string__ = Some(map.next_value()?);
                        }
                        GeneratedField::FilePathPattern => {
                            if file_path_pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filePathPattern"));
                            }
                            file_path_pattern__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxResults => {
                            if max_results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxResults"));
                            }
                            max_results__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MatchCase => {
                            if match_case__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchCase"));
                            }
                            match_case__ = Some(map.next_value()?);
                        }
                        GeneratedField::MatchWholeWord => {
                            if match_whole_word__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchWholeWord"));
                            }
                            match_whole_word__ = Some(map.next_value()?);
                        }
                        GeneratedField::IncludeSymLinks => {
                            if include_sym_links__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeSymLinks"));
                            }
                            include_sym_links__ = Some(map.next_value()?);
                        }
                        GeneratedField::Regex => {
                            if regex__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regex"));
                            }
                            regex__ = Some(map.next_value()?);
                        }
                        GeneratedField::UseRe2Engine => {
                            if use_re2_engine__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useRe2Engine"));
                            }
                            use_re2_engine__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SearchParams {
                    search_string: search_string__.unwrap_or_default(),
                    file_path_pattern: file_path_pattern__.unwrap_or_default(),
                    max_results: max_results__.unwrap_or_default(),
                    match_case: match_case__.unwrap_or_default(),
                    match_whole_word: match_whole_word__.unwrap_or_default(),
                    include_sym_links: include_sym_links__.unwrap_or_default(),
                    regex: regex__.unwrap_or_default(),
                    use_re2_engine: use_re2_engine__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.SearchParams", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TypedEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subtype.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.TypedEvent", len)?;
        if let Some(v) = self.subtype.as_ref() {
            match v {
                typed_event::Subtype::PairedTypedEvent(v) => {
                    struct_ser.serialize_field("pairedTypedEvent", v)?;
                }
                typed_event::Subtype::ProgressReportEvent(v) => {
                    struct_ser.serialize_field("progressReportEvent", v)?;
                }
                typed_event::Subtype::IndexingServerStateChangedEvent(v) => {
                    struct_ser.serialize_field("indexingServerStateChangedEvent", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TypedEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "paired_typed_event",
            "pairedTypedEvent",
            "progress_report_event",
            "progressReportEvent",
            "indexing_server_state_changed_event",
            "indexingServerStateChangedEvent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PairedTypedEvent,
            ProgressReportEvent,
            IndexingServerStateChangedEvent,
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
                            "pairedTypedEvent" | "paired_typed_event" => Ok(GeneratedField::PairedTypedEvent),
                            "progressReportEvent" | "progress_report_event" => Ok(GeneratedField::ProgressReportEvent),
                            "indexingServerStateChangedEvent" | "indexing_server_state_changed_event" => Ok(GeneratedField::IndexingServerStateChangedEvent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TypedEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.TypedEvent")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TypedEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subtype__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PairedTypedEvent => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pairedTypedEvent"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_event::Subtype::PairedTypedEvent)
;
                        }
                        GeneratedField::ProgressReportEvent => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("progressReportEvent"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_event::Subtype::ProgressReportEvent)
;
                        }
                        GeneratedField::IndexingServerStateChangedEvent => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("indexingServerStateChangedEvent"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_event::Subtype::IndexingServerStateChangedEvent)
;
                        }
                    }
                }
                Ok(TypedEvent {
                    subtype: subtype__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.TypedEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TypedMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.class_name.is_empty() {
            len += 1;
        }
        if self.subtype.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.TypedMessage", len)?;
        if !self.class_name.is_empty() {
            struct_ser.serialize_field("className", &self.class_name)?;
        }
        if let Some(v) = self.subtype.as_ref() {
            match v {
                typed_message::Subtype::TypedRequest(v) => {
                    struct_ser.serialize_field("typedRequest", v)?;
                }
                typed_message::Subtype::TypedResponse(v) => {
                    struct_ser.serialize_field("typedResponse", v)?;
                }
                typed_message::Subtype::TypedEvent(v) => {
                    struct_ser.serialize_field("typedEvent", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TypedMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "class_name",
            "className",
            "typed_request",
            "typedRequest",
            "typed_response",
            "typedResponse",
            "typed_event",
            "typedEvent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClassName,
            TypedRequest,
            TypedResponse,
            TypedEvent,
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
                            "className" | "class_name" => Ok(GeneratedField::ClassName),
                            "typedRequest" | "typed_request" => Ok(GeneratedField::TypedRequest),
                            "typedResponse" | "typed_response" => Ok(GeneratedField::TypedResponse),
                            "typedEvent" | "typed_event" => Ok(GeneratedField::TypedEvent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TypedMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.TypedMessage")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TypedMessage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_name__ = None;
                let mut subtype__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClassName => {
                            if class_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("className"));
                            }
                            class_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::TypedRequest => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedRequest"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_message::Subtype::TypedRequest)
;
                        }
                        GeneratedField::TypedResponse => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedResponse"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_message::Subtype::TypedResponse)
;
                        }
                        GeneratedField::TypedEvent => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedEvent"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_message::Subtype::TypedEvent)
;
                        }
                    }
                }
                Ok(TypedMessage {
                    class_name: class_name__.unwrap_or_default(),
                    subtype: subtype__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.TypedMessage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TypedRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subtype.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.TypedRequest", len)?;
        if let Some(v) = self.subtype.as_ref() {
            match v {
                typed_request::Subtype::RegisterFileRequest(v) => {
                    struct_ser.serialize_field("registerFileRequest", v)?;
                }
                typed_request::Subtype::UnregisterFileRequest(v) => {
                    struct_ser.serialize_field("unregisterFileRequest", v)?;
                }
                typed_request::Subtype::GetFileSystemRequest(v) => {
                    struct_ser.serialize_field("getFileSystemRequest", v)?;
                }
                typed_request::Subtype::SearchFilePathsRequest(v) => {
                    struct_ser.serialize_field("searchFilePathsRequest", v)?;
                }
                typed_request::Subtype::SearchCodeRequest(v) => {
                    struct_ser.serialize_field("searchCodeRequest", v)?;
                }
                typed_request::Subtype::GetFileSystemVersionRequest(v) => {
                    struct_ser.serialize_field("getFileSystemVersionRequest", v)?;
                }
                typed_request::Subtype::GetFileExtractsRequest(v) => {
                    struct_ser.serialize_field("getFileExtractsRequest", v)?;
                }
                typed_request::Subtype::GetDirectoryStatisticsRequest(v) => {
                    struct_ser.serialize_field("getDirectoryStatisticsRequest", v)?;
                }
                typed_request::Subtype::RefreshFilesystemTreeRequest(v) => {
                    struct_ser.serialize_field("refreshFilesystemTreeRequest", v)?;
                }
                typed_request::Subtype::GetDatabaseStatisticsRequest(v) => {
                    struct_ser.serialize_field("getDatabaseStatisticsRequest", v)?;
                }
                typed_request::Subtype::PauseIndexingRequest(v) => {
                    struct_ser.serialize_field("pauseIndexingRequest", v)?;
                }
                typed_request::Subtype::ResumeIndexingRequest(v) => {
                    struct_ser.serialize_field("resumeIndexingRequest", v)?;
                }
                typed_request::Subtype::GetDatabaseDetailsRequest(v) => {
                    struct_ser.serialize_field("getDatabaseDetailsRequest", v)?;
                }
                typed_request::Subtype::GetProjectDetailsRequest(v) => {
                    struct_ser.serialize_field("getProjectDetailsRequest", v)?;
                }
                typed_request::Subtype::GetDirectoryDetailsRequest(v) => {
                    struct_ser.serialize_field("getDirectoryDetailsRequest", v)?;
                }
                typed_request::Subtype::GetFilesystemTreeRequest(v) => {
                    struct_ser.serialize_field("getFilesystemTreeRequest", v)?;
                }
                typed_request::Subtype::GetDirectoryEntriesRequest(v) => {
                    struct_ser.serialize_field("getDirectoryEntriesRequest", v)?;
                }
                typed_request::Subtype::GetDirectoryEntriesMultipleRequest(v) => {
                    struct_ser.serialize_field("getDirectoryEntriesMultipleRequest", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TypedRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "register_file_request",
            "registerFileRequest",
            "unregister_file_request",
            "unregisterFileRequest",
            "get_file_system_request",
            "getFileSystemRequest",
            "search_file_paths_request",
            "searchFilePathsRequest",
            "search_code_request",
            "searchCodeRequest",
            "get_file_system_version_request",
            "getFileSystemVersionRequest",
            "get_file_extracts_request",
            "getFileExtractsRequest",
            "get_directory_statistics_request",
            "getDirectoryStatisticsRequest",
            "refresh_filesystem_tree_request",
            "refreshFilesystemTreeRequest",
            "get_database_statistics_request",
            "getDatabaseStatisticsRequest",
            "pause_indexing_request",
            "pauseIndexingRequest",
            "resume_indexing_request",
            "resumeIndexingRequest",
            "get_database_details_request",
            "getDatabaseDetailsRequest",
            "get_project_details_request",
            "getProjectDetailsRequest",
            "get_directory_details_request",
            "getDirectoryDetailsRequest",
            "get_filesystem_tree_request",
            "getFilesystemTreeRequest",
            "get_directory_entries_request",
            "getDirectoryEntriesRequest",
            "get_directory_entries_multiple_request",
            "getDirectoryEntriesMultipleRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RegisterFileRequest,
            UnregisterFileRequest,
            GetFileSystemRequest,
            SearchFilePathsRequest,
            SearchCodeRequest,
            GetFileSystemVersionRequest,
            GetFileExtractsRequest,
            GetDirectoryStatisticsRequest,
            RefreshFilesystemTreeRequest,
            GetDatabaseStatisticsRequest,
            PauseIndexingRequest,
            ResumeIndexingRequest,
            GetDatabaseDetailsRequest,
            GetProjectDetailsRequest,
            GetDirectoryDetailsRequest,
            GetFilesystemTreeRequest,
            GetDirectoryEntriesRequest,
            GetDirectoryEntriesMultipleRequest,
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
                            "registerFileRequest" | "register_file_request" => Ok(GeneratedField::RegisterFileRequest),
                            "unregisterFileRequest" | "unregister_file_request" => Ok(GeneratedField::UnregisterFileRequest),
                            "getFileSystemRequest" | "get_file_system_request" => Ok(GeneratedField::GetFileSystemRequest),
                            "searchFilePathsRequest" | "search_file_paths_request" => Ok(GeneratedField::SearchFilePathsRequest),
                            "searchCodeRequest" | "search_code_request" => Ok(GeneratedField::SearchCodeRequest),
                            "getFileSystemVersionRequest" | "get_file_system_version_request" => Ok(GeneratedField::GetFileSystemVersionRequest),
                            "getFileExtractsRequest" | "get_file_extracts_request" => Ok(GeneratedField::GetFileExtractsRequest),
                            "getDirectoryStatisticsRequest" | "get_directory_statistics_request" => Ok(GeneratedField::GetDirectoryStatisticsRequest),
                            "refreshFilesystemTreeRequest" | "refresh_filesystem_tree_request" => Ok(GeneratedField::RefreshFilesystemTreeRequest),
                            "getDatabaseStatisticsRequest" | "get_database_statistics_request" => Ok(GeneratedField::GetDatabaseStatisticsRequest),
                            "pauseIndexingRequest" | "pause_indexing_request" => Ok(GeneratedField::PauseIndexingRequest),
                            "resumeIndexingRequest" | "resume_indexing_request" => Ok(GeneratedField::ResumeIndexingRequest),
                            "getDatabaseDetailsRequest" | "get_database_details_request" => Ok(GeneratedField::GetDatabaseDetailsRequest),
                            "getProjectDetailsRequest" | "get_project_details_request" => Ok(GeneratedField::GetProjectDetailsRequest),
                            "getDirectoryDetailsRequest" | "get_directory_details_request" => Ok(GeneratedField::GetDirectoryDetailsRequest),
                            "getFilesystemTreeRequest" | "get_filesystem_tree_request" => Ok(GeneratedField::GetFilesystemTreeRequest),
                            "getDirectoryEntriesRequest" | "get_directory_entries_request" => Ok(GeneratedField::GetDirectoryEntriesRequest),
                            "getDirectoryEntriesMultipleRequest" | "get_directory_entries_multiple_request" => Ok(GeneratedField::GetDirectoryEntriesMultipleRequest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TypedRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.TypedRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TypedRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subtype__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RegisterFileRequest => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registerFileRequest"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_request::Subtype::RegisterFileRequest)
;
                        }
                        GeneratedField::UnregisterFileRequest => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unregisterFileRequest"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_request::Subtype::UnregisterFileRequest)
;
                        }
                        GeneratedField::GetFileSystemRequest => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getFileSystemRequest"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_request::Subtype::GetFileSystemRequest)
;
                        }
                        GeneratedField::SearchFilePathsRequest => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchFilePathsRequest"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_request::Subtype::SearchFilePathsRequest)
;
                        }
                        GeneratedField::SearchCodeRequest => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchCodeRequest"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_request::Subtype::SearchCodeRequest)
;
                        }
                        GeneratedField::GetFileSystemVersionRequest => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getFileSystemVersionRequest"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_request::Subtype::GetFileSystemVersionRequest)
;
                        }
                        GeneratedField::GetFileExtractsRequest => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getFileExtractsRequest"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_request::Subtype::GetFileExtractsRequest)
;
                        }
                        GeneratedField::GetDirectoryStatisticsRequest => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getDirectoryStatisticsRequest"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_request::Subtype::GetDirectoryStatisticsRequest)
;
                        }
                        GeneratedField::RefreshFilesystemTreeRequest => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshFilesystemTreeRequest"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_request::Subtype::RefreshFilesystemTreeRequest)
;
                        }
                        GeneratedField::GetDatabaseStatisticsRequest => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getDatabaseStatisticsRequest"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_request::Subtype::GetDatabaseStatisticsRequest)
;
                        }
                        GeneratedField::PauseIndexingRequest => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pauseIndexingRequest"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_request::Subtype::PauseIndexingRequest)
;
                        }
                        GeneratedField::ResumeIndexingRequest => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resumeIndexingRequest"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_request::Subtype::ResumeIndexingRequest)
;
                        }
                        GeneratedField::GetDatabaseDetailsRequest => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getDatabaseDetailsRequest"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_request::Subtype::GetDatabaseDetailsRequest)
;
                        }
                        GeneratedField::GetProjectDetailsRequest => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getProjectDetailsRequest"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_request::Subtype::GetProjectDetailsRequest)
;
                        }
                        GeneratedField::GetDirectoryDetailsRequest => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getDirectoryDetailsRequest"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_request::Subtype::GetDirectoryDetailsRequest)
;
                        }
                        GeneratedField::GetFilesystemTreeRequest => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getFilesystemTreeRequest"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_request::Subtype::GetFilesystemTreeRequest)
;
                        }
                        GeneratedField::GetDirectoryEntriesRequest => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getDirectoryEntriesRequest"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_request::Subtype::GetDirectoryEntriesRequest)
;
                        }
                        GeneratedField::GetDirectoryEntriesMultipleRequest => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getDirectoryEntriesMultipleRequest"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_request::Subtype::GetDirectoryEntriesMultipleRequest)
;
                        }
                    }
                }
                Ok(TypedRequest {
                    subtype: subtype__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.TypedRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TypedResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subtype.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.TypedResponse", len)?;
        if let Some(v) = self.subtype.as_ref() {
            match v {
                typed_response::Subtype::DoneResponse(v) => {
                    struct_ser.serialize_field("doneResponse", v)?;
                }
                typed_response::Subtype::GetFileSystemResponse(v) => {
                    struct_ser.serialize_field("getFileSystemResponse", v)?;
                }
                typed_response::Subtype::SearchFilePathsResponse(v) => {
                    struct_ser.serialize_field("searchFilePathsResponse", v)?;
                }
                typed_response::Subtype::SearchCodeResponse(v) => {
                    struct_ser.serialize_field("searchCodeResponse", v)?;
                }
                typed_response::Subtype::GetFileSystemVersionResponse(v) => {
                    struct_ser.serialize_field("getFileSystemVersionResponse", v)?;
                }
                typed_response::Subtype::GetFileExtractsResponse(v) => {
                    struct_ser.serialize_field("getFileExtractsResponse", v)?;
                }
                typed_response::Subtype::GetDirectoryStatisticsResponse(v) => {
                    struct_ser.serialize_field("getDirectoryStatisticsResponse", v)?;
                }
                typed_response::Subtype::RefreshFileSystemTreeResponse(v) => {
                    struct_ser.serialize_field("refreshFileSystemTreeResponse", v)?;
                }
                typed_response::Subtype::GetDatabaseStatisticsResponse(v) => {
                    struct_ser.serialize_field("getDatabaseStatisticsResponse", v)?;
                }
                typed_response::Subtype::PauseResumeIndexingResponse(v) => {
                    struct_ser.serialize_field("pauseResumeIndexingResponse", v)?;
                }
                typed_response::Subtype::GetDatabaseDetailsResponse(v) => {
                    struct_ser.serialize_field("getDatabaseDetailsResponse", v)?;
                }
                typed_response::Subtype::GetProjectDetailsResponse(v) => {
                    struct_ser.serialize_field("getProjectDetailsResponse", v)?;
                }
                typed_response::Subtype::GetDirectoryDetailsResponse(v) => {
                    struct_ser.serialize_field("getDirectoryDetailsResponse", v)?;
                }
                typed_response::Subtype::GetFilesystemTreeResponse(v) => {
                    struct_ser.serialize_field("getFilesystemTreeResponse", v)?;
                }
                typed_response::Subtype::GetDirectoryEntriesResponse(v) => {
                    struct_ser.serialize_field("getDirectoryEntriesResponse", v)?;
                }
                typed_response::Subtype::GetDirectoryEntriesMultipleResponse(v) => {
                    struct_ser.serialize_field("getDirectoryEntriesMultipleResponse", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TypedResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "done_response",
            "doneResponse",
            "get_file_system_response",
            "getFileSystemResponse",
            "search_file_paths_response",
            "searchFilePathsResponse",
            "search_code_response",
            "searchCodeResponse",
            "get_file_system_version_response",
            "getFileSystemVersionResponse",
            "get_file_extracts_response",
            "getFileExtractsResponse",
            "get_directory_statistics_response",
            "getDirectoryStatisticsResponse",
            "refresh_file_system_tree_response",
            "refreshFileSystemTreeResponse",
            "get_database_statistics_response",
            "getDatabaseStatisticsResponse",
            "pause_resume_indexing_response",
            "pauseResumeIndexingResponse",
            "get_database_details_response",
            "getDatabaseDetailsResponse",
            "get_project_details_response",
            "getProjectDetailsResponse",
            "get_directory_details_response",
            "getDirectoryDetailsResponse",
            "get_filesystem_tree_response",
            "getFilesystemTreeResponse",
            "get_directory_entries_response",
            "getDirectoryEntriesResponse",
            "get_directory_entries_multiple_response",
            "getDirectoryEntriesMultipleResponse",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DoneResponse,
            GetFileSystemResponse,
            SearchFilePathsResponse,
            SearchCodeResponse,
            GetFileSystemVersionResponse,
            GetFileExtractsResponse,
            GetDirectoryStatisticsResponse,
            RefreshFileSystemTreeResponse,
            GetDatabaseStatisticsResponse,
            PauseResumeIndexingResponse,
            GetDatabaseDetailsResponse,
            GetProjectDetailsResponse,
            GetDirectoryDetailsResponse,
            GetFilesystemTreeResponse,
            GetDirectoryEntriesResponse,
            GetDirectoryEntriesMultipleResponse,
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
                            "doneResponse" | "done_response" => Ok(GeneratedField::DoneResponse),
                            "getFileSystemResponse" | "get_file_system_response" => Ok(GeneratedField::GetFileSystemResponse),
                            "searchFilePathsResponse" | "search_file_paths_response" => Ok(GeneratedField::SearchFilePathsResponse),
                            "searchCodeResponse" | "search_code_response" => Ok(GeneratedField::SearchCodeResponse),
                            "getFileSystemVersionResponse" | "get_file_system_version_response" => Ok(GeneratedField::GetFileSystemVersionResponse),
                            "getFileExtractsResponse" | "get_file_extracts_response" => Ok(GeneratedField::GetFileExtractsResponse),
                            "getDirectoryStatisticsResponse" | "get_directory_statistics_response" => Ok(GeneratedField::GetDirectoryStatisticsResponse),
                            "refreshFileSystemTreeResponse" | "refresh_file_system_tree_response" => Ok(GeneratedField::RefreshFileSystemTreeResponse),
                            "getDatabaseStatisticsResponse" | "get_database_statistics_response" => Ok(GeneratedField::GetDatabaseStatisticsResponse),
                            "pauseResumeIndexingResponse" | "pause_resume_indexing_response" => Ok(GeneratedField::PauseResumeIndexingResponse),
                            "getDatabaseDetailsResponse" | "get_database_details_response" => Ok(GeneratedField::GetDatabaseDetailsResponse),
                            "getProjectDetailsResponse" | "get_project_details_response" => Ok(GeneratedField::GetProjectDetailsResponse),
                            "getDirectoryDetailsResponse" | "get_directory_details_response" => Ok(GeneratedField::GetDirectoryDetailsResponse),
                            "getFilesystemTreeResponse" | "get_filesystem_tree_response" => Ok(GeneratedField::GetFilesystemTreeResponse),
                            "getDirectoryEntriesResponse" | "get_directory_entries_response" => Ok(GeneratedField::GetDirectoryEntriesResponse),
                            "getDirectoryEntriesMultipleResponse" | "get_directory_entries_multiple_response" => Ok(GeneratedField::GetDirectoryEntriesMultipleResponse),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TypedResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.TypedResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TypedResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subtype__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DoneResponse => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("doneResponse"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_response::Subtype::DoneResponse)
;
                        }
                        GeneratedField::GetFileSystemResponse => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getFileSystemResponse"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_response::Subtype::GetFileSystemResponse)
;
                        }
                        GeneratedField::SearchFilePathsResponse => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchFilePathsResponse"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_response::Subtype::SearchFilePathsResponse)
;
                        }
                        GeneratedField::SearchCodeResponse => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchCodeResponse"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_response::Subtype::SearchCodeResponse)
;
                        }
                        GeneratedField::GetFileSystemVersionResponse => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getFileSystemVersionResponse"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_response::Subtype::GetFileSystemVersionResponse)
;
                        }
                        GeneratedField::GetFileExtractsResponse => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getFileExtractsResponse"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_response::Subtype::GetFileExtractsResponse)
;
                        }
                        GeneratedField::GetDirectoryStatisticsResponse => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getDirectoryStatisticsResponse"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_response::Subtype::GetDirectoryStatisticsResponse)
;
                        }
                        GeneratedField::RefreshFileSystemTreeResponse => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshFileSystemTreeResponse"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_response::Subtype::RefreshFileSystemTreeResponse)
;
                        }
                        GeneratedField::GetDatabaseStatisticsResponse => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getDatabaseStatisticsResponse"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_response::Subtype::GetDatabaseStatisticsResponse)
;
                        }
                        GeneratedField::PauseResumeIndexingResponse => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pauseResumeIndexingResponse"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_response::Subtype::PauseResumeIndexingResponse)
;
                        }
                        GeneratedField::GetDatabaseDetailsResponse => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getDatabaseDetailsResponse"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_response::Subtype::GetDatabaseDetailsResponse)
;
                        }
                        GeneratedField::GetProjectDetailsResponse => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getProjectDetailsResponse"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_response::Subtype::GetProjectDetailsResponse)
;
                        }
                        GeneratedField::GetDirectoryDetailsResponse => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getDirectoryDetailsResponse"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_response::Subtype::GetDirectoryDetailsResponse)
;
                        }
                        GeneratedField::GetFilesystemTreeResponse => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getFilesystemTreeResponse"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_response::Subtype::GetFilesystemTreeResponse)
;
                        }
                        GeneratedField::GetDirectoryEntriesResponse => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getDirectoryEntriesResponse"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_response::Subtype::GetDirectoryEntriesResponse)
;
                        }
                        GeneratedField::GetDirectoryEntriesMultipleResponse => {
                            if subtype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getDirectoryEntriesMultipleResponse"));
                            }
                            subtype__ = map.next_value::<::std::option::Option<_>>()?.map(typed_response::Subtype::GetDirectoryEntriesMultipleResponse)
;
                        }
                    }
                }
                Ok(TypedResponse {
                    subtype: subtype__,
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.TypedResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnregisterFileRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("searchium.v1.UnregisterFileRequest", len)?;
        if !self.file_name.is_empty() {
            struct_ser.serialize_field("fileName", &self.file_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnregisterFileRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_name",
            "fileName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FileName,
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
                            "fileName" | "file_name" => Ok(GeneratedField::FileName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnregisterFileRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct searchium.v1.UnregisterFileRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UnregisterFileRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FileName => {
                            if file_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileName"));
                            }
                            file_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UnregisterFileRequest {
                    file_name: file_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("searchium.v1.UnregisterFileRequest", FIELDS, GeneratedVisitor)
    }
}
