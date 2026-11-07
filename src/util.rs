//! engrampa model — MATE archive manager.

#[derive(Debug, Clone)]
pub struct Archive {
    pub path: String,
    pub format: String,
    pub size: u64,
    pub entries: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub archives: Vec<Archive>,
    pub next_id: u64,
}

impl Default for Model {
    fn default() -> Self { Self { archives: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn open(&mut self, path: &str, format: &str) -> bool {
        if std::path::Path::new(path).exists() {
            self.archives.push(Archive { path: path.into(), format: format.into(), size: 0, entries: vec![] });
            true
        } else { false }
    }

    pub fn create(&mut self, path: &str, format: &str, files: &[String]) -> bool {
        self.archives.push(Archive { path: path.into(), format: format.into(), size: 0, entries: files.to_vec() });
        true
    }

    pub fn extract(&mut self, path: &str, dest: &str) -> bool {
        if let Some(a) = self.archives.iter_mut().find(|a| a.path == path) {
            a.entries.clear();
            true
        } else { false }
    }

    pub fn list(&self) -> &[Archive] { &self.archives }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_extract_list() {
        let mut e = Model::new();
        assert!(e.create("/tmp/test.tar.gz", "tar.gz", &["a.txt".into()]));
        assert!(e.extract("/tmp/test.tar.gz", "/tmp/extract"));
        assert_eq!(e.list()[0].format, "tar.gz");
    }
}
