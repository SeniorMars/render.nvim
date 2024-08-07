/*
     All Kitty graphics commands are of the form:
    '<ESC>_G<control data>;<payload><ESC>\'
     <control keys> - a=T,f=100....
          <payload> - base64 enc. file data
              <ESC> - \x1b or \27
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Control {
    keys: Keys,
    action: Action,
    delete: Delete,
    transmit_format: TransmitFormat,
    transmit_medium: TransmitMedium,
    transmit_compress: TransmitCompress,
    display_cursor_policy: DisplayCursorPolicy,
    display_virtual_placeholder: DisplayVirtualPlaceholder,
}

impl Control {
    const PLACEHOLDER: &'static str = "\u{10EEEE}";
    const DIACRITICS: [&'static str; 297] = [
        "\u{000305}",
        "\u{00030D}",
        "\u{00030E}",
        "\u{000310}",
        "\u{000312}",
        "\u{00033D}",
        "\u{00033E}",
        "\u{00033F}",
        "\u{000346}",
        "\u{00034A}",
        "\u{00034B}",
        "\u{00034C}",
        "\u{000350}",
        "\u{000351}",
        "\u{000352}",
        "\u{000357}",
        "\u{00035B}",
        "\u{000363}",
        "\u{000364}",
        "\u{000365}",
        "\u{000366}",
        "\u{000367}",
        "\u{000368}",
        "\u{000369}",
        "\u{00036A}",
        "\u{00036B}",
        "\u{00036C}",
        "\u{00036D}",
        "\u{00036E}",
        "\u{00036F}",
        "\u{000483}",
        "\u{000484}",
        "\u{000485}",
        "\u{000486}",
        "\u{000487}",
        "\u{000592}",
        "\u{000593}",
        "\u{000594}",
        "\u{000595}",
        "\u{000597}",
        "\u{000598}",
        "\u{000599}",
        "\u{00059C}",
        "\u{00059D}",
        "\u{00059E}",
        "\u{00059F}",
        "\u{0005A0}",
        "\u{0005A1}",
        "\u{0005A8}",
        "\u{0005A9}",
        "\u{0005AB}",
        "\u{0005AC}",
        "\u{0005AF}",
        "\u{0005C4}",
        "\u{000610}",
        "\u{000611}",
        "\u{000612}",
        "\u{000613}",
        "\u{000614}",
        "\u{000615}",
        "\u{000616}",
        "\u{000617}",
        "\u{000657}",
        "\u{000658}",
        "\u{000659}",
        "\u{00065A}",
        "\u{00065B}",
        "\u{00065D}",
        "\u{00065E}",
        "\u{0006D6}",
        "\u{0006D7}",
        "\u{0006D8}",
        "\u{0006D9}",
        "\u{0006DA}",
        "\u{0006DB}",
        "\u{0006DC}",
        "\u{0006DF}",
        "\u{0006E0}",
        "\u{0006E1}",
        "\u{0006E2}",
        "\u{0006E4}",
        "\u{0006E7}",
        "\u{0006E8}",
        "\u{0006EB}",
        "\u{0006EC}",
        "\u{000730}",
        "\u{000732}",
        "\u{000733}",
        "\u{000735}",
        "\u{000736}",
        "\u{00073A}",
        "\u{00073D}",
        "\u{00073F}",
        "\u{000740}",
        "\u{000741}",
        "\u{000743}",
        "\u{000745}",
        "\u{000747}",
        "\u{000749}",
        "\u{00074A}",
        "\u{0007EB}",
        "\u{0007EC}",
        "\u{0007ED}",
        "\u{0007EE}",
        "\u{0007EF}",
        "\u{0007F0}",
        "\u{0007F1}",
        "\u{0007F3}",
        "\u{000816}",
        "\u{000817}",
        "\u{000818}",
        "\u{000819}",
        "\u{00081B}",
        "\u{00081C}",
        "\u{00081D}",
        "\u{00081E}",
        "\u{00081F}",
        "\u{000820}",
        "\u{000821}",
        "\u{000822}",
        "\u{000823}",
        "\u{000825}",
        "\u{000826}",
        "\u{000827}",
        "\u{000829}",
        "\u{00082A}",
        "\u{00082B}",
        "\u{00082C}",
        "\u{00082D}",
        "\u{000951}",
        "\u{000953}",
        "\u{000954}",
        "\u{000F82}",
        "\u{000F83}",
        "\u{000F86}",
        "\u{000F87}",
        "\u{00135D}",
        "\u{00135E}",
        "\u{00135F}",
        "\u{0017DD}",
        "\u{00193A}",
        "\u{001A17}",
        "\u{001A75}",
        "\u{001A76}",
        "\u{001A77}",
        "\u{001A78}",
        "\u{001A79}",
        "\u{001A7A}",
        "\u{001A7B}",
        "\u{001A7C}",
        "\u{001B6B}",
        "\u{001B6D}",
        "\u{001B6E}",
        "\u{001B6F}",
        "\u{001B70}",
        "\u{001B71}",
        "\u{001B72}",
        "\u{001B73}",
        "\u{001CD0}",
        "\u{001CD1}",
        "\u{001CD2}",
        "\u{001CDA}",
        "\u{001CDB}",
        "\u{001CE0}",
        "\u{001DC0}",
        "\u{001DC1}",
        "\u{001DC3}",
        "\u{001DC4}",
        "\u{001DC5}",
        "\u{001DC6}",
        "\u{001DC7}",
        "\u{001DC8}",
        "\u{001DC9}",
        "\u{001DCB}",
        "\u{001DCC}",
        "\u{001DD1}",
        "\u{001DD2}",
        "\u{001DD3}",
        "\u{001DD4}",
        "\u{001DD5}",
        "\u{001DD6}",
        "\u{001DD7}",
        "\u{001DD8}",
        "\u{001DD9}",
        "\u{001DDA}",
        "\u{001DDB}",
        "\u{001DDC}",
        "\u{001DDD}",
        "\u{001DDE}",
        "\u{001DDF}",
        "\u{001DE0}",
        "\u{001DE1}",
        "\u{001DE2}",
        "\u{001DE3}",
        "\u{001DE4}",
        "\u{001DE5}",
        "\u{001DE6}",
        "\u{001DFE}",
        "\u{0020D0}",
        "\u{0020D1}",
        "\u{0020D4}",
        "\u{0020D5}",
        "\u{0020D6}",
        "\u{0020D7}",
        "\u{0020DB}",
        "\u{0020DC}",
        "\u{0020E1}",
        "\u{0020E7}",
        "\u{0020E9}",
        "\u{0020F0}",
        "\u{002CEF}",
        "\u{002CF0}",
        "\u{002CF1}",
        "\u{002DE0}",
        "\u{002DE1}",
        "\u{002DE2}",
        "\u{002DE3}",
        "\u{002DE4}",
        "\u{002DE5}",
        "\u{002DE6}",
        "\u{002DE7}",
        "\u{002DE8}",
        "\u{002DE9}",
        "\u{002DEA}",
        "\u{002DEB}",
        "\u{002DEC}",
        "\u{002DED}",
        "\u{002DEE}",
        "\u{002DEF}",
        "\u{002DF0}",
        "\u{002DF1}",
        "\u{002DF2}",
        "\u{002DF3}",
        "\u{002DF4}",
        "\u{002DF5}",
        "\u{002DF6}",
        "\u{002DF7}",
        "\u{002DF8}",
        "\u{002DF9}",
        "\u{002DFA}",
        "\u{002DFB}",
        "\u{002DFC}",
        "\u{002DFD}",
        "\u{002DFE}",
        "\u{002DFF}",
        "\u{00A66F}",
        "\u{00A67C}",
        "\u{00A67D}",
        "\u{00A6F0}",
        "\u{00A6F1}",
        "\u{00A8E0}",
        "\u{00A8E1}",
        "\u{00A8E2}",
        "\u{00A8E3}",
        "\u{00A8E4}",
        "\u{00A8E5}",
        "\u{00A8E6}",
        "\u{00A8E7}",
        "\u{00A8E8}",
        "\u{00A8E9}",
        "\u{00A8EA}",
        "\u{00A8EB}",
        "\u{00A8EC}",
        "\u{00A8ED}",
        "\u{00A8EE}",
        "\u{00A8EF}",
        "\u{00A8F0}",
        "\u{00A8F1}",
        "\u{00AAB0}",
        "\u{00AAB2}",
        "\u{00AAB3}",
        "\u{00AAB7}",
        "\u{00AAB8}",
        "\u{00AABE}",
        "\u{00AABF}",
        "\u{00AAC1}",
        "\u{00FE20}",
        "\u{00FE21}",
        "\u{00FE22}",
        "\u{00FE23}",
        "\u{00FE24}",
        "\u{00FE25}",
        "\u{00FE26}",
        "\u{010A0F}",
        "\u{010A38}",
        "\u{01D185}",
        "\u{01D186}",
        "\u{01D187}",
        "\u{01D188}",
        "\u{01D189}",
        "\u{01D1AA}",
        "\u{01D1AB}",
        "\u{01D1AC}",
        "\u{01D1AD}",
        "\u{01D242}",
        "\u{01D243}",
        "\u{01D244}",
    ];
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Keys {
    Action = b'a',
    ImageId = b'i',
    ImageNumber = b'I',
    PlacementId = b'p',
    Quiet = b'q',
    TransmitFormat = b'f',
    TransmitMedium = b't',
    TransmitMore = b'm',
    TransmitWidth = b's',
    TransmitHeight = b'v',
    TransmitFileSize = b'S',
    TransmitFileOffset = b'O',
    TransmitCompression = b'o',
    DisplayX = b'x',
    DisplayY = b'y',
    DisplayWidth = b'w',
    DisplayHeight = b'h',
    DisplayXOffset = b'X',
    DisplayYOffset = b'Y',
    DisplayColumns = b'c',
    DisplayRows = b'r',
    DisplayCursorPolicy = b'C',
    DisplayVirtualPlaceholder = b'U',
    DisplayZindex = b'z',
    DisplayDelete = b'd',
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Action {
    Transmit = b't',
    TransmitAndDisplay = b'T',
    Display = b'p',
    Delete = b'd',
    TransmitAnimationFrames = b'f',
    ComposeAnimationFrames = b'c',
    ControlAnimation = b'a',
    Query = b'q',
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Delete {
    All = b'a',
    ImageId = b'i',
    PlacementId = b'p',
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum TransmitFormat {
    Rgba32 = 32,
    Rgb24 = 24,
    Png = 100,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum TransmitMedium {
    Direct = b'd',
    File = b'f',
    TemporaryFile = b't',
    SharedMemoryObject = b's',
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum TransmitCompress {
    Zlib = b'z',
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DisplayCursorPolicy {
    Move = 0,
    DoNotMove = 1,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DisplayVirtualPlaceholder {
    Yes = 1,
    No = 0,
}
