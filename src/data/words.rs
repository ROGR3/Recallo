// Auto-classified from dump.txt — reviewed and corrected manually.
// Each entry: (korean, english, category)
// Categories: Noun, Verb, Adjective, Adverb, Expression, Grammar, Loanword, Food

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Category {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Expression,
    Grammar,
    Loanword,
    Food,
}

impl Category {
    pub fn display_name(&self) -> &'static str {
        match self {
            Category::Noun => "Nouns",
            Category::Verb => "Verbs",
            Category::Adjective => "Adjectives",
            Category::Adverb => "Adverbs",
            Category::Expression => "Expressions",
            Category::Grammar => "Grammar",
            Category::Loanword => "Loanwords",
            Category::Food => "Food",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            Category::Noun => "📦",
            Category::Verb => "⚡",
            Category::Adjective => "🎨",
            Category::Adverb => "🏃",
            Category::Expression => "💬",
            Category::Grammar => "🔧",
            Category::Loanword => "🌐",
            Category::Food => "🍜",
        }
    }

    pub fn all() -> &'static [Category] {
        &[
            Category::Noun,
            Category::Verb,
            Category::Adjective,
            Category::Adverb,
            Category::Expression,
            Category::Grammar,
            Category::Loanword,
            Category::Food,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word {
    pub korean: &'static str,
    pub english: &'static str,
    pub category: Category,
}

pub static WORDS: &[Word] = &[
    // ── FOOD ──────────────────────────────────────────────────────────────────
    Word {
        korean: "짜장면",
        english: "jajangmyeon (black bean noodles)",
        category: Category::Food,
    },
    Word {
        korean: "김치찌개",
        english: "kimchi stew",
        category: Category::Food,
    },
    Word {
        korean: "비빔밥",
        english: "bibimbap",
        category: Category::Food,
    },
    Word {
        korean: "단무지",
        english: "pickled radish",
        category: Category::Food,
    },
    Word {
        korean: "김밥",
        english: "kimbap",
        category: Category::Food,
    },
    Word {
        korean: "떡",
        english: "rice cake",
        category: Category::Food,
    },
    Word {
        korean: "냉면",
        english: "naengmyeon (cold noodles)",
        category: Category::Food,
    },
    Word {
        korean: "갈비",
        english: "grilled ribs",
        category: Category::Food,
    },
    Word {
        korean: "된장찌개",
        english: "doenjang jjigae (soybean paste stew)",
        category: Category::Food,
    },
    Word {
        korean: "떡볶이",
        english: "tteokbokki (spicy rice cakes)",
        category: Category::Food,
    },
    Word {
        korean: "라면",
        english: "ramyeon (instant noodles)",
        category: Category::Food,
    },
    Word {
        korean: "갈비탕",
        english: "galbitang (rib bone soup)",
        category: Category::Food,
    },
    Word {
        korean: "김치",
        english: "kimchi",
        category: Category::Food,
    },
    Word {
        korean: "만두",
        english: "dumpling",
        category: Category::Food,
    },
    Word {
        korean: "잡채",
        english: "japchae (glass noodle stir-fry)",
        category: Category::Food,
    },
    Word {
        korean: "두부",
        english: "tofu",
        category: Category::Food,
    },
    Word {
        korean: "밥",
        english: "rice (cooked)",
        category: Category::Food,
    },
    Word {
        korean: "찌개",
        english: "stew",
        category: Category::Food,
    },
    Word {
        korean: "고기",
        english: "meat",
        category: Category::Food,
    },
    Word {
        korean: "빵",
        english: "bread",
        category: Category::Food,
    },
    Word {
        korean: "과자",
        english: "crackers",
        category: Category::Food,
    },
    Word {
        korean: "사탕",
        english: "candy",
        category: Category::Food,
    },
    Word {
        korean: "디저트",
        english: "dessert",
        category: Category::Food,
    },
    Word {
        korean: "간식",
        english: "snack",
        category: Category::Food,
    },
    Word {
        korean: "음식",
        english: "food",
        category: Category::Food,
    },
    Word {
        korean: "과일",
        english: "fruit",
        category: Category::Food,
    },
    Word {
        korean: "포도",
        english: "grape",
        category: Category::Food,
    },
    Word {
        korean: "바나나",
        english: "banana",
        category: Category::Food,
    },
    Word {
        korean: "사과",
        english: "apple",
        category: Category::Food,
    },
    Word {
        korean: "딸기",
        english: "strawberry",
        category: Category::Food,
    },
    Word {
        korean: "귤",
        english: "tangerine",
        category: Category::Food,
    },
    Word {
        korean: "멜론",
        english: "melon",
        category: Category::Food,
    },
    Word {
        korean: "초콜릿",
        english: "chocolate",
        category: Category::Food,
    },
    Word {
        korean: "아이스크림",
        english: "ice cream",
        category: Category::Food,
    },
    Word {
        korean: "케이크",
        english: "cake",
        category: Category::Food,
    },
    Word {
        korean: "샌드위치",
        english: "sandwich",
        category: Category::Food,
    },
    Word {
        korean: "빙수",
        english: "bingsu (shaved ice dessert)",
        category: Category::Food,
    },
    Word {
        korean: "팝콘",
        english: "popcorn",
        category: Category::Food,
    },
    Word {
        korean: "치킨",
        english: "fried chicken",
        category: Category::Food,
    },
    Word {
        korean: "파스타",
        english: "pasta",
        category: Category::Food,
    },
    Word {
        korean: "맥주",
        english: "beer",
        category: Category::Food,
    },
    Word {
        korean: "음료수",
        english: "beverage",
        category: Category::Food,
    },
    Word {
        korean: "콜라",
        english: "cola",
        category: Category::Food,
    },
    Word {
        korean: "주스",
        english: "juice",
        category: Category::Food,
    },
    Word {
        korean: "버블티",
        english: "bubble tea",
        category: Category::Food,
    },
    Word {
        korean: "커피",
        english: "coffee",
        category: Category::Food,
    },
    Word {
        korean: "차",
        english: "tea",
        category: Category::Food,
    },
    Word {
        korean: "물",
        english: "water",
        category: Category::Food,
    },
    Word {
        korean: "우유",
        english: "milk",
        category: Category::Food,
    },
    Word {
        korean: "삼계탕",
        english: "samgyetang (ginseng chicken soup)",
        category: Category::Food,
    },
    // ── LOANWORDS ─────────────────────────────────────────────────────────────
    Word {
        korean: "캐릭터",
        english: "character",
        category: Category::Loanword,
    },
    Word {
        korean: "레벨",
        english: "level",
        category: Category::Loanword,
    },
    Word {
        korean: "서비스",
        english: "service",
        category: Category::Loanword,
    },
    Word {
        korean: "피씨방",
        english: "PC room",
        category: Category::Loanword,
    },
    Word {
        korean: "엘리베이터",
        english: "elevator",
        category: Category::Loanword,
    },
    Word {
        korean: "택시",
        english: "taxi",
        category: Category::Loanword,
    },
    Word {
        korean: "피아노",
        english: "piano",
        category: Category::Loanword,
    },
    Word {
        korean: "드럼",
        english: "drum",
        category: Category::Loanword,
    },
    Word {
        korean: "테이블",
        english: "table",
        category: Category::Loanword,
    },
    Word {
        korean: "소파",
        english: "sofa",
        category: Category::Loanword,
    },
    Word {
        korean: "텔레비전",
        english: "television",
        category: Category::Loanword,
    },
    Word {
        korean: "노트북",
        english: "laptop",
        category: Category::Loanword,
    },
    Word {
        korean: "복사기",
        english: "copier",
        category: Category::Loanword,
    },
    Word {
        korean: "컴퓨터",
        english: "computer",
        category: Category::Loanword,
    },
    Word {
        korean: "인터넷",
        english: "internet",
        category: Category::Loanword,
    },
    Word {
        korean: "카메라",
        english: "camera",
        category: Category::Loanword,
    },
    Word {
        korean: "볼펜",
        english: "ballpoint pen",
        category: Category::Loanword,
    },
    Word {
        korean: "핸드폰",
        english: "cell phone",
        category: Category::Loanword,
    },
    Word {
        korean: "카드",
        english: "card",
        category: Category::Loanword,
    },
    Word {
        korean: "헬멧",
        english: "helmet",
        category: Category::Loanword,
    },
    Word {
        korean: "터미널",
        english: "bus terminal",
        category: Category::Loanword,
    },
    Word {
        korean: "테니스",
        english: "tennis",
        category: Category::Loanword,
    },
    Word {
        korean: "게임",
        english: "game",
        category: Category::Loanword,
    },
    Word {
        korean: "랩",
        english: "rap",
        category: Category::Loanword,
    },
    Word {
        korean: "콘서트",
        english: "concert",
        category: Category::Loanword,
    },
    Word {
        korean: "케이팝",
        english: "K-pop",
        category: Category::Loanword,
    },
    Word {
        korean: "재즈",
        english: "jazz",
        category: Category::Loanword,
    },
    Word {
        korean: "액션",
        english: "action",
        category: Category::Loanword,
    },
    Word {
        korean: "드라마",
        english: "drama (TV series)",
        category: Category::Loanword,
    },
    Word {
        korean: "멜로",
        english: "melodrama",
        category: Category::Loanword,
    },
    Word {
        korean: "데이트",
        english: "date (romantic)",
        category: Category::Loanword,
    },
    Word {
        korean: "캠핑",
        english: "camping",
        category: Category::Loanword,
    },
    Word {
        korean: "텐트",
        english: "tent",
        category: Category::Loanword,
    },
    Word {
        korean: "쇼핑",
        english: "shopping",
        category: Category::Loanword,
    },
    Word {
        korean: "마트",
        english: "supermarket",
        category: Category::Loanword,
    },
    Word {
        korean: "티셔츠",
        english: "T-shirt",
        category: Category::Loanword,
    },
    Word {
        korean: "에어컨",
        english: "air conditioner",
        category: Category::Loanword,
    },
    Word {
        korean: "세탁기",
        english: "washing machine",
        category: Category::Loanword,
    },
    Word {
        korean: "냉장고",
        english: "refrigerator",
        category: Category::Loanword,
    },
    Word {
        korean: "선풍기",
        english: "electric fan",
        category: Category::Loanword,
    },
    Word {
        korean: "메시지",
        english: "message",
        category: Category::Loanword,
    },
    Word {
        korean: "이메일",
        english: "email",
        category: Category::Loanword,
    },
    Word {
        korean: "비누",
        english: "soap",
        category: Category::Loanword,
    },
    Word {
        korean: "수건",
        english: "towel",
        category: Category::Loanword,
    },
    Word {
        korean: "파티",
        english: "party",
        category: Category::Loanword,
    },
    Word {
        korean: "체크인",
        english: "check-in",
        category: Category::Loanword,
    },
    Word {
        korean: "올림픽",
        english: "Olympics",
        category: Category::Loanword,
    },
    // ── NOUNS ─────────────────────────────────────────────────────────────────
    Word {
        korean: "불",
        english: "fire / light",
        category: Category::Noun,
    },
    Word {
        korean: "이모",
        english: "aunt (mother's sister)",
        category: Category::Noun,
    },
    Word {
        korean: "걱정",
        english: "worry",
        category: Category::Noun,
    },
    Word {
        korean: "반창고",
        english: "band-aid",
        category: Category::Noun,
    },
    Word {
        korean: "손",
        english: "hand",
        category: Category::Noun,
    },
    Word {
        korean: "남동생",
        english: "younger brother",
        category: Category::Noun,
    },
    Word {
        korean: "발",
        english: "foot",
        category: Category::Noun,
    },
    Word {
        korean: "문자",
        english: "text message",
        category: Category::Noun,
    },
    Word {
        korean: "아주머니",
        english: "middle-aged woman",
        category: Category::Noun,
    },
    Word {
        korean: "숟가락",
        english: "spoon",
        category: Category::Noun,
    },
    Word {
        korean: "젓가락",
        english: "chopsticks",
        category: Category::Noun,
    },
    Word {
        korean: "영수증",
        english: "receipt",
        category: Category::Noun,
    },
    Word {
        korean: "메뉴판",
        english: "menu",
        category: Category::Noun,
    },
    Word {
        korean: "한식당",
        english: "Korean restaurant",
        category: Category::Noun,
    },
    Word {
        korean: "초등학교",
        english: "elementary school",
        category: Category::Noun,
    },
    Word {
        korean: "때",
        english: "moment",
        category: Category::Noun,
    },
    Word {
        korean: "회의",
        english: "meeting",
        category: Category::Noun,
    },
    Word {
        korean: "장소",
        english: "venue",
        category: Category::Noun,
    },
    Word {
        korean: "몸",
        english: "body",
        category: Category::Noun,
    },
    Word {
        korean: "잠",
        english: "sleep",
        category: Category::Noun,
    },
    Word {
        korean: "약속",
        english: "appointment",
        category: Category::Noun,
    },
    Word {
        korean: "등산",
        english: "hiking",
        category: Category::Noun,
    },
    Word {
        korean: "박물관",
        english: "museum",
        category: Category::Noun,
    },
    Word {
        korean: "태국",
        english: "Thailand",
        category: Category::Noun,
    },
    Word {
        korean: "여행",
        english: "trip",
        category: Category::Noun,
    },
    Word {
        korean: "비",
        english: "rain",
        category: Category::Noun,
    },
    Word {
        korean: "귀",
        english: "ear",
        category: Category::Noun,
    },
    Word {
        korean: "글",
        english: "writing",
        category: Category::Noun,
    },
    Word {
        korean: "아기",
        english: "baby",
        category: Category::Noun,
    },
    Word {
        korean: "소풍",
        english: "picnic",
        category: Category::Noun,
    },
    Word {
        korean: "공원",
        english: "park",
        category: Category::Noun,
    },
    Word {
        korean: "구두",
        english: "dress shoes",
        category: Category::Noun,
    },
    Word {
        korean: "치마",
        english: "skirt",
        category: Category::Noun,
    },
    Word {
        korean: "아들",
        english: "son",
        category: Category::Noun,
    },
    Word {
        korean: "과학",
        english: "science",
        category: Category::Noun,
    },
    Word {
        korean: "소설책",
        english: "novel",
        category: Category::Noun,
    },
    Word {
        korean: "교실",
        english: "classroom",
        category: Category::Noun,
    },
    Word {
        korean: "바닥",
        english: "floor",
        category: Category::Noun,
    },
    Word {
        korean: "칠판",
        english: "chalkboard",
        category: Category::Noun,
    },
    Word {
        korean: "그림",
        english: "drawing",
        category: Category::Noun,
    },
    Word {
        korean: "생신",
        english: "birthday (honorific)",
        category: Category::Noun,
    },
    Word {
        korean: "달",
        english: "month / moon",
        category: Category::Noun,
    },
    Word {
        korean: "밤",
        english: "night",
        category: Category::Noun,
    },
    Word {
        korean: "기차",
        english: "train",
        category: Category::Noun,
    },
    Word {
        korean: "윷놀이",
        english: "yutnori (Korean board game)",
        category: Category::Noun,
    },
    Word {
        korean: "수요일",
        english: "Wednesday",
        category: Category::Noun,
    },
    Word {
        korean: "시장",
        english: "market",
        category: Category::Noun,
    },
    Word {
        korean: "날",
        english: "day",
        category: Category::Noun,
    },
    Word {
        korean: "언니",
        english: "older sister (used by females)",
        category: Category::Noun,
    },
    Word {
        korean: "한복",
        english: "hanbok (traditional Korean clothing)",
        category: Category::Noun,
    },
    Word {
        korean: "댁",
        english: "home (honorific)",
        category: Category::Noun,
    },
    Word {
        korean: "추석",
        english: "Chuseok (Korean harvest festival)",
        category: Category::Noun,
    },
    Word {
        korean: "다음",
        english: "next",
        category: Category::Noun,
    },
    Word {
        korean: "주",
        english: "week",
        category: Category::Noun,
    },
    Word {
        korean: "할머니",
        english: "grandmother",
        category: Category::Noun,
    },
    Word {
        korean: "서류",
        english: "document",
        category: Category::Noun,
    },
    Word {
        korean: "층",
        english: "floor (of a building)",
        category: Category::Noun,
    },
    Word {
        korean: "사무실",
        english: "office",
        category: Category::Noun,
    },
    Word {
        korean: "부장님",
        english: "department head",
        category: Category::Noun,
    },
    Word {
        korean: "사진",
        english: "photo",
        category: Category::Noun,
    },
    Word {
        korean: "이름",
        english: "name",
        category: Category::Noun,
    },
    Word {
        korean: "주소",
        english: "address",
        category: Category::Noun,
    },
    Word {
        korean: "회사",
        english: "company",
        category: Category::Noun,
    },
    Word {
        korean: "문",
        english: "door",
        category: Category::Noun,
    },
    Word {
        korean: "열쇠",
        english: "key",
        category: Category::Noun,
    },
    Word {
        korean: "수영장",
        english: "swimming pool",
        category: Category::Noun,
    },
    Word {
        korean: "할아버지",
        english: "grandfather",
        category: Category::Noun,
    },
    Word {
        korean: "바지",
        english: "pants",
        category: Category::Noun,
    },
    Word {
        korean: "백화점",
        english: "department store",
        category: Category::Noun,
    },
    Word {
        korean: "모자",
        english: "hat",
        category: Category::Noun,
    },
    Word {
        korean: "옷",
        english: "clothes",
        category: Category::Noun,
    },
    Word {
        korean: "기숙사",
        english: "dormitory",
        category: Category::Noun,
    },
    Word {
        korean: "편의점",
        english: "convenience store",
        category: Category::Noun,
    },
    Word {
        korean: "운동장",
        english: "sports field",
        category: Category::Noun,
    },
    Word {
        korean: "남자 친구",
        english: "boyfriend",
        category: Category::Noun,
    },
    Word {
        korean: "시간",
        english: "time",
        category: Category::Noun,
    },
    Word {
        korean: "수업",
        english: "class",
        category: Category::Noun,
    },
    Word {
        korean: "숙제",
        english: "homework",
        category: Category::Noun,
    },
    Word {
        korean: "전공",
        english: "major (academic)",
        category: Category::Noun,
    },
    Word {
        korean: "학교",
        english: "school",
        category: Category::Noun,
    },
    Word {
        korean: "동아리",
        english: "student club",
        category: Category::Noun,
    },
    Word {
        korean: "음악",
        english: "music",
        category: Category::Noun,
    },
    Word {
        korean: "딸",
        english: "daughter",
        category: Category::Noun,
    },
    Word {
        korean: "대학생",
        english: "college student",
        category: Category::Noun,
    },
    Word {
        korean: "약사",
        english: "pharmacist",
        category: Category::Noun,
    },
    Word {
        korean: "약국",
        english: "pharmacy",
        category: Category::Noun,
    },
    Word {
        korean: "상처",
        english: "wound",
        category: Category::Noun,
    },
    Word {
        korean: "병",
        english: "illness",
        category: Category::Noun,
    },
    Word {
        korean: "약",
        english: "medicine",
        category: Category::Noun,
    },
    Word {
        korean: "증상",
        english: "symptom",
        category: Category::Noun,
    },
    Word {
        korean: "배",
        english: "stomach",
        category: Category::Noun,
    },
    Word {
        korean: "머리",
        english: "head / hair",
        category: Category::Noun,
    },
    Word {
        korean: "다리",
        english: "leg",
        category: Category::Noun,
    },
    Word {
        korean: "아이",
        english: "child",
        category: Category::Noun,
    },
    Word {
        korean: "병원",
        english: "hospital",
        category: Category::Noun,
    },
    Word {
        korean: "간호사",
        english: "nurse",
        category: Category::Noun,
    },
    Word {
        korean: "봉투",
        english: "envelope",
        category: Category::Noun,
    },
    Word {
        korean: "손님",
        english: "customer",
        category: Category::Noun,
    },
    Word {
        korean: "직원",
        english: "employee",
        category: Category::Noun,
    },
    Word {
        korean: "할인",
        english: "discount",
        category: Category::Noun,
    },
    Word {
        korean: "물건",
        english: "item",
        category: Category::Noun,
    },
    Word {
        korean: "교통",
        english: "transportation",
        category: Category::Noun,
    },
    Word {
        korean: "강남",
        english: "Gangnam (area in Seoul)",
        category: Category::Noun,
    },
    Word {
        korean: "지하철",
        english: "subway",
        category: Category::Noun,
    },
    Word {
        korean: "역",
        english: "station",
        category: Category::Noun,
    },
    Word {
        korean: "계단",
        english: "stairs",
        category: Category::Noun,
    },
    Word {
        korean: "길",
        english: "road",
        category: Category::Noun,
    },
    Word {
        korean: "운전",
        english: "driving",
        category: Category::Noun,
    },
    Word {
        korean: "주차",
        english: "parking",
        category: Category::Noun,
    },
    Word {
        korean: "자동차",
        english: "car",
        category: Category::Noun,
    },
    Word {
        korean: "자전거",
        english: "bicycle",
        category: Category::Noun,
    },
    Word {
        korean: "신문",
        english: "newspaper",
        category: Category::Noun,
    },
    Word {
        korean: "만화",
        english: "comic",
        category: Category::Noun,
    },
    Word {
        korean: "잡지",
        english: "magazine",
        category: Category::Noun,
    },
    Word {
        korean: "교과서",
        english: "textbook",
        category: Category::Noun,
    },
    Word {
        korean: "사전",
        english: "dictionary",
        category: Category::Noun,
    },
    Word {
        korean: "쓰레기통",
        english: "trash can",
        category: Category::Noun,
    },
    Word {
        korean: "번호",
        english: "number",
        category: Category::Noun,
    },
    Word {
        korean: "영화",
        english: "movie",
        category: Category::Noun,
    },
    Word {
        korean: "영화관",
        english: "cinema",
        category: Category::Noun,
    },
    Word {
        korean: "무대",
        english: "stage",
        category: Category::Noun,
    },
    Word {
        korean: "노래",
        english: "song",
        category: Category::Noun,
    },
    Word {
        korean: "가수",
        english: "singer",
        category: Category::Noun,
    },
    Word {
        korean: "걸그룹",
        english: "girl group (K-pop)",
        category: Category::Noun,
    },
    Word {
        korean: "눈",
        english: "snow / eye",
        category: Category::Noun,
    },
    Word {
        korean: "꽃",
        english: "flower",
        category: Category::Noun,
    },
    Word {
        korean: "운동화",
        english: "sneakers",
        category: Category::Noun,
    },
    Word {
        korean: "가격",
        english: "price",
        category: Category::Noun,
    },
    Word {
        korean: "방",
        english: "room",
        category: Category::Noun,
    },
    Word {
        korean: "창문",
        english: "window",
        category: Category::Noun,
    },
    Word {
        korean: "크기",
        english: "size",
        category: Category::Noun,
    },
    Word {
        korean: "아파트",
        english: "apartment",
        category: Category::Noun,
    },
    Word {
        korean: "동네",
        english: "neighborhood",
        category: Category::Noun,
    },
    Word {
        korean: "이웃",
        english: "neighbor",
        category: Category::Noun,
    },
    Word {
        korean: "집",
        english: "home",
        category: Category::Noun,
    },
    Word {
        korean: "모기",
        english: "mosquito",
        category: Category::Noun,
    },
    Word {
        korean: "아침",
        english: "morning",
        category: Category::Noun,
    },
    Word {
        korean: "나무",
        english: "tree",
        category: Category::Noun,
    },
    Word {
        korean: "호수",
        english: "lake",
        category: Category::Noun,
    },
    Word {
        korean: "기온",
        english: "temperature",
        category: Category::Noun,
    },
    Word {
        korean: "산",
        english: "mountain",
        category: Category::Noun,
    },
    Word {
        korean: "날씨",
        english: "weather",
        category: Category::Noun,
    },
    Word {
        korean: "금요일",
        english: "Friday",
        category: Category::Noun,
    },
    Word {
        korean: "학생",
        english: "student",
        category: Category::Noun,
    },
    Word {
        korean: "변호사",
        english: "lawyer",
        category: Category::Noun,
    },
    Word {
        korean: "교수",
        english: "professor",
        category: Category::Noun,
    },
    Word {
        korean: "서울",
        english: "Seoul (capital of Korea)",
        category: Category::Noun,
    },
    Word {
        korean: "아빠",
        english: "dad",
        category: Category::Noun,
    },
    Word {
        korean: "도시",
        english: "city",
        category: Category::Noun,
    },
    Word {
        korean: "사촌",
        english: "cousin",
        category: Category::Noun,
    },
    Word {
        korean: "형",
        english: "older brother (used by males)",
        category: Category::Noun,
    },
    Word {
        korean: "이분",
        english: "this person (honorific)",
        category: Category::Noun,
    },
    Word {
        korean: "강아지",
        english: "dog",
        category: Category::Noun,
    },
    Word {
        korean: "부모님",
        english: "parents",
        category: Category::Noun,
    },
    Word {
        korean: "고향",
        english: "hometown",
        category: Category::Noun,
    },
    Word {
        korean: "부산",
        english: "Busan (city in Korea)",
        category: Category::Noun,
    },
    Word {
        korean: "여권",
        english: "passport",
        category: Category::Noun,
    },
    Word {
        korean: "근처",
        english: "nearby area",
        category: Category::Noun,
    },
    Word {
        korean: "게이트",
        english: "gate (airport)",
        category: Category::Noun,
    },
    Word {
        korean: "카페",
        english: "café",
        category: Category::Noun,
    },
    Word {
        korean: "출구",
        english: "exit",
        category: Category::Noun,
    },
    Word {
        korean: "식당",
        english: "restaurant",
        category: Category::Noun,
    },
    Word {
        korean: "화장실",
        english: "bathroom",
        category: Category::Noun,
    },
    Word {
        korean: "표",
        english: "ticket",
        category: Category::Noun,
    },
    Word {
        korean: "공항",
        english: "airport",
        category: Category::Noun,
    },
    Word {
        korean: "가족",
        english: "family",
        category: Category::Noun,
    },
    Word {
        korean: "버스",
        english: "bus",
        category: Category::Noun,
    },
    Word {
        korean: "팀",
        english: "team",
        category: Category::Noun,
    },
    Word {
        korean: "가게",
        english: "shop",
        category: Category::Noun,
    },
    Word {
        korean: "자리",
        english: "seat",
        category: Category::Noun,
    },
    Word {
        korean: "우리",
        english: "we",
        category: Category::Noun,
    },
    Word {
        korean: "공",
        english: "ball",
        category: Category::Noun,
    },
    Word {
        korean: "경기장",
        english: "stadium",
        category: Category::Noun,
    },
    Word {
        korean: "야구",
        english: "baseball",
        category: Category::Noun,
    },
    Word {
        korean: "경기",
        english: "match",
        category: Category::Noun,
    },
    Word {
        korean: "수첩",
        english: "notebook",
        category: Category::Noun,
    },
    Word {
        korean: "잔",
        english: "cup",
        category: Category::Noun,
    },
    Word {
        korean: "여행사",
        english: "travel agency",
        category: Category::Noun,
    },
    Word {
        korean: "제주도",
        english: "Jeju Island",
        category: Category::Noun,
    },
    Word {
        korean: "엽서",
        english: "postcard",
        category: Category::Noun,
    },
    Word {
        korean: "지도",
        english: "map",
        category: Category::Noun,
    },
    Word {
        korean: "우체국",
        english: "post office",
        category: Category::Noun,
    },
    Word {
        korean: "정류장",
        english: "bus stop",
        category: Category::Noun,
    },
    Word {
        korean: "돈",
        english: "money",
        category: Category::Noun,
    },
    Word {
        korean: "학생증",
        english: "student ID",
        category: Category::Noun,
    },
    Word {
        korean: "배우",
        english: "actor",
        category: Category::Noun,
    },
    Word {
        korean: "작가",
        english: "author",
        category: Category::Noun,
    },
    Word {
        korean: "축구",
        english: "soccer",
        category: Category::Noun,
    },
    Word {
        korean: "선수",
        english: "athlete",
        category: Category::Noun,
    },
    Word {
        korean: "태권도",
        english: "taekwondo",
        category: Category::Noun,
    },
    Word {
        korean: "팬",
        english: "fan (admirer)",
        category: Category::Noun,
    },
    Word {
        korean: "취미",
        english: "hobby",
        category: Category::Noun,
    },
    Word {
        korean: "독서",
        english: "reading (hobby)",
        category: Category::Noun,
    },
    Word {
        korean: "운동",
        english: "exercise",
        category: Category::Noun,
    },
    Word {
        korean: "목걸이",
        english: "necklace",
        category: Category::Noun,
    },
    Word {
        korean: "시계",
        english: "watch",
        category: Category::Noun,
    },
    Word {
        korean: "선물",
        english: "gift",
        category: Category::Noun,
    },
    Word {
        korean: "여동생",
        english: "younger sister",
        category: Category::Noun,
    },
    Word {
        korean: "오늘",
        english: "today",
        category: Category::Noun,
    },
    Word {
        korean: "생일",
        english: "birthday",
        category: Category::Noun,
    },
    Word {
        korean: "인천",
        english: "Incheon (city in Korea)",
        category: Category::Noun,
    },
    Word {
        korean: "소개",
        english: "introduction",
        category: Category::Noun,
    },
    Word {
        korean: "소개팅",
        english: "blind date",
        category: Category::Noun,
    },
    Word {
        korean: "미소",
        english: "smile",
        category: Category::Noun,
    },
    Word {
        korean: "말",
        english: "horse / words / speech",
        category: Category::Noun,
    },
    Word {
        korean: "한국어",
        english: "Korean language",
        category: Category::Noun,
    },
    Word {
        korean: "영어",
        english: "English language",
        category: Category::Noun,
    },
    Word {
        korean: "오빠",
        english: "oppa (older brother, used by females)",
        category: Category::Noun,
    },
    Word {
        korean: "누나",
        english: "older sister (used by males)",
        category: Category::Noun,
    },
    Word {
        korean: "엄마",
        english: "mom",
        category: Category::Noun,
    },
    Word {
        korean: "고양이",
        english: "cat",
        category: Category::Noun,
    },
    Word {
        korean: "친구",
        english: "friend",
        category: Category::Noun,
    },
    Word {
        korean: "미국",
        english: "USA",
        category: Category::Noun,
    },
    Word {
        korean: "한국",
        english: "Korea",
        category: Category::Noun,
    },
    Word {
        korean: "사람",
        english: "person",
        category: Category::Noun,
    },
    Word {
        korean: "중국",
        english: "China",
        category: Category::Noun,
    },
    Word {
        korean: "선생님",
        english: "teacher",
        category: Category::Noun,
    },
    Word {
        korean: "회사원",
        english: "office worker",
        category: Category::Noun,
    },
    Word {
        korean: "화가",
        english: "painter",
        category: Category::Noun,
    },
    Word {
        korean: "웨이터",
        english: "waiter",
        category: Category::Noun,
    },
    Word {
        korean: "의사",
        english: "doctor",
        category: Category::Noun,
    },
    Word {
        korean: "책",
        english: "book",
        category: Category::Noun,
    },
    Word {
        korean: "지갑",
        english: "wallet",
        category: Category::Noun,
    },
    Word {
        korean: "가방",
        english: "bag",
        category: Category::Noun,
    },
    Word {
        korean: "나",
        english: "I",
        category: Category::Noun,
    },
    Word {
        korean: "저",
        english: "I (formal/humble)",
        category: Category::Noun,
    },
    Word {
        korean: "바다",
        english: "ocean",
        category: Category::Noun,
    },
    Word {
        korean: "서점",
        english: "bookstore",
        category: Category::Noun,
    },
    Word {
        korean: "시내",
        english: "downtown",
        category: Category::Noun,
    },
    Word {
        korean: "맛집",
        english: "popular restaurant",
        category: Category::Noun,
    },
    Word {
        korean: "노래방",
        english: "karaoke room (norebang)",
        category: Category::Noun,
    },
    Word {
        korean: "도서관",
        english: "library",
        category: Category::Noun,
    },
    Word {
        korean: "곳",
        english: "place",
        category: Category::Noun,
    },
    Word {
        korean: "침실",
        english: "bedroom",
        category: Category::Noun,
    },
    Word {
        korean: "책상",
        english: "desk",
        category: Category::Noun,
    },
    Word {
        korean: "거실",
        english: "living room",
        category: Category::Noun,
    },
    Word {
        korean: "부엌",
        english: "kitchen",
        category: Category::Noun,
    },
    Word {
        korean: "전철역",
        english: "subway station",
        category: Category::Noun,
    },
    Word {
        korean: "침대",
        english: "bed",
        category: Category::Noun,
    },
    Word {
        korean: "장난감",
        english: "toy",
        category: Category::Noun,
    },
    Word {
        korean: "토요일",
        english: "Saturday",
        category: Category::Noun,
    },
    Word {
        korean: "화요일",
        english: "Tuesday",
        category: Category::Noun,
    },
    Word {
        korean: "삼월",
        english: "March",
        category: Category::Noun,
    },
    Word {
        korean: "마당",
        english: "yard",
        category: Category::Noun,
    },
    Word {
        korean: "크리스마스",
        english: "Christmas",
        category: Category::Noun,
    },
    Word {
        korean: "내일",
        english: "tomorrow",
        category: Category::Noun,
    },
    Word {
        korean: "방금",
        english: "just now",
        category: Category::Noun,
    },
    Word {
        korean: "아까",
        english: "a little while ago",
        category: Category::Noun,
    },
    Word {
        korean: "오랜만",
        english: "long time (no see)",
        category: Category::Noun,
    },
    Word {
        korean: "씨",
        english: "Mr./Ms./suffix for names",
        category: Category::Noun,
    },
    Word {
        korean: "하늘",
        english: "sky",
        category: Category::Noun,
    },
    Word {
        korean: "감기",
        english: "cold (illness)",
        category: Category::Noun,
    },
    Word {
        korean: "겨울",
        english: "winter",
        category: Category::Noun,
    },
    Word {
        korean: "동물",
        english: "animal",
        category: Category::Noun,
    },
    Word {
        korean: "작년",
        english: "last year",
        category: Category::Noun,
    },
    Word {
        korean: "가을",
        english: "autumn",
        category: Category::Noun,
    },
    Word {
        korean: "오후",
        english: "afternoon",
        category: Category::Noun,
    },
    Word {
        korean: "우산",
        english: "umbrella",
        category: Category::Noun,
    },
    Word {
        korean: "바람",
        english: "wind",
        category: Category::Noun,
    },
    Word {
        korean: "올해",
        english: "this year",
        category: Category::Noun,
    },
    Word {
        korean: "어제",
        english: "yesterday",
        category: Category::Noun,
    },
    Word {
        korean: "단어",
        english: "word",
        category: Category::Noun,
    },
    Word {
        korean: "연필",
        english: "pencil",
        category: Category::Noun,
    },
    Word {
        korean: "필통",
        english: "pencil case",
        category: Category::Noun,
    },
    Word {
        korean: "안경",
        english: "glasses",
        category: Category::Noun,
    },
    Word {
        korean: "것",
        english: "thing",
        category: Category::Noun,
    },
    Word {
        korean: "건물",
        english: "building",
        category: Category::Noun,
    },
    Word {
        korean: "의자",
        english: "chair",
        category: Category::Noun,
    },
    Word {
        korean: "공책",
        english: "notebook",
        category: Category::Noun,
    },
    Word {
        korean: "색",
        english: "color",
        category: Category::Noun,
    },
    Word {
        korean: "외국어",
        english: "foreign language",
        category: Category::Noun,
    },
    Word {
        korean: "시험",
        english: "exam",
        category: Category::Noun,
    },
    Word {
        korean: "출발",
        english: "departure",
        category: Category::Noun,
    },
    Word {
        korean: "인도네시아",
        english: "Indonesia",
        category: Category::Noun,
    },
    Word {
        korean: "탁구",
        english: "table tennis",
        category: Category::Noun,
    },
    Word {
        korean: "일본",
        english: "Japan",
        category: Category::Noun,
    },
    Word {
        korean: "농구",
        english: "basketball",
        category: Category::Noun,
    },
    Word {
        korean: "연습",
        english: "practice",
        category: Category::Noun,
    },
    Word {
        korean: "달리기",
        english: "running",
        category: Category::Noun,
    },
    Word {
        korean: "이번",
        english: "this time",
        category: Category::Noun,
    },
    Word {
        korean: "프랑스",
        english: "France",
        category: Category::Noun,
    },
    // ── VERBS ─────────────────────────────────────────────────────────────────
    Word {
        korean: "끄다",
        english: "to turn off",
        category: Category::Verb,
    },
    Word {
        korean: "바꾸다",
        english: "to change",
        category: Category::Verb,
    },
    Word {
        korean: "고르다",
        english: "to choose",
        category: Category::Verb,
    },
    Word {
        korean: "세다",
        english: "to count",
        category: Category::Verb,
    },
    Word {
        korean: "이기다",
        english: "to win",
        category: Category::Verb,
    },
    Word {
        korean: "생기다",
        english: "to arise / form / open (new place)",
        category: Category::Verb,
    },
    Word {
        korean: "걱정하다",
        english: "to worry",
        category: Category::Verb,
    },
    Word {
        korean: "붙이다",
        english: "to attach",
        category: Category::Verb,
    },
    Word {
        korean: "설명하다",
        english: "to explain",
        category: Category::Verb,
    },
    Word {
        korean: "전화하다",
        english: "to call (phone)",
        category: Category::Verb,
    },
    Word {
        korean: "긴장하다",
        english: "to be nervous",
        category: Category::Verb,
    },
    Word {
        korean: "조심하다",
        english: "to be careful",
        category: Category::Verb,
    },
    Word {
        korean: "넘어지다",
        english: "to fall / trip / slip",
        category: Category::Verb,
    },
    Word {
        korean: "열다",
        english: "to open",
        category: Category::Verb,
    },
    Word {
        korean: "가르치다",
        english: "to teach",
        category: Category::Verb,
    },
    Word {
        korean: "닦다",
        english: "to wipe",
        category: Category::Verb,
    },
    Word {
        korean: "걸다",
        english: "to hang / bet / make a call",
        category: Category::Verb,
    },
    Word {
        korean: "그리다",
        english: "to draw / paint",
        category: Category::Verb,
    },
    Word {
        korean: "포장하다",
        english: "to wrap",
        category: Category::Verb,
    },
    Word {
        korean: "도와주다",
        english: "to help",
        category: Category::Verb,
    },
    Word {
        korean: "고치다",
        english: "to fix",
        category: Category::Verb,
    },
    Word {
        korean: "만들다",
        english: "to make",
        category: Category::Verb,
    },
    Word {
        korean: "받다",
        english: "to receive",
        category: Category::Verb,
    },
    Word {
        korean: "확인하다",
        english: "to confirm",
        category: Category::Verb,
    },
    Word {
        korean: "올라가다",
        english: "to go up",
        category: Category::Verb,
    },
    Word {
        korean: "보내다",
        english: "to send / spend (time)",
        category: Category::Verb,
    },
    Word {
        korean: "닫다",
        english: "to close",
        category: Category::Verb,
    },
    Word {
        korean: "넣다",
        english: "to put in",
        category: Category::Verb,
    },
    Word {
        korean: "운전하다",
        english: "to drive",
        category: Category::Verb,
    },
    Word {
        korean: "알려주다",
        english: "to inform",
        category: Category::Verb,
    },
    Word {
        korean: "어울리다",
        english: "to match / suit / go well together",
        category: Category::Verb,
    },
    Word {
        korean: "청소하다",
        english: "to clean up",
        category: Category::Verb,
    },
    Word {
        korean: "공부하다",
        english: "to study",
        category: Category::Verb,
    },
    Word {
        korean: "대화하다",
        english: "to have a conversation",
        category: Category::Verb,
    },
    Word {
        korean: "인사하다",
        english: "to greet",
        category: Category::Verb,
    },
    Word {
        korean: "산책하다",
        english: "to go for a walk",
        category: Category::Verb,
    },
    Word {
        korean: "수영하다",
        english: "to swim",
        category: Category::Verb,
    },
    Word {
        korean: "정리하다",
        english: "to organize",
        category: Category::Verb,
    },
    Word {
        korean: "기억하다",
        english: "to remember",
        category: Category::Verb,
    },
    Word {
        korean: "싫어하다",
        english: "to dislike",
        category: Category::Verb,
    },
    Word {
        korean: "사다",
        english: "to buy",
        category: Category::Verb,
    },
    Word {
        korean: "팔다",
        english: "to sell",
        category: Category::Verb,
    },
    Word {
        korean: "들어가다",
        english: "to enter",
        category: Category::Verb,
    },
    Word {
        korean: "운동하다",
        english: "to exercise",
        category: Category::Verb,
    },
    Word {
        korean: "먹다",
        english: "to eat",
        category: Category::Verb,
    },
    Word {
        korean: "적다",
        english: "to write down",
        category: Category::Verb,
    },
    Word {
        korean: "치료하다",
        english: "to treat",
        category: Category::Verb,
    },
    Word {
        korean: "알다",
        english: "to know",
        category: Category::Verb,
    },
    Word {
        korean: "부르다",
        english: "to call / summon / sing",
        category: Category::Verb,
    },
    Word {
        korean: "되다",
        english: "to become / be okay",
        category: Category::Verb,
    },
    Word {
        korean: "신선하다",
        english: "to be fresh",
        category: Category::Verb,
    },
    Word {
        korean: "앉다",
        english: "to sit (down)",
        category: Category::Verb,
    },
    Word {
        korean: "보다",
        english: "to watch",
        category: Category::Verb,
    },
    Word {
        korean: "찾다",
        english: "to look for",
        category: Category::Verb,
    },
    Word {
        korean: "읽다",
        english: "to read",
        category: Category::Verb,
    },
    Word {
        korean: "빌리다",
        english: "to borrow / rent",
        category: Category::Verb,
    },
    Word {
        korean: "만나다",
        english: "to meet",
        category: Category::Verb,
    },
    Word {
        korean: "마시다",
        english: "to drink",
        category: Category::Verb,
    },
    Word {
        korean: "타다",
        english: "to ride / take (transport)",
        category: Category::Verb,
    },
    Word {
        korean: "걷다",
        english: "to walk",
        category: Category::Verb,
    },
    Word {
        korean: "치다",
        english: "to play (instrument) / hit / set up",
        category: Category::Verb,
    },
    Word {
        korean: "주다",
        english: "to give",
        category: Category::Verb,
    },
    Word {
        korean: "드리다",
        english: "to give (humble/honorific)",
        category: Category::Verb,
    },
    Word {
        korean: "시키다",
        english: "to order",
        category: Category::Verb,
    },
    Word {
        korean: "남다",
        english: "to remain",
        category: Category::Verb,
    },
    Word {
        korean: "낫다",
        english: "to get better",
        category: Category::Verb,
    },
    Word {
        korean: "오다",
        english: "to come",
        category: Category::Verb,
    },
    Word {
        korean: "찍다",
        english: "to take (photo) / dip / stamp",
        category: Category::Verb,
    },
    Word {
        korean: "가다",
        english: "to go",
        category: Category::Verb,
    },
    Word {
        korean: "하다",
        english: "to do",
        category: Category::Verb,
    },
    Word {
        korean: "없다",
        english: "to not have / not exist",
        category: Category::Verb,
    },
    Word {
        korean: "있다",
        english: "to have / to exist",
        category: Category::Verb,
    },
    Word {
        korean: "이다",
        english: "to be (copula)",
        category: Category::Verb,
    },
    Word {
        korean: "아니다",
        english: "to not be / not",
        category: Category::Verb,
    },
    Word {
        korean: "좋아하다",
        english: "to like",
        category: Category::Verb,
    },
    Word {
        korean: "주문하다",
        english: "to order (food/item)",
        category: Category::Verb,
    },
    Word {
        korean: "검사하다",
        english: "to examine",
        category: Category::Verb,
    },
    Word {
        korean: "쓰다",
        english: "to write / use / wear (hat)",
        category: Category::Verb,
    },
    Word {
        korean: "고장나다",
        english: "to break down",
        category: Category::Verb,
    },
    Word {
        korean: "걸리다",
        english: "to take (time) / to catch (a cold)",
        category: Category::Verb,
    },
    Word {
        korean: "내리다",
        english: "to fall / come down",
        category: Category::Verb,
    },
    Word {
        korean: "보이다",
        english: "to be visible / look",
        category: Category::Verb,
    },
    Word {
        korean: "불다",
        english: "to blow",
        category: Category::Verb,
    },
    Word {
        korean: "말하다",
        english: "to speak",
        category: Category::Verb,
    },
    Word {
        korean: "끝나다",
        english: "to end",
        category: Category::Verb,
    },
    // ── ADJECTIVES ────────────────────────────────────────────────────────────
    Word {
        korean: "기쁘다",
        english: "to be happy",
        category: Category::Adjective,
    },
    Word {
        korean: "게으르다",
        english: "to be lazy",
        category: Category::Adjective,
    },
    Word {
        korean: "어렵다",
        english: "to be difficult",
        category: Category::Adjective,
    },
    Word {
        korean: "덥다",
        english: "to be hot (weather)",
        category: Category::Adjective,
    },
    Word {
        korean: "뜨겁다",
        english: "to be hot (to touch)",
        category: Category::Adjective,
    },
    Word {
        korean: "복잡하다",
        english: "to be crowded / complicated",
        category: Category::Adjective,
    },
    Word {
        korean: "불편하다",
        english: "to be uncomfortable / inconvenient",
        category: Category::Adjective,
    },
    Word {
        korean: "익숙하다",
        english: "to be familiar",
        category: Category::Adjective,
    },
    Word {
        korean: "느리다",
        english: "to be slow",
        category: Category::Adjective,
    },
    Word {
        korean: "유명하다",
        english: "to be famous",
        category: Category::Adjective,
    },
    Word {
        korean: "싸다",
        english: "to be cheap",
        category: Category::Adjective,
    },
    Word {
        korean: "비싸다",
        english: "to be expensive",
        category: Category::Adjective,
    },
    Word {
        korean: "맵다",
        english: "to be spicy",
        category: Category::Adjective,
    },
    Word {
        korean: "가깝다",
        english: "to be close",
        category: Category::Adjective,
    },
    Word {
        korean: "춥다",
        english: "to be cold",
        category: Category::Adjective,
    },
    Word {
        korean: "예쁘다",
        english: "to be pretty",
        category: Category::Adjective,
    },
    Word {
        korean: "재미있다",
        english: "to be fun / funny",
        category: Category::Adjective,
    },
    Word {
        korean: "멋있다",
        english: "to be cool",
        category: Category::Adjective,
    },
    Word {
        korean: "괜찮다",
        english: "to be okay",
        category: Category::Adjective,
    },
    Word {
        korean: "피곤하다",
        english: "to be tired",
        category: Category::Adjective,
    },
    Word {
        korean: "특별하다",
        english: "to be special",
        category: Category::Adjective,
    },
    Word {
        korean: "깨끗하다",
        english: "to be clean",
        category: Category::Adjective,
    },
    Word {
        korean: "따뜻하다",
        english: "to be warm",
        category: Category::Adjective,
    },
    Word {
        korean: "친절하다",
        english: "to be kind",
        category: Category::Adjective,
    },
    Word {
        korean: "조용하다",
        english: "to be quiet",
        category: Category::Adjective,
    },
    Word {
        korean: "한가하다",
        english: "to be free (not busy)",
        category: Category::Adjective,
    },
    Word {
        korean: "필요하다",
        english: "to need",
        category: Category::Adjective,
    },
    Word {
        korean: "많다",
        english: "to be many",
        category: Category::Adjective,
    },
    Word {
        korean: "좋다",
        english: "to be good",
        category: Category::Adjective,
    },
    Word {
        korean: "맑다",
        english: "to be clear / sunny",
        category: Category::Adjective,
    },
    Word {
        korean: "높다",
        english: "to be tall / high",
        category: Category::Adjective,
    },
    Word {
        korean: "화려하다",
        english: "to be flashy",
        category: Category::Adjective,
    },
    Word {
        korean: "이상하다",
        english: "to be strange",
        category: Category::Adjective,
    },
    Word {
        korean: "아름답다",
        english: "to be beautiful",
        category: Category::Adjective,
    },
    Word {
        korean: "고맙다",
        english: "to be grateful",
        category: Category::Adjective,
    },
    Word {
        korean: "맛있다",
        english: "to be delicious",
        category: Category::Adjective,
    },
    Word {
        korean: "아프다",
        english: "to be sick / hurt",
        category: Category::Adjective,
    },
    Word {
        korean: "무섭다",
        english: "to be scary",
        category: Category::Adjective,
    },
    Word {
        korean: "어둡다",
        english: "to be dark",
        category: Category::Adjective,
    },
    Word {
        korean: "무겁다",
        english: "to be heavy",
        category: Category::Adjective,
    },
    Word {
        korean: "크다",
        english: "to be big",
        category: Category::Adjective,
    },
    Word {
        korean: "작다",
        english: "to be small",
        category: Category::Adjective,
    },
    Word {
        korean: "달다",
        english: "to be sweet",
        category: Category::Adjective,
    },
    Word {
        korean: "넓다",
        english: "to be spacious",
        category: Category::Adjective,
    },
    Word {
        korean: "멀다",
        english: "to be far",
        category: Category::Adjective,
    },
    Word {
        korean: "가까운",
        english: "nearby",
        category: Category::Adjective,
    },
    Word {
        korean: "웃긴",
        english: "funny",
        category: Category::Adjective,
    },
    Word {
        korean: "친한",
        english: "close (friendship modifier)",
        category: Category::Adjective,
    },
    Word {
        korean: "궁금하다",
        english: "to be curious",
        category: Category::Adjective,
    },
    Word {
        korean: "빠르다",
        english: "to be fast",
        category: Category::Adjective,
    },
    // ── ADVERBS ───────────────────────────────────────────────────────────────
    Word {
        korean: "벌써",
        english: "already",
        category: Category::Adverb,
    },
    Word {
        korean: "빨리",
        english: "quickly",
        category: Category::Adverb,
    },
    Word {
        korean: "조용히",
        english: "quietly",
        category: Category::Adverb,
    },
    Word {
        korean: "아직",
        english: "not yet",
        category: Category::Adverb,
    },
    Word {
        korean: "천천히",
        english: "slowly",
        category: Category::Adverb,
    },
    Word {
        korean: "먼저",
        english: "first",
        category: Category::Adverb,
    },
    Word {
        korean: "따뜻하게",
        english: "warmly",
        category: Category::Adverb,
    },
    Word {
        korean: "편하게",
        english: "comfortably",
        category: Category::Adverb,
    },
    Word {
        korean: "너무",
        english: "too / very",
        category: Category::Adverb,
    },
    Word {
        korean: "꼭",
        english: "definitely",
        category: Category::Adverb,
    },
    Word {
        korean: "바로",
        english: "right away",
        category: Category::Adverb,
    },
    Word {
        korean: "보통",
        english: "usually",
        category: Category::Adverb,
    },
    Word {
        korean: "친절히",
        english: "kindly",
        category: Category::Adverb,
    },
    Word {
        korean: "잘",
        english: "well / good at",
        category: Category::Adverb,
    },
    Word {
        korean: "간단히",
        english: "briefly",
        category: Category::Adverb,
    },
    Word {
        korean: "특별히",
        english: "especially",
        category: Category::Adverb,
    },
    Word {
        korean: "별로",
        english: "not very (negative)",
        category: Category::Adverb,
    },
    Word {
        korean: "또",
        english: "again / also",
        category: Category::Adverb,
    },
    Word {
        korean: "매일",
        english: "every day",
        category: Category::Adverb,
    },
    Word {
        korean: "매주",
        english: "every week",
        category: Category::Adverb,
    },
    Word {
        korean: "자주",
        english: "often",
        category: Category::Adverb,
    },
    Word {
        korean: "가끔",
        english: "sometimes",
        category: Category::Adverb,
    },
    Word {
        korean: "항상",
        english: "always",
        category: Category::Adverb,
    },
    Word {
        korean: "열심히",
        english: "diligently",
        category: Category::Adverb,
    },
    Word {
        korean: "조금",
        english: "a little",
        category: Category::Adverb,
    },
    Word {
        korean: "많이",
        english: "a lot",
        category: Category::Adverb,
    },
    Word {
        korean: "다",
        english: "everything",
        category: Category::Adverb,
    },
    Word {
        korean: "진짜",
        english: "really",
        category: Category::Adverb,
    },
    Word {
        korean: "정말",
        english: "truly",
        category: Category::Adverb,
    },
    Word {
        korean: "깨끗이",
        english: "cleanly",
        category: Category::Adverb,
    },
    Word {
        korean: "이제",
        english: "from now on",
        category: Category::Adverb,
    },
    Word {
        korean: "요즘",
        english: "these days",
        category: Category::Adverb,
    },
    Word {
        korean: "덜",
        english: "less",
        category: Category::Adverb,
    },
    Word {
        korean: "더",
        english: "more",
        category: Category::Adverb,
    },
    Word {
        korean: "지금",
        english: "now",
        category: Category::Adverb,
    },
    // ── EXPRESSIONS ───────────────────────────────────────────────────────────
    Word {
        korean: "필요 없어요",
        english: "I don't need it",
        category: Category::Expression,
    },
    Word {
        korean: "잘 먹었습니다",
        english: "Thank you for the meal (after eating)",
        category: Category::Expression,
    },
    Word {
        korean: "잘 먹겠습니다",
        english: "I will eat well (before eating)",
        category: Category::Expression,
    },
    Word {
        korean: "죄송해요",
        english: "I'm sorry (formal)",
        category: Category::Expression,
    },
    Word {
        korean: "어떡해요",
        english: "What should I do?",
        category: Category::Expression,
    },
    Word {
        korean: "수고하셨습니다",
        english: "Good job (thanks for your work)",
        category: Category::Expression,
    },
    Word {
        korean: "또 오세요",
        english: "Please come again",
        category: Category::Expression,
    },
    Word {
        korean: "또 봐요",
        english: "See you again",
        category: Category::Expression,
    },
    Word {
        korean: "우와",
        english: "Wow!",
        category: Category::Expression,
    },
    Word {
        korean: "이런",
        english: "Oh my",
        category: Category::Expression,
    },
    Word {
        korean: "늦었어요",
        english: "I'm late",
        category: Category::Expression,
    },
    Word {
        korean: "고장났어요",
        english: "It's broken",
        category: Category::Expression,
    },
    Word {
        korean: "배고파요",
        english: "I'm hungry",
        category: Category::Expression,
    },
    Word {
        korean: "있어요",
        english: "There is",
        category: Category::Expression,
    },
    Word {
        korean: "메리 크리스마스",
        english: "Merry Christmas",
        category: Category::Expression,
    },
    Word {
        korean: "생일 축하해요",
        english: "Happy birthday",
        category: Category::Expression,
    },
    Word {
        korean: "화이팅",
        english: "Go for it!",
        category: Category::Expression,
    },
    Word {
        korean: "마음에 들어요",
        english: "I like it",
        category: Category::Expression,
    },
    Word {
        korean: "잘하네요",
        english: "Well done",
        category: Category::Expression,
    },
    Word {
        korean: "어때요",
        english: "How about it?",
        category: Category::Expression,
    },
    Word {
        korean: "드릴까요",
        english: "Shall I give you?",
        category: Category::Expression,
    },
    Word {
        korean: "정말요",
        english: "Really?",
        category: Category::Expression,
    },
    Word {
        korean: "반가워요",
        english: "Nice to meet you",
        category: Category::Expression,
    },
    Word {
        korean: "환영해요",
        english: "Welcome!",
        category: Category::Expression,
    },
    Word {
        korean: "실례합니다",
        english: "Excuse me (formal)",
        category: Category::Expression,
    },
    Word {
        korean: "조심해요",
        english: "Be careful!",
        category: Category::Expression,
    },
    Word {
        korean: "잘 가요",
        english: "Take care",
        category: Category::Expression,
    },
    Word {
        korean: "감사합니다",
        english: "Thank you (formal)",
        category: Category::Expression,
    },
    Word {
        korean: "안녕하세요",
        english: "Hello (formal)",
        category: Category::Expression,
    },
    Word {
        korean: "주세요",
        english: "Please give me",
        category: Category::Expression,
    },
    Word {
        korean: "아",
        english: "Ah!",
        category: Category::Expression,
    },
    Word {
        korean: "음",
        english: "Hmm (thinking sound)",
        category: Category::Expression,
    },
    Word {
        korean: "와",
        english: "Wow",
        category: Category::Expression,
    },
    Word {
        korean: "네",
        english: "Yes",
        category: Category::Expression,
    },
    Word {
        korean: "아니요",
        english: "No",
        category: Category::Expression,
    },
    // ── GRAMMAR ───────────────────────────────────────────────────────────────
    Word {
        korean: "을",
        english: "object particle (after consonant)",
        category: Category::Grammar,
    },
    Word {
        korean: "를",
        english: "object particle (after vowel)",
        category: Category::Grammar,
    },
    Word {
        korean: "은",
        english: "topic particle (after consonant)",
        category: Category::Grammar,
    },
    Word {
        korean: "는",
        english: "topic particle (after vowel)",
        category: Category::Grammar,
    },
    Word {
        korean: "에서",
        english: "at / in / from (location/action particle)",
        category: Category::Grammar,
    },
    Word {
        korean: "으로",
        english: "toward / with / by means of (after consonant)",
        category: Category::Grammar,
    },
    Word {
        korean: "로",
        english: "toward / with / by means of (after vowel)",
        category: Category::Grammar,
    },
    Word {
        korean: "에",
        english: "at / on / in / to (location/time particle)",
        category: Category::Grammar,
    },
    Word {
        korean: "서",
        english: "standing / in (casual particle form)",
        category: Category::Grammar,
    },
    Word {
        korean: "으러",
        english: "in order to (go/come, after consonant)",
        category: Category::Grammar,
    },
    Word {
        korean: "러",
        english: "in order to (go/come, after vowel)",
        category: Category::Grammar,
    },
    Word {
        korean: "이나",
        english: "or / as many as",
        category: Category::Grammar,
    },
    Word {
        korean: "도",
        english: "too / also / even / degrees",
        category: Category::Grammar,
    },
    Word {
        korean: "랑",
        english: "and / with (casual)",
        category: Category::Grammar,
    },
    Word {
        korean: "이랑",
        english: "and / with (after consonant, casual)",
        category: Category::Grammar,
    },
    Word {
        korean: "의",
        english: "possessive particle ('s / of)",
        category: Category::Grammar,
    },
    Word {
        korean: "와",
        english: "and / with (formal, after vowel)",
        category: Category::Grammar,
    },
    Word {
        korean: "그러면",
        english: "then / in that case / if so",
        category: Category::Grammar,
    },
    Word {
        korean: "그런데",
        english: "however / but / by the way",
        category: Category::Grammar,
    },
    Word {
        korean: "안",
        english: "not / inside (negation prefix)",
        category: Category::Grammar,
    },
    Word {
        korean: "못",
        english: "can't / unable to (negation)",
        category: Category::Grammar,
    },
    Word {
        korean: "누구",
        english: "who (object form)",
        category: Category::Grammar,
    },
    Word {
        korean: "누가",
        english: "who (subject form)",
        category: Category::Grammar,
    },
    Word {
        korean: "어디",
        english: "where",
        category: Category::Grammar,
    },
    Word {
        korean: "언제",
        english: "when",
        category: Category::Grammar,
    },
    Word {
        korean: "뭐",
        english: "what (informal)",
        category: Category::Grammar,
    },
    Word {
        korean: "얼마",
        english: "how much / how many",
        category: Category::Grammar,
    },
    Word {
        korean: "몇",
        english: "how many / a few / which (number)",
        category: Category::Grammar,
    },
    Word {
        korean: "어떤",
        english: "which / what kind of",
        category: Category::Grammar,
    },
    Word {
        korean: "저것",
        english: "that (over there)",
        category: Category::Grammar,
    },
    Word {
        korean: "이것",
        english: "this (object)",
        category: Category::Grammar,
    },
    Word {
        korean: "저기",
        english: "over there",
        category: Category::Grammar,
    },
    Word {
        korean: "이쪽",
        english: "this way / this side",
        category: Category::Grammar,
    },
    Word {
        korean: "저쪽",
        english: "that way / over there",
        category: Category::Grammar,
    },
    Word {
        korean: "오른쪽",
        english: "right side / to the right",
        category: Category::Grammar,
    },
    Word {
        korean: "위",
        english: "above",
        category: Category::Grammar,
    },
    Word {
        korean: "밖",
        english: "outside",
        category: Category::Grammar,
    },
    Word {
        korean: "삼",
        english: "three (Sino-Korean)",
        category: Category::Grammar,
    },
    Word {
        korean: "다섯",
        english: "five (native Korean)",
        category: Category::Grammar,
    },
    Word {
        korean: "여섯",
        english: "six (native Korean)",
        category: Category::Grammar,
    },
    Word {
        korean: "서른",
        english: "thirty (native Korean)",
        category: Category::Grammar,
    },
    Word {
        korean: "열다섯",
        english: "fifteen (native Korean)",
        category: Category::Grammar,
    },
    Word {
        korean: "열아홉",
        english: "nineteen (native Korean)",
        category: Category::Grammar,
    },
    Word {
        korean: "살",
        english: "years old / to buy (native)",
        category: Category::Grammar,
    },
    Word {
        korean: "이에요",
        english: "is / am / are (after vowel, informal)",
        category: Category::Grammar,
    },
    Word {
        korean: "예요",
        english: "is / am / are (after vowel ending, informal)",
        category: Category::Grammar,
    },
    Word {
        korean: "가",
        english: "subject particle (after vowel) / to go",
        category: Category::Grammar,
    },
    Word {
        korean: "이",
        english: "subject particle (after consonant) / this",
        category: Category::Grammar,
    },
    Word {
        korean: "단",
        english: "sweet / but (conditional)",
        category: Category::Grammar,
    },
    Word {
        korean: "어느",
        english: "which",
        category: Category::Grammar,
    },
    Word {
        korean: "일",
        english: "one / first",
        category: Category::Grammar,
    },
    Word {
        korean: "넷",
        english: "four",
        category: Category::Grammar,
    },
    Word {
        korean: "열둘",
        english: "twelve",
        category: Category::Grammar,
    },
    Word {
        korean: "둘",
        english: "two",
        category: Category::Grammar,
    },
    Word {
        korean: "반",
        english: "half / thirty (minutes)",
        category: Category::Grammar,
    },
    Word {
        korean: "열",
        english: "ten",
        category: Category::Grammar,
    },
    Word {
        korean: "시",
        english: "o'clock",
        category: Category::Grammar,
    },
];
