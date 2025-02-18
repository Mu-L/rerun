use crate::codegen::common::ExampleInfo;
use crate::objects::FieldKind;
use crate::CodeGenerator;
use crate::GeneratedFiles;
use crate::Object;
use crate::ObjectKind;
use crate::Objects;
use crate::Reporter;
use camino::Utf8PathBuf;
use std::fmt::Write;

type ObjectMap = std::collections::BTreeMap<String, Object>;

use super::common::get_documentation;

macro_rules! putln {
    ($o:ident) => ( writeln!($o).ok() );
    ($o:ident, $($tt:tt)*) => ( writeln!($o, $($tt)*).ok() );
}

pub struct DocsCodeGenerator {
    docs_dir: Utf8PathBuf,
}

impl DocsCodeGenerator {
    pub fn new(docs_dir: impl Into<Utf8PathBuf>) -> Self {
        Self {
            docs_dir: docs_dir.into(),
        }
    }
}

impl CodeGenerator for DocsCodeGenerator {
    fn generate(
        &mut self,
        reporter: &Reporter,
        objects: &Objects,
        _arrow_registry: &crate::ArrowRegistry,
    ) -> GeneratedFiles {
        re_tracing::profile_function!();

        let mut files_to_write = GeneratedFiles::default();

        let (mut archetypes, mut components, mut datatypes) = (Vec::new(), Vec::new(), Vec::new());
        let object_map = &objects.objects;
        for object in object_map.values() {
            // skip test-only archetypes
            if object.is_testing() {
                continue;
            }

            match object.kind {
                ObjectKind::Datatype => datatypes.push(object),
                ObjectKind::Component => components.push(object),
                ObjectKind::Archetype => archetypes.push(object),
                ObjectKind::Blueprint => continue, // skip blueprint stuff, too early
            }

            let page = object_page(reporter, object, object_map);
            let path = self.docs_dir.join(format!(
                "{}/{}.md",
                object.kind.plural_snake_case(),
                object.snake_case_name()
            ));
            files_to_write.insert(path, page);
        }

        for (kind, order, prelude, objects) in [
            (
                ObjectKind::Archetype,
                1,
                "Archetypes are bundles of components",
                &archetypes,
            ),
            (
                ObjectKind::Component,
                2,
                "Archetypes are bundles of components",
                &components,
            ),
            (
                ObjectKind::Datatype,
                3,
                "Data types are the lowest layer of the data model hierarchy",
                &datatypes,
            ),
        ] {
            let page = index_page(kind, order, prelude, objects);
            let path = self
                .docs_dir
                .join(format!("{}.md", kind.plural_snake_case()));
            files_to_write.insert(path, page);
        }

        files_to_write
    }
}

fn index_page(kind: ObjectKind, order: u64, prelude: &str, objects: &[&Object]) -> String {
    let mut page = String::new();

    write_frontmatter(&mut page, kind.plural_name(), Some(order));
    putln!(page);
    putln!(page, "{prelude}");
    putln!(page);
    if !objects.is_empty() {
        putln!(page, "## Available {}", kind.plural_name().to_lowercase());
        putln!(page);
        for object in objects {
            putln!(
                page,
                "* [`{}`]({}/{}.md)",
                object.name,
                object.kind.plural_snake_case(),
                object.snake_case_name()
            );
        }
    }

    page
}

fn object_page(reporter: &Reporter, object: &Object, object_map: &ObjectMap) -> String {
    let is_unreleased = object.is_attr_set(crate::ATTR_DOCS_UNRELEASED);

    let top_level_docs = get_documentation(&object.docs, &[]);

    if top_level_docs.is_empty() {
        reporter.error(&object.virtpath, &object.fqname, "Undocumented object");
    }

    let examples = object
        .docs
        .tagged_docs
        .get("example")
        .iter()
        .flat_map(|v| v.iter())
        .map(ExampleInfo::parse)
        .collect::<Vec<_>>();

    let mut page = String::new();

    write_frontmatter(&mut page, &object.name, None);
    putln!(page);
    for line in top_level_docs {
        putln!(page, "{line}");
    }
    putln!(page);

    match object.kind {
        ObjectKind::Datatype | ObjectKind::Component => {
            write_fields(&mut page, object, object_map);
        }
        ObjectKind::Archetype => write_archetype_fields(&mut page, object, object_map),
        ObjectKind::Blueprint => {}
    }

    {
        let speculative_marker = if is_unreleased {
            "?speculative-link"
        } else {
            ""
        };
        putln!(page);
        putln!(page, "## Links");
        // TODO(#2919): link to C++ docs
        putln!(
            page,
            " * 🐍 [Python API docs for `{}`](https://ref.rerun.io/docs/python/stable/common/{}{}#rerun.{}.{})",
            object.name,
            object.kind.plural_snake_case(),
            speculative_marker,
            object.kind.plural_snake_case(),
            object.name
        );
        putln!(
            page,
            " * 🦀 [Rust API docs for `{}`](https://docs.rs/rerun/latest/rerun/{}/{}.{}.html{})",
            object.name,
            object.kind.plural_snake_case(),
            if object.is_struct() { "struct" } else { "enum" },
            object.name,
            speculative_marker
        );
    }

    putln!(page);
    write_example_list(&mut page, &examples);

    match object.kind {
        ObjectKind::Datatype | ObjectKind::Component => {
            putln!(page);
            write_used_by(&mut page, reporter, object, object_map);
        }
        ObjectKind::Blueprint | ObjectKind::Archetype => {}
    }

    page
}

fn write_frontmatter(o: &mut String, title: &str, order: Option<u64>) {
    putln!(o, "---");
    putln!(o, "title: {title:?}");
    if let Some(order) = order {
        // The order is used to sort `rerun.io/docs` side navigation
        putln!(o, "order: {order}");
    }
    putln!(o, "---");
}

fn write_fields(o: &mut String, object: &Object, object_map: &ObjectMap) {
    if object.fields.is_empty() {
        return;
    }

    let mut fields = Vec::new();
    for field in &object.fields {
        let Some(fqname) = field.typ.fqname() else {
            continue;
        };
        let Some(ty) = object_map.get(fqname) else {
            continue;
        };
        fields.push(format!(
            "* {}: [`{}`](../{}/{}.md)",
            field.name,
            ty.name,
            ty.kind.plural_snake_case(),
            ty.snake_case_name()
        ));
    }

    if !fields.is_empty() {
        putln!(o, "## Fields");
        putln!(o);
        for field in fields {
            putln!(o, "{field}");
        }
    }
}

fn write_used_by(o: &mut String, reporter: &Reporter, object: &Object, object_map: &ObjectMap) {
    let mut used_by = Vec::new();
    for ty in object_map.values() {
        for field in &ty.fields {
            if field.typ.fqname() == Some(object.fqname.as_str()) {
                used_by.push(format!(
                    "* [`{}`](../{}/{}.md)",
                    ty.name,
                    ty.kind.plural_snake_case(),
                    ty.snake_case_name()
                ));
            }
        }
    }

    if used_by.is_empty() {
        // NOTE: there are some false positives here, because unions can only
        // reference other tables, but they are unwrapped in the codegen.
        // So for instance: `union Angle` uses `rerun.datatypes.Float32` in
        // `angle.fbs`, but in the generated code that datatype is unused.
        reporter.warn(&object.virtpath, &object.fqname, "Unused object");
    } else {
        putln!(o, "## Used by");
        putln!(o);
        for ty in used_by {
            putln!(o, "{ty}");
        }
    }
}

fn write_archetype_fields(o: &mut String, object: &Object, object_map: &ObjectMap) {
    if object.fields.is_empty() {
        return;
    }

    // collect names of field _components_ by their `FieldKind`
    let (mut required, mut recommended, mut optional) = (Vec::new(), Vec::new(), Vec::new());
    for field in &object.fields {
        let Some(fqname) = field.typ.fqname() else {
            continue;
        };
        let Some(ty) = object_map.get(fqname) else {
            continue;
        };
        let target = match field.kind() {
            Some(FieldKind::Required) => &mut required,
            Some(FieldKind::Recommended) => &mut recommended,
            Some(FieldKind::Optional) => &mut optional,
            _ => continue,
        };
        target.push(format!(
            "[`{}`](../{}/{}.md)",
            ty.name,
            ty.kind.plural_snake_case(),
            ty.snake_case_name()
        ));
    }

    if required.is_empty() && recommended.is_empty() && optional.is_empty() {
        return;
    }

    putln!(o, "## Components");
    if !required.is_empty() {
        putln!(o);
        putln!(o, "**Required**: {}", required.join(", "));
    }
    if !recommended.is_empty() {
        putln!(o);
        putln!(o, "**Recommended**: {}", recommended.join(", "));
    }
    if !optional.is_empty() {
        putln!(o);
        putln!(o, "**Optional**: {}", optional.join(", "));
    }
}

fn write_example_list(o: &mut String, examples: &[ExampleInfo<'_>]) {
    if examples.is_empty() {
        return;
    }

    if examples.len() > 1 {
        putln!(o, "## Examples");
    } else {
        putln!(o, "## Example");
    };
    putln!(o);

    for ExampleInfo {
        name,
        title,
        image,
        exclude_from_api_docs: _,
    } in examples
    {
        let title = title.unwrap_or(name);
        putln!(o, "### {title}");
        putln!(o);
        putln!(o, "code-example: {name}");
        if let Some(image_url) = image {
            putln!(o);
            for line in image_url.image_stack() {
                putln!(o, "{line}");
            }
        }
        putln!(o);
    }
}
