use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UnitInfo {
    pub section: u8,
    pub unit: u8,
    pub name: &'static str,
    pub description: &'static str,
    pub emoji: &'static str,
}

pub static UNITS: &[UnitInfo] = &[
    // ── Section 1 · CEFR Intro ──────────────────────────────────────────────
    UnitInfo { section: 1, unit:  1, name: "Cafe",      description: "Order at a cafe",                     emoji: "☕" },
    UnitInfo { section: 1, unit:  2, name: "Restaurant", description: "Order food and drinks",               emoji: "🍽️" },
    UnitInfo { section: 1, unit:  3, name: "Belongings", description: "Use possessive pronouns",             emoji: "🎒" },
    UnitInfo { section: 1, unit:  4, name: "Intros",     description: "Introduce yourself and others",       emoji: "👋" },
    UnitInfo { section: 1, unit:  5, name: "Age",        description: "Talk about age",                      emoji: "🔢" },
    UnitInfo { section: 1, unit:  6, name: "Matjib",     description: "Ask questions about food",            emoji: "🍲" },
    UnitInfo { section: 1, unit:  7, name: "Birthday",   description: "Celebrate a birthday",                emoji: "🎂" },
    UnitInfo { section: 1, unit:  8, name: "Airport",    description: "Find your way at the airport",        emoji: "✈️" },
    UnitInfo { section: 1, unit:  9, name: "Hobbies",    description: "Talk about your hobbies",             emoji: "🎨" },
    UnitInfo { section: 1, unit: 10, name: "Game Day",   description: "Enjoy a sports event",                emoji: "⚽" },
    // ── Section 2 · CEFR A1 ─────────────────────────────────────────────────
    UnitInfo { section: 2, unit:  1, name: "Travel",     description: "Ask for directions",                  emoji: "🗺️" },
    UnitInfo { section: 2, unit:  2, name: "Hometown",   description: "Talk about your hometown",            emoji: "🏡" },
    UnitInfo { section: 2, unit:  3, name: "Camping",    description: "Discuss the weather",                 emoji: "⛺" },
    UnitInfo { section: 2, unit:  4, name: "New House",  description: "Look for a new home",                 emoji: "🏢" },
    UnitInfo { section: 2, unit:  5, name: "Blind Date", description: "Introduce new friends",               emoji: "💑" },
    UnitInfo { section: 2, unit:  6, name: "Incheon",    description: "Use location and destination markers", emoji: "📍" },
    UnitInfo { section: 2, unit:  7, name: "At Home",    description: "Describe your home",                  emoji: "🛋️" },
    UnitInfo { section: 2, unit:  8, name: "Christmas",  description: "Celebrate Christmas",                 emoji: "🎄" },
    UnitInfo { section: 2, unit:  9, name: "Concert",    description: "Use the present tense",               emoji: "🎵" },
    UnitInfo { section: 2, unit: 10, name: "Theatre",    description: "Enjoy a movie",                       emoji: "🎬" },
    UnitInfo { section: 2, unit: 11, name: "Library",    description: "Visit the library",                   emoji: "📚" },
    UnitInfo { section: 2, unit: 12, name: "Transport",  description: "Navigate local transport",            emoji: "🚌" },
    UnitInfo { section: 2, unit: 13, name: "Market",     description: "Use adjectives to describe nouns",    emoji: "🛒" },
    UnitInfo { section: 2, unit: 14, name: "Doctor",     description: "Discuss health concerns",             emoji: "🏥" },
    UnitInfo { section: 2, unit: 15, name: "On Campus",  description: "Navigate campus life",                emoji: "🎓" },
    UnitInfo { section: 2, unit: 16, name: "Shopping",   description: "Shop for clothes and items",          emoji: "👗" },
    UnitInfo { section: 2, unit: 17, name: "Cool Pool",  description: "Use the imperative mood",             emoji: "🏊" },
    UnitInfo { section: 2, unit: 18, name: "New Job",    description: "Use formal commands",                 emoji: "💼" },
    UnitInfo { section: 2, unit: 19, name: "Chuseok",    description: "Plan for Chuseok",                    emoji: "🎑" },
    UnitInfo { section: 2, unit: 20, name: "Volunteer",  description: "Use the future tense",                emoji: "🤝" },
    UnitInfo { section: 2, unit: 21, name: "Picky Kids", description: "Express preferences",                 emoji: "🍔" },
    UnitInfo { section: 2, unit: 22, name: "Excuses",    description: "Explain why you can't do something",  emoji: "🙅" },
    UnitInfo { section: 2, unit: 23, name: "Take-out",   description: "Use the past tense",                  emoji: "🥡" },
    UnitInfo { section: 2, unit: 24, name: "Help",       description: "Check on someone's well-being",       emoji: "🆘" },
    UnitInfo { section: 2, unit: 25, name: "PC Room",    description: "Talk about games",                    emoji: "🎮" },
    UnitInfo { section: 2, unit: 26, name: "Olympics",   description: "Use time expressions",                emoji: "🏅" },
    UnitInfo { section: 2, unit: 27, name: "Exams",      description: "Discuss exams and study habits",      emoji: "📝" },
    UnitInfo { section: 2, unit: 28, name: "Climate",    description: "Talk about the weather",              emoji: "🌡️" },
];

// ── Word lists per unit ─────────────────────────────────────────────────────
// Each word (Korean string) is assigned to exactly one unit.
// Words not listed here are accessible only through the "By Type" category view.

const S1U01: &[&str] = &[
    "빵", "디저트", "샌드위치", "콜라", "주스", "버블티", "커피", "차", "물", "우유",
    "카페", "잔", "마시다", "주문하다", "주세요", "드릴까요", "감사합니다",
];
const S1U02: &[&str] = &[
    "짜장면", "김치찌개", "비빔밥", "김치", "밥", "음식", "파스타", "식당",
    "숟가락", "젓가락", "영수증", "메뉴판", "웨이터", "시키다", "먹다", "앉다",
    "잘 먹었습니다", "잘 먹겠습니다", "또 오세요",
];
const S1U03: &[&str] = &[
    "가방", "지갑", "책", "핸드폰", "안경", "연필", "필통", "공책", "볼펜",
    "것", "나", "저", "우리", "의", "이것", "저것", "을", "를", "없다", "있다", "있어요",
];
const S1U04: &[&str] = &[
    "이름", "사람", "친구", "한국", "미국", "중국", "한국어", "영어",
    "선생님", "학생", "씨", "안녕하세요", "반가워요", "인사하다",
    "하다", "이다", "아니다", "잘 가요", "또 봐요",
    "은", "는", "이에요", "예요", "가", "이", "네", "아니요", "아", "잘",
];
const S1U05: &[&str] = &[
    "살", "열", "열둘", "열다섯", "열아홉", "서른", "넷", "둘",
    "삼", "다섯", "여섯", "몇", "일", "반", "시", "세다",
];
const S1U06: &[&str] = &[
    "맛집", "한식당", "음료수", "맥주", "고기", "과일", "찌개", "삼계탕",
    "된장찌개", "갈비탕", "냉면", "갈비", "단무지",
    "맵다", "달다", "신선하다", "맛있다", "뭐", "어떤", "얼마", "어느",
    "배고파요", "음",
];
const S1U07: &[&str] = &[
    "생일", "생신", "케이크", "선물", "초콜릿", "시계", "목걸이",
    "여동생", "오늘", "달",
    "생일 축하해요", "특별하다", "기쁘다", "받다", "주다", "드리다", "우와",
];
const S1U08: &[&str] = &[
    "공항", "여권", "게이트", "출구", "표", "화장실", "체크인", "출발",
    "이쪽", "저쪽", "오른쪽", "저기", "위", "어디", "근처",
    "찾다", "확인하다", "실례합니다",
];
const S1U09: &[&str] = &[
    "취미", "독서", "운동", "등산", "피아노", "드럼", "카메라",
    "노래방", "그림", "사진",
    "운동하다", "산책하다", "치다", "그리다", "좋아하다", "좋다", "가끔",
];
const S1U10: &[&str] = &[
    "야구", "축구", "경기", "경기장", "팀", "공", "선수", "팬",
    "테니스", "운동장", "자리", "탁구", "농구", "달리기",
    "이기다", "재미있다", "멋있다", "화이팅", "와",
];

const S2U01: &[&str] = &[
    "여행", "여행사", "제주도", "엽서", "지도", "우체국", "봉투",
    "길", "공원", "박물관", "태국", "바다",
    "가깝다", "멀다", "가까운", "올라가다", "알려주다", "찍다", "가다", "걷다",
];
const S2U02: &[&str] = &[
    "고향", "서울", "부산", "도시", "동네", "이웃", "집",
    "사촌", "형", "아빠", "엄마", "오빠", "누나", "언니",
    "남동생", "부모님", "가족", "할머니", "할아버지", "이모", "아들", "딸",
    "익숙하다", "외국어", "프랑스", "인도네시아", "일본",
];
const S2U03: &[&str] = &[
    "날씨", "비", "하늘", "바람", "우산", "모기", "나무", "산", "호수",
    "소풍", "꽃", "캠핑", "텐트", "불",
    "덥다", "맑다", "아름답다", "보통", "밖",
];
const S2U04: &[&str] = &[
    "아파트", "방", "창문", "크기", "문", "열쇠", "마당", "바닥",
    "층", "계단", "엘리베이터", "건물", "주소",
    "크다", "작다", "넓다", "높다", "생기다", "열다",
];
const S2U05: &[&str] = &[
    "소개", "소개팅", "데이트", "남자 친구", "미소", "오랜만", "말",
    "랑", "이랑", "누구", "누가",
    "예쁘다", "친절하다", "친한", "어울리다", "대화하다", "만나다", "알다", "말하다",
    "같이", "마음에 들어요", "어때요", "정말요",
];
const S2U06: &[&str] = &[
    "인천", "강남", "시내", "전철역", "곳", "번호",
    "에서", "으로", "로", "에", "서", "으러", "러", "도",
];
const S2U07: &[&str] = &[
    "침실", "책상", "거실", "부엌", "침대", "테이블", "소파",
    "텔레비전", "에어컨", "세탁기", "냉장고", "선풍기", "비누", "의자", "쓰레기통",
    "깨끗하다", "끄다", "닫다", "넣다", "걸다", "편하게",
];
const S2U08: &[&str] = &[
    "크리스마스", "파티",
    "메리 크리스마스", "환영해요", "고맙다", "오다", "먼저", "특별히", "또",
];
const S2U09: &[&str] = &[
    "콘서트", "랩", "케이팝", "재즈", "무대", "가수", "걸그룹", "음악", "노래", "장소",
    "유명하다", "화려하다", "부르다", "그러면",
];
const S2U10: &[&str] = &[
    "영화", "영화관", "배우", "액션", "드라마", "멜로", "팝콘",
    "이상하다", "무섭다", "웃긴", "보이다", "보다", "진짜", "정말",
];
const S2U11: &[&str] = &[
    "도서관", "소설책", "신문", "만화", "잡지", "사전", "서점", "작가", "글",
    "읽다", "빌리다", "조용하다", "조용히",
];
const S2U12: &[&str] = &[
    "버스", "택시", "지하철", "역", "정류장", "기차",
    "자동차", "자전거", "터미널", "헬멧", "편의점", "교통", "운전", "주차",
    "운전하다", "타다", "복잡하다", "느리다", "천천히",
];
const S2U13: &[&str] = &[
    "시장", "가게", "손님", "물건", "아주머니", "색",
    "포도", "바나나", "사과", "딸기", "귤", "멜론",
    "많다", "무겁다", "팔다", "조금", "많이",
];
const S2U14: &[&str] = &[
    "반창고", "손", "발", "귀", "몸", "배", "머리", "다리", "감기",
    "약사", "약국", "상처", "병", "약", "증상", "병원", "간호사", "의사",
    "불편하다", "괜찮다", "아프다", "치료하다", "검사하다", "걸리다", "낫다",
];
const S2U15: &[&str] = &[
    "초등학교", "교실", "칠판", "기숙사", "노트북",
    "수업", "전공", "학교", "동아리", "대학생", "학생증", "교수", "볼펜",
    "들어가다",
];
const S2U16: &[&str] = &[
    "구두", "치마", "바지", "백화점", "모자", "옷", "운동화",
    "가격", "할인", "돈", "카드", "쇼핑", "마트", "티셔츠",
    "싸다", "비싸다", "사다",
];
const S2U17: &[&str] = &[
    "수영장", "수건", "빙수",
    "수영하다", "닦다", "청소하다", "빨리", "깨끗이",
];
const S2U18: &[&str] = &[
    "회의", "서류", "사무실", "부장님", "회사", "직원", "회사원",
    "서비스", "복사기", "이메일",
    "붙이다", "정리하다", "적다", "간단히", "변호사",
    "수고하셨습니다",
];
const S2U19: &[&str] = &[
    "추석", "한복", "댁", "윷놀이", "떡", "잡채", "이분",
    "만들다",
];
const S2U20: &[&str] = &[
    "강아지", "고양이", "동물", "화가", "내일",
    "가르치다", "되다", "보내다",
    "꼭", "친절히", "다", "이제",
];
const S2U21: &[&str] = &[
    "떡볶이", "두부", "과자", "사탕", "간식", "아이스크림",
    "아이", "아기", "장난감",
    "이나", "단", "너무", "별로",
    "고르다", "싫어하다",
];
const S2U22: &[&str] = &[
    "걱정", "잠", "약속",
    "그런데", "안", "못",
    "바꾸다", "걱정하다", "고치다", "남다", "고장나다",
    "게으르다", "피곤하다", "한가하다",
    "필요 없어요", "늦었어요", "고장났어요",
];
const S2U23: &[&str] = &[
    "김밥", "라면", "만두", "치킨",
    "포장하다",
];
const S2U24: &[&str] = &[
    "메시지", "문자",
    "전화하다", "조심하다", "넘어지다", "도와주다",
    "필요하다", "바로",
    "죄송해요", "어떡해요", "이런", "조심해요",
];
const S2U25: &[&str] = &[
    "캐릭터", "레벨", "피씨방", "컴퓨터", "인터넷", "게임",
];
const S2U26: &[&str] = &[
    "올림픽", "선수", "태권도", "이번",
    "때", "날", "시간", "다음", "주",
    "수요일", "금요일", "토요일", "화요일", "삼월",
    "오후", "올해", "어제", "작년", "방금", "아까",
    "언제", "빠르다", "요즘", "지금",
    "벌써", "아직", "매주", "자주",
];
const S2U27: &[&str] = &[
    "과학", "숙제", "교과서", "수첩", "시험", "단어", "연습",
    "설명하다", "공부하다", "기억하다", "쓰다", "끝나다", "긴장하다",
    "어렵다", "궁금하다",
    "잘하네요", "매일", "항상", "열심히",
];
const S2U28: &[&str] = &[
    "밤", "눈", "아침", "기온", "겨울", "가을", "하늘",
    "뜨겁다", "춥다", "따뜻하다", "어둡다",
    "내리다", "불다",
    "따뜻하게", "덜", "더",
];

static UNIT_WORDS: &[((u8, u8), &[&str])] = &[
    ((1,  1), S1U01), ((1,  2), S1U02), ((1,  3), S1U03), ((1,  4), S1U04),
    ((1,  5), S1U05), ((1,  6), S1U06), ((1,  7), S1U07), ((1,  8), S1U08),
    ((1,  9), S1U09), ((1, 10), S1U10),
    ((2,  1), S2U01), ((2,  2), S2U02), ((2,  3), S2U03), ((2,  4), S2U04),
    ((2,  5), S2U05), ((2,  6), S2U06), ((2,  7), S2U07), ((2,  8), S2U08),
    ((2,  9), S2U09), ((2, 10), S2U10), ((2, 11), S2U11), ((2, 12), S2U12),
    ((2, 13), S2U13), ((2, 14), S2U14), ((2, 15), S2U15), ((2, 16), S2U16),
    ((2, 17), S2U17), ((2, 18), S2U18), ((2, 19), S2U19), ((2, 20), S2U20),
    ((2, 21), S2U21), ((2, 22), S2U22), ((2, 23), S2U23), ((2, 24), S2U24),
    ((2, 25), S2U25), ((2, 26), S2U26), ((2, 27), S2U27), ((2, 28), S2U28),
];

pub fn word_unit(korean: &str) -> Option<(u8, u8)> {
    for &((section, unit), words) in UNIT_WORDS {
        if words.contains(&korean) {
            return Some((section, unit));
        }
    }
    None
}

pub fn unit_key(section: u8, unit: u8) -> String {
    format!("s{}u{:02}", section, unit)
}

pub fn parse_unit_key(key: &str) -> Option<(u8, u8)> {
    let key = key.strip_prefix('s')?;
    let (sec_str, unit_str) = key.split_once('u')?;
    let section = sec_str.parse().ok()?;
    let unit = unit_str.parse().ok()?;
    Some((section, unit))
}

pub fn get_unit(section: u8, unit: u8) -> Option<&'static UnitInfo> {
    UNITS.iter().find(|u| u.section == section && u.unit == unit)
}

pub fn section_name(section: u8) -> &'static str {
    match section {
        1 => "Section 1 · Intro",
        2 => "Section 2 · A1",
        _ => "Unknown",
    }
}

pub fn all_sections() -> &'static [u8] {
    &[1, 2]
}

pub fn units_in_section(section: u8) -> Vec<&'static UnitInfo> {
    UNITS.iter().filter(|u| u.section == section).collect()
}
