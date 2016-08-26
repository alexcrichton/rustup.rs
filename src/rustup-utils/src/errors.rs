use url::Url;

error_chain! {
    errors {
        ReadingFile {
            name: &'static str,
            path: Vec<u8>,
        } {
        }
        ReadingDirectory {
            path: Vec<u8>,
        } {
        }
        CreatingDirectory {
            name: &'static str,
            path: Vec<u8>,
        } {
        }
        ExpectedType(t: &'static str, n: String) {
        }
        FilteringFile {
            name: &'static str,
            src: Vec<u8>,
            dest: Vec<u8>,
        } {
        }
        RenamingFile {
            name: &'static str,
            src: Vec<u8>,
            dest: Vec<u8>,
        } {
        }
        RenamingDirectory {
            name: &'static str,
            src: Vec<u8>,
            dest: Vec<u8>,
        } {
        }
        DownloadNotExists {
            url: Url,
            path: Vec<u8>,
        } {
        }
        NotADirectory {
            path: Vec<u8>,
        } {
        }
        LinkingFile {
            src: Vec<u8>,
            dest: Vec<u8>,
        } {
        }
        CopyingFile {
            src: Vec<u8>,
            dest: Vec<u8>,
        } {
        }
        RemovingDirectory {
            name: &'static str,
            path: Vec<u8>,
        } {
        }
        SettingPermissions {
            path: Vec<u8>,
        } {
        }
    }
}
