use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Meta {
    title: String,
    meta_title: String,
    section_title: String,
    short_description: String,
    page_description: String,
    weight: i32,
    uuid: String,
    created_at: String,
    updated_at: String,
}

pub struct Vkdoc {
    path: PathBuf,
}

impl Vkdoc {
    pub fn new(path: &Path) -> Vkdoc {
        fs::create_dir_all(path).expect(
            format!(
                "Unable to initialize a vkdoc project at the specified dir {}",
                path.to_str().unwrap()
            )
            .as_str(),
        );

        Vkdoc {
            path: path.to_path_buf(),
        }
    }

    pub fn upsert(&self, path: &Path, content: String) -> bool {
        let path = self.path.join(path);
        fs::create_dir_all(&path).expect("Unable to create an entry at the specified dir");

        let name = path
            .file_name()
            .expect("Unable to load an entry at the specified dir")
            .to_os_string()
            .into_string()
            .expect("Unable to load an entry at the specified dir");
        let doc_path = path.join(format!("{}.md", name));

        let upsert = if doc_path.exists() {
            let old_content = fs::read_to_string(&doc_path)
                .expect(format!("Unable to read entry content at {}", path.display()).as_str());
            let old_checksum = md5::compute(&old_content);
            let checksum = md5::compute(&content);

            old_checksum != checksum
        } else {
            true
        };

        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.%fZ")
            .to_string();
        let meta_path = path.join(format!("{}.meta.json", name));
        let meta = if meta_path.exists() {
            let meta_content = &fs::read_to_string(&meta_path)
                .expect(format!("Unable to read entry meta at {}", path.display()).as_str());
            let meta: Meta = serde_json::from_str(meta_content)
                .expect(format!("Unable to parse entry meta json at {}", path.display()).as_str());
            let updated_at = if upsert { now } else { meta.updated_at };
            Meta { updated_at, ..meta }
        } else {
            Meta {
                title: name.clone(),
                meta_title: name.clone(),
                section_title: name.clone(),
                short_description: "*No description*".to_string(),
                page_description: "*No description*".to_string(),
                weight: 1,
                uuid: uuid::Uuid::new_v4().to_string(),
                created_at: now.clone(),
                updated_at: now.clone(),
            }
        };

        if upsert {
            fs::write(doc_path, content)
                .expect("Unable to write a vkdoc entry at the specified dir");
            let meta_content =
                serde_json::to_string(&meta).expect("Unable to serialize entry meta json");
            fs::write(meta_path, meta_content)
                .expect("Unable to write a vkdoc meta at the specified dir");
        }
        upsert
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn basic_upsert() {
        let tempdir = TempDir::new("vkdoc").unwrap();
        let path = tempdir.path().join("test");
        let content = "content".to_string();

        let vkdoc = Vkdoc::new(&path);
        assert_eq!(vkdoc.path, path);

        assert_eq!(vkdoc.upsert(&path, content.clone()), true);

        let content_path = path.join("test.md");
        let real_content = fs::read_to_string(&content_path).unwrap();
        assert_eq!(real_content, content);

        let meta_path = path.join("test.meta.json");
        let real_meta: Meta =
            serde_json::from_str(fs::read_to_string(&meta_path).unwrap().as_str()).unwrap();
        assert_eq!(real_meta.title, "test");
        assert_eq!(real_meta.meta_title, "test");
        assert_eq!(real_meta.section_title, "test");
        assert_eq!(real_meta.short_description, "*No description*");
        assert_eq!(real_meta.page_description, "*No description*");

        assert_eq!(vkdoc.upsert(&path, content.clone()), false);
        let real_content = fs::read_to_string(&content_path).unwrap();
        assert_eq!(real_content, content);

        let real_meta: Meta =
            serde_json::from_str(fs::read_to_string(&meta_path).unwrap().as_str()).unwrap();
        assert_eq!(real_meta.updated_at, real_meta.created_at);

        let content = "new content".to_string();
        assert_eq!(vkdoc.upsert(&path, content.clone()), true);

        let real_content = fs::read_to_string(&content_path).unwrap();
        assert_eq!(real_content, content);

        let real_meta: Meta =
            serde_json::from_str(fs::read_to_string(&meta_path).unwrap().as_str()).unwrap();
        assert_ne!(real_meta.updated_at, real_meta.created_at);
    }
}
