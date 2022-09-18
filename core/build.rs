use std::{env, io};
use std::borrow::Cow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const RAW_DATA: &str = include_str!(concat!(
env!("CARGO_MANIFEST_DIR"),
"/../source/dick/pinyin.txt"
));
type InputData = Vec<(u32, Vec<&'static str>)>;
type PinyinDataIndex = HashMap<&'static str, usize>;
type Style = (&'static str, fn(&str) -> Cow<'_, str>);

#[rustfmt::skip]
const LETTER_TABLE: &[char] = &[
    'b', 'p', 'm', 'f', 'd',
    't', 'n', 'l', 'g', 'k',
    'h', 'j', 'q', 'x', 'r',
    'z', 'c', 's',
    'w', 'y',
    // 因为数据源里面不会使用 `v` 以及其它的 简写字母，所以这里注释掉
    // 'v', 'ẑ', 'ĉ', 'ŝ', 'ŋ',

    '\u{0304}', '\u{030C}', '\u{0300}', // Unicode 声调连字符
    'a', 'ā', 'á', 'ǎ', 'à',
    'e', 'ē', 'é', 'ě', 'è',
    'i', 'ī', 'í', 'ǐ', 'ì',
    //   "m̄"       "m̌"  "m̀"
    'm',      'ḿ',
    //   "n̄"
    'n',      'ń', 'ň', 'ǹ',
    'o', 'ō', 'ó', 'ǒ', 'ò',
    'u', 'ū', 'ú', 'ǔ', 'ù',
    //   "ê̄"       "ê̌"
    'ê',      'ế',      'ề',
    //   'ǖ'
    'ü',      'ǘ', 'ǚ', 'ǜ',
];

const STYLES: &[Style] = &[
        #[cfg(feature = "al")]
    ("al", |input| {
        input.chars().filter_map(|c| get_char_info(c).0).collect()
    }),
        #[cfg(feature = "py")]
    ("py", |input| Cow::from(input)),
        #[cfg(feature = "with_tone_num")]
    ("with_tone_num", |input| {
        let mut result = String::new();
        for ch in input.chars() {
            let (ch, tone) = get_char_info(ch);
            if let Some(ch) = ch {
                result.push(ch);
            }
            if tone > 0 {
                result.push(TONE_NUMS[usize::try_from(tone).unwrap()]);
            }
        }
        result.into()
    }),
        #[cfg(feature = "with_tone_num_end")]
    ("with_tone_num_end", |input| {
        let mut result = String::new();
        let mut output_tone = None;
        for ch in input.chars() {
            let (ch, tone) = get_char_info(ch);
            if let Some(ch) = ch {
                result.push(ch);
            }
            if tone > 0 {
                assert!(output_tone.is_none());
                output_tone = Some(TONE_NUMS[usize::try_from(tone).unwrap()]);
            }
        }
        if let Some(tone) = output_tone {
            result.push(tone);
        }
        result.into()
    }),
];

fn build_data() -> InputData {
    let mut input_data = RAW_DATA
        .lines()
        .enumerate()
        // 移除注释和空格
        .map(|(i, mut line)| {
            if let Some(hash_pos) = line.find('#') {
                line = &line[..hash_pos];
            }
            (i, line.trim())
        })
        // 移除空行
        .filter(|(_, line)| !line.is_empty())
        .map(|(i, line)| {
            // Split the line by colon
            let colon_pos = match line.find(':') {
                Some(pos) => pos,
                None => unreachable!("no colon found in line {}", i),
            };
            let code_point = line[..colon_pos].trim();
            let pinyin_list: Vec<_> = line[colon_pos + 1..].trim().split(',').collect();

            // 确保输入数据的字符全部在我们预料之中。
            // 同时也可以提前知道一些被遗弃的码位，如: U+E7C8 和 U+E7C7
            for pinyin in pinyin_list.iter() {
                for ch in pinyin.chars() {
                    let is_known = LETTER_TABLE.contains(&ch);
                    assert!(
                        is_known,
                        "unknown character {:?} at line {}: {}",
                        ch, i, line,
                    );
                }
            }

            // 解析码位
            const CODE_POINT_PREFIX: &str = "U+";
            assert!(code_point.starts_with(CODE_POINT_PREFIX));
            let code = &code_point[CODE_POINT_PREFIX.len()..];
            let code = match u32::from_str_radix(code, 16) {
                Ok(code) => code,
                Err(_) => unreachable!("invalid code point {} at line {}", code, i),
            };
            (code, pinyin_list)
        })
        .collect::<Vec<_>>();
    input_data.sort_by_key(|(code, _)| *code);
    input_data
}

fn generate_pinyin_data(data: &InputData) -> io::Result<PinyinDataIndex> {
    let mut output = create_out_file("py_dicks.rs")?;
    let mut pinyin_data = HashMap::new();
    writeln!(output, "&[")?;
    let mut process_pinyin = |pinyin| {
        let index = pinyin_data.len();
        match pinyin_data.entry(pinyin) {
            Entry::Occupied(_) => return Ok(()),
            Entry::Vacant(entry) => {
                entry.insert(index);
            }
        }
        write!(output, "    Dk {{ ")?;
        for (field, converter) in STYLES.iter() {
            write!(output, r#"{}: "{}", "#, field, converter(pinyin))?;
        }
        #[cfg(feature = "compat")]
        {
            // 计算切分声母和韵母的位置
            const INITIALS: &[&str] = &[
                "b", "p", "m", "f", "d", "t", "n", "l", "g", "k", "h", "j", "q", "x", "r", "zh",
                "ch", "sh", "z", "c", "s",
            ];
            let split = INITIALS
                .iter()
                .find(|initial| pinyin.starts_with(*initial))
                .map_or(0, |initial| initial.len());
            write!(output, "split: {}, ", split)?;
        }
        writeln!(output, "}},")?;
        Ok(())
    };
    // 插入一个空的拼音数据作为零位
    process_pinyin("")?;
    data.iter()
        .flat_map(|(_, list)| list.iter().map(|s| *s))
        .map(process_pinyin)
        .collect::<io::Result<()>>()?;
    writeln!(output, "]")?;
    Ok(pinyin_data)
}


fn create_out_file(name: &str) -> io::Result<impl Write> {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join(name);
    Ok(BufWriter::new(File::create(&path)?))
}

fn main_bk()-> io::Result<()> {
    let data = build_data();
    generate_pinyin_data(&data)?;

    // 输出这行以保证改动项目的其他文件不会触发编译脚本重新执行
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}


type ExtractData = Vec<(u32, Vec<&'static str>)>;

fn extract_data() -> ExtractData{
    let mut input_data = RAW_DATA
        .lines()
        .enumerate()
        // 移除注释和空格
        .map(|(i, mut line)| {
            if let Some(hash_pos) = line.find('#') {
                line = &line[..hash_pos];
            }
            (i, line.trim())
        })
        // 移除空行
        .filter(|(_, line)| !line.is_empty())
        .map(|(i, line)| {
            // Split the line by colon
            let colon_pos = match line.find(':') {
                Some(pos) => pos,
                None => unreachable!("no colon found in line {}", i),
            };
            let code_point = line[..colon_pos].trim();
            let pinyin_list: Vec<_> = line[colon_pos + 1..].trim().split(',').collect();

            // 确保输入数据的字符全部在我们预料之中。
            // 同时也可以提前知道一些被遗弃的码位，如: U+E7C8 和 U+E7C7
            for pinyin in pinyin_list.iter() {
                for ch in pinyin.chars() {
                    let is_known = LETTER_TABLE.contains(&ch);
                    assert!(
                        is_known,
                        "unknown character {:?} at line {}: {}",
                        ch, i, line,
                    );
                }
            }

            // 解析码位
            const CODE_POINT_PREFIX: &str = "U+";
            assert!(code_point.starts_with(CODE_POINT_PREFIX));
            let code = &code_point[CODE_POINT_PREFIX.len()..];
            let code = match u32::from_str_radix(code, 16) {
                Ok(code) => code,
                Err(_) => unreachable!("invalid code point {} at line {}", code, i),
            };
            (code, pinyin_list)
        })
        .collect::<Vec<_>>();
    input_data.sort_by_key(|(code, _)| *code);
    input_data
}

fn generate_pinyin_dick(data: &InputData) -> io::Result<PinyinDataIndex> {
    let mut output = create_out_file("py_dicks.rs")?;
    let mut pinyin_data = HashMap::new();
    writeln!(output, "&[")?;
    let mut process_pinyin = |pinyin| {
        write!(output, "    Dk {{ ")?;
        write!(output, r#"py:"{}""#, pinyin)?;
        writeln!(output, "}},")?;
        Ok(())
    };
    // 插入一个空的拼音数据作为零位
    process_pinyin("")?;
    data.iter()
        .flat_map(|(_, list)| list.iter().map(|s| *s))
        .map(process_pinyin)
        .collect::<io::Result<()>>()?;
    writeln!(output, "]")?;
    Ok(pinyin_data)
}

fn main()-> io::Result<()> {
    let data = extract_data();  // 提取日期
    generate_pinyin_dick(&data)?;

    // 输出这行以保证改动项目的其他文件不会触发编译脚本重新执行
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
