use std::env;

#[cfg(test)]
mod tests;

pub struct Config {
    page_size: u32,
    patchsets_cache_dir: String,
    bookmarked_patchsets_path: String,
    mailing_lists_path: String,
    reviewed_patchsets_path: String,
}

impl Config {
    pub fn build() -> Self {
        let page_size: u32;
        let patchsets_cache_dir: String;
        let bookmarked_patchsets_path: String;
        let mailing_lists_path: String;
        let reviewed_patchsets_path: String;

        page_size = match env::var("PATCH_HUB_PAGE_SIZE") {
            Ok(value) => value.parse().unwrap(),
            Err(_) => 30,
        };

        patchsets_cache_dir = match env::var("PATCH_HUB_CACHE_DIR") {
            Ok(value) => format!("{value}/patchsets"),
            Err(_) => format!("{}/.cache/patch_hub/patchsets", env::var("HOME").unwrap()),
        };

        bookmarked_patchsets_path = match env::var("PATCH_HUB_DATA_DIR") {
            Ok(value) => format!("{value}/bookmarked_patchsets.json"),
            Err(_) => format!("{}/.local/share/patch_hub/bookmarked_patchsets.json", env::var("HOME").unwrap()),
        };

        mailing_lists_path = match env::var("PATCH_HUB_DATA_DIR") {
            Ok(value) => format!("{value}/mailing_lists.json"),
            Err(_) => format!("{}/.local/share/patch_hub/mailing_lists.json", env::var("HOME").unwrap()),
        };

        reviewed_patchsets_path = match env::var("PATCH_HUB_DATA_DIR") {
            Ok(value) => format!("{value}/reviewed_patchsets.json"),
            Err(_) => format!("{}/.local/share/patch_hub/reviewed_patchsets.json", env::var("HOME").unwrap()),
        };

        Config {
            page_size,
            patchsets_cache_dir,
            bookmarked_patchsets_path,
            mailing_lists_path,
            reviewed_patchsets_path,
        }
    }

    pub fn get_page_size(&self) -> u32 {
        self.page_size
    }

    pub fn get_patchsets_cache_dir(&self) -> &str {
        &self.patchsets_cache_dir
    }
    
    pub fn get_bookmarked_patchsets_path(&self) -> &str {
        &self.bookmarked_patchsets_path
    }

    pub fn get_mailing_lists_path(&self) -> &str {
        &self.mailing_lists_path
    }

    pub fn get_reviewed_patchsets_path(&self) -> &str {
        &self.reviewed_patchsets_path
    }
}
