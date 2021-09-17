mod czech;
mod italian;
mod french;
mod english;
mod spanish;
mod portuguese;
mod japanese;
mod korean;
mod chinese_traditional;
mod chinese_simplified;
pub mod langs{
    use super::czech::CZECH_WORDS;
    use super::italian::ITALIAN_WORDS;
    use super::french::FRENCH_WORDS;
    use super::english::ENGLISH_WORDS;
    use super::spanish::SPANISH_WORDS;
    use super::portuguese::PORTUGUESE_WORDS;
    use super::japanese::JAPANESE_WORDS;
    use super::korean::KOREAN_WORDS;
    use super::chinese_traditional::CHINESE_TRADITIONAL_WORDS;
    use super::chinese_simplified::CHINESE_SIMPLIFIED_WORDS;

    pub static CZECH: [&str;2048] = CZECH_WORDS;
    pub static ITALIAN: [&str;2048] = ITALIAN_WORDS;
    pub static FRENCH: [&str;2048] = FRENCH_WORDS;
    pub static ENGLISH: [&str;2048] = ENGLISH_WORDS;
    pub static SPANISH: [&str;2048] = SPANISH_WORDS;
    pub static PORTUGUESE: [&str;2048] = PORTUGUESE_WORDS;
    pub static JAPANESE: [&str;2048] = JAPANESE_WORDS;
    pub static KOREAN: [&str;2048] = KOREAN_WORDS;
    pub static CHINESE_TRADITIONAL: [&str;2048] = CHINESE_TRADITIONAL_WORDS;
    pub static CHINESE_SIMPLIFIED: [&str;2048] = CHINESE_SIMPLIFIED_WORDS;
}