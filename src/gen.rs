#![allow(clippy::all)]
use crate::sites::SiteType;
use crate::sites::status::StatusSite;
use crate::sites::message::MessageSite;
pub(crate) static SITES: phf::Map<&'static str, SiteType> = ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 65),
        (0, 50),
        (0, 0),
        (0, 130),
        (0, 0),
        (1, 108),
        (0, 55),
        (2, 85),
        (0, 35),
        (0, 0),
        (0, 49),
        (0, 0),
        (0, 84),
        (0, 176),
        (1, 173),
        (0, 11),
        (1, 75),
        (0, 54),
        (1, 122),
        (0, 9),
        (4, 66),
        (0, 223),
        (0, 123),
        (1, 141),
        (0, 4),
        (13, 223),
        (3, 218),
        (0, 2),
        (1, 79),
        (0, 4),
        (1, 209),
        (0, 53),
        (0, 10),
        (0, 2),
        (11, 214),
        (37, 254),
        (1, 67),
        (0, 39),
        (0, 238),
        (0, 16),
        (0, 26),
        (0, 63),
        (1, 0),
        (4, 240),
        (2, 198),
        (0, 16),
        (13, 156),
        (0, 22),
        (1, 233),
        (0, 89),
        (0, 80),
        (0, 13),
        (0, 11),
    ],
    entries: &[
        ("RoyalCams", SiteType::StatusCode(StatusSite {
                    url: "https://royalcams.com/profile/{}"
                })),
        ("ShitpostBot5000", SiteType::StatusCode(StatusSite {
                    url: "https://www.shitpostbot.com/user/{}"
                })),
        ("Launchpad", SiteType::StatusCode(StatusSite {
                    url: "https://launchpad.net/~{}"
                })),
        ("Redbubble", SiteType::StatusCode(StatusSite {
                    url: "https://www.redbubble.com/people/{}"
                })),
        ("SublimeForum", SiteType::StatusCode(StatusSite {
                    url: "https://forum.sublimetext.com/u/{}"
                })),
        ("Trakt", SiteType::StatusCode(StatusSite {
                    url: "https://www.trakt.tv/users/{}"
                })),
        ("note", SiteType::StatusCode(StatusSite {
                    url: "https://note.com/{}"
                })),
        ("Etsy", SiteType::StatusCode(StatusSite {
                    url: "https://www.etsy.com/shop/{}"
                })),
        ("DEV Community", SiteType::StatusCode(StatusSite {
                    url: "https://dev.to/{}"
                })),
        ("Splice", SiteType::StatusCode(StatusSite {
                    url: "https://splice.com/{}"
                })),
        ("Giant Bomb", SiteType::StatusCode(StatusSite {
                    url: "https://www.giantbomb.com/profile/{}/"
                })),
        ("Flickr", SiteType::StatusCode(StatusSite {
                    url: "https://www.flickr.com/people/{}"
                })),
        ("fl", SiteType::StatusCode(StatusSite {
                    url: "https://www.fl.ru/users/{}"
                })),
        ("FortniteTracker", SiteType::StatusCode(StatusSite {
                    url: "https://fortnitetracker.com/profile/all/{}"
                })),
        ("Sportlerfrage", SiteType::StatusCode(StatusSite {
                    url: "https://www.sportlerfrage.net/nutzer/{}"
                })),
        ("Harvard Scholar", SiteType::StatusCode(StatusSite {
                    url: "https://scholar.harvard.edu/{}"
                })),
        ("social.tchncs.de", SiteType::StatusCode(StatusSite {
                    url: "https://social.tchncs.de/@{}"
                })),
        ("Discogs", SiteType::StatusCode(StatusSite {
                    url: "https://www.discogs.com/user/{}"
                })),
        ("Unsplash", SiteType::StatusCode(StatusSite {
                    url: "https://unsplash.com/@{}"
                })),
        ("drive2", SiteType::StatusCode(StatusSite {
                    url: "https://www.drive2.ru/users/{}"
                })),
        ("PromoDJ", SiteType::StatusCode(StatusSite {
                    url: "http://promodj.com/{}"
                })),
        ("TradingView", SiteType::StatusCode(StatusSite {
                    url: "https://www.tradingview.com/u/{}/"
                })),
        ("GitHub", SiteType::StatusCode(StatusSite {
                    url: "https://www.github.com/{}"
                })),
        ("spletnik", SiteType::StatusCode(StatusSite {
                    url: "https://spletnik.ru/user/{}"
                })),
        ("About.me", SiteType::StatusCode(StatusSite {
                    url: "https://about.me/{}"
                })),
        ("Jimdo", SiteType::StatusCode(StatusSite {
                    url: "https://{}.jimdosite.com"
                })),
        ("nnRU", SiteType::StatusCode(StatusSite {
                    url: "https://{}.www.nn.ru/"
                })),
        ("LOR", SiteType::StatusCode(StatusSite {
                    url: "https://www.linux.org.ru/people/{}/profile"
                })),
        ("Bitwarden Forum", SiteType::StatusCode(StatusSite {
                    url: "https://community.bitwarden.com/u/{}/summary"
                })),
        ("MixCloud", SiteType::StatusCode(StatusSite {
                    url: "https://www.mixcloud.com/{}/"
                })),
        ("Gesundheitsfrage", SiteType::StatusCode(StatusSite {
                    url: "https://www.gesundheitsfrage.net/nutzer/{}"
                })),
        ("mstdn.io", SiteType::StatusCode(StatusSite {
                    url: "https://mstdn.io/@{}"
                })),
        ("NotABug.org", SiteType::StatusCode(StatusSite {
                    url: "https://notabug.org/{}"
                })),
        ("eGPU", SiteType::StatusCode(StatusSite {
                    url: "https://egpu.io/forums/profile/{}/"
                })),
        ("mastodon.cloud", SiteType::StatusCode(StatusSite {
                    url: "https://mastodon.cloud/@{}"
                })),
        ("Codepen", SiteType::StatusCode(StatusSite {
                    url: "https://codepen.io/{}"
                })),
        ("Twitch", SiteType::StatusCode(StatusSite {
                    url: "https://www.twitch.tv/{}"
                })),
        ("Tellonym.me", SiteType::StatusCode(StatusSite {
                    url: "https://tellonym.me/{}"
                })),
        ("Bezuzyteczna", SiteType::StatusCode(StatusSite {
                    url: "https://bezuzyteczna.pl/uzytkownicy/{}"
                })),
        ("wiki.vg", SiteType::StatusCode(StatusSite {
                    url: "https://wiki.vg/User:{}"
                })),
        ("Airbit", SiteType::StatusCode(StatusSite {
                    url: "https://airbit.com/{}"
                })),
        ("F3.cool", SiteType::StatusCode(StatusSite {
                    url: "https://f3.cool/{}/"
                })),
        ("RubyGems", SiteType::StatusCode(StatusSite {
                    url: "https://rubygems.org/profiles/{}"
                })),
        ("moikrug", SiteType::StatusCode(StatusSite {
                    url: "https://moikrug.ru/{}"
                })),
        ("ThemeForest", SiteType::StatusCode(StatusSite {
                    url: "https://themeforest.net/user/{}"
                })),
        ("ChaturBate", SiteType::StatusCode(StatusSite {
                    url: "https://chaturbate.com/{}"
                })),
        ("Naver", SiteType::StatusCode(StatusSite {
                    url: "https://blog.naver.com/{}"
                })),
        ("osu!", SiteType::StatusCode(StatusSite {
                    url: "https://osu.ppy.sh/users/{}"
                })),
        ("Blogger", SiteType::StatusCode(StatusSite {
                    url: "https://{}.blogspot.com"
                })),
        ("LottieFiles", SiteType::StatusCode(StatusSite {
                    url: "https://lottiefiles.com/{}"
                })),
        ("leasehackr", SiteType::StatusCode(StatusSite {
                    url: "https://forum.leasehackr.com/u/{}/summary/"
                })),
        ("Sketchfab", SiteType::StatusCode(StatusSite {
                    url: "https://sketchfab.com/{}"
                })),
        ("Windy", SiteType::StatusCode(StatusSite {
                    url: "https://community.windy.com/user/{}"
                })),
        ("ArtStation", SiteType::StatusCode(StatusSite {
                    url: "https://www.artstation.com/{}"
                })),
        ("SlideShare", SiteType::StatusCode(StatusSite {
                    url: "https://slideshare.net/{}"
                })),
        ("WolframalphaForum", SiteType::StatusCode(StatusSite {
                    url: "https://community.wolfram.com/web/{}/home"
                })),
        ("nairaland.com", SiteType::StatusCode(StatusSite {
                    url: "https://www.nairaland.com/{}"
                })),
        ("Myspace", SiteType::StatusCode(StatusSite {
                    url: "https://myspace.com/{}"
                })),
        ("Rate Your Music", SiteType::StatusCode(StatusSite {
                    url: "https://rateyourmusic.com/~{}"
                })),
        ("pikabu", SiteType::StatusCode(StatusSite {
                    url: "https://pikabu.ru/@{}"
                })),
        ("chaos.social", SiteType::StatusCode(StatusSite {
                    url: "https://chaos.social/@{}"
                })),
        ("BuyMeACoffee", SiteType::StatusCode(StatusSite {
                    url: "https://buymeacoff.ee/{}"
                })),
        ("GoodReads", SiteType::StatusCode(StatusSite {
                    url: "https://www.goodreads.com/{}"
                })),
        ("VirusTotal", SiteType::StatusCode(StatusSite {
                    url: "https://www.virustotal.com/gui/user/{}"
                })),
        ("Shpock", SiteType::StatusCode(StatusSite {
                    url: "https://www.shpock.com/shop/{}/items"
                })),
        ("queer.af", SiteType::StatusCode(StatusSite {
                    url: "https://queer.af/@{}"
                })),
        ("LessWrong", SiteType::StatusCode(StatusSite {
                    url: "https://www.lesswrong.com/users/@{}"
                })),
        ("Replit.com", SiteType::StatusCode(StatusSite {
                    url: "https://replit.com/@{}"
                })),
        ("Championat", SiteType::StatusCode(StatusSite {
                    url: "https://www.championat.com/user/{}"
                })),
        ("Wykop", SiteType::StatusCode(StatusSite {
                    url: "https://www.wykop.pl/ludzie/{}"
                })),
        ("Vimeo", SiteType::StatusCode(StatusSite {
                    url: "https://vimeo.com/{}"
                })),
        ("Ask Fedora", SiteType::StatusCode(StatusSite {
                    url: "https://ask.fedoraproject.org/u/{}"
                })),
        ("Keybase", SiteType::StatusCode(StatusSite {
                    url: "https://keybase.io/{}"
                })),
        ("znanylekarz.pl", SiteType::StatusCode(StatusSite {
                    url: "https://www.znanylekarz.pl/{}"
                })),
        ("BLIP.fm", SiteType::StatusCode(StatusSite {
                    url: "https://blip.fm/{}"
                })),
        ("ICQ", SiteType::StatusCode(StatusSite {
                    url: "https://icq.im/{}/en"
                })),
        ("authorSTREAM", SiteType::StatusCode(StatusSite {
                    url: "http://www.authorstream.com/{}/"
                })),
        ("Choice Community", SiteType::StatusCode(StatusSite {
                    url: "https://choice.community/u/{}/summary"
                })),
        ("Weblate", SiteType::StatusCode(StatusSite {
                    url: "https://hosted.weblate.org/user/{}/"
                })),
        ("Tweakers", SiteType::StatusCode(StatusSite {
                    url: "https://tweakers.net/gallery/{}"
                })),
        ("Amino", SiteType::StatusCode(StatusSite {
                    url: "https://aminoapps.com/u/{}"
                })),
        ("Opensource", SiteType::StatusCode(StatusSite {
                    url: "https://opensource.com/users/{}"
                })),
        ("Alik.cz", SiteType::StatusCode(StatusSite {
                    url: "https://www.alik.cz/u/{}"
                })),
        ("mercadolivre", SiteType::StatusCode(StatusSite {
                    url: "https://www.mercadolivre.com.br/perfil/{}"
                })),
        ("Instructables", SiteType::StatusCode(StatusSite {
                    url: "https://www.instructables.com/member/{}"
                })),
        ("Codeforces", SiteType::StatusCode(StatusSite {
                    url: "https://codeforces.com/profile/{}"
                })),
        ("Docker Hub", SiteType::StatusCode(StatusSite {
                    url: "https://hub.docker.com/u/{}/"
                })),
        ("BiggerPockets", SiteType::StatusCode(StatusSite {
                    url: "https://www.biggerpockets.com/users/{}"
                })),
        ("Exposure", SiteType::StatusCode(StatusSite {
                    url: "https://{}.exposure.co/"
                })),
        ("TnAFlix", SiteType::StatusCode(StatusSite {
                    url: "https://www.tnaflix.com/profile/{}"
                })),
        ("pr0gramm", SiteType::StatusCode(StatusSite {
                    url: "https://pr0gramm.com/user/{}"
                })),
        ("CloudflareCommunity", SiteType::StatusCode(StatusSite {
                    url: "https://community.cloudflare.com/u/{}"
                })),
        ("uid", SiteType::StatusCode(StatusSite {
                    url: "http://uid.me/{}"
                })),
        ("CTAN", SiteType::StatusCode(StatusSite {
                    url: "https://ctan.org/author/{}"
                })),
        ("Xbox Gamertag", SiteType::StatusCode(StatusSite {
                    url: "https://xboxgamertag.com/search/{}"
                })),
        ("Car Talk Community", SiteType::StatusCode(StatusSite {
                    url: "https://community.cartalk.com/u/{}/summary"
                })),
        ("DeviantART", SiteType::StatusCode(StatusSite {
                    url: "https://{}.deviantart.com"
                })),
        ("Bookcrossing", SiteType::StatusCode(StatusSite {
                    url: "https://www.bookcrossing.com/mybookshelf/{}/"
                })),
        ("Coinvote", SiteType::StatusCode(StatusSite {
                    url: "https://coinvote.cc/profile/{}"
                })),
        ("PyPi", SiteType::StatusCode(StatusSite {
                    url: "https://pypi.org/user/{}"
                })),
        ("hackster", SiteType::StatusCode(StatusSite {
                    url: "https://www.hackster.io/{}"
                })),
        ("gfycat", SiteType::StatusCode(StatusSite {
                    url: "https://gfycat.com/@{}"
                })),
        ("LushStories", SiteType::StatusCode(StatusSite {
                    url: "https://www.lushstories.com/profile/{}"
                })),
        ("livelib", SiteType::StatusCode(StatusSite {
                    url: "https://www.livelib.ru/reader/{}"
                })),
        ("Warrior Forum", SiteType::StatusCode(StatusSite {
                    url: "https://www.warriorforum.com/members/{}.html"
                })),
        ("ImgUp.cz", SiteType::StatusCode(StatusSite {
                    url: "https://imgup.cz/{}"
                })),
        ("YouPic", SiteType::StatusCode(StatusSite {
                    url: "https://youpic.com/photographer/{}/"
                })),
        ("Behance", SiteType::StatusCode(StatusSite {
                    url: "https://www.behance.net/{}"
                })),
        ("Hackaday", SiteType::StatusCode(StatusSite {
                    url: "https://hackaday.io/{}"
                })),
        ("TrashboxRU", SiteType::StatusCode(StatusSite {
                    url: "https://trashbox.ru/users/{}"
                })),
        ("toster", SiteType::StatusCode(StatusSite {
                    url: "https://www.toster.ru/user/{}/answers"
                })),
        ("GitBook", SiteType::StatusCode(StatusSite {
                    url: "https://{}.gitbook.io/"
                })),
        ("Hashnode", SiteType::StatusCode(StatusSite {
                    url: "https://hashnode.com/@{}"
                })),
        ("Nightbot", SiteType::StatusCode(StatusSite {
                    url: "https://nightbot.tv/t/{}/commands"
                })),
        ("Modelhub", SiteType::StatusCode(StatusSite {
                    url: "https://www.modelhub.com/{}/videos"
                })),
        ("RedTube", SiteType::StatusCode(StatusSite {
                    url: "https://www.redtube.com/users/{}"
                })),
        ("kwork", SiteType::StatusCode(StatusSite {
                    url: "https://kwork.ru/user/{}"
                })),
        ("Slack", SiteType::StatusCode(StatusSite {
                    url: "https://{}.slack.com"
                })),
        ("YandexMusic", SiteType::StatusCode(StatusSite {
                    url: "https://music.yandex/users/{}/playlists"
                })),
        ("satsisRU", SiteType::StatusCode(StatusSite {
                    url: "https://satsis.info/user/{}"
                })),
        ("Autofrage", SiteType::StatusCode(StatusSite {
                    url: "https://www.autofrage.net/nutzer/{}"
                })),
        ("sessionize", SiteType::StatusCode(StatusSite {
                    url: "https://sessionize.com/{}"
                })),
        ("SourceForge", SiteType::StatusCode(StatusSite {
                    url: "https://sourceforge.net/u/{}"
                })),
        ("Asciinema", SiteType::StatusCode(StatusSite {
                    url: "https://asciinema.org/~{}"
                })),
        ("LiveJournal", SiteType::StatusCode(StatusSite {
                    url: "https://{}.livejournal.com"
                })),
        ("mastodon.social", SiteType::StatusCode(StatusSite {
                    url: "https://mastodon.social/@{}"
                })),
        ("Jellyfin Weblate", SiteType::StatusCode(StatusSite {
                    url: "https://translate.jellyfin.org/user/{}/"
                })),
        ("geocaching", SiteType::StatusCode(StatusSite {
                    url: "https://www.geocaching.com/p/default.aspx?u={}"
                })),
        ("Codewars", SiteType::StatusCode(StatusSite {
                    url: "https://www.codewars.com/users/{}"
                })),
        ("OpenStreetMap", SiteType::StatusCode(StatusSite {
                    url: "https://www.openstreetmap.org/user/{}"
                })),
        ("Caddy Community", SiteType::StatusCode(StatusSite {
                    url: "https://caddy.community/u/{}/summary"
                })),
        ("Gitee", SiteType::StatusCode(StatusSite {
                    url: "https://gitee.com/{}"
                })),
        ("Academia.edu", SiteType::StatusCode(StatusSite {
                    url: "https://independent.academia.edu/{}"
                })),
        ("Crowdin", SiteType::StatusCode(StatusSite {
                    url: "https://crowdin.com/profile/{}"
                })),
        ("MyMiniFactory", SiteType::StatusCode(StatusSite {
                    url: "https://www.myminifactory.com/users/{}"
                })),
        ("OGUsers", SiteType::StatusCode(StatusSite {
                    url: "https://ogu.gg/{}"
                })),
        ("Genius (Artists)", SiteType::StatusCode(StatusSite {
                    url: "https://genius.com/artists/{}"
                })),
        ("Memrise", SiteType::StatusCode(StatusSite {
                    url: "https://www.memrise.com/user/{}/"
                })),
        ("last.fm", SiteType::StatusCode(StatusSite {
                    url: "https://last.fm/user/{}"
                })),
        ("Envato Forum", SiteType::StatusCode(StatusSite {
                    url: "https://forums.envato.com/u/{}"
                })),
        ("Fosstodon", SiteType::StatusCode(StatusSite {
                    url: "https://fosstodon.org/@{}"
                })),
        ("jbzd.com.pl", SiteType::StatusCode(StatusSite {
                    url: "https://jbzd.com.pl/uzytkownik/{}"
                })),
        ("Gutefrage", SiteType::StatusCode(StatusSite {
                    url: "https://www.gutefrage.net/nutzer/{}"
                })),
        ("Discuss.Elastic.co", SiteType::StatusCode(StatusSite {
                    url: "https://discuss.elastic.co/u/{}"
                })),
        ("Ionic Forum", SiteType::StatusCode(StatusSite {
                    url: "https://forum.ionicframework.com/u/{}"
                })),
        ("VSCO", SiteType::StatusCode(StatusSite {
                    url: "https://vsco.co/{}"
                })),
        ("Gravatar", SiteType::StatusCode(StatusSite {
                    url: "http://en.gravatar.com/{}"
                })),
        ("xHamster", SiteType::StatusCode(StatusSite {
                    url: "https://xhamster.com/users/{}"
                })),
        ("Archive of Our Own", SiteType::StatusCode(StatusSite {
                    url: "https://archiveofourown.org/users/{}"
                })),
        ("couchsurfing", SiteType::StatusCode(StatusSite {
                    url: "https://www.couchsurfing.com/people/{}"
                })),
        ("Pornhub", SiteType::StatusCode(StatusSite {
                    url: "https://pornhub.com/users/{}"
                })),
        ("Linktree", SiteType::StatusCode(StatusSite {
                    url: "https://linktr.ee/{}"
                })),
        ("WebNode", SiteType::StatusCode(StatusSite {
                    url: "https://{}.webnode.cz/"
                })),
        ("Polygon", SiteType::StatusCode(StatusSite {
                    url: "https://www.polygon.com/users/{}"
                })),
        ("Tuna", SiteType::StatusCode(StatusSite {
                    url: "https://tuna.voicemod.net/user/{}"
                })),
        ("SportsRU", SiteType::StatusCode(StatusSite {
                    url: "https://www.sports.ru/profile/{}/"
                })),
        ("mastodon.technology", SiteType::StatusCode(StatusSite {
                    url: "https://mastodon.technology/@{}"
                })),
        ("YouPorn", SiteType::StatusCode(StatusSite {
                    url: "https://youporn.com/uservids/{}"
                })),
        ("svidbook", SiteType::StatusCode(StatusSite {
                    url: "https://www.svidbook.ru/user/{}"
                })),
        ("Ultimate-Guitar", SiteType::StatusCode(StatusSite {
                    url: "https://ultimate-guitar.com/u/{}"
                })),
        ("KEAKR", SiteType::StatusCode(StatusSite {
                    url: "https://www.keakr.com/en/profile/{}"
                })),
        ("skyrock", SiteType::StatusCode(StatusSite {
                    url: "https://{}.skyrock.com/"
                })),
        ("Imgur", SiteType::StatusCode(StatusSite {
                    url: "https://imgur.com/user/{}"
                })),
        ("Audiojungle", SiteType::StatusCode(StatusSite {
                    url: "https://audiojungle.net/user/{}"
                })),
        ("Anilist", SiteType::StatusCode(StatusSite {
                    url: "https://anilist.co/user/{}/"
                })),
        ("Newgrounds", SiteType::StatusCode(StatusSite {
                    url: "https://{}.newgrounds.com"
                })),
        ("akniga", SiteType::StatusCode(StatusSite {
                    url: "https://akniga.org/profile/{}"
                })),
        ("Airliners", SiteType::StatusCode(StatusSite {
                    url: "https://www.airliners.net/user/{}/profile/photos"
                })),
        ("Coderwall", SiteType::StatusCode(StatusSite {
                    url: "https://coderwall.com/{}"
                })),
        ("Gamespot", SiteType::StatusCode(StatusSite {
                    url: "https://www.gamespot.com/profile/{}/"
                })),
        ("datingRU", SiteType::StatusCode(StatusSite {
                    url: "http://dating.ru/{}"
                })),
        ("Scratch", SiteType::StatusCode(StatusSite {
                    url: "https://scratch.mit.edu/users/{}"
                })),
        ("npm", SiteType::StatusCode(StatusSite {
                    url: "https://www.npmjs.com/~{}"
                })),
        ("Patreon", SiteType::StatusCode(StatusSite {
                    url: "https://www.patreon.com/{}"
                })),
        ("BuzzFeed", SiteType::StatusCode(StatusSite {
                    url: "https://buzzfeed.com/{}"
                })),
        ("mastodon.xyz", SiteType::StatusCode(StatusSite {
                    url: "https://mastodon.xyz/@{}"
                })),
        ("eintracht", SiteType::StatusCode(StatusSite {
                    url: "https://community.eintracht.de/fans/{}"
                })),
        ("Lobsters", SiteType::StatusCode(StatusSite {
                    url: "https://lobste.rs/u/{}"
                })),
        ("HackTheBox", SiteType::StatusCode(StatusSite {
                    url: "https://forum.hackthebox.eu/profile/{}"
                })),
        ("Pokemon Showdown", SiteType::StatusCode(StatusSite {
                    url: "https://pokemonshowdown.com/users/{}"
                })),
        ("Chaos", SiteType::StatusCode(StatusSite {
                    url: "https://chaos.social/@{}"
                })),
        ("Polarsteps", SiteType::StatusCode(StatusSite {
                    url: "https://polarsteps.com/{}"
                })),
        ("EyeEm", SiteType::StatusCode(StatusSite {
                    url: "https://www.eyeem.com/u/{}"
                })),
        ("habr", SiteType::StatusCode(StatusSite {
                    url: "https://habr.com/ru/users/{}"
                })),
        ("Weebly", SiteType::StatusCode(StatusSite {
                    url: "https://{}.weebly.com/"
                })),
        ("Monkeytype", SiteType::StatusCode(StatusSite {
                    url: "https://monkeytype.com/profile/{}"
                })),
        ("Slides", SiteType::StatusCode(StatusSite {
                    url: "https://slides.com/{}"
                })),
        ("Finanzfrage", SiteType::StatusCode(StatusSite {
                    url: "https://www.finanzfrage.net/nutzer/{}"
                })),
        ("Giphy", SiteType::StatusCode(StatusSite {
                    url: "https://giphy.com/{}"
                })),
        ("Itch.io", SiteType::StatusCode(StatusSite {
                    url: "https://{}.itch.io/"
                })),
        ("Apple Developer", SiteType::StatusCode(StatusSite {
                    url: "https://developer.apple.com/forums/profile/{}"
                })),
        ("Snapchat", SiteType::StatusCode(StatusSite {
                    url: "https://www.snapchat.com/add/{}"
                })),
        ("NintendoLife", SiteType::StatusCode(StatusSite {
                    url: "https://www.nintendolife.com/users/{}"
                })),
        ("LeetCode", SiteType::StatusCode(StatusSite {
                    url: "https://leetcode.com/{}"
                })),
        ("Nyaa.si", SiteType::StatusCode(StatusSite {
                    url: "https://nyaa.si/user/{}"
                })),
        ("Tenor", SiteType::StatusCode(StatusSite {
                    url: "https://tenor.com/users/{}"
                })),
        ("Joplin Forum", SiteType::StatusCode(StatusSite {
                    url: "https://discourse.joplinapp.org/u/{}"
                })),
        ("Sbazar.cz", SiteType::StatusCode(StatusSite {
                    url: "https://www.sbazar.cz/{}"
                })),
        ("Oracle Community", SiteType::StatusCode(StatusSite {
                    url: "https://community.oracle.com/people/{}"
                })),
        ("Sporcle", SiteType::StatusCode(StatusSite {
                    url: "https://www.sporcle.com/user/{}/people"
                })),
        ("9GAG", SiteType::StatusCode(StatusSite {
                    url: "https://www.9gag.com/u/{}"
                })),
        ("Cryptomator Forum", SiteType::StatusCode(StatusSite {
                    url: "https://community.cryptomator.org/u/{}"
                })),
        ("Gradle", SiteType::StatusCode(StatusSite {
                    url: "https://plugins.gradle.org/u/{}"
                })),
        ("TLDR Legal", SiteType::StatusCode(StatusSite {
                    url: "https://tldrlegal.com/users/{}/"
                })),
        ("TRAKTRAIN", SiteType::StatusCode(StatusSite {
                    url: "https://traktrain.com/{}"
                })),
        ("PlayStore", SiteType::StatusCode(StatusSite {
                    url: "https://play.google.com/store/apps/developer?id={}"
                })),
        ("Xvideos", SiteType::StatusCode(StatusSite {
                    url: "https://xvideos.com/profiles/{}"
                })),
        ("irecommend", SiteType::StatusCode(StatusSite {
                    url: "https://irecommend.ru/users/{}"
                })),
        ("Rumble", SiteType::StatusCode(StatusSite {
                    url: "https://rumble.com/user/{}"
                })),
        ("Fameswap", SiteType::StatusCode(StatusSite {
                    url: "https://fameswap.com/user/{}"
                })),
        ("BongaCams", SiteType::StatusCode(StatusSite {
                    url: "https://pt.bongacams.com/profile/{}"
                })),
        ("Kaggle", SiteType::StatusCode(StatusSite {
                    url: "https://www.kaggle.com/{}"
                })),
        ("Pinkbike", SiteType::StatusCode(StatusSite {
                    url: "https://www.pinkbike.com/u/{}/"
                })),
        ("Wix", SiteType::StatusCode(StatusSite {
                    url: "https://{}.wix.com"
                })),
        ("HubPages", SiteType::StatusCode(StatusSite {
                    url: "https://hubpages.com/@{}"
                })),
        ("Bikemap", SiteType::StatusCode(StatusSite {
                    url: "https://www.bikemap.net/en/u/{}/routes/created/"
                })),
        ("Needrom", SiteType::StatusCode(StatusSite {
                    url: "https://www.needrom.com/author/{}/"
                })),
        ("Flightradar24", SiteType::StatusCode(StatusSite {
                    url: "https://my.flightradar24.com/{}"
                })),
        ("Nextcloud Forum", SiteType::StatusCode(StatusSite {
                    url: "https://help.nextcloud.com/u/{}/summary"
                })),
        ("Rclone Forum", SiteType::StatusCode(StatusSite {
                    url: "https://forum.rclone.org/u/{}"
                })),
        ("Coroflot", SiteType::StatusCode(StatusSite {
                    url: "https://www.coroflot.com/{}"
                })),
        ("BioHacking", SiteType::StatusCode(StatusSite {
                    url: "https://forum.dangerousthings.com/u/{}"
                })),
        ("ColourLovers", SiteType::StatusCode(StatusSite {
                    url: "https://www.colourlovers.com/lover/{}"
                })),
        ("Vero", SiteType::StatusCode(StatusSite {
                    url: "https://vero.co/{}"
                })),
        ("d3RU", SiteType::StatusCode(StatusSite {
                    url: "https://d3.ru/user/{}/posts"
                })),
        ("MyAnimeList", SiteType::StatusCode(StatusSite {
                    url: "https://myanimelist.net/profile/{}"
                })),
        ("Motorradfrage", SiteType::StatusCode(StatusSite {
                    url: "https://www.motorradfrage.net/nutzer/{}"
                })),
        ("BitBucket", SiteType::StatusCode(StatusSite {
                    url: "https://bitbucket.org/{}/"
                })),
        ("SoundCloud", SiteType::StatusCode(StatusSite {
                    url: "https://soundcloud.com/{}"
                })),
        ("Crevado", SiteType::StatusCode(StatusSite {
                    url: "https://{}.crevado.com"
                })),
        ("Genius (Users)", SiteType::StatusCode(StatusSite {
                    url: "https://genius.com/{}"
                })),
        ("Splits.io", SiteType::StatusCode(StatusSite {
                    url: "https://splits.io/users/{}"
                })),
        ("Spotify", SiteType::StatusCode(StatusSite {
                    url: "https://open.spotify.com/user/{}"
                })),
        ("Wattpad", SiteType::StatusCode(StatusSite {
                    url: "https://www.wattpad.com/user/{}"
                })),
        ("DailyMotion", SiteType::StatusCode(StatusSite {
                    url: "https://www.dailymotion.com/{}"
                })),
        ("Eintracht Frankfurt Forum", SiteType::StatusCode(StatusSite {
                    url: "https://community.eintracht.de/fans/{}"
                })),
        ("fixya", SiteType::StatusCode(StatusSite {
                    url: "https://www.fixya.com/users/{}"
                })),
        ("7Cups", SiteType::StatusCode(StatusSite {
                    url: "https://www.7cups.com/@{}"
                })),
        ("BraveCommunity", SiteType::StatusCode(StatusSite {
                    url: "https://community.brave.com/u/{}/"
                })),
        ("WICG Forum", SiteType::StatusCode(StatusSite {
                    url: "https://discourse.wicg.io/u/{}/summary"
                })),
        ("Bandcamp", SiteType::StatusCode(StatusSite {
                    url: "https://www.bandcamp.com/{}"
                })),
        ("Minecraft", SiteType::StatusCode(StatusSite {
                    url: "https://api.mojang.com/users/profiles/minecraft/{}"
                })),
        ("SmugMug", SiteType::StatusCode(StatusSite {
                    url: "https://{}.smugmug.com"
                })),
        ("Reisefrage", SiteType::StatusCode(StatusSite {
                    url: "https://www.reisefrage.net/nutzer/{}"
                })),
        ("Flipboard", SiteType::StatusCode(StatusSite {
                    url: "https://flipboard.com/@{}"
                })),
        ("Fandom", SiteType::StatusCode(StatusSite {
                    url: "https://www.fandom.com/u/{}"
                })),
        ("MMORPG Forum", SiteType::StatusCode(StatusSite {
                    url: "https://forums.mmorpg.com/profile/{}"
                })),
        ("Intigriti", SiteType::StatusCode(StatusSite {
                    url: "https://app.intigriti.com/profile/{}"
                })),
        ("Trawelling", SiteType::StatusCode(StatusSite {
                    url: "https://traewelling.de/@{}"
                })),
        ("Periscope", SiteType::StatusCode(StatusSite {
                    url: "https://www.periscope.tv/{}/"
                })),
        ("Erome", SiteType::StatusCode(StatusSite {
                    url: "https://www.erome.com/{}"
                })),
        ("Disqus", SiteType::StatusCode(StatusSite {
                    url: "https://disqus.com/{}"
                })),
        ("Icons8 Community", SiteType::StatusCode(StatusSite {
                    url: "https://community.icons8.com/u/{}/summary"
                })),
        ("wykop.pl", SiteType::StatusCode(StatusSite {
                    url: "https://www.wykop.pl/ludzie/{}"
                })),
        ("Freesound", SiteType::StatusCode(StatusSite {
                    url: "https://freesound.org/people/{}/"
                })),
        ("GetMyUni", SiteType::StatusCode(StatusSite {
                    url: "https://www.getmyuni.com/user/{}"
                })),
        ("Issuu", SiteType::StatusCode(StatusSite {
                    url: "https://issuu.com/{}"
                })),
        ("Clubhouse", SiteType::StatusCode(StatusSite {
                    url: "https://www.clubhouse.com/@{}"
                })),
        ("Clapper", SiteType::StatusCode(StatusSite {
                    url: "https://clapperapp.com/{}"
                })),
        ("Rajce.net", SiteType::StatusCode(StatusSite {
                    url: "https://{}.rajce.idnes.cz/"
                })),
        ("Slant", SiteType::StatusCode(StatusSite {
                    url: "https://www.slant.co/users/{}"
                })),
        ("2Dimensions", SiteType::StatusCode(StatusSite {
                    url: "https://2Dimensions.com/a/{}"
                })),
        ("SWAPD", SiteType::StatusCode(StatusSite {
                    url: "https://swapd.co/u/{}"
                })),
    ],
};