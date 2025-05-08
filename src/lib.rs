use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

// See https://github.com/vk-cs/docs-public/blob/master/guides/how-it-works.md.

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Meta {
    title: String,
    meta_title: String,
    section_title: String,
    short_description: String,
    page_description: String,
    meta_description: String,
    weight: i32,
    uuid: String,
}

impl Meta {
    fn apply(self, page: Page) -> (Self, bool) {
        let mut updated = false;

        let title = page
            .title
            .inspect(|title| updated |= self.title.ne(title))
            .unwrap_or(self.title);
        let meta_title = page
            .meta_title
            .inspect(|meta_title| updated |= self.meta_title.ne(meta_title))
            .unwrap_or(self.meta_title);
        let section_title = page
            .section_title
            .inspect(|section_title| updated |= self.section_title.ne(section_title))
            .unwrap_or(self.section_title);
        let short_description = page
            .short_description
            .inspect(|short_description| updated |= self.short_description.ne(short_description))
            .unwrap_or(self.short_description);
        let page_description = page
            .page_description
            .inspect(|page_description| updated |= self.page_description.ne(page_description))
            .unwrap_or(self.page_description);
        let meta_description = page
            .meta_description
            .inspect(|meta_description| updated |= self.meta_description.ne(meta_description))
            .unwrap_or(self.meta_description);
        let weight = page
            .weight
            .inspect(|weight| updated |= self.weight.ne(weight))
            .unwrap_or(self.weight);

        let uuid = self.uuid;

        let meta = Meta {
            title,
            meta_title,
            section_title,
            short_description,
            page_description,
            meta_description,
            weight,
            uuid,
        };

        (meta, updated)
    }
}

#[derive(Default)]
pub struct Vkdoc {
    path: PathBuf,
}

pub struct Page {
    title: Option<String>,
    meta_title: Option<String>,
    meta_description: Option<String>,
    short_description: Option<String>,
    page_description: Option<String>,
    section_title: Option<String>,
    content: Option<String>,
    weight: Option<i32>,
}

const DEFAULT_WEIGHT: i32 = 1;

impl Page {
    pub fn new() -> Page {
        Page {
            title: None,
            meta_title: None,
            section_title: None,
            meta_description: None,
            short_description: None,
            page_description: None,
            content: None,
            weight: None,
        }
    }

    pub fn with_title(self, title: String) -> Page {
        Page {
            title: Some(title),
            ..self
        }
    }

    pub fn with_meta_title(self, meta_title: String) -> Page {
        Page {
            meta_title: Some(meta_title),
            ..self
        }
    }

    pub fn with_section_title(self, section_title: String) -> Page {
        Page {
            section_title: Some(section_title),
            ..self
        }
    }

    pub fn with_short_description(self, short_description: String) -> Page {
        Page {
            short_description: Some(short_description),
            ..self
        }
    }

    pub fn with_page_description(self, page_description: String) -> Page {
        Page {
            page_description: Some(page_description),
            ..self
        }
    }

    pub fn with_content(self, content: String) -> Page {
        Page {
            content: Some(content),
            ..self
        }
    }

    pub fn with_weight(self, weight: i32) -> Page {
        Page {
            weight: Some(weight),
            ..self
        }
    }
}

impl TryInto<Meta> for Page {
    type Error = String;

    fn try_into(self) -> Result<Meta, Self::Error> {
        let title = self
            .title
            .ok_or("A title is required for a newly created page")?;
        let meta_title = self.meta_title.unwrap_or(title.clone());
        let section_title = self.section_title.unwrap_or(title.clone());

        let short_description = self.short_description.unwrap_or("".to_string());
        let page_description = self.page_description.unwrap_or(short_description.clone());
        let meta_description = self.meta_description.unwrap_or(short_description.clone());

        let weight = self.weight.unwrap_or(DEFAULT_WEIGHT);
        let uuid = uuid::Uuid::new_v4().to_string();

        Ok(Meta {
            title,
            meta_title,
            section_title,
            short_description,
            page_description,
            meta_description,
            weight,
            uuid,
        })
    }
}

impl Vkdoc {
    pub fn new(path: &Path) -> Result<Vkdoc, String> {
        fs::create_dir_all(path).map_err(|err| {
            format!(
                "Unable to initialize a vkdoc project at the specified dir {}: {}",
                path.display(),
                err
            )
        })?;

        Ok(Vkdoc {
            path: path.to_path_buf(),
        })
    }

    fn upsert_sections(&self, path: Option<&Path>) -> Result<(), String> {
        match path {
            Some(path) => match path.file_name() {
                Some(name) => {
                    let name = name
                        .to_os_string()
                        .into_string()
                        .map_err(|_| "Unable to convert filename to string".to_string())?;
                    let meta_path = self.path.join(path).join(format!("{}.meta.json", name));
                    if !meta_path.exists() {
                        let meta: Meta = Page::new().with_title(name).try_into()?;
                        let meta_content = serde_json::to_string_pretty(&meta).map_err(|err| {
                            format!("Unable to serialize entry meta json: {}", err)
                        })?;
                        fs::write(meta_path, meta_content).map_err(|err| {
                            format!("Unable to write a vkdoc meta at the specified dir: {}", err)
                        })?;
                    }
                    self.upsert_sections(path.parent())
                }
                None => Ok(()),
            },
            None => Ok(()),
        }
    }

    pub fn upsert(&self, path: &Path, page: Page) -> Result<bool, String> {
        if !path.is_relative() {
            return Err("Only relative paths are supported".to_string());
        }

        let full_path = self.path.join(path);
        fs::create_dir_all(&full_path)
            .map_err(|err| format!("Unable to create an entry at the specified dir: {}", err))?;

        self.upsert_sections(path.parent())?;

        let name = full_path
            .file_name()
            .ok_or("Unable to load an entry at the specified dir")?
            .to_os_string()
            .into_string()
            .map_err(|_| "Unable to load an entry at the specified dir")?;
        let doc_path = full_path.join(format!("{}.md", name));

        let content_updated = if doc_path.exists() {
            let old_content = fs::read_to_string(&doc_path).map_err(|err| {
                format!(
                    "Unable to read entry content at {}: {}",
                    doc_path.display(),
                    err
                )
            })?;
            page.content.as_ref().is_none_or(|content| {
                let old_checksum = md5::compute(&old_content);
                let checksum = md5::compute(&content);

                old_checksum != checksum
            })
        } else {
            page.content.is_some()
        };

        if content_updated {
            page.content.as_ref().map_or_else(
                || Ok::<(), String>(()),
                |content| {
                    fs::write(doc_path, content).map_err(|err| {
                        format!(
                            "Unable to write a vkdoc entry at the specified dir: {}",
                            err
                        )
                    })?;
                    Ok(())
                },
            )?;
        }

        let meta_path = full_path.join(format!("{}.meta.json", name));
        let (meta, meta_updated) = if meta_path.exists() {
            let meta_content = &fs::read_to_string(&meta_path).map_err(|err| {
                format!(
                    "Unable to read entry meta at {}: {}",
                    full_path.display(),
                    err
                )
            })?;
            let meta: Meta = serde_json::from_str(meta_content).map_err(|err| {
                format!(
                    "Unable to parse entry meta json at {}: {}",
                    full_path.display(),
                    err
                )
            })?;
            let (meta, updated) = meta.apply(page);
            (meta, updated)
        } else {
            (page.try_into()?, true)
        };

        let updated = meta_updated || content_updated;

        if updated {
            let meta_content = serde_json::to_string(&meta)
                .map_err(|err| format!("Unable to serialize entry meta json: {}", err))?;
            fs::write(meta_path, meta_content).map_err(|err| {
                format!("Unable to write a vkdoc meta at the specified dir: {}", err)
            })?;
        }

        Ok(updated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn basic_upsert_content() {
        let tempdir = TempDir::new("vkdoc").unwrap();
        let root_path = tempdir.path();
        let entry_path = Path::new("test");
        let full_entry_path = root_path.join(entry_path);
        let content = "content".to_string();
        let page = Page::new()
            .with_title("test".to_string())
            .with_content(content.clone());

        let vkdoc = Vkdoc::new(root_path).unwrap();
        assert!(root_path.exists() && root_path.is_dir());

        assert_eq!(vkdoc.upsert(&entry_path, page).unwrap(), true);

        let content_path = full_entry_path.join("test.md");
        let real_content = fs::read_to_string(&content_path).unwrap();
        assert_eq!(real_content, content);

        let meta_path = full_entry_path.join("test.meta.json");
        let real_meta: Meta =
            serde_json::from_str(fs::read_to_string(&meta_path).unwrap().as_str()).unwrap();
        assert_eq!(real_meta.title, "test");
        assert_eq!(real_meta.meta_title, "test");
        assert_eq!(real_meta.section_title, "test");
        assert_eq!(real_meta.short_description, "");
        assert_eq!(real_meta.page_description, "");

        assert_eq!(
            vkdoc
                .upsert(&entry_path, Page::new().with_content(content.clone()))
                .unwrap(),
            false
        );
        let real_content = fs::read_to_string(&content_path).unwrap();
        assert_eq!(real_content, content);

        let content = "new content".to_string();
        assert_eq!(
            vkdoc
                .upsert(&entry_path, Page::new().with_content(content.clone()))
                .unwrap(),
            true
        );

        let real_content = fs::read_to_string(&content_path).unwrap();
        assert_eq!(real_content, content);
    }

    #[test]
    fn basic_upsert_meta() {
        let tempdir = TempDir::new("vkdoc").unwrap();
        let root_path = tempdir.path();
        let entry_path = Path::new("test");
        let full_entry_path = root_path.join(entry_path);
        let page = Page::new().with_title("test".to_string());

        let vkdoc = Vkdoc::new(&root_path).unwrap();

        assert_eq!(vkdoc.upsert(&entry_path, page).unwrap(), true);

        let meta_path = full_entry_path.join("test.meta.json");
        let real_meta: Meta =
            serde_json::from_str(fs::read_to_string(&meta_path).unwrap().as_str()).unwrap();
        assert_eq!(real_meta.title, "test");

        let page = Page::new().with_title("test".to_string());
        assert_eq!(vkdoc.upsert(&entry_path, page).unwrap(), false);

        let page = Page::new().with_title("test1".to_string());
        assert_eq!(vkdoc.upsert(&entry_path, page).unwrap(), true);

        let real_meta: Meta =
            serde_json::from_str(fs::read_to_string(&meta_path).unwrap().as_str()).unwrap();
        assert_eq!(real_meta.title, "test1");
    }

    #[test]
    fn basic_upsert_with_sections() {
        let tempdir = TempDir::new("vkdoc").unwrap();
        let root_path = tempdir.path();
        let entry_path = Path::new("section/test");
        let full_entry_path = root_path.join(entry_path);
        let section_path = full_entry_path.parent().unwrap();
        let page = Page::new().with_title("test".to_string());

        let vkdoc = Vkdoc::new(&root_path).unwrap();

        assert_eq!(vkdoc.upsert(&entry_path, page).unwrap(), true);

        let meta_path = full_entry_path.join("test.meta.json");
        assert!(meta_path.exists());
        let section_meta_path = section_path.join("section.meta.json");
        assert!(section_meta_path.exists())
    }
}
