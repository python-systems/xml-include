pub mod error;
mod processor;

use crate::error::Result;
use crate::processor::get_event_stream;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;
use xml::EmitterConfig;

/// Merge includes found in an XML file and return the result as a string.
///
/// The includes are resolved recursively, only "xi:includes" elements are considered.
///
/// # Arguments
/// - `file_path` - the path to the XML file to resolve `xi:include` elements in
///
/// # Returns
/// The resolved XML content as a string.
pub fn resolve_xml_includes(file_path: &Path) -> Result<String> {
    let buffer = Vec::new();
    let cursor = Cursor::new(buffer);

    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(cursor);

    let output = get_event_stream(&File::open(file_path)?, Some(file_path))?;

    for elem in output {
        if let Some(event) = elem.as_writer_event() {
            writer.write(event)?;
        }
    }

    let resolved_content = String::from_utf8(writer.into_inner().into_inner())?;

    Ok(resolved_content)
}
