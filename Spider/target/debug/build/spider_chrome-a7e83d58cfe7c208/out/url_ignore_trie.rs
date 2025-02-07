use crate::handler::blockers::Trie;
lazy_static::lazy_static! {
pub static ref GLASSDOOR_COM_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://www.glassdoor.com/garnish/static/js/gd-sw-register.");
trie.insert("https://cdnjs.cloudflare.com/ajax/libs/prop-types/15.7.2/prop-types.min.js");
trie.insert("https://www.glassdoor.com/autocomplete/location?");
trie
};
pub static ref EBAY_COM_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://www.ebay.com/sch/ajax/autocomplete");
trie.insert("https://www.ebay.com/blueberry/v1/ads/identity/pixelUrls");
trie.insert("https://svcs.ebay.com/ufeservice/v1/events");
trie.insert("https://www.ebay.com/gh/useracquisition?");
trie.insert("https://vi.vipr.ebaydesc.com/");
trie.insert("https://srv.main.ebayrtm.com/");
trie.insert("https://www.ebay.com/nap/napkinapi/");
trie.insert("https://ir.ebaystatic.com/rs/c/scandal/ScandalJS-");
trie
};
pub static ref UPWORK_COM_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://www.upwork.com/shitake/suit");
trie.insert("https://www.upwork.com/upi/jslogger");
trie.insert("https://mpsnare.iesnare.com/5.8.1/logo.js");
trie.insert("https://first.iovation.com/");
trie.insert("https://zn0izjiulta2j2t4o-upwork.siteintercept.qualtrics.com/");
trie.insert("https://cdn123.forter.com/");
trie.insert("https://www.upwork.com/static/assets/TopNavSsi/visitor-v2/js/manifest.");
trie.insert("https://www.upwork.com/iojs/general5/static_wdp.js");
trie.insert("https://www.upwork.com/static/suit2-tracker/");
trie.insert("https://www.upwork.com/api/graphql/v1?alias=spellCheck");
trie.insert("https://www.upwork.com/api/graphql/v1?alias=relatedSuggestions");
trie.insert("https://www.upwork.com/api/graphql/v1?alias=autoSuggestions");
trie.insert(".siteintercept.qualtrics.com/");
trie.insert(".forter.com");
trie
};
pub static ref TIKTOK_COM_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://mcs.tiktokw.us/v1/list");
trie.insert("https://www.tiktok.com/ttwid/check");
trie.insert("https://www.tiktok.com/api/share/settings");
trie.insert("https://webcast.us.tiktok.com/");
trie.insert("https://www.tiktok.com/api/ba/business/suite/permission/list");
trie.insert("https://www.tiktok.com/api/policy/notice/");
trie.insert("https://www.tiktok.com/api/v1/web-cookie-privacy");
trie.insert("https://www.tiktok.com/aweme/v1/report/inbox/notice");
trie.insert("https://www.tiktok.com/api/inbox/notice_count/");
trie.insert("https://mcs.tiktokv.us/v1/user/webid");
trie.insert("https://mon16-normal-useast5.tiktokv.us/monitor_browser/collect/batch/?bid=tiktok_pns_web_runtime");
trie.insert("https://webcast.tiktok.com/webcast/wallet_api/fs/diamond_buy");
trie.insert("https://lf16-tiktok-web.tiktokcdn-us.com/obj/tiktok-web-tx/tiktok_privacy_protection_framework/loader/");
trie.insert("https://lf16-tiktok-web.tiktokcdn-us.com/obj/tiktok-web-tx/tiktok/webapp/main/webapp-desktop/npm-async-bric_verify_sec_sdk_build_captcha");
trie.insert("/tiktok_privacy_protection_framework/loader");
trie.insert("/obj/tiktok-web-tx/tiktok_privacy_protection_framework/loader");
trie.insert("/service/2/abtest_config/");
trie.insert("collect/batch/?bid=tiktok_pns_web_runtime");
trie.insert("monitor_browser/collect/batch/?bid=tiktok_pns_web_runtime");
trie.insert("/tiktok-cookie-banner/");
trie.insert("/tiktok/webapp/main/webapp-desktop-islands/npm-async-bric_verify_sec_sdk_build_captcha_");
trie
};
pub static ref CNN_COM_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://eus.rubiconproject.com/usync.html?p=20986&endpoint=us-east");
trie.insert("https://widgets.outbrain.com/keystone/conv/KS_conversions.js");
trie.insert("https://simage2.pubmatic.com/AdServer/");
trie.insert("https://segment-data-us-east.zqtk.net");
trie.insert("https://cdn.ml314.com/taglw.js");
trie.insert("https://cdn.optimizely.com/public/");
trie.insert("https://s.ntv.io/serve/load.js");
trie.insert("https://ads.pubmatic.com");
trie.insert("https://widgets.outbrain.com/nanoWidget/externals/obPixelFrame/obPixelFrame.htm");
trie.insert("https://js-sec.indexww.com/um/ixmatch.html");
trie
};
pub static ref NETFLIX_COM_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("/log");
trie.insert("https://assets.nflxext.com/web/");
trie.insert("https://ae.nflximg.net/monet/scripts/adtech_iframe");
trie
};
pub static ref WASHINGTONPOST_COM_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://static.files.bbci.co.uk/core/bundle-component-consent-banner.");
trie.insert("https://static.files.bbci.co.uk/core/bundle-consent-banner.");
trie.insert("https://static.files.bbci.co.uk/core/website/assets/static/scripts/riddle/");
trie
};
pub static ref REDDIT_COM_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://www.redditstatic.com/shreddit/sentry-");
trie
};
pub static ref TECHCRUNCH_COM_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://js.hubspot.com/web-interactives-embed.js");
trie.insert("https://js.hs-banner.com/v2/44101848/banner.js");
trie.insert("https://ssp-sync.criteo.com/user-sync/iframe?");
trie.insert("https://assets.a-mo.net/js/cframe.js");
trie.insert("https://eb2.3lift.com/sync");
trie.insert("https://secure-assets.rubiconproject.com/utils/xapi/multi-sync.");
trie.insert("https://cs.emxdgt.com/");
trie.insert("https://u.openx.net/w/");
trie.insert("https://sdk.mrf.io/statics/");
trie.insert("https://eus.rubiconproject.com/usync.js");
trie.insert("https://js.hs-scripts.com/44101848.js");
trie.insert("https://tsdtocl.com/");
trie.insert("https://opus.analytics.yahoo.com/");
trie.insert("https://opus.analytics.yahoo.com/tag/opus.js");
trie.insert("https://jac.yahoosandbox.com/2.0.0/safeframe.html");
trie.insert("https://s.yimg.com/");
trie
};
pub static ref LINKEDIN_COM_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("/log");
trie.insert("https://www.linkedin.com/li/track");
trie.insert("https://li.protechts.net");
trie.insert("https://www.linkedin.com/platform-telemetry/li");
trie.insert("https://www.linkedin.com/organization-guest/api/feedUpdates/");
trie.insert("https://www.linkedin.com/feedcontent-guest/api/ingraphs/gauge");
trie.insert("https://www.linkedin.com/voyager/api/");
trie.insert("https://platform.linkedin.com/litms/allowlist/voyager-web-global");
trie
};
pub static ref FOXNEWS_COM_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://amplify.outbrain.com/cp/obtp.js");
trie.insert("https://static.criteo.net/js/ld/ld.js");
trie.insert("https://fundingchoicesmessages.google.com/");
trie.insert("https://static.foxnews.com/static/leap/sites/fnc/metrics.js");
trie.insert("https://static.foxnews.com/static/isa/app/lib/VisitorAPI.js");
trie.insert("https://static.foxnews.com/static/orion/scripts/core/utils/geo.js");
trie
};
pub static ref ATLASSIAN_COM_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://atl-global.atlassian.com/js/2.219.0/xid/atl-xid.js");
trie.insert("https://atl-global.atlassian.com/js/atl-global.min.js");
trie.insert("https://xxid.atl-paas.net");
trie
};
pub static ref WEB_ARCHIVE_ORG_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://www.gstatic.com/");
trie.insert("https://archive.org/includes/donate.php");
trie.insert("https://www.googletagmanager.com/");
trie.insert("/ads/tpc-check.html");
trie.insert("securepubads.g.doubleclick.net");
trie.insert("https://archive.org/includes/athena.js");
trie.insert("https://news.google.com/swg/");
trie.insert("tag/js/gpt.js");
trie.insert("swg/js/v1/swg.js");
trie.insert("ads/tpc-check.html");
trie
};
pub static ref AMAZON_COM_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://cognito-identity.us-east-1.amazonaws.com");
trie.insert("https://completion.amazon.com/api/2017/suggestions");
trie.insert("https://sts.us-east-1.amazonaws.com/");
trie.insert("https://www.amazon.com/cross_border_interstitial_sp/render");
trie.insert("https://aax-us-east-retail-direct.amazon.com/e/xsp/getAd");
trie.insert("https://fls-na.amazon.com/1/batch/1/OE/");
trie.insert("https://unagi.amazon.com/1/events/");
trie.insert("https://images-na.ssl-images-amazon.com/images/S/apesafeframe/ape/sf/desktop/");
trie.insert("https://m.media-amazon.com/images/G/01/csm/showads");
trie.insert("https://dataplane.rum");
trie.insert("https://client.rum");
trie.insert(".amazon-adsystem.com");
trie.insert("SearchPartnerAssets");
trie.insert("inner-host.min.js");
trie.insert("https://s.amazon-adsystem.com/");
trie
};
pub static ref FACEBOOK_COM_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref WIKIPEDIA_ORG_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://meta.wikimedia.org/w/index.php?title=MediaWiki:Wikiminiatlas.js&action=raw&ctype=text/javascript");
trie.insert("https://login.wikimedia.org/wiki/Special:CentralAutoLogin/checkLoggedIn?useformat=desktop&wikiid=ptwiki&type=script&wikiid=ptwiki&type=script");
trie.insert(".wikipedia.org/w/load.php?lang=pt&modules=ext.centralNotice.choiceData%2CgeoIP%2CstartUp%7Cext.centralauth.ForeignApi%2Ccentralautologin%7Cext.checkUser.clientHints%7Cext.cite.ux-enhancements%7Cext.cx.eventlogging.campaigns");
trie.insert(".wikipedia.org/w/load.php?lang=pt&modules=startup&only=scripts&raw=1&skin=vector-2022");
trie.insert(".eventlogging.campaigns");
trie.insert("%2CFeedbackHighlight%2");
trie.insert(".quicksurveys.");
trie.insert("Special:CentralAutoLogin/start?type=script");
trie
};
pub static ref USATODAY_COM_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://gum.criteo.com/sync?");
trie.insert("https://pm-widget.taboola.com/usatodaydemo/");
trie.insert("https://www.washingtonpost.com/subscribe/privacy-");
trie.insert("https://g.3gl.net/jp/3543/v3.3.9/M");
trie
};
pub static ref NYTIMES_COM_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://purr.nytimes.com/v1/purr-cache");
trie.insert("https://static01.nyt.com/ads/tpc-check.html");
trie.insert("https://www.nytimes.com/vi-assets/static-assets/adslot");
trie.insert("https://purr.nytimes.com/v2/tcf");
trie.insert("https://a.et.nytimes.com//.status");
trie.insert("https://www.nytimes.com/fides/api/v1/privacy-experience?");
trie.insert("https://o82024.ingest.us.sentry.io/");
trie.insert("https://a.nytimes.com/svc/nyt/data-layer?");
trie.insert("https://www.nytimes.com/ads/prebid9.11.0.js");
trie
};
pub static ref BBC_COM_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://static.files.bbci.co.uk/core/bundle-component-consent-banner.");
trie.insert("https://static.files.bbci.co.uk/core/bundle-consent-banner.");
trie.insert("https://static.files.bbci.co.uk/core/website/assets/static/scripts/riddle/");
trie
};
pub static ref X_COM_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://accounts.google.com/gsi/");
trie.insert("https://appleid.cdn-apple.com/appleauth/static/jsapi/appleid/1/en_US/appleid.auth.js");
trie.insert("https://api.x.com/1.1/onboarding/sso_init.json");
trie.insert("https://api.x.com/1.1/jot/client_event.json");
trie.insert("https://api.x.com/1.1/jot/error_log.json");
trie.insert("https://api.x.com/1.1/hashflags.json");
trie
};
pub static ref CLAY_COM_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://js.hs-scripts.com/20785370.js");
trie.insert("https://wt.inflection.io/scripts/wt-script.js");
trie.insert("https://js.hs-banner.com/v2/20785370/banner.js");
trie.insert("https://r.wdfl.co/rw.js");
trie
};
pub static ref TCGPLAYER_COM_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://data.tcgplayer.com/suggestions/trending");
trie.insert("https://mpapi.tcgplayer.com/v2/kickbacks?active=true");
trie.insert("https://homepage.marketplace.tcgplayer.com/sitealert.json");
trie.insert("https://infinite-api.tcgplayer.com/signup/?");
trie.insert("https://features.tcgplayer.com/v1/optimizely/Variation/");
trie.insert("https://mpapi.tcgplayer.com/v2/address/countryCodes?mpfev=3031");
trie
};
pub static ref BLEACHERREPORT_COM_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://api.bounceexchange.com/bounce/");
trie.insert("https://lightning.bleacherreport.com/launch/");
trie.insert("https://sp.auth.adobe.com/entitlement/v4/AccessEnablerProxy.js");
trie.insert("https://cadmus.script.ac/d2uap9jskdzp2/script.js");
trie.insert("https://sp.auth.adobe.com/entitlement/v4/AccessEnablerProxy.html");
trie.insert("https://entitlement.auth.adobe.com/entitlement/v4/AccessEnabler.js");
trie
};
pub static ref MEDIUM_COM_SCRIPTS_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://cdn-client.medium.com/lite/static/js/instrumentation.");
trie.insert("https://medium.com/_/clientele/reports/performance/");
trie.insert("https://cdn-client.medium.com/lite/static/js/reporting.f");
trie.insert("https://medium.com/_/clientele/reports/performance/");
trie.insert("https://cdn-client.medium.com/lite/static/js/manifest.");
trie.insert("clientele/reports/performance/");
trie.insert("https://www.google.com/js/bg/");
trie.insert("https://chitaranjanbiswal93.medium.com/_/clientele/reports/performance/");
trie
};
pub static ref GLASSDOOR_COM_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref EBAY_COM_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref UPWORK_COM_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref TIKTOK_COM_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref CNN_COM_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://mcdp-chidc2.outbrain.com/");
trie.insert("https://receive.wmcdp.io/v1/reg");
trie.insert("https://collector.cdp.cnn.com/com.snowplowanalytics.snowplow/tp2");
trie.insert("https://i.clean.gg/1a");
trie.insert("https://www.cnn.com/public/api/alerts");
trie.insert("https://collector-px611dwqp1.px-cloud.net/api/v2/collector");
trie.insert("https://ids.cdnwidget.com/");
trie.insert("https://dfp.bouncex.net/pub/v2/segment/2qi31PsFb3FLPiCctqR86TDa7Ws");
trie
};
pub static ref NETFLIX_COM_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref WASHINGTONPOST_COM_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://ny77jj.washingtonpost.com/");
trie.insert("https://api.permutive.com/v2.0/");
trie.insert("https://targeting.washingtonpost.com/api/v1/targeting");
trie.insert("https://subscribe.washingtonpost.com/offers/service/v2/offers/");
trie.insert("https://www.washingtonpost.com/prism/api/alerts");
trie.insert("https://mug.criteo.com/sid?");
trie.insert("https://subscribe.washingtonpost.com/offers/service/get-adot-offer/");
trie.insert("https://subscribe.washingtonpost.com/logging/logHandledError");
trie.insert("https://ib.adnxs.com/getuidj");
trie
};
pub static ref REDDIT_COM_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://w3-reporting.reddit.com/reports");
trie.insert("https://www.reddit.com/svc/shreddit/events");
trie
};
pub static ref TECHCRUNCH_COM_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://i.clean.gg/1a");
trie.insert("https://c2shb-oao.ssp.yahoo.com/admax/bid/partners/");
trie.insert("https://s.seedtag.com/c/hb/bid");
trie.insert("https://display.bidder.taboola.com/OpenRTB/TaboolaHB/auction?publisher=");
trie.insert("https://fastlane.rubiconproject.com/a/api/fastlane.json");
trie.insert("https:/ads.yieldmo.com/exchange/prebid");
trie.insert("https://rtb.openx.net/openrtbb/prebidjs");
trie.insert("https://pbs.yahoo.com/cookie_sync");
trie.insert("https://pbs.yahoo.com/openrtb2/auction");
trie.insert("https://hbopenbid.pubmatic.com/translator?source=prebid-client");
trie.insert("https://events.newsroom.bi/ingest.php");
trie.insert("https://events.newsroom.bi/data/rfv.php");
trie.insert("https://u.openx.net/w/");
trie.insert("https://prebid.media.net/rtb/prebid");
trie.insert("https://beacon.bidder.taboola.com/bidError");
trie.insert("https://tlx.3lift.com/header/auction?");
trie.insert("https://pbd.yahoo.com/data/analytics");
trie.insert("prebid-config");
trie.insert("https://3p-udc.yahoo.com/");
trie.insert("https://direct-events-collector.spot.im");
trie.insert("https://static-cdn.spot.im/production/icons/sprites/sprite.svg");
trie.insert("https://api-2-0.spot.im/v1.0.0/device-load");
trie.insert("https://s.yimg.com/eh/prebid-config/");
trie.insert("https://techcrunch.com/wp-json/tc/v1/users/is-logged-in");
trie
};
pub static ref LINKEDIN_COM_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref FOXNEWS_COM_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://prod.pyxis.atp.fox/pyxis/submit");
trie.insert("https://sdk.iad-05.braze.com/api/v3/data/");
trie.insert("https://psb.taboola.com/topics_api");
trie.insert("https://direct-events-collector.spot.im/api/v2/events?stream_name=init");
trie.insert("https://launcher.spot.im/spot/sp_ANQXRpqH");
trie.insert("https://p.flipp.com/beacons");
trie.insert("https://api.foxnews.com/v3/video-player/");
trie
};
pub static ref ATLASSIAN_COM_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://o55978.ingest.sentry.io/api");
trie.insert("https://api.atlassian.com/flags/api/v2/frontend/clientSdkKey/bxp-magnolia_web");
trie.insert("https://api.atlassian.com/flags/api/v2/frontend/experimentValues");
trie.insert("https://app.launchdarkly.com/sdk/goals/5afa3d7607a72221591aeb73");
trie.insert("https://events.launchdarkly.com/events/diagnostic/5afa3d7607a72221591aeb73");
trie.insert("https://app.launchdarkly.com/sdk/evalx/5afa3d7607a72221591aeb73/contexts/");
trie.insert("https://xp.atlassian.com/v1/rgstr");
trie.insert("https://www.atlassian.com/gateway/api/ais/available-products");
trie.insert("https://www.atlassian.com/gateway/api/cookie-integrator/api/onetrust/jwt");
trie.insert("ingest.sentry.io/api/");
trie
};
pub static ref WEB_ARCHIVE_ORG_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("http://web.archive.org/screenshot/");
trie.insert("https://web.archive.org/web/");
trie
};
pub static ref AMAZON_COM_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("?pageViewLogging=1");
trie.insert(".amazon-adsystem.com/");
trie
};
pub static ref FACEBOOK_COM_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://www.facebook.com/ajax/webstorage/process_keys/?state=1");
trie
};
pub static ref WIKIPEDIA_ORG_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref USATODAY_COM_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://hlsmedia.gannett-cdn.com/authoring/videos/");
trie.insert("https://hp.taboola.com/usatodaydemo/");
trie
};
pub static ref NYTIMES_COM_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref BBC_COM_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://www.bbc.com/userinfo");
trie.insert("https://www.bbc.co.uk/wc-data/container/consent-banner");
trie.insert("https://idcta.api.bbc.com/idcta/config");
trie
};
pub static ref X_COM_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref CLAY_COM_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://videsigns-staging.co.uk/counter");
trie.insert("https://forms.hscollectedforms.net/collected-forms/");
trie.insert("https://wt.inflection.io/config");
trie.insert("https://api.jetboost.io/sites/clyq6n8xs00bu0ry10esg40y3");
trie.insert("https://api.ashbyhq.com/posting-api/job-board/claylab");
trie
};
pub static ref TCGPLAYER_COM_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref BLEACHERREPORT_COM_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://sync.search.spotxchange.com/audience_id");
trie.insert("https://media.bleacherreport.com/video/upload/");
trie.insert("https://api2.branch.io/v1/open");
trie.insert("https://receive.wmcdp.io/v1/reg");
trie.insert("https://sp.auth.adobe.com/adobe-services/config/Bleacher");
trie.insert("https://sp.auth.adobe.com/o/client/token");
trie.insert("https://media.bleacherreport.com/video/upload/");
trie.insert("https://0c03b8bbf14bc87e6497b4f7d75ff5b4d35853b1.cws.conviva.com/0/wsg");
trie.insert("https://atlas.ngtv.io/v2/locate");
trie.insert("https://licensing.bitmovin.com/licensing");
trie.insert("https://live-fallback.bleacherreport.com/oam/v2/anonymous/register");
trie.insert("https://wmff.warnermediacdn.com/psm_2_prod_full.json?version=1");
trie
};
pub static ref MEDIUM_COM_XHR_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref GLASSDOOR_COM_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://www.glassdoor.com/sam-global-nav/static/");
trie.insert("https://www.glassdoor.com/garnish/static/js/gd-");
trie.insert("https://unpkg.com/@dotlottie/player-component@");
trie.insert("https://www.glassdoor.com/job-search-next/assets/_next/static/");
trie.insert("https://www.glassdoor.com/ei-overview-next/assets/_next/static/");
trie.insert("https://www.glassdoor.com/occ-salaries-web/assets/_next/static/");
trie
};
pub static ref EBAY_COM_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref UPWORK_COM_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://www.upwork.com/static/assets/TopNavSsi/visitor-v2/");
trie.insert("https://www.upwork.com/static/assets/UniversalSearchNuxt/styles~");
trie.insert("https://www.upwork.com/static/assets/Brontes/styles");
trie.insert("https://www.upwork.com/static/assets/Brontes/google-one-tap.6226625d.js");
trie
};
pub static ref TIKTOK_COM_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref CNN_COM_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://libs.outbrain.com/video/");
trie.insert("https://turnip.cdn.turner.com/top/player-ui/");
trie.insert("https://warnermediagroup-com.videoplayerhub.com/galleryloader.js");
trie
};
pub static ref NETFLIX_COM_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref WASHINGTONPOST_COM_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://libs.outbrain.com/video/");
trie.insert("https://turnip.cdn.turner.com/top/player-ui/");
trie.insert("https://warnermediagroup-com.videoplayerhub.com/galleryloader.js");
trie
};
pub static ref REDDIT_COM_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://www.redditstatic.com/shreddit");
trie
};
pub static ref TECHCRUNCH_COM_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref LINKEDIN_COM_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref FOXNEWS_COM_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://player.h-cdn.com/loader.js?");
trie.insert("https://static.foxnews.com/static/orion/scripts/core/video/");
trie.insert("https://foxnewsplayer-a.akamaihd.net/player/");
trie
};
pub static ref ATLASSIAN_COM_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref WEB_ARCHIVE_ORG_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("http://web.archive.org/screenshot/");
trie.insert("https://news.google.com/");
trie.insert("https://static01.nyt.com/video-static/vhs3/vhs.min.js");
trie.insert("video-static/vhs3/vhs.min.js");
trie.insert("https://web.archive.org/_static/js/ruffle/ruffle.js");
trie.insert("https://web.archive.org/_static/js/bundle-playback.js");
trie
};
pub static ref AMAZON_COM_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref FACEBOOK_COM_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref WIKIPEDIA_ORG_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref USATODAY_COM_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://vidstat.taboola.com/vpaid/");
trie.insert("https://imprnjmp.taboola.com/");
trie.insert("https://cdn.taboola.com/libtrc/usatodaydemo/loader.js");
trie
};
pub static ref NYTIMES_COM_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://static01.nyt.com/video-static/vhs3/vhs.min.js");
trie.insert("https://www.nytimes.com/vi-assets/static-assets/vendors~");
trie.insert("https://als-svc.nytimes.com/als");
trie
};
pub static ref BBC_COM_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://libs.outbrain.com/video/");
trie.insert("https://turnip.cdn.turner.com/top/player-ui/");
trie.insert("https://warnermediagroup-com.videoplayerhub.com/galleryloader.js");
trie
};
pub static ref X_COM_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref CLAY_COM_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://transcend-cdn.com/cm/e091122d-95d3-4fb5-b663-7121c10f3329/ui.js");
trie.insert("https://js.hs-banner.com/v2/20785370/banner.js");
trie.insert("https://js.hscollectedforms.net/collectedforms.js");
trie.insert("https://xxndfp.csb.app/src/components/swipers.js");
trie
};
pub static ref TCGPLAYER_COM_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
pub static ref BLEACHERREPORT_COM_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie.insert("https://static-assets.bleacherreport.net/js/vendors~bitmovin-player");
trie.insert("https://vid.bleacherreport.com/videos/");
trie
};
pub static ref MEDIUM_COM_STYLES_TRIE: Trie = {
let mut trie = Trie::new();
trie
};
}
