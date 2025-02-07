mod blockers;
mod url_ignore_trie;
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum NetworkInterceptManager {
    GLASSDOOR,
    EBAY,
    UPWORK,
    TIKTOK,
    CNN,
    NETFLIX,
    WASHINGTONPOST,
    REDDIT,
    TECHCRUNCH,
    LINKEDIN,
    FOXNEWS,
    ATLASSIAN,
    WEB_ARCHIVE,
    AMAZON,
    FACEBOOK,
    WIKIPEDIA,
    USATODAY,
    NYTIMES,
    BBC,
    X,
    CLAY,
    TCGPLAYER,
    BLEACHERREPORT,
    MEDIUM,
    #[default]
    UNKNOWN,
}

static DOMAIN_MAP: phf::Map<&'static str, NetworkInterceptManager> = ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 13),
        (0, 0),
        (0, 0),
        (2, 21),
        (7, 8),
    ],
    entries: &[
        ("tcgplayer", NetworkInterceptManager::TCGPLAYER),
        ("tiktok", NetworkInterceptManager::TIKTOK),
        ("web.archive", NetworkInterceptManager::WEB_ARCHIVE),
        ("techcrunch", NetworkInterceptManager::TECHCRUNCH),
        ("washingtonpost", NetworkInterceptManager::WASHINGTONPOST),
        ("usatoday", NetworkInterceptManager::USATODAY),
        ("clay", NetworkInterceptManager::CLAY),
        ("bbc", NetworkInterceptManager::BBC),
        ("bleacherreport", NetworkInterceptManager::BLEACHERREPORT),
        ("glassdoor", NetworkInterceptManager::GLASSDOOR),
        ("cnn", NetworkInterceptManager::CNN),
        ("medium", NetworkInterceptManager::MEDIUM),
        ("wikipedia", NetworkInterceptManager::WIKIPEDIA),
        ("foxnews", NetworkInterceptManager::FOXNEWS),
        ("upwork", NetworkInterceptManager::UPWORK),
        ("atlassian", NetworkInterceptManager::ATLASSIAN),
        ("amazon", NetworkInterceptManager::AMAZON),
        ("reddit", NetworkInterceptManager::REDDIT),
        ("facebook", NetworkInterceptManager::FACEBOOK),
        ("linkedin", NetworkInterceptManager::LINKEDIN),
        ("x", NetworkInterceptManager::X),
        ("ebay", NetworkInterceptManager::EBAY),
        ("netflix", NetworkInterceptManager::NETFLIX),
        ("nytimes", NetworkInterceptManager::NYTIMES),
    ],
};
impl NetworkInterceptManager {
    pub fn intercept_detection(&self, url: &str, ignore_visuals: bool, is_xhr: bool) -> bool {
        let mut should_block = false;
        match self {
            NetworkInterceptManager::GLASSDOOR => {
                if is_xhr {
                    should_block = blockers::glassdoor_blockers::block_xhr(url);
                } else {
                    should_block = blockers::glassdoor_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::glassdoor_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::EBAY => {
                if is_xhr {
                    should_block = blockers::ebay_blockers::block_xhr(url);
                } else {
                    should_block = blockers::ebay_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::ebay_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::UPWORK => {
                if is_xhr {
                    should_block = blockers::upwork_blockers::block_xhr(url);
                } else {
                    should_block = blockers::upwork_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::upwork_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::TIKTOK => {
                if is_xhr {
                    should_block = blockers::tiktok_blockers::block_xhr(url);
                } else {
                    should_block = blockers::tiktok_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::tiktok_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::CNN => {
                if is_xhr {
                    should_block = blockers::cnn_blockers::block_xhr(url);
                } else {
                    should_block = blockers::cnn_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::cnn_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::NETFLIX => {
                if is_xhr {
                    should_block = blockers::netflix_blockers::block_xhr(url);
                } else {
                    should_block = blockers::netflix_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::netflix_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::WASHINGTONPOST => {
                if is_xhr {
                    should_block = blockers::washingtonpost_blockers::block_xhr(url);
                } else {
                    should_block = blockers::washingtonpost_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::washingtonpost_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::REDDIT => {
                if is_xhr {
                    should_block = blockers::reddit_blockers::block_xhr(url);
                } else {
                    should_block = blockers::reddit_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::reddit_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::TECHCRUNCH => {
                if is_xhr {
                    should_block = blockers::techcrunch_blockers::block_xhr(url);
                } else {
                    should_block = blockers::techcrunch_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::techcrunch_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::LINKEDIN => {
                if is_xhr {
                    should_block = blockers::linkedin_blockers::block_xhr(url);
                } else {
                    should_block = blockers::linkedin_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::linkedin_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::FOXNEWS => {
                if is_xhr {
                    should_block = blockers::foxnews_blockers::block_xhr(url);
                } else {
                    should_block = blockers::foxnews_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::foxnews_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::ATLASSIAN => {
                if is_xhr {
                    should_block = blockers::atlassian_blockers::block_xhr(url);
                } else {
                    should_block = blockers::atlassian_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::atlassian_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::WEB_ARCHIVE => {
                if is_xhr {
                    should_block = blockers::web_blockers::block_xhr(url);
                } else {
                    should_block = blockers::web_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::web_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::AMAZON => {
                if is_xhr {
                    should_block = blockers::amazon_blockers::block_xhr(url);
                } else {
                    should_block = blockers::amazon_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::amazon_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::FACEBOOK => {
                if is_xhr {
                    should_block = blockers::facebook_blockers::block_xhr(url);
                } else {
                    should_block = blockers::facebook_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::facebook_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::WIKIPEDIA => {
                if is_xhr {
                    should_block = blockers::wikipedia_blockers::block_xhr(url);
                } else {
                    should_block = blockers::wikipedia_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::wikipedia_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::USATODAY => {
                if is_xhr {
                    should_block = blockers::usatoday_blockers::block_xhr(url);
                } else {
                    should_block = blockers::usatoday_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::usatoday_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::NYTIMES => {
                if is_xhr {
                    should_block = blockers::nytimes_blockers::block_xhr(url);
                } else {
                    should_block = blockers::nytimes_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::nytimes_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::BBC => {
                if is_xhr {
                    should_block = blockers::bbc_blockers::block_xhr(url);
                } else {
                    should_block = blockers::bbc_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::bbc_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::X => {
                if is_xhr {
                    should_block = blockers::x_blockers::block_xhr(url);
                } else {
                    should_block = blockers::x_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::x_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::CLAY => {
                if is_xhr {
                    should_block = blockers::clay_blockers::block_xhr(url);
                } else {
                    should_block = blockers::clay_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::clay_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::TCGPLAYER => {
                if is_xhr {
                    should_block = blockers::tcgplayer_blockers::block_xhr(url);
                } else {
                    should_block = blockers::tcgplayer_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::tcgplayer_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::BLEACHERREPORT => {
                if is_xhr {
                    should_block = blockers::bleacherreport_blockers::block_xhr(url);
                } else {
                    should_block = blockers::bleacherreport_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::bleacherreport_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::MEDIUM => {
                if is_xhr {
                    should_block = blockers::medium_blockers::block_xhr(url);
                } else {
                    should_block = blockers::medium_blockers::block_scripts(url);
                    if !should_block && ignore_visuals {
                        should_block = blockers::medium_blockers::block_styles(url);
                    }
                }
            },
            NetworkInterceptManager::UNKNOWN => (),
        }
        should_block
    }
}
