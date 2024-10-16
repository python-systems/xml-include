use crate::error::{IncludeError, Result};

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use xml::reader::XmlEvent;
use xml::EventReader;

/// Resolve (string) href to file path. If the href is relative, a reference path is
/// required in order to properly resolve it.
///
/// The reference path may be path to a file or a directory.
///
/// # Arguments
/// - `href` - the href to resolve
/// - `reference_dir` - the reference path to resolve relative hrefs
///
/// # Returns
/// The resolved file path.
fn get_file_path_to_read(href: &str, reference_dir: Option<&Path>) -> Result<PathBuf> {
    let mut href = PathBuf::from(href);
    if href.is_relative() {
        if reference_dir.is_none() {
            return Err(IncludeError::XmlReferenceError(
                "xi:include with a relative path without a specified reference dir",
            ));
        }

        let mut file_path = reference_dir.unwrap().to_path_buf();
        if file_path.is_file() {
            file_path.pop();
        }

        file_path.push(href);
        href = file_path;
    }

    let href = href.canonicalize()?;

    Ok(href)
}

/// Read XML from a file and return a vector of XML events.
/// Resolve any `xi:include` refs encountered in the file (recursively).
/// If the XML contains any relative `xi:include` refs, a reference path is required in order to properly resolve them.
/// The reference path can be path to the file being read from.
///
/// # Arguments
/// - `file` - the file to read from
/// - `reference_path` - the reference path to resolve relative `xi:include` refs
///
/// # Returns
/// A fully-resolved vector of XML events.
pub(crate) fn get_event_stream(
    file: &File,
    reference_path: Option<&Path>,
) -> Result<Vec<XmlEvent>> {
    let file = BufReader::new(file);
    let parser = EventReader::new(file);

    let resolved_event_stream = parser
        .into_iter()
        .map(|event| -> Result<Vec<XmlEvent>> {
            let events = match event? {
                XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } if name.local_name == "include"
                    && name.prefix_ref().is_some_and(|x| x == "xi") =>
                {
                    let href = attributes
                        .iter()
                        .filter(|x| x.name.local_name == "href")
                        .map(|attr| attr.borrow().value)
                        .next()
                        .ok_or(IncludeError::XmlReferenceError(
                            "xi:include without href attribute",
                        ))?;

                    let href = get_file_path_to_read(href, reference_path)?;

                    get_event_stream(&File::open(href)?, reference_path)?
                }

                XmlEvent::EndElement { ref name, .. }
                    if name.local_name == "include"
                        && name.prefix_ref().is_some_and(|x| x == "xi") =>
                {
                    vec![]
                }
                XmlEvent::StartDocument { .. } | XmlEvent::EndDocument => vec![],
                event => vec![event],
            };

            Ok(events)
        })
        .collect::<Result<Vec<Vec<XmlEvent>>>>()?
        .into_iter()
        .flatten()
        .collect();

    Ok(resolved_event_stream)
}
