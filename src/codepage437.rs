use crate::Glyph;

/// Converts a unicode character to a CP437 equivalent, returning 0 if it didn't have a match
pub fn to_glyph(c: char) -> Glyph {
    match c {
        '☺' => 1,
        '☻' => 2,
        '♥' => 3,
        '♦' => 4,
        '♣' => 5,
        '♠' => 6,
        '•' => 7,
        '◘' => 8,
        '○' => 9,
        '◙' => 10,
        '♂' => 11,
        '♀' => 12,
        '♪' => 13,
        '♫' => 14,
        '☼' => 15,

        '►' => 16,
        '◄' => 17,
        '↕' => 18,
        '‼' => 19,
        '¶' => 20,
        '§' => 21,
        '▬' => 22,
        '↨' => 23,
        '↑' => 24,
        '↓' => 25,
        '→' => 26,
        '←' => 27,
        '∟' => 28,
        '↔' => 29,
        '▲' => 30,
        '▼' => 31,

        ' ' => 32,
        '!' => 33,
        '"' => 34,
        '#' => 35,
        '$' => 36,
        '%' => 37,
        '&' => 38,
        '\'' => 39,
        '(' => 40,
        ')' => 41,
        '*' => 42,
        '+' => 43,
        ',' => 44,
        '-' => 45,
        '.' => 46,
        '/' => 47,

        '0' => 48,
        '1' => 49,
        '2' => 50,
        '3' => 51,
        '4' => 52,
        '5' => 53,
        '6' => 54,
        '7' => 55,
        '8' => 56,
        '9' => 57,
        ':' => 58,
        ';' => 59,
        '<' => 60,
        '=' => 61,
        '>' => 62,
        '?' => 63,

        '@' => 64,
        'A' => 65,
        'B' => 66,
        'C' => 67,
        'D' => 68,
        'E' => 69,
        'F' => 70,
        'G' => 71,
        'H' => 72,
        'I' => 73,
        'J' => 74,
        'K' => 75,
        'L' => 76,
        'M' => 77,
        'N' => 78,
        'O' => 79,

        'P' => 80,
        'Q' => 81,
        'R' => 82,
        'S' => 83,
        'T' => 84,
        'U' => 85,
        'V' => 86,
        'W' => 87,
        'X' => 88,
        'Y' => 89,
        'Z' => 90,
        '[' => 91,
        '\\' => 92,
        ']' => 93,
        '^' => 94,
        '_' => 95,

        '`' => 96,
        'a' => 97,
        'b' => 98,
        'c' => 99,
        'd' => 100,
        'e' => 101,
        'f' => 102,
        'g' => 103,
        'h' => 104,
        'i' => 105,
        'j' => 106,
        'k' => 107,
        'l' => 108,
        'm' => 109,
        'n' => 110,
        'o' => 111,

        'p' => 112,
        'q' => 113,
        'r' => 114,
        's' => 115,
        't' => 116,
        'u' => 117,
        'v' => 118,
        'w' => 119,
        'x' => 120,
        'y' => 121,
        'z' => 122,
        '{' => 123,
        '|' => 124,
        '}' => 125,
        '~' => 126,
        '⌂' => 127,

        'Ç' => 128,
        'ü' => 129,
        'é' => 130,
        'â' => 131,
        'ä' => 132,
        'à' => 133,
        'å' => 134,
        'ç' => 135,
        'ê' => 136,
        'ë' => 137,
        'è' => 138,
        'ï' => 139,
        'î' => 140,
        'ì' => 141,
        'Ä' => 142,
        'Å' => 143,

        'É' => 144,
        'æ' => 145,
        'Æ' => 146,
        'ô' => 147,
        'ö' => 148,
        'ò' => 149,
        'û' => 150,
        'ù' => 151,
        'ÿ' => 152,
        'Ö' => 153,
        'Ü' => 154,
        '¢' => 155,
        '£' => 156,
        '¥' => 157,
        '₧' => 158,
        'ƒ' => 159,

        'á' => 160,
        'í' => 161,
        'ó' => 162,
        'ú' => 163,
        'ñ' => 164,
        'Ñ' => 165,
        'ª' => 166,
        'º' => 167,
        '¿' => 168,
        '⌐' => 169,
        '¬' => 170,
        '½' => 171,
        '¼' => 172,
        '¡' => 173,
        '«' => 174,
        '»' => 175,

        '░' => 176,
        '▒' => 177,
        '▓' => 178,
        '│' => 179,
        '┤' => 180,
        '╡' => 181,
        '╢' => 182,
        '╖' => 183,
        '╕' => 184,
        '╣' => 185,
        '║' => 186,
        '╗' => 187,
        '╝' => 188,
        '╜' => 189,
        '╛' => 190,
        '┐' => 191,

        '└' => 192,
        '┴' => 193,
        '┬' => 194,
        '├' => 195,
        '─' => 196,
        '┼' => 197,
        '╞' => 198,
        '╟' => 199,
        '╚' => 200,
        '╔' => 201,
        '╩' => 202,
        '╦' => 203,
        '╠' => 204,
        '═' => 205,
        '╬' => 206,
        '╧' => 207,

        '╨' => 208,
        '╤' => 209,
        '╥' => 210,
        '╙' => 211,
        '╘' => 212,
        '╒' => 213,
        '╓' => 214,
        '╫' => 215,
        '╪' => 216,
        '┘' => 217,
        '┌' => 218,
        '█' => 219,
        '▄' => 220,
        '▌' => 221,
        '▐' => 222,
        '▀' => 223,

        'α' => 224,
        'ß' => 225,
        'Γ' => 226,
        'π' => 227,
        'Σ' => 228,
        'σ' => 229,
        'µ' => 230,
        'τ' => 231,
        'Φ' => 232,
        'Θ' => 233,
        'Ω' => 234,
        'δ' => 235,
        '∞' => 236,
        'φ' => 237,
        'ε' => 238,
        '∩' => 239,

        '≡' => 240,
        '±' => 241,
        '≥' => 242,
        '≤' => 243,
        '⌠' => 244,
        '⌡' => 245,
        '÷' => 246,
        '≈' => 247,
        '°' => 248,
        '∙' => 249,
        '·' => 250,
        '√' => 251,
        'ⁿ' => 252,
        '²' => 253,
        '■' => 254,

        _ => c as u32,
    }
}

pub fn from_glyph(c: Glyph) -> char {
    match c {
        0 => ' ',
        1 => '☺',
        2 => '☻',
        3 => '♥',
        4 => '♦',
        5 => '♣',
        6 => '♠',
        7 => '•',
        8 => '◘',
        9 => '○',
        10 => '◙',
        11 => '♂',
        12 => '♀',
        13 => '♪',
        14 => '♫',
        15 => '☼',

        16 => '►',
        17 => '◄',
        18 => '↕',
        19 => '‼',
        20 => '¶',
        21 => '§',
        22 => '▬',
        23 => '↨',
        24 => '↑',
        25 => '↓',
        26 => '→',
        27 => '←',
        28 => '∟',
        29 => '↔',
        30 => '▲',
        31 => '▼',

        32 => ' ',
        33 => '!',
        34 => '"',
        35 => '#',
        36 => '$',
        37 => '%',
        38 => '&',
        39 => '\'',
        40 => '(',
        41 => ')',
        42 => '*',
        43 => '+',
        44 => ',',
        45 => '-',
        46 => '.',
        47 => '/',

        48 => '0',
        49 => '1',
        50 => '2',
        51 => '3',
        52 => '4',
        53 => '5',
        54 => '6',
        55 => '7',
        56 => '8',
        57 => '9',
        58 => ':',
        59 => ';',
        60 => '<',
        61 => '=',
        62 => '>',
        63 => '?',

        64 => '@',
        65 => 'A',
        66 => 'B',
        67 => 'C',
        68 => 'D',
        69 => 'E',
        70 => 'F',
        71 => 'G',
        72 => 'H',
        73 => 'I',
        74 => 'J',
        75 => 'K',
        76 => 'L',
        77 => 'M',
        78 => 'N',
        79 => 'O',

        80 => 'P',
        81 => 'Q',
        82 => 'R',
        83 => 'S',
        84 => 'T',
        85 => 'U',
        86 => 'V',
        87 => 'W',
        88 => 'X',
        89 => 'Y',
        90 => 'Z',
        91 => '[',
        92 => '\\',
        93 => ']',
        94 => '^',
        95 => '_',

        96 => '`',
        97 => 'a',
        98 => 'b',
        99 => 'c',
        100 => 'd',
        101 => 'e',
        102 => 'f',
        103 => 'g',
        104 => 'h',
        105 => 'i',
        106 => 'j',
        107 => 'k',
        108 => 'l',
        109 => 'm',
        110 => 'n',
        111 => 'o',

        112 => 'p',
        113 => 'q',
        114 => 'r',
        115 => 's',
        116 => 't',
        117 => 'u',
        118 => 'v',
        119 => 'w',
        120 => 'x',
        121 => 'y',
        122 => 'z',
        123 => '{',
        124 => '|',
        125 => '}',
        126 => '~',
        127 => '⌂',

        128 => 'Ç',
        129 => 'ü',
        130 => 'é',
        131 => 'â',
        132 => 'ä',
        133 => 'à',
        134 => 'å',
        135 => 'ç',
        136 => 'ê',
        137 => 'ë',
        138 => 'è',
        139 => 'ï',
        140 => 'î',
        141 => 'ì',
        142 => 'Ä',
        143 => 'Å',

        144 => 'É',
        145 => 'æ',
        146 => 'Æ',
        147 => 'ô',
        148 => 'ö',
        149 => 'ò',
        150 => 'û',
        151 => 'ù',
        152 => 'ÿ',
        153 => 'Ö',
        154 => 'Ü',
        155 => '¢',
        156 => '£',
        157 => '¥',
        158 => '₧',
        159 => 'ƒ',

        160 => 'á',
        161 => 'í',
        162 => 'ó',
        163 => 'ú',
        164 => 'ñ',
        165 => 'Ñ',
        166 => 'ª',
        167 => 'º',
        168 => '¿',
        169 => '⌐',
        170 => '¬',
        171 => '½',
        172 => '¼',
        173 => '¡',
        174 => '«',
        175 => '»',

        176 => '░',
        177 => '▒',
        178 => '▓',
        179 => '│',
        180 => '┤',
        181 => '╡',
        182 => '╢',
        183 => '╖',
        184 => '╕',
        185 => '╣',
        186 => '║',
        187 => '╗',
        188 => '╝',
        189 => '╜',
        190 => '╛',
        191 => '┐',

        192 => '└',
        193 => '┴',
        194 => '┬',
        195 => '├',
        196 => '─',
        197 => '┼',
        198 => '╞',
        199 => '╟',
        200 => '╚',
        201 => '╔',
        202 => '╩',
        203 => '╦',
        204 => '╠',
        205 => '═',
        206 => '╬',
        207 => '╧',

        208 => '╨',
        209 => '╤',
        210 => '╥',
        211 => '╙',
        212 => '╘',
        213 => '╒',
        214 => '╓',
        215 => '╫',
        216 => '╪',
        217 => '┘',
        218 => '┌',
        219 => '█',
        220 => '▄',
        221 => '▌',
        222 => '▐',
        223 => '▀',

        224 => 'α',
        225 => 'ß',
        226 => 'Γ',
        227 => 'π',
        228 => 'Σ',
        229 => 'σ',
        230 => 'µ',
        231 => 'τ',
        232 => 'Φ',
        233 => 'Θ',
        234 => 'Ω',
        235 => 'δ',
        236 => '∞',
        237 => 'φ',
        238 => 'ε',
        239 => '∩',

        240 => '≡',
        241 => '±',
        242 => '≥',
        243 => '≤',
        244 => '⌠',
        245 => '⌡',
        246 => '÷',
        247 => '≈',
        248 => '°',
        249 => '∙',
        250 => '·',
        251 => '√',
        252 => 'ⁿ',
        253 => '²',
        254 => '■',

        _ => match char::from_u32(c) {
            None => ' ',
            Some(ch) => ch,
        },
    }
}

/// Converts a string into a vector of u8, CP437 representations of the string
pub fn string_to_glyph<S: AsRef<str>>(input: S) -> Vec<Glyph> {
    input.as_ref().chars().map(to_glyph).collect()
}

#[cfg(test)]
mod tests {
    use super::string_to_glyph;
    use crate::Glyph;

    #[test]
    // Tests that we make an RGB triplet at defaults and it is black.
    fn test_hello() {
        let test: Vec<Glyph> = vec![72, 101, 108, 108, 111];
        let convert = string_to_glyph("Hello");
        assert_eq!(test, convert);
    }

    #[test]
    // Tests that we make an RGB triplet at defaults and it is black.
    fn test_fancy() {
        let test: Vec<Glyph> = vec![171, 165, 176, 206, 234, 247];
        let convert = string_to_glyph("½Ñ░╬Ω≈");
        assert_eq!(test, convert);
    }

    #[test]
    // Tests that we make an RGB triplet at defaults and it is black.
    fn test_first_group() {
        let test: Vec<Glyph> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let convert = string_to_glyph("☺☻♥♦♣♠•◘○◙♂♀♪♫☼");
        assert_eq!(test, convert);
    }
}
