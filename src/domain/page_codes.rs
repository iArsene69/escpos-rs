//! List of page codes

use crate::domain::PageCode;
use crate::errors::PrinterError;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::iter::{IntoIterator, Iterator};

/// Page codes table list
#[derive(Debug, Clone, Copy)]
pub(crate) enum PageCodeTable {
    PC437,
    Katakana,
    PC850,
    PC852,
    PC858,
    PC860,
    PC863,
    PC865,
    PC851,
    PC853,
    PC857,
    PC737,
    ISO8859_2,
    ISO8859_7,
    ISO8859_15,
    WPC1252,
    PC866,
    WPC775,
    PC855,
    PC861,
    PC862,
    PC869,
    PC1118,
    PC1119,
    PC1125,
    WPC1250,
    WPC1251,
    WPC1253,
    WPC1254,
    WPC1257,
    KZ1048,
}

impl PageCodeTable {
    /// Get the table for the page code
    pub(crate) fn get_table(&self) -> &HashMap<char, u8> {
        match self {
            Self::PC437 => &PC437_TABLE,
            Self::Katakana => &KATAKANA_TABLE,
            Self::PC850 => &PC850_TABLE,
            Self::PC852 => &PC852_TABLE,
            Self::PC858 => &PC858_TABLE,
            Self::PC860 => &PC860_TABLE,
            Self::PC863 => &PC863_TABLE,
            Self::PC865 => &PC865_TABLE,
            Self::PC851 => &PC851_TABLE,
            Self::PC853 => &PC853_TABLE,
            Self::PC857 => &PC857_TABLE,
            Self::PC737 => &PC737_TABLE,
            Self::ISO8859_2 => &ISO8859_2_TABLE,
            Self::ISO8859_7 => &ISO8859_7_TABLE,
            Self::ISO8859_15 => &ISO8859_15_TABLE,
            Self::WPC1252 => &WPC1252_TABLE,
            Self::PC866 => &PC866_TABLE,
            Self::WPC775 => &WPC775_TABLE,
            Self::PC855 => &PC855_TABLE,
            Self::PC861 => &PC861_TABLE,
            Self::PC862 => &PC862_TABLE,
            Self::PC869 => &PC869_TABLE,
            Self::PC1118 => &PC1118_TABLE,
            Self::PC1119 => &PC1119_TABLE,
            Self::PC1125 => &PC1125_TABLE,
            Self::WPC1250 => &WPC1250_TABLE,
            Self::WPC1251 => &WPC1251_TABLE,
            Self::WPC1253 => &WPC1253_TABLE,
            Self::WPC1254 => &WPC1254_TABLE,
            Self::WPC1257 => &WPC1257_TABLE,
            Self::KZ1048 => &KZ1048_TABLE,
        }
    }
}

impl TryFrom<PageCode> for PageCodeTable {
    type Error = PrinterError;

    fn try_from(value: PageCode) -> Result<Self, Self::Error> {
        match value {
            PageCode::PC437 => Ok(Self::PC437),
            PageCode::Katakana => Ok(Self::Katakana),
            PageCode::PC850 => Ok(Self::PC850),
            PageCode::PC852 => Ok(Self::PC852),
            PageCode::PC858 => Ok(Self::PC858),
            PageCode::PC860 => Ok(Self::PC860),
            PageCode::PC863 => Ok(Self::PC863),
            PageCode::PC865 => Ok(Self::PC865),
            PageCode::PC851 => Ok(Self::PC851),
            PageCode::PC853 => Ok(Self::PC853),
            PageCode::PC857 => Ok(Self::PC857),
            PageCode::PC737 => Ok(Self::PC737),
            PageCode::ISO8859_2 => Ok(Self::ISO8859_2),
            PageCode::ISO8859_7 => Ok(Self::ISO8859_7),
            PageCode::ISO8859_15 => Ok(Self::ISO8859_15),
            PageCode::WPC1252 => Ok(Self::WPC1252),
            PageCode::PC866 => Ok(Self::PC866),
            PageCode::WPC775 => Ok(Self::WPC775),
            PageCode::PC855 => Ok(Self::PC855),
            PageCode::PC861 => Ok(Self::PC861),
            PageCode::PC862 => Ok(Self::PC862),
            PageCode::PC869 => Ok(Self::PC869),
            PageCode::PC1118 => Ok(Self::PC1118),
            PageCode::PC1119 => Ok(Self::PC1119),
            PageCode::PC1125 => Ok(Self::PC1125),
            PageCode::WPC1250 => Ok(Self::WPC1250),
            PageCode::WPC1251 => Ok(Self::WPC1251),
            PageCode::WPC1253 => Ok(Self::WPC1253),
            PageCode::WPC1254 => Ok(Self::WPC1254),
            PageCode::WPC1257 => Ok(Self::WPC1257),
            PageCode::KZ1048 => Ok(Self::KZ1048),
            _ => Err(PrinterError::Input(format!("no table for this page code: {value}"))),
        }
    }
}

lazy_static! {
    /// PC437 Page code table
    static ref PC437_TABLE: HashMap<char, u8> = [
        'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å',
        'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', '¢', '£', '¥', '₧', 'ƒ',
        'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '⌐', '¬', '½', '¼', '¡', '«', '»',
        '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐',
        '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧',
        '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀',
        'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩',
        '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// Katakana Page code table (CP932 or IBM-932)
    static ref KATAKANA_TABLE: HashMap<char, u8> = [
             '｡', '｢', '｣', '､', '･', 'ｦ', 'ｧ', 'ｨ', 'ｩ', 'ｪ', 'ｫ', 'ｬ', 'ｭ', 'ｮ', 'ｯ',
        'ｰ', 'ｱ', 'ｲ', 'ｳ', 'ｴ', 'ｵ', 'ｶ', 'ｷ', 'ｸ', 'ｹ', 'ｺ', 'ｻ', 'ｼ', 'ｽ', 'ｾ', 'ｿ',
        'ﾀ', 'ﾁ', 'ﾂ', 'ﾃ', 'ﾄ', 'ﾅ', 'ﾆ', 'ﾇ', 'ﾈ', 'ﾉ', 'ﾊ', 'ﾋ', 'ﾌ', 'ﾍ', 'ﾎ', 'ﾏ',
        'ﾐ', 'ﾑ', 'ﾒ', 'ﾓ', 'ﾔ', 'ﾕ', 'ﾖ', 'ﾗ', 'ﾘ', 'ﾙ', 'ﾚ', 'ﾛ', 'ﾜ', 'ﾝ', 'ﾞ', 'ﾟ',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 0xA1) as u8))
    .collect();

    /// PC850 Page code table
    static ref PC850_TABLE: HashMap<char, u8> = [
        'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å',
        'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', 'ø', '£', 'Ø', '×', 'ƒ',
        'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '®', '¬', '½', '¼', '¡', '«', '»',
        '░', '▒', '▓', '│', '┤', 'Á', 'Â', 'À', '©', '╣', '║', '╗', '╝', '¢', '¥', '┐',
        '└', '┴', '┬', '├', '─', '┼', 'ã', 'Ã', '╚', '╔', '╩', '╦', '╠', '═', '╬', '¤',
        'ð', 'Ð', 'Ê', 'Ë', 'È', 'ı', 'Í', 'Î', 'Ï', '┘', '┌', '█', '▄', '¦', 'Ì', '▀',
        'Ó', 'ß', 'Ô', 'Ò', 'õ', 'Õ', 'µ', 'þ', 'Þ', 'Ú', 'Û', 'Ù', 'ý', 'Ý', '¯', '´',
        '-', '±', '‗', '¾', '¶', '§', '÷', '¸', '°', '¨', '·', '¹', '³', '²', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// PC863 Page code table
    static ref PC863_TABLE: HashMap<char, u8> = [
        'Ç', 'ü', 'é', 'â', 'Â', 'à', '¶', 'ç', 'ê', 'ë', 'è', 'ï', 'î', '‗', 'À', '§',
        'É', 'È', 'Ê', 'ô', 'Ë', 'Ï', 'û', 'ù', '¤', 'Ô', 'Ü', '¢', '£', 'Ù', 'Û', 'ƒ',
        '¦', '´', 'ó', 'ú', '¨', '¸', '³', '¯', 'Î', '⌐', '¬', '½', '¼', '¾', '«', '»',
        '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐',
        '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧',
        '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀',
        'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩',
        '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// PC852 Page code table
    static ref PC852_TABLE: HashMap<char, u8> = [
        'Ç', 'ü', 'é', 'â', 'ä', 'ů', 'ć', 'ç', 'ł', 'ë', 'Ő', 'ő', 'î', 'Ź', 'Ä', 'Ć',
        'É', 'Ĺ', 'ĺ', 'ô', 'ö', 'Ľ', 'ľ', 'Ś', 'ś', 'Ö', 'Ü', 'Ť', 'ť', 'Ł', '×', 'č',
        'á', 'í', 'ó', 'ú', 'Ą', 'ą', 'Ž', 'ž', 'Ę', 'ę', '¬', 'ź', 'Č', 'ş', '«', '»',
        '░', '▒', '▓', '│', '┤', 'Á', 'Â', 'Ě', 'Ş', '╣', '║', '╗', '╝', 'Ż', 'ż', '┐',
        '└', '┴', '┬', '├', '─', '┼', 'Ă', 'ă', '╚', '╔', '╩', '╦', '╠', '═', '╬', '¤',
        'đ', 'Đ', 'Ď', 'Ë', 'ď', 'Ň', 'Í', 'Î', 'ě', '┘', '┌', '█', '▄', 'Ţ', 'Ů', '▀',
        'Ó', 'ß', 'Ô', 'Ń', 'ń', 'ň', 'Š', 'š', 'Ŕ', 'Ú', 'ŕ', 'Ű', 'ý', 'Ý', 'ţ', '´',
        '\u{AD}', '˝', '˛', 'ˇ', '˘', '§', '÷', '¸', '°', '¨', '˙', 'ű', 'Ř', 'ř', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// PC858 Page code table
    static ref PC858_TABLE: HashMap<char, u8> = [
        'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å',
        'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', 'ø', '£', 'Ø', '×', 'ƒ',
        'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '®', '⌐', '¬', '½', '¼', '¡', '«', '»',
        '░', '▒', '▓', '│', '┤', 'Á', 'Â', 'À', '©', '╣', '║', '╗', '╝', '¢', '¥', '┐',
        '└', '┴', '┬', '├', '─', '┼', 'ã', 'Ã', '╚', '╔', '╩', '╦', '╠', '═', '╬', '¤',
        'ð', 'Ð', 'Ê', 'Ë', 'È', '€', 'Í', 'Î', 'Ï', '┘', '┌', '█', '▄', '¦', 'Ì', '▀',
        'Ó', 'ß', 'Ô', 'Ô', 'õ', 'Õ', 'µ', 'þ', 'Þ', 'Ú', 'Û', 'Ù', 'ý', 'Ý', '¯', '´',
        '-', '±', '‗', '¾', '¶', '§', '÷', '¸', '°', '¨', '·', '¹', '³', '²', '■', '\u{00A0}']
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// PC860 Page code table
    static ref PC860_TABLE: HashMap<char, u8> = [
        'Ç', 'ü', 'é', 'â', 'ã', 'à', 'Á', 'ç', 'ê', 'Ê', 'è', 'Í', 'Ô', 'ì', 'Ã', 'Â',
        'É', 'À', 'È', 'ô', 'õ', 'ò', 'Ú', 'ù', 'Ì', 'Õ', 'Ü', '¢', '£', 'Ù', '₧', 'Ó',
        'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', 'Ò', '¬', '½', '¼', '¡', '«', '»',
        '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐',
        '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧',
        '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀',
        'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩',
        '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// PC865 Page code table
    static ref PC865_TABLE: HashMap<char, u8> = [
        'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å',
        'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', 'ø', '£', 'Ø', '₧', 'ƒ',
        'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '⌐', '¬', '½', '¼', '¡', '«', '¤',
        '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐',
        '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧',
        '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀',
        'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩',
        '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// PC851 Page code table
    /// Uses '\0' as placeholder for empty spots
    static ref PC851_TABLE: HashMap<char, u8> = [
        'Ç', 'ü', 'é', 'â', 'ä', 'à', 'Ά', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'Έ', 'Ä', 'Ή',
        'Ί', '\0', 'Ό', 'ô', 'ö', 'Ύ', 'û', 'ù', 'Ώ', 'Ö', 'Ü', 'ά', '£', 'έ', 'ή', 'ί',
        'ϊ', 'ΐ', 'ό', 'ύ', 'Α', 'Β', 'Γ', 'Δ', 'Ε', 'Ζ', 'Η', '½', 'Θ', 'Ι', '«', '»',
        '░', '▒', '▓', '│', '┤', 'Κ', 'Λ', 'Μ', 'Ν', '╣', '║', '╗', '╝', 'Ξ', 'Ο', '┐',
        '└', '┴', '┬', '├', '─', '┼', 'Π', 'Ρ', '╚', '╔', '╩', '╦', '╠', '═', '╬', 'Σ',
        'Τ', 'Υ', 'Φ', 'Χ', 'Ψ', 'Ω', 'α', 'β', 'γ', '┘', '┌', '█', '▄', 'δ', 'ε', '▀',
        'ζ', 'η', 'θ', 'ι', 'κ', 'λ', 'μ', 'ν', 'ξ', 'ο', 'π', 'ρ', 'σ', 'ς', 'τ', '´',
        '-', '±', 'υ', 'φ', 'χ', '§', 'ψ', '¸', '°', '¨', 'ω', 'ϋ', 'ΰ', 'ώ', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .filter(|(_, c)| *c != '\0')
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// PC853 Page code table
    /// Uses '\0' as placeholder for empty spots
    static ref PC853_TABLE: HashMap<char, u8> = [
        'Ç', 'ü', 'é', 'â', 'ä', 'à', 'ĉ', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Ĉ',
        'É', 'ċ', 'Ċ', 'ô', 'ö', 'ò', 'û', 'ù', 'İ', 'Ö', 'Ü', 'ĝ', '£', 'Ĝ', '×', 'ĵ',
        'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'Ğ', 'ğ', 'Ĥ', 'ĥ', '\0', '½', 'Ĵ', 'ş', '«', '»',
        '░', '▒', '▓', '│', '┤', 'Á', 'Â', 'À', 'Ş', '╣', '║', '╗', '╝', 'Ż', 'ż', '┐',
        '└', '┴', '┬', '├', '─', '┼', 'Ŝ', 'ŝ', '╚', '╔', '╩', '╦', '╠', '═', '╬', '¤',
        '\0', '\0', 'Ê', 'Ë', 'È', 'ı', 'Í', 'Î', 'Ï', '┘', '┌', '█', '▄', '\0', 'Ì', '▀',
        'Ó', 'ß', 'Ô', 'Ò', 'Ġ', 'ġ', 'µ', 'Ħ', 'ħ', 'Ú', 'Û', 'Ù', 'Ŭ', 'ŭ', '·', '´',
        '-', '\0', 'ℓ', 'ŉ', '˘', '§', '÷', '¸', '°', '¨', '˙', '\0', '³', '²', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .filter(|(_, c)| *c != '\0')
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// PC857 Page code table
    /// Uses '\0' as placeholder for empty spots
    static ref PC857_TABLE: HashMap<char, u8> = [
        'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ı', 'Ä', 'Å',
        'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'İ', 'Ö', 'Ü', 'ø', '£', 'Ø', 'Ş', 'ş',
        'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'Ğ', 'ğ', '¿', '®', '¬', '½', '¼', '¡', '«', '»',
        '░', '▒', '▓', '│', '┤', 'Á', 'Â', 'À', '©', '╣', '║', '╗', '╝', '¢', '¥', '┐',
        '└', '┴', '┬', '├', '─', '┼', 'ã', 'Ã', '╚', '╔', '╩', '╦', '╠', '═', '╬', '¤',
        'º', 'ª', 'Ê', 'Ë', 'È', '€', 'Í', 'Î', 'Ï', '┘', '┌', '█', '▄', '¦', 'Ì', '▀',
        'Ó', 'ß', 'Ô', 'Ò', 'õ', 'Õ', 'µ', '.', '×', 'Ú', 'Û', 'Ù', 'ì', 'ÿ', '¯', '´',
        '-', '±', '\0', '¾', '¶', '§', '÷', '¸', '°', '¨', '·', '¹', '³', '²', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .filter(|(_, c)| *c != '\0')
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// PC737 Page code table
    static ref PC737_TABLE: HashMap<char, u8> = [
        'Α', 'Β', 'Γ', 'Δ', 'Ε', 'Ζ', 'Η', 'Θ', 'Ι', 'Κ', 'Λ', 'Μ', 'Ν', 'Ξ', 'Ο', 'Π',
        'Ρ', 'Σ', 'Τ', 'Υ', 'Φ', 'Χ', 'Ψ', 'Ω', 'α', 'β', 'γ', 'δ', 'ε', 'ζ', 'η', 'θ',
        'ι', 'κ', 'λ', 'μ', 'ν', 'ξ', 'ο', 'π', 'ρ', 'σ', 'ς', 'τ', 'υ', 'φ', 'χ', 'ψ',
        '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐',
        '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧',
        '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀',
        'ω', 'ά', 'έ', 'ή', 'ϊ', 'ί', 'ό', 'ύ', 'ϋ', 'ώ', 'Ά', 'Έ', 'Ή', 'Ί', 'Ό', 'Ύ',
        'Ώ', '±', '≥', '≤', 'Ϊ', 'Ϋ', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// ISO8859_2 Page code table
    static ref ISO8859_2_TABLE: HashMap<char, u8> = [
        '\u{00A0}', // NO-BREAK SPACE
        'Ą', '˘', 'Ł', '¤', 'Ľ', 'Ś', '§', '¨', 'Š', 'Ş', 'Ť', 'Ź',
        '\u{00AD}', // SOFT HYPHEN
        'Ž', 'Ż',
        '°', 'ą', '˛', 'ł', '´', 'ľ', 'ś', 'ˇ', '¸', 'š', 'ş', 'ť', 'ź', '˝', 'ž', 'ż',
        'Ŕ', 'Á', 'Â', 'Ă', 'Ä', 'Ĺ', 'Ć', 'Ç', 'Č', 'É', 'Ę', 'Ë', 'Ě', 'Í', 'Î', 'Ď',
        'Đ', 'Ń', 'Ň', 'Ó', 'Ô', 'Ő', 'Ö', '×', 'Ř', 'Ů', 'Ú', 'Ű', 'Ü', 'Ý', 'Ţ', 'ß',
        'ŕ', 'á', 'â', 'ă', 'ä', 'ĺ', 'ć', 'ç', 'č', 'é', 'ę', 'ë', 'ě', 'í', 'î', 'ď',
        'đ', 'ń', 'ň', 'ó', 'ô', 'ő', 'ö', '÷', 'ř', 'ů', 'ú', 'ű', 'ü', 'ý', 'ţ', '˙',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 0xA0) as u8))
    .collect();

    /// ISO8859_7 Page code table
    /// Uses '\0' as placeholder for empty spots
    static ref ISO8859_7_TABLE: HashMap<char, u8> = [
        '\u{00A0}', // NO-BREAK SPACE
        '‘', '’', '£', '€', '₯', '¦', '§', '¨', '©', 'ͺ', '«', '¬',
        '\u{00AD}', // SOFT HYPHEN
        '\0', '―',
        '°', '±', '²', '³', '΄', '΅', 'Ά', '·', 'Έ', 'Ή', 'Ί', '»', 'Ό', '½', 'Ύ', 'Ώ',
        'ΐ', 'Α', 'Β', 'Γ', 'Δ', 'Ε', 'Ζ', 'Η', 'Θ', 'Ι', 'Κ', 'Λ', 'Μ', 'Ν', 'Ξ', 'Ο',
        'Π', 'Ρ', '\0', 'Σ', 'Τ', 'Υ', 'Φ', 'Χ', 'Ψ', 'Ω', 'Ϊ', 'Ϋ', 'ά', 'έ', 'ή', 'ί',
        'ΰ', 'α', 'β', 'γ', 'δ', 'ε', 'ζ', 'η', 'θ', 'ι', 'κ', 'λ', 'μ', 'ν', 'ξ', 'ο',
        'π', 'ρ', 'ς', 'σ', 'τ', 'υ', 'φ', 'χ', 'ψ', 'ω', 'ϊ', 'ϋ', 'ό', 'ύ', 'ώ',
    ]
    .into_iter().enumerate()
    .filter(|(_, c)| *c != '\0')
    .map(|(i, c)| (c, (i + 0xA0) as u8))
    .collect();

    /// ISO8859_15 Page code table
    static ref ISO8859_15_TABLE: HashMap<char, u8> = [
        '\u{00A0}', // NO-BREAK SPACE
        '¡', '¢', '£', '€', '¥', 'Š', '§', 'š', '©', 'ª', '«', '¬',
        '\u{00AD}', // SOFT HYPHEN
        '®', '¯',
        '°', '±', '²', '³', 'Ž', 'µ', '¶', '·', 'ž', '¹', 'º', '»', 'Œ', 'œ', 'Ÿ', '¿',
        'À', 'Á', 'Â', 'Ã', 'Ä', 'Å', 'Æ', 'Ç', 'È', 'É', 'Ê', 'Ë', 'Ì', 'Í', 'Î', 'Ï',
        'Ð', 'Ñ', 'Ò', 'Ó', 'Ô', 'Õ', 'Ö', '×', 'Ø', 'Ù', 'Ú', 'Û', 'Ü', 'Ý', 'Þ', 'ß',
        'à', 'á', 'â', 'ã', 'ä', 'å', 'æ', 'ç', 'è', 'é', 'ê', 'ë', 'ì', 'í', 'î', 'ï',
        'ð', 'ñ', 'ò', 'ó', 'ô', 'õ', 'ö', '÷', 'ø', 'ù', 'ú', 'û', 'ü', 'ý', 'þ', 'ÿ',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 0xA0) as u8))
    .collect();

    /// WPC1252 Page code table
    /// Uses '\0' as placeholder for empty spots
    static ref WPC1252_TABLE: HashMap<char, u8> = [
        '€', '\0', '‚', 'ƒ', '„', '…', '†', '‡', 'ˆ', '‰', 'Š', '‹', 'Œ', '\0', 'Ž', '\0',
        '\0', '‘', '’', '“', '”', '•', '–', '—', '˜', '™', 'š', '›', 'œ', '\0', 'ž', 'Ÿ',
        '\u{00A0}', '¡', '¢', '£', '¤', '¥', '¦', '§', '¨', '©', 'ª', '«', '¬', '\u{00AD}', '®', '¯',
        '°', '±', '²', '³', '´', 'µ', '¶', '·', '¸', '¹', 'º', '»', '¼', '½', '¾', '¿',
        'À', 'Á', 'Â', 'Ã', 'Ä', 'Å', 'Æ', 'Ç', 'È', 'É', 'Ê', 'Ë', 'Ì', 'Í', 'Î', 'Ï',
        'Ð', 'Ñ', 'Ò', 'Ó', 'Ô', 'Õ', 'Ö', '×', 'Ø', 'Ù', 'Ú', 'Û', 'Ü', 'Ý', 'Þ', 'ß',
        'à', 'á', 'â', 'ã', 'ä', 'å', 'æ', 'ç', 'è', 'é', 'ê', 'ë', 'ì', 'í', 'î', 'ï',
        'ð', 'ñ', 'ò', 'ó', 'ô', 'õ', 'ö', '÷', 'ø', 'ù', 'ú', 'û', 'ü', 'ý', 'þ', 'ÿ',
    ]
    .into_iter().enumerate()
    .filter(|(_, c)| *c != '\0')
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// PC866 Page code table
    static ref PC866_TABLE: HashMap<char, u8> = [
        'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П',
        'Р', 'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я',
        'а', 'б', 'в', 'г', 'д', 'е', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н', 'о', 'п',
        '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐',
        '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧',
        '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀',
        'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э', 'ю', 'я',
        'Ё', 'ё', 'Є', 'є', 'Ї', 'ї', 'Ў', 'ў', '°', '∙', '·', '√', '№', '¤', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// WPC775 Page code table
    static ref WPC775_TABLE: HashMap<char, u8> = [
        'Ć', 'ü', 'é', 'ā', 'ä', 'ģ', 'å', 'ć', 'ł', 'ē', 'Ŗ', 'ŗ', 'ī', 'Ź', 'Ä', 'Å',
        'É', 'æ', 'Æ', 'ō', 'ö', 'Ģ', '¢', 'Ś', 'ś', 'Ö', 'Ü', 'ø', '£', 'Ø', '×', '¤',
        'Ā', 'Ī', 'ó', 'Ż', 'ż', 'ź', '”', '¦', '©', '®', '¬', '½', '¼', 'Ł', '«', '»',
        '░', '▒', '▓', '│', '┤', 'Ą', 'Č', 'Ę', 'Ė', '╣', '║', '╗', '╝', 'Į', 'Š', '┐',
        '└', '┴', '┬', '├', '─', '┼', 'Ų', 'Ū', '╚', '╔', '╩', '╦', '╠', '═', '╬', 'Ž',
        'ą', 'č', 'ę', 'ė', 'į', 'š', 'ų', 'ū', 'ž', '┘', '┌', '█', '▄', '▌', '▐', '▀',
        'Ó', 'ß', 'Ō', 'Ń', 'õ', 'Õ', 'µ', 'ń', 'Ķ', 'ķ', 'Ļ', 'ļ', 'ņ', 'Ē', 'Ņ', '’',
        '-', '±', '“', '¾', '¶', '§', '÷', '„', '°', '∙', '·', '¹', '³', '²', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// PC855 Page code table
    static ref PC855_TABLE: HashMap<char, u8> = [
        'ђ', 'Ђ', 'ѓ', 'Ѓ', 'ё', 'Ё', 'є', 'Є', 'ѕ', 'Ѕ', 'і', 'І', 'ї', 'Ї', 'ј', 'Ј',
        'љ', 'Љ', 'њ', 'Њ', 'ћ', 'Ћ', 'ќ', 'Ќ', 'ў', 'Ў', 'џ', 'Џ', 'ю', 'Ю', 'ъ', 'Ъ',
        'а', 'А', 'б', 'Б', 'ц', 'Ц', 'д', 'Д', 'е', 'Е', 'ф', 'Ф', 'г', 'Г', '«', '»',
        '░', '▒', '▓', '│', '┤', 'х', 'Х', 'и', 'И', '╣', '║', '╗', '╝', 'й', 'Й', '┐',
        '└', '┴', '┬', '├', '─', '┼', 'к', 'К', '╚', '╔', '╩', '╦', '╠', '═', '╬', '¤',
        'л', 'Л', 'м', 'М', 'н', 'Н', 'о', 'О', 'п', '┘', '┌', '█', '▄', 'П', 'я', '▀',
        'Я', 'р', 'Р', 'с', 'С', 'т', 'Т', 'у', 'У', 'ж', 'Ж', 'в', 'В', 'ь', 'Ь', '№',
        '-', 'ы', 'Ы', 'з', 'З', 'ш', 'Ш', 'э', 'Э', 'щ', 'Щ', 'ч', 'Ч', '§', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// PC861 Page code table
    static ref PC861_TABLE: HashMap<char, u8> = [
        'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'Ð', 'ð', 'Þ', 'Ä', 'Å',
        'É', 'æ', 'Æ', 'ô', 'ö', 'þ', 'û', 'Ý', 'ý', 'Ö', 'Ü', 'ø', '£', 'Ø', '₧', 'ƒ',
        'á', 'í', 'ó', 'ú', 'Á', 'Í', 'Ó', 'Ú', '¿', '⌐', '¬', '½', '¼', '¡', '«', '»',
        '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐',
        '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧',
        '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀',
        'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩',
        '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// PC862 Page code table
    static ref PC862_TABLE: HashMap<char, u8> = [
        'א', 'ב', 'ג', 'ד', 'ה', 'ו', 'ז', 'ח', 'ט', 'י', 'ך', 'כ', 'ל', 'ם', 'מ', 'ן',
        'נ', 'ס', 'ע', 'ף', 'פ', 'ץ', 'צ', 'ק', 'ר', 'ש', 'ת', '¢', '£', '¥', '₧', 'ƒ',
        'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '⌐', '¬', '½', '¼', '¡', '«', '»',
        '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐',
        '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧',
        '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀',
        'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩',
        '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// PC869 Page code table
    /// Uses '\0' as placeholder for empty spots
    static ref PC869_TABLE: HashMap<char, u8> = [
        'Ά', '€', '·', '¬', '¦', '‘', '’', 'Έ', '―', 'Ή',
        'Ί', 'Ϊ', 'Ό', '\0', '\0', 'Ύ', 'Ϋ', '©', 'Ώ', '²', '³', 'ά', '£', 'έ', 'ή', 'ί',
        'ϊ', 'ΐ', 'ό', 'ύ', 'Α', 'Β', 'Γ', 'Δ', 'Ε', 'Ζ', 'Η', '½', 'Θ', 'Ι', '«', '»',
        '░', '▒', '▓', '│', '┤', 'Κ', 'Λ', 'Μ', 'Ν', '╣', '║', '╗', '╝', 'Ξ', 'Ο', '┐',
        '└', '┴', '┬', '├', '─', '┼', 'Π', 'Ρ', '╚', '╔', '╩', '╦', '╠', '═', '╬', 'Σ',
        'Τ', 'Υ', 'Φ', 'Χ', 'Ψ', 'Ω', 'α', 'β', 'γ', '┘', '┌', '█', '▄', 'δ', 'ε', '▀',
        'ζ', 'η', 'θ', 'ι', 'κ', 'λ', 'μ', 'ν', 'ξ', 'ο', 'π', 'ρ', 'σ', 'ς', 'τ', '΄',
        '-', '±', 'υ', 'φ', 'χ', '§', 'ψ', '΅', '°', '¨', 'ω', 'ϋ', 'ΰ', 'ώ', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .filter(|(_, c)| *c != '\0')
    .map(|(i, c)| (c, (i + 0x86) as u8))
    .collect();

    /// PC1118 Page code table
    static ref PC1118_TABLE: HashMap<char, u8> = [
        'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å',
        'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', '¢', '£', '¥', '₧', 'ƒ',
        'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '⌐', '¬', '½', '¼', '¡', '«', '»',
        '░', '▒', '▓', '│', '┤', 'Ą', 'Č', 'Ę', 'Ė', '╣', '║', '╗', '╝', 'Į', 'Š', '┐',
        '└', '┴', '┬', '├', '─', '┼', 'Ų', 'Ū', '╚', '╔', '╩', '╦', '╠', '═', '╬', 'Ž',
        'ą', 'č', 'ę', 'ė', 'į', 'š', 'ų', 'ū', 'ž', '┘', '┌', '█', '▄', '▌', '▐', '▀',
        'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩',
        '≡', '±', '≥', '≤', '„', '“', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// PC1119 Page code table
    static ref PC1119_TABLE: HashMap<char, u8> = [
        'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П',
        'Р', 'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я',
        'а', 'б', 'в', 'г', 'д', 'е', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н', 'о', 'п',
        '░', '▒', '▓', '│', '┤', 'Ą', 'Č', 'Ę', 'Ė', '╣', '║', '╗', '╝', 'Į', 'Š', '┐',
        '└', '┴', '┬', '├', '─', '┼', 'Ų', 'Ū', '╚', '╔', '╩', '╦', '╠', '═', '╬', 'Ž',
        'ą', 'č', 'ę', 'ė', 'į', 'š', 'ų', 'ū', 'ž', '┘', '┌', '█', '▄', '▌', '▐', '▀',
        'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э', 'ю', 'я',
        'Ё', 'ё', '≥', '≤', '„', '“', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// PC1125 Page code table
    static ref PC1125_TABLE: HashMap<char, u8> = [
        'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П',
        'Р', 'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я',
        'а', 'б', 'в', 'г', 'д', 'е', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н', 'о', 'п',
        '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐',
        '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧',
        '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀',
        'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э', 'ю', 'я',
        'Ё', 'ё', 'Ґ', 'ґ', 'Є', 'є', 'І', 'і', 'Ї', 'ї', '÷', '±', '№', '¤', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// WPC1250 Page code table
    /// Uses '\0' as placeholder for empty spots
    static ref WPC1250_TABLE: HashMap<char, u8> = [
        '€', '\0', '‚', '\0', '„', '…', '†', '‡', '\0', '‰', 'Š', '‹', 'Ś', 'Ť', 'Ž', 'Ź',
        '\0', '‘', '’', '“', '”', '•', '–', '—', '\0', '™', 'š', '›', 'ś', 'ť', 'ž', 'ź',
        '\u{00A0}', 'ˇ', '˘', 'Ł', '¤', 'Ą', '¦', '§', '¨', '©', 'Ş', '«', '¬', '-', '®', 'Ż',
        '°', '±', '˛', 'ł', '´', 'µ', '¶', '·', '¸', 'ą', 'ş', '»', 'Ľ', '˝', 'ľ', 'ż',
        'Ŕ', 'Á', 'Â', 'Ă', 'Ä', 'Ĺ', 'Ć', 'Ç', 'Č', 'É', 'Ę', 'Ë', 'Ě', 'Í', 'Î', 'Ď',
        'Đ', 'Ń', 'Ň', 'Ó', 'Ô', 'Ő', 'Ö', '×', 'Ř', 'Ů', 'Ú', 'Ű', 'Ü', 'Ý', 'Ţ', 'ß',
        'ŕ', 'á', 'â', 'ă', 'ä', 'ĺ', 'ć', 'ç', 'č', 'é', 'ę', 'ë', 'ě', 'í', 'î', 'ď',
        'đ', 'ń', 'ň', 'ó', 'ô', 'ő', 'ö', '÷', 'ř', 'ů', 'ú', 'ű', 'ü', 'ý', 'ţ', '˙',
    ]
    .into_iter().enumerate()
    .filter(|(_, c)| *c != '\0')
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// WPC1251 Page code table
    /// Uses '\0' as placeholder for empty spots
    static ref WPC1251_TABLE: HashMap<char, u8> = [
        'Ђ', 'Ѓ', '‚', 'ѓ', '„', '…', '†', '‡', '€', '‰', 'Љ', '‹', 'Њ', 'Ќ', 'Ћ', 'Џ',
        'ђ', '‘', '’', '“', '”', '•', '–', '—', '\0', '™', 'љ', '›', 'њ', 'ќ', 'ћ', 'џ',
        '\u{00A0}', 'Ў', 'ў', 'Ј', '¤', 'Ґ', '¦', '§', 'Ё', '©', 'Є', '«', '¬', '-', '®', 'Ї',
        '°', '±', 'І', 'і', 'ґ', 'µ', '¶', '·', 'ё', '№', 'є', '»', 'ј', 'Ѕ', 'ѕ', 'ї',
        'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П',
        'Р', 'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я',
        'а', 'б', 'в', 'г', 'д', 'е', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н', 'о', 'п',
        'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э', 'ю', 'я',
    ]
    .into_iter().enumerate()
    .filter(|(_, c)| *c != '\0')
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// WPC1253 Page code table
    /// Uses '\0' as placeholder for empty spots
    static ref WPC1253_TABLE: HashMap<char, u8> = [
        '€', '\0', '‚', 'ƒ', '„', '…', '†', '‡', '\0', '‰', '\0', '‹', '\0', '\0', '\0', '\0',
        '\0', '‘', '’', '“', '”', '•', '–', '—', '\0', '™', '\0', '›', '\0', '\0', '\0', '\0',
        '\u{00A0}', '΅', 'Ά', '£', '¤', '¥', '¦', '§', '¨', '©', '\0', '«', '¬', '-', '®', '―',
        '°', '±', '²', '³', '΄', 'µ', '¶', '·', 'Έ', 'Ή', 'Ί', '»', 'Ό', '½', 'Ύ', 'Ώ',
        'ΐ', 'Α', 'Β', 'Γ', 'Δ', 'Ε', 'Ζ', 'Η', 'Θ', 'Ι', 'Κ', 'Λ', 'Μ', 'Ν', 'Ξ', 'Ο',
        'Π', 'Ρ', '\0', 'Σ', 'Τ', 'Υ', 'Φ', 'Χ', 'Ψ', 'Ω', 'Ϊ', 'Ϋ', 'ά', 'έ', 'ή', 'ί',
        'ΰ', 'α', 'β', 'γ', 'δ', 'ε', 'ζ', 'η', 'θ', 'ι', 'κ', 'λ', 'μ', 'ν', 'ξ', 'ο',
        'π', 'ρ', 'ς', 'σ', 'τ', 'υ', 'φ', 'χ', 'ψ', 'ω', 'ϊ', 'ϋ', 'ό', 'ύ', 'ώ', '\0',
    ]
    .into_iter().enumerate()
    .filter(|(_, c)| *c != '\0')
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// WPC1254 Page code table
    /// Uses '\0' as placeholder for empty spots
    static ref WPC1254_TABLE: HashMap<char, u8> = [
        '€', '\0', '‚', 'ƒ', '„', '…', '†', '‡', 'ˆ', '‰', 'Š', '‹', 'Œ', '\0', '\0', '\0',
        '\0', '‘', '’', '“', '”', '•', '–', '—', '˜', '™', 'š', '›', 'œ', '\0', '\0', 'Ÿ',
        '\u{00A0}', '¡', '¢', '£', '¤', '¥', '¦', '§', '¨', '©', 'ª', '«', '¬', '-', '®', '¯',
        '°', '±', '²', '³', '´', 'µ', '¶', '·', '¸', '¹', 'º', '»', '¼', '½', '¾', '¿',
        'À', 'Á', 'Â', 'Ã', 'Ä', 'Å', 'Æ', 'Ç', 'È', 'É', 'Ê', 'Ë', 'Ì', 'Í', 'Î', 'Ï',
        'Ğ', 'Ñ', 'Ò', 'Ó', 'Ô', 'Õ', 'Ö', '×', 'Ø', 'Ù', 'Ú', 'Û', 'Ü', 'İ', 'Ş', 'ß',
        'à', 'á', 'â', 'ã', 'ä', 'å', 'æ', 'ç', 'è', 'é', 'ê', 'ë', 'ì', 'í', 'î', 'ï',
        'ğ', 'ñ', 'ò', 'ó', 'ô', 'õ', 'ö', '÷', 'ø', 'ù', 'ú', 'û', 'ü', 'ı', 'ş', 'ÿ',
    ]
    .into_iter().enumerate()
    .filter(|(_, c)| *c != '\0')
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// WPC1257 Page code table
    /// Uses '\0' as placeholder for empty spots
    static ref WPC1257_TABLE: HashMap<char, u8> = [
        '€', '\0', '‚', '\0', '„', '…', '†', '‡', '\0', '‰', '\0', '‹', '\0', '¨', 'ˇ', '¸',
        '\0', '‘', '’', '“', '”', '•', '–', '—', '\0', '™', '\0', '›', '\0', '¯', '˛', '\0',
        '\u{00A0}', '\0', '¢', '£', '¤', '\0', '¦', '§', 'Ø', '©', 'Ŗ', '«', '¬', '-', '®', 'Æ',
        '°', '±', '²', '³', '´', 'µ', '¶', '·', 'ø', '¹', 'ŗ', '»', '¼', '½', '¾', 'æ',
        'Ą', 'Į', 'Ā', 'Ć', 'Ä', 'Å', 'Ę', 'Ē', 'Č', 'É', 'Ź', 'Ė', 'Ģ', 'Ķ', 'Ī', 'Ļ',
        'Š', 'Ń', 'Ņ', 'Ó', 'Ō', 'Õ', 'Ö', '×', 'Ų', 'Ł', 'Ś', 'Ū', 'Ü', 'Ż', 'Ž', 'ß',
        'ą', 'į', 'ā', 'ć', 'ä', 'å', 'ę', 'ē', 'č', 'é', 'ź', 'ė', 'ģ', 'ķ', 'ī', 'ļ',
        'š', 'ń', 'ņ', 'ó', 'ō', 'õ', 'ö', '÷', 'ų', 'ł', 'ś', 'ū', 'ü', 'ż', 'ž', '˙',
    ]
    .into_iter().enumerate()
    .filter(|(_, c)| *c != '\0')
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();

    /// KZ1048 Page code table
    /// Uses '\0' as placeholder for empty spots
    static ref KZ1048_TABLE: HashMap<char, u8> = [
        'Ђ', 'Ѓ', '‚', 'ѓ', '„', '…', '†', '‡', '€', '‰', 'Љ', '‹', 'Њ', 'Қ', 'Һ', 'Џ',
        'ђ', '‘', '’', '“', '”', '•', '–', '—', '\0', '™', 'љ', '›', 'њ', 'қ', 'һ', 'џ',
        '\u{00A0}', 'Ұ', 'ұ', 'Ә', '¤', 'Ө', '¦', '§', 'Ё', '©', 'Ғ', '«', '¬', '-', '®', 'Ү',
        '°', '±', 'І', 'і', 'ө', 'µ', '¶', '·', 'ё', '№', 'ғ', '»', 'ә', 'Ң', 'ң', 'ү',
        'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П',
        'Р', 'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я',
        'а', 'б', 'в', 'г', 'д', 'е', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н', 'о', 'п',
        'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э', 'ю', 'я',
    ]
    .into_iter().enumerate()
    .filter(|(_, c)| *c != '\0')
    .map(|(i, c)| (c, (i + 0x80) as u8))
    .collect();
}
