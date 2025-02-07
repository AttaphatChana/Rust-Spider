pub fn block_scripts(url: &str) -> bool {
    crate::handler::blockers::intercept_manager::url_ignore_trie::TCGPLAYER_COM_SCRIPTS_TRIE.contains_prefix(url)
}

pub fn block_styles(url: &str) -> bool {
    crate::handler::blockers::intercept_manager::url_ignore_trie::TCGPLAYER_COM_STYLES_TRIE.contains_prefix(url)
}

pub fn block_xhr(url: &str) -> bool {
    crate::handler::blockers::intercept_manager::url_ignore_trie::TCGPLAYER_COM_XHR_TRIE.contains_prefix(url)
}

