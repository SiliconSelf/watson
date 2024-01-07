use reqwest::Client;
use crate::gen::SITES;
#[tokio::test]
async fn test_social_tchncs_de_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("social.tchncs.de").expect("Entry not found").test("Milan").await.is_some()); }
#[tokio::test]
async fn test_jellyfin_weblate_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Jellyfin Weblate").expect("Entry not found").test("EraYaN").await.is_some()); }
#[tokio::test]
async fn test_mmorpg_forum_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("MMORPG Forum").expect("Entry not found").test("goku").await.is_some()); }
#[tokio::test]
async fn test_sublimeforum_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("SublimeForum").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_mixcloud_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("MixCloud").expect("Entry not found").test("jenny").await.is_some()); }
#[tokio::test]
async fn test_slideshare_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("SlideShare").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_smugmug_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("SmugMug").expect("Entry not found").test("winchester").await.is_some()); }
#[tokio::test]
async fn test_asciinema_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Asciinema").expect("Entry not found").test("red").await.is_some()); }
#[tokio::test]
async fn test_sportsru_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("SportsRU").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_coderwall_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Coderwall").expect("Entry not found").test("hacker").await.is_some()); }
#[tokio::test]
async fn test_soundcloud_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("SoundCloud").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_rate_your_music_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Rate Your Music").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_rubygems_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("RubyGems").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_bookcrossing_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Bookcrossing").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_genius_users_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Genius (Users)").expect("Entry not found").test("genius").await.is_some()); }
#[tokio::test]
async fn test_gitee_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Gitee").expect("Entry not found").test("wizzer").await.is_some()); }
#[tokio::test]
async fn test_issuu_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Issuu").expect("Entry not found").test("jenny").await.is_some()); }
#[tokio::test]
async fn test_ogusers_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("OGUsers").expect("Entry not found").test("ogusers").await.is_some()); }
#[tokio::test]
async fn test_gutefrage_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Gutefrage").expect("Entry not found").test("gutefrage").await.is_some()); }
#[tokio::test]
async fn test_youpic_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("YouPic").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_openstreetmap_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("OpenStreetMap").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_myminifactory_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("MyMiniFactory").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_needrom_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Needrom").expect("Entry not found").test("needrom").await.is_some()); }
#[tokio::test]
async fn test_cryptomator_forum_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Cryptomator Forum").expect("Entry not found").test("michael").await.is_some()); }
#[tokio::test]
async fn test_pinkbike_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Pinkbike").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_pr0gramm_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("pr0gramm").expect("Entry not found").test("cha0s").await.is_some()); }
#[tokio::test]
async fn test_replit_com_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Replit.com").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_sourceforge_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("SourceForge").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_deviantart_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("DeviantART").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_biohacking_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("BioHacking").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_clubhouse_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Clubhouse").expect("Entry not found").test("waniathar").await.is_some()); }
#[tokio::test]
async fn test_fortnitetracker_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("FortniteTracker").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_docker_hub_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Docker Hub").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_f3_cool_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("F3.cool").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_clapper_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Clapper").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_erome_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Erome").expect("Entry not found").test("bob").await.is_some()); }
#[tokio::test]
async fn test_ionic_forum_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Ionic Forum").expect("Entry not found").test("theblue222").await.is_some()); }
#[tokio::test]
async fn test_lobsters_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Lobsters").expect("Entry not found").test("jcs").await.is_some()); }
#[tokio::test]
async fn test_notabug_org_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("NotABug.org").expect("Entry not found").test("red").await.is_some()); }
#[tokio::test]
async fn test_2dimensions_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("2Dimensions").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_hackaday_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Hackaday").expect("Entry not found").test("adam").await.is_some()); }
#[tokio::test]
async fn test_bravecommunity_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("BraveCommunity").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_myanimelist_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("MyAnimeList").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_rclone_forum_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Rclone Forum").expect("Entry not found").test("ncw").await.is_some()); }
#[tokio::test]
async fn test_drive2_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("drive2").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_minecraft_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Minecraft").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_vero_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Vero").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_weebly_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Weebly").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_nairaland_com_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("nairaland.com").expect("Entry not found").test("red").await.is_some()); }
#[tokio::test]
async fn test_imgur_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Imgur").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_coinvote_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Coinvote").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_pypi_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("PyPi").expect("Entry not found").test("Blue").await.is_some()); }
#[tokio::test]
async fn test_mercadolivre_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("mercadolivre").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_wiki_vg_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("wiki.vg").expect("Entry not found").test("Auri").await.is_some()); }
#[tokio::test]
async fn test_royalcams_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("RoyalCams").expect("Entry not found").test("asuna-black").await.is_some()); }
#[tokio::test]
async fn test_wattpad_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Wattpad").expect("Entry not found").test("Dogstho7951").await.is_some()); }
#[tokio::test]
async fn test_eintracht_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("eintracht").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_9gag_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("9GAG").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_memrise_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Memrise").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_freesound_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Freesound").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_virustotal_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("VirusTotal").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_spotify_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Spotify").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_gamespot_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Gamespot").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_chaturbate_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("ChaturBate").expect("Entry not found").test("cute18cute").await.is_some()); }
#[tokio::test]
async fn test_slides_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Slides").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_skyrock_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("skyrock").expect("Entry not found").test("red").await.is_some()); }
#[tokio::test]
async fn test_codepen_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Codepen").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_flipboard_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Flipboard").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_shitpostbot5000_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("ShitpostBot5000").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_hashnode_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Hashnode").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_anilist_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Anilist").expect("Entry not found").test("Josh").await.is_some()); }
#[tokio::test]
async fn test_d3ru_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("d3RU").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_satsisru_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("satsisRU").expect("Entry not found").test("red").await.is_some()); }
#[tokio::test]
async fn test_kwork_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("kwork").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_imgup_cz_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("ImgUp.cz").expect("Entry not found").test("adam").await.is_some()); }
#[tokio::test]
async fn test_monkeytype_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Monkeytype").expect("Entry not found").test("Lost_Arrow").await.is_some()); }
#[tokio::test]

            async fn test_monkeytype_unclaimed() {
                crate::REQWEST_CLIENT.get_or_init(crate::create_client);
                assert!(SITES.get("Monkeytype").expect("Entry not found").test("noonewouldeverusethis7").await.is_none()); }
#[tokio::test]
async fn test_periscope_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Periscope").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_splits_io_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Splits.io").expect("Entry not found").test("cambosteve").await.is_some()); }
#[tokio::test]
async fn test_nnru_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("nnRU").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_youporn_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("YouPorn").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_egpu_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("eGPU").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_hubpages_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("HubPages").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_gfycat_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("gfycat").expect("Entry not found").test("Test").await.is_some()); }
#[tokio::test]
async fn test_about_me_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("About.me").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_tldr_legal_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("TLDR Legal").expect("Entry not found").test("kevin").await.is_some()); }
#[tokio::test]
async fn test_themeforest_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("ThemeForest").expect("Entry not found").test("user").await.is_some()); }
#[tokio::test]
async fn test_itch_io_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Itch.io").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_pornhub_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Pornhub").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_lottiefiles_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("LottieFiles").expect("Entry not found").test("lottiefiles").await.is_some()); }
#[tokio::test]
async fn test_trawelling_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Trawelling").expect("Entry not found").test("lassestolley").await.is_some()); }
#[tokio::test]
async fn test_sporcle_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Sporcle").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_windy_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Windy").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_fixya_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("fixya").expect("Entry not found").test("adam").await.is_some()); }
#[tokio::test]
async fn test_jbzd_com_pl_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("jbzd.com.pl").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_mstdn_io_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("mstdn.io").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_alik_cz_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Alik.cz").expect("Entry not found").test("julian").await.is_some()); }
#[tokio::test]
async fn test_kaggle_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Kaggle").expect("Entry not found").test("dansbecker").await.is_some()); }
#[tokio::test]
async fn test_wykop_pl_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("wykop.pl").expect("Entry not found").test("janusz-nowak").await.is_some()); }
#[tokio::test]
async fn test_dailymotion_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("DailyMotion").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_giphy_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Giphy").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_audiojungle_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Audiojungle").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_livejournal_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("LiveJournal").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_newgrounds_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Newgrounds").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_promodj_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("PromoDJ").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_etsy_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Etsy").expect("Entry not found").test("JennyKrafts").await.is_some()); }
#[tokio::test]
async fn test_yandexmusic_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("YandexMusic").expect("Entry not found").test("ya.playlist").await.is_some()); }
#[tokio::test]
async fn test_sessionize_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("sessionize").expect("Entry not found").test("jason-mayes").await.is_some()); }
#[tokio::test]
async fn test_sbazar_cz_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Sbazar.cz").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_oracle_community_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Oracle Community").expect("Entry not found").test("dev").await.is_some()); }
#[tokio::test]
async fn test_redtube_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("RedTube").expect("Entry not found").test("hacker").await.is_some()); }
#[tokio::test]
async fn test_crevado_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Crevado").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_autofrage_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Autofrage").expect("Entry not found").test("autofrage").await.is_some()); }
#[tokio::test]
async fn test_linktree_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Linktree").expect("Entry not found").test("anne").await.is_some()); }
#[tokio::test]
async fn test_naver_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Naver").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_polygon_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Polygon").expect("Entry not found").test("swiftstickler").await.is_some()); }
#[tokio::test]
async fn test_splice_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Splice").expect("Entry not found").test("splice").await.is_some()); }
#[tokio::test]
async fn test_airliners_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Airliners").expect("Entry not found").test("yushinlin").await.is_some()); }
#[tokio::test]
async fn test_icq_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("ICQ").expect("Entry not found").test("Micheal").await.is_some()); }
#[tokio::test]
async fn test_geocaching_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("geocaching").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_finanzfrage_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Finanzfrage").expect("Entry not found").test("finanzfrage").await.is_some()); }
#[tokio::test]
async fn test_chaos_social_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("chaos.social").expect("Entry not found").test("rixx").await.is_some()); }
#[tokio::test]
async fn test_couchsurfing_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("couchsurfing").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_harvard_scholar_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Harvard Scholar").expect("Entry not found").test("ousmanekane").await.is_some()); }
#[tokio::test]
async fn test_caddy_community_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Caddy Community").expect("Entry not found").test("taako_magnusen").await.is_some()); }
#[tokio::test]
async fn test_keybase_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Keybase").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_tnaflix_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("TnAFlix").expect("Entry not found").test("hacker").await.is_some()); }
#[tokio::test]
async fn test_shpock_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Shpock").expect("Entry not found").test("user").await.is_some()); }
#[tokio::test]
async fn test_codewars_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Codewars").expect("Entry not found").test("example").await.is_some()); }
#[tokio::test]
async fn test_buymeacoffee_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("BuyMeACoffee").expect("Entry not found").test("red").await.is_some()); }
#[tokio::test]
async fn test_giant_bomb_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Giant Bomb").expect("Entry not found").test("bob").await.is_some()); }
#[tokio::test]
async fn test_ask_fedora_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Ask Fedora").expect("Entry not found").test("red").await.is_some()); }
#[tokio::test]
async fn test_behance_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Behance").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_tuna_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Tuna").expect("Entry not found").test("bob").await.is_some()); }
#[tokio::test]
async fn test_mastodon_social_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("mastodon.social").expect("Entry not found").test("Gargron").await.is_some()); }
#[tokio::test]
async fn test_uid_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("uid").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_xbox_gamertag_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Xbox Gamertag").expect("Entry not found").test("red").await.is_some()); }
#[tokio::test]
async fn test_last_fm_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("last.fm").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_webnode_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("WebNode").expect("Entry not found").test("radkabalcarova").await.is_some()); }
#[tokio::test]
async fn test_crowdin_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Crowdin").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_vimeo_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Vimeo").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_fandom_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Fandom").expect("Entry not found").test("Jungypoo").await.is_some()); }
#[tokio::test]
async fn test_moikrug_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("moikrug").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_rumble_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Rumble").expect("Entry not found").test("John").await.is_some()); }
#[tokio::test]
async fn test_biggerpockets_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("BiggerPockets").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_pikabu_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("pikabu").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_bitbucket_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("BitBucket").expect("Entry not found").test("white").await.is_some()); }
#[tokio::test]
async fn test_lesswrong_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("LessWrong").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_tradingview_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("TradingView").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_bikemap_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Bikemap").expect("Entry not found").test("bikemap").await.is_some()); }
#[tokio::test]
async fn test_discogs_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Discogs").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_svidbook_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("svidbook").expect("Entry not found").test("green").await.is_some()); }
#[tokio::test]
async fn test_genius_artists_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Genius (Artists)").expect("Entry not found").test("genius").await.is_some()); }
#[tokio::test]
async fn test_tellonym_me_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Tellonym.me").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_hackster_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("hackster").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_trashboxru_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("TrashboxRU").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_authorstream_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("authorSTREAM").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_airbit_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Airbit").expect("Entry not found").test("airbit").await.is_some()); }
#[tokio::test]
async fn test_colourlovers_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("ColourLovers").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_rajce_net_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Rajce.net").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_discuss_elastic_co_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Discuss.Elastic.co").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_lor_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("LOR").expect("Entry not found").test("red").await.is_some()); }
#[tokio::test]
async fn test_wicg_forum_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("WICG Forum").expect("Entry not found").test("stefano").await.is_some()); }
#[tokio::test]
async fn test_livelib_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("livelib").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_car_talk_community_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Car Talk Community").expect("Entry not found").test("always_fixing").await.is_some()); }
#[tokio::test]
async fn test_weblate_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Weblate").expect("Entry not found").test("adam").await.is_some()); }
#[tokio::test]
async fn test_queer_af_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("queer.af").expect("Entry not found").test("erincandescent").await.is_some()); }
#[tokio::test]
async fn test_traktrain_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("TRAKTRAIN").expect("Entry not found").test("traktrain").await.is_some()); }
#[tokio::test]
async fn test_blogger_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Blogger").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_gesundheitsfrage_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Gesundheitsfrage").expect("Entry not found").test("gutefrage").await.is_some()); }
#[tokio::test]
async fn test_artstation_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("ArtStation").expect("Entry not found").test("Blue").await.is_some()); }
#[tokio::test]
async fn test_dev_community_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("DEV Community").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_mastodon_cloud_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("mastodon.cloud").expect("Entry not found").test("TheAdmin").await.is_some()); }
#[tokio::test]
async fn test_bitwarden_forum_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Bitwarden Forum").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_nyaa_si_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Nyaa.si").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_slack_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Slack").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_unsplash_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Unsplash").expect("Entry not found").test("jenny").await.is_some()); }
#[tokio::test]
async fn test_getmyuni_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("GetMyUni").expect("Entry not found").test("Upneet.Grover17").await.is_some()); }
#[tokio::test]
async fn test_exposure_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Exposure").expect("Entry not found").test("jonasjacobsson").await.is_some()); }
#[tokio::test]
async fn test_championat_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Championat").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_leetcode_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("LeetCode").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_patreon_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Patreon").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_redbubble_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Redbubble").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_launchpad_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Launchpad").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_irecommend_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("irecommend").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_bongacams_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("BongaCams").expect("Entry not found").test("asuna-black").await.is_some()); }
#[tokio::test]
async fn test_leasehackr_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("leasehackr").expect("Entry not found").test("adam").await.is_some()); }
#[tokio::test]
async fn test_blip_fm_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("BLIP.fm").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_eyeem_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("EyeEm").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_eintracht_frankfurt_forum_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Eintracht Frankfurt Forum").expect("Entry not found").test("mmammu").await.is_some()); }
#[tokio::test]
async fn test_envato_forum_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Envato Forum").expect("Entry not found").test("enabled").await.is_some()); }
#[tokio::test]
async fn test_intigriti_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Intigriti").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_sportlerfrage_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Sportlerfrage").expect("Entry not found").test("sportlerfrage").await.is_some()); }
#[tokio::test]
async fn test_warrior_forum_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Warrior Forum").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_toster_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("toster").expect("Entry not found").test("adam").await.is_some()); }
#[tokio::test]
async fn test_xvideos_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Xvideos").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_flightradar24_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Flightradar24").expect("Entry not found").test("jebbrooks").await.is_some()); }
#[tokio::test]
async fn test_gradle_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Gradle").expect("Entry not found").test("jetbrains").await.is_some()); }
#[tokio::test]
async fn test_fl_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("fl").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_mastodon_technology_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("mastodon.technology").expect("Entry not found").test("ashfurrow").await.is_some()); }
#[tokio::test]
async fn test_fameswap_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Fameswap").expect("Entry not found").test("fameswap").await.is_some()); }
#[tokio::test]
async fn test_flickr_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Flickr").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_bezuzyteczna_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Bezuzyteczna").expect("Entry not found").test("Jackson").await.is_some()); }
#[tokio::test]
async fn test_gitbook_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("GitBook").expect("Entry not found").test("gitbook").await.is_some()); }
#[tokio::test]
async fn test_cloudflarecommunity_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("CloudflareCommunity").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_icons8_community_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Icons8 Community").expect("Entry not found").test("thefourCraft").await.is_some()); }
#[tokio::test]
async fn test_nintendolife_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("NintendoLife").expect("Entry not found").test("goku").await.is_some()); }
#[tokio::test]
async fn test_trakt_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Trakt").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_habr_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("habr").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_znanylekarz_pl_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("znanylekarz.pl").expect("Entry not found").test("janusz-nowak").await.is_some()); }
#[tokio::test]
async fn test_myspace_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Myspace").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_academia_edu_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Academia.edu").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_scratch_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Scratch").expect("Entry not found").test("griffpatch").await.is_some()); }
#[tokio::test]
async fn test_choice_community_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Choice Community").expect("Entry not found").test("gordon").await.is_some()); }
#[tokio::test]
async fn test_gravatar_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Gravatar").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_lushstories_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("LushStories").expect("Entry not found").test("chris_brown").await.is_some()); }
#[tokio::test]
async fn test_tweakers_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Tweakers").expect("Entry not found").test("femme").await.is_some()); }
#[tokio::test]
async fn test_sketchfab_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Sketchfab").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_osu_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("osu!").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_archive_of_our_own_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Archive of Our Own").expect("Entry not found").test("test").await.is_some()); }
#[tokio::test]
async fn test_ultimate_guitar_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Ultimate-Guitar").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_instructables_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Instructables").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_disqus_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Disqus").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_amino_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Amino").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_nextcloud_forum_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Nextcloud Forum").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_opensource_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Opensource").expect("Entry not found").test("red").await.is_some()); }
#[tokio::test]
async fn test_wolframalphaforum_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("WolframalphaForum").expect("Entry not found").test("unico").await.is_some()); }
#[tokio::test]
async fn test_modelhub_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Modelhub").expect("Entry not found").test("secretcrush").await.is_some()); }
#[tokio::test]
async fn test_note_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("note").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_jimdo_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Jimdo").expect("Entry not found").test("jenny").await.is_some()); }
#[tokio::test]
async fn test_vsco_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("VSCO").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_keakr_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("KEAKR").expect("Entry not found").test("beats").await.is_some()); }
#[tokio::test]
async fn test_spletnik_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("spletnik").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_xhamster_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("xHamster").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_goodreads_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("GoodReads").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_hackthebox_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("HackTheBox").expect("Entry not found").test("angar").await.is_some()); }
#[tokio::test]
async fn test_polarsteps_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Polarsteps").expect("Entry not found").test("james").await.is_some()); }
#[tokio::test]
async fn test_playstore_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("PlayStore").expect("Entry not found").test("Facebook").await.is_some()); }
#[tokio::test]
async fn test_tenor_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Tenor").expect("Entry not found").test("red").await.is_some()); }
#[tokio::test]
async fn test_mastodon_xyz_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("mastodon.xyz").expect("Entry not found").test("TheKinrar").await.is_some()); }
#[tokio::test]
async fn test_motorradfrage_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Motorradfrage").expect("Entry not found").test("gutefrage").await.is_some()); }
#[tokio::test]
async fn test_joplin_forum_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Joplin Forum").expect("Entry not found").test("laurent").await.is_some()); }
#[tokio::test]
async fn test_nightbot_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Nightbot").expect("Entry not found").test("green").await.is_some()); }
#[tokio::test]
async fn test_apple_developer_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Apple Developer").expect("Entry not found").test("lio24d").await.is_some()); }
#[tokio::test]
async fn test_ctan_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("CTAN").expect("Entry not found").test("briggs").await.is_some()); }
#[tokio::test]
async fn test_fosstodon_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Fosstodon").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_datingru_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("datingRU").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_bandcamp_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Bandcamp").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_7cups_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("7Cups").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_chaos_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Chaos").expect("Entry not found").test("ordnung").await.is_some()); }
#[tokio::test]
async fn test_buzzfeed_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("BuzzFeed").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_coroflot_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Coroflot").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_reisefrage_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Reisefrage").expect("Entry not found").test("reisefrage").await.is_some()); }
#[tokio::test]
async fn test_pokemon_showdown_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Pokemon Showdown").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_github_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("GitHub").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_swapd_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("SWAPD").expect("Entry not found").test("swapd").await.is_some()); }
#[tokio::test]
async fn test_snapchat_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Snapchat").expect("Entry not found").test("teamsnapchat").await.is_some()); }
#[tokio::test]
async fn test_wix_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Wix").expect("Entry not found").test("support").await.is_some()); }
#[tokio::test]
async fn test_wykop_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Wykop").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_codeforces_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Codeforces").expect("Entry not found").test("tourist").await.is_some()); }
#[tokio::test]

            async fn test_codeforces_unclaimed() {
                crate::REQWEST_CLIENT.get_or_init(crate::create_client);
                assert!(SITES.get("Codeforces").expect("Entry not found").test("noonewouldeverusethis7").await.is_none()); }
#[tokio::test]
async fn test_slant_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Slant").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_twitch_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("Twitch").expect("Entry not found").test("jenny").await.is_some()); }
#[tokio::test]
async fn test_akniga_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("akniga").expect("Entry not found").test("blue").await.is_some()); }
#[tokio::test]
async fn test_npm_claimed() { crate::REQWEST_CLIENT.get_or_init(crate::create_client); assert!(SITES.get("npm").expect("Entry not found").test("kennethsweezy").await.is_some()); }
