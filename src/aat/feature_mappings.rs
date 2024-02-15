use ttf_parser::Tag;

use crate::aat::feature_selector::*;
use crate::aat::FeatureType;

pub struct FeatureMapping {
    pub ot_feature_tag: Tag,
    pub aat_feature_type: FeatureType,
    pub selector_to_enable: u8,
    pub selector_to_disable: u8,
}

impl FeatureMapping {
    const fn new(
        ot_feature_tag: &[u8; 4],
        aat_feature_type: FeatureType,
        selector_to_enable: u8,
        selector_to_disable: u8,
    ) -> Self {
        FeatureMapping {
            ot_feature_tag: Tag::from_bytes(ot_feature_tag),
            aat_feature_type,
            selector_to_enable,
            selector_to_disable,
        }
    }
}

/// Mapping from OpenType feature tags to AAT feature names and selectors.
///
/// Table data courtesy of Apple.
/// Converted from mnemonics to integers when moving to this file.
#[rustfmt::skip]
pub const FEATURE_MAPPINGS: &[FeatureMapping] = &[
    FeatureMapping::new(b"afrc", FeatureType::Fractions, VERTICAL_FRACTIONS, NO_FRACTIONS),
    FeatureMapping::new(b"c2pc", FeatureType::UpperCase, UPPER_CASE_PETITE_CAPS, DEFAULT_UPPER_CASE),
    FeatureMapping::new(b"c2sc", FeatureType::UpperCase, UPPER_CASE_SMALL_CAPS, DEFAULT_UPPER_CASE),
    FeatureMapping::new(b"calt", FeatureType::ContextualAlternatives, CONTEXTUAL_ALTERNATES_ON, CONTEXTUAL_ALTERNATES_OFF),
    FeatureMapping::new(b"case", FeatureType::CaseSensitiveLayout, CASE_SENSITIVE_LAYOUT_ON, CASE_SENSITIVE_LAYOUT_OFF),
    FeatureMapping::new(b"clig", FeatureType::Ligatures, CONTEXTUAL_LIGATURES_ON, CONTEXTUAL_LIGATURES_OFF),
    FeatureMapping::new(b"cpsp", FeatureType::CaseSensitiveLayout, CASE_SENSITIVE_SPACING_ON, CASE_SENSITIVE_SPACING_OFF),
    FeatureMapping::new(b"cswh", FeatureType::ContextualAlternatives, CONTEXTUAL_SWASH_ALTERNATES_ON, CONTEXTUAL_SWASH_ALTERNATES_OFF),
    FeatureMapping::new(b"dlig", FeatureType::Ligatures, RARE_LIGATURES_ON, RARE_LIGATURES_OFF),
    FeatureMapping::new(b"expt", FeatureType::CharacterShape, EXPERT_CHARACTERS, 16),
    FeatureMapping::new(b"frac", FeatureType::Fractions, DIAGONAL_FRACTIONS, NO_FRACTIONS),
    FeatureMapping::new(b"fwid", FeatureType::TextSpacing, MONOSPACED_TEXT, 7),
    FeatureMapping::new(b"halt", FeatureType::TextSpacing, ALT_HALF_WIDTH_TEXT, 7),
    FeatureMapping::new(b"hist", FeatureType::Dummy, 0, 1),
    FeatureMapping::new(b"hkna", FeatureType::AlternateKana, ALTERNATE_HORIZ_KANA_ON, ALTERNATE_HORIZ_KANA_OFF),
    FeatureMapping::new(b"hlig", FeatureType::Ligatures, HISTORICAL_LIGATURES_ON, HISTORICAL_LIGATURES_OFF),
    FeatureMapping::new(b"hngl", FeatureType::Transliteration, HANJA_TO_HANGUL, NO_TRANSLITERATION),
    FeatureMapping::new(b"hojo", FeatureType::CharacterShape, HOJO_CHARACTERS, 16),
    FeatureMapping::new(b"hwid", FeatureType::TextSpacing, HALF_WIDTH_TEXT, 7),
    FeatureMapping::new(b"ital", FeatureType::ItalicCjkRoman, CJK_ITALIC_ROMAN_ON, CJK_ITALIC_ROMAN_OFF),
    FeatureMapping::new(b"jp04", FeatureType::CharacterShape, JIS2004_CHARACTERS, 16),
    FeatureMapping::new(b"jp78", FeatureType::CharacterShape, JIS1978_CHARACTERS, 16),
    FeatureMapping::new(b"jp83", FeatureType::CharacterShape, JIS1983_CHARACTERS, 16),
    FeatureMapping::new(b"jp90", FeatureType::CharacterShape, JIS1990_CHARACTERS, 16),
    FeatureMapping::new(b"liga", FeatureType::Ligatures, COMMON_LIGATURES_ON, COMMON_LIGATURES_OFF),
    FeatureMapping::new(b"lnum", FeatureType::NumberCase, UPPER_CASE_NUMBERS, 2),
    FeatureMapping::new(b"mgrk", FeatureType::MathematicalExtras, MATHEMATICAL_GREEK_ON, MATHEMATICAL_GREEK_OFF),
    FeatureMapping::new(b"nlck", FeatureType::CharacterShape, NLCCHARACTERS, 16),
    FeatureMapping::new(b"onum", FeatureType::NumberCase, LOWER_CASE_NUMBERS, 2),
    FeatureMapping::new(b"ordn", FeatureType::VerticalPosition, ORDINALS, NORMAL_POSITION),
    FeatureMapping::new(b"palt", FeatureType::TextSpacing, ALT_PROPORTIONAL_TEXT, 7),
    FeatureMapping::new(b"pcap", FeatureType::LowerCase, LOWER_CASE_PETITE_CAPS, DEFAULT_LOWER_CASE),
    FeatureMapping::new(b"pkna", FeatureType::TextSpacing, PROPORTIONAL_TEXT, 7),
    FeatureMapping::new(b"pnum", FeatureType::NumberSpacing, PROPORTIONAL_NUMBERS, 4),
    FeatureMapping::new(b"pwid", FeatureType::TextSpacing, PROPORTIONAL_TEXT, 7),
    FeatureMapping::new(b"qwid", FeatureType::TextSpacing, QUARTER_WIDTH_TEXT, 7),
    FeatureMapping::new(b"ruby", FeatureType::RubyKana, RUBY_KANA_ON, RUBY_KANA_OFF),
    FeatureMapping::new(b"sinf", FeatureType::VerticalPosition, SCIENTIFIC_INFERIORS, NORMAL_POSITION),
    FeatureMapping::new(b"smcp", FeatureType::LowerCase, LOWER_CASE_SMALL_CAPS, DEFAULT_LOWER_CASE),
    FeatureMapping::new(b"smpl", FeatureType::CharacterShape, SIMPLIFIED_CHARACTERS, 16),
    FeatureMapping::new(b"ss01", FeatureType::StylisticAlternatives, STYLISTIC_ALT_ONE_ON, STYLISTIC_ALT_ONE_OFF),
    FeatureMapping::new(b"ss02", FeatureType::StylisticAlternatives, STYLISTIC_ALT_TWO_ON, STYLISTIC_ALT_TWO_OFF),
    FeatureMapping::new(b"ss03", FeatureType::StylisticAlternatives, STYLISTIC_ALT_THREE_ON, STYLISTIC_ALT_THREE_OFF),
    FeatureMapping::new(b"ss04", FeatureType::StylisticAlternatives, STYLISTIC_ALT_FOUR_ON, STYLISTIC_ALT_FOUR_OFF),
    FeatureMapping::new(b"ss05", FeatureType::StylisticAlternatives, STYLISTIC_ALT_FIVE_ON, STYLISTIC_ALT_FIVE_OFF),
    FeatureMapping::new(b"ss06", FeatureType::StylisticAlternatives, STYLISTIC_ALT_SIX_ON, STYLISTIC_ALT_SIX_OFF),
    FeatureMapping::new(b"ss07", FeatureType::StylisticAlternatives, STYLISTIC_ALT_SEVEN_ON, STYLISTIC_ALT_SEVEN_OFF),
    FeatureMapping::new(b"ss08", FeatureType::StylisticAlternatives, STYLISTIC_ALT_EIGHT_ON, STYLISTIC_ALT_EIGHT_OFF),
    FeatureMapping::new(b"ss09", FeatureType::StylisticAlternatives, STYLISTIC_ALT_NINE_ON, STYLISTIC_ALT_NINE_OFF),
    FeatureMapping::new(b"ss10", FeatureType::StylisticAlternatives, STYLISTIC_ALT_TEN_ON, STYLISTIC_ALT_TEN_OFF),
    FeatureMapping::new(b"ss11", FeatureType::StylisticAlternatives, STYLISTIC_ALT_ELEVEN_ON, STYLISTIC_ALT_ELEVEN_OFF),
    FeatureMapping::new(b"ss12", FeatureType::StylisticAlternatives, STYLISTIC_ALT_TWELVE_ON, STYLISTIC_ALT_TWELVE_OFF),
    FeatureMapping::new(b"ss13", FeatureType::StylisticAlternatives, STYLISTIC_ALT_THIRTEEN_ON, STYLISTIC_ALT_THIRTEEN_OFF),
    FeatureMapping::new(b"ss14", FeatureType::StylisticAlternatives, STYLISTIC_ALT_FOURTEEN_ON, STYLISTIC_ALT_FOURTEEN_OFF),
    FeatureMapping::new(b"ss15", FeatureType::StylisticAlternatives, STYLISTIC_ALT_FIFTEEN_ON, STYLISTIC_ALT_FIFTEEN_OFF),
    FeatureMapping::new(b"ss16", FeatureType::StylisticAlternatives, STYLISTIC_ALT_SIXTEEN_ON, STYLISTIC_ALT_SIXTEEN_OFF),
    FeatureMapping::new(b"ss17", FeatureType::StylisticAlternatives, STYLISTIC_ALT_SEVENTEEN_ON, STYLISTIC_ALT_SEVENTEEN_OFF),
    FeatureMapping::new(b"ss18", FeatureType::StylisticAlternatives, STYLISTIC_ALT_EIGHTEEN_ON, STYLISTIC_ALT_EIGHTEEN_OFF),
    FeatureMapping::new(b"ss19", FeatureType::StylisticAlternatives, STYLISTIC_ALT_NINETEEN_ON, STYLISTIC_ALT_NINETEEN_OFF),
    FeatureMapping::new(b"ss20", FeatureType::StylisticAlternatives, STYLISTIC_ALT_TWENTY_ON, STYLISTIC_ALT_TWENTY_OFF),
    FeatureMapping::new(b"subs", FeatureType::VerticalPosition, INFERIORS, NORMAL_POSITION),
    FeatureMapping::new(b"sups", FeatureType::VerticalPosition, SUPERIORS, NORMAL_POSITION),
    FeatureMapping::new(b"swsh", FeatureType::ContextualAlternatives, SWASH_ALTERNATES_ON, SWASH_ALTERNATES_OFF),
    FeatureMapping::new(b"titl", FeatureType::StyleOptions, TITLING_CAPS, NO_STYLE_OPTIONS),
    FeatureMapping::new(b"tnam", FeatureType::CharacterShape, TRADITIONAL_NAMES_CHARACTERS, 16),
    FeatureMapping::new(b"tnum", FeatureType::NumberSpacing, MONOSPACED_NUMBERS, 4),
    FeatureMapping::new(b"trad", FeatureType::CharacterShape, TRADITIONAL_CHARACTERS, 16),
    FeatureMapping::new(b"twid", FeatureType::TextSpacing, THIRD_WIDTH_TEXT, 7),
    FeatureMapping::new(b"unic", FeatureType::LetterCase, 14, 15),
    FeatureMapping::new(b"valt", FeatureType::TextSpacing, ALT_PROPORTIONAL_TEXT, 7),
    FeatureMapping::new(b"vert", FeatureType::VerticalSubstitution, SUBSTITUTE_VERTICAL_FORMS_ON, SUBSTITUTE_VERTICAL_FORMS_OFF),
    FeatureMapping::new(b"vhal", FeatureType::TextSpacing, ALT_HALF_WIDTH_TEXT, 7),
    FeatureMapping::new(b"vkna", FeatureType::AlternateKana, ALTERNATE_VERT_KANA_ON, ALTERNATE_VERT_KANA_OFF),
    FeatureMapping::new(b"vpal", FeatureType::TextSpacing, ALT_PROPORTIONAL_TEXT, 7),
    FeatureMapping::new(b"vrt2", FeatureType::VerticalSubstitution, SUBSTITUTE_VERTICAL_FORMS_ON, SUBSTITUTE_VERTICAL_FORMS_OFF),
    FeatureMapping::new(b"vrtr", FeatureType::VerticalSubstitution, 2, 3),
    FeatureMapping::new(b"zero", FeatureType::TypographicExtras, SLASHED_ZERO_ON, SLASHED_ZERO_OFF),
];
