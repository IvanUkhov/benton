use primitive::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WindowsMetrics {
    Version3(WindowsMetrics3),
    Version5(WindowsMetrics5),
}

spec! {
    pub WindowsMetrics3 {
        version             (USHORT   ),
        xAvgCharWidth       (SHORT    ),
        usWeightClass       (USHORT   ),
        usWidthClass        (USHORT   ),
        fsType              (USHORT   ),
        ySubscriptXSize     (SHORT    ),
        ySubscriptYSize     (SHORT    ),
        ySubscriptXOffset   (SHORT    ),
        ySubscriptYOffset   (SHORT    ),
        ySuperscriptXSize   (SHORT    ),
        ySuperscriptYSize   (SHORT    ),
        ySuperscriptXOffset (SHORT    ),
        ySuperscriptYOffset (SHORT    ),
        yStrikeoutSize      (SHORT    ),
        yStrikeoutPosition  (SHORT    ),
        sFamilyClass        (SHORT    ),
        panose              (Vec<BYTE>) |this| { Ok(10) },
        ulUnicodeRange1     (ULONG    ),
        ulUnicodeRange2     (ULONG    ),
        ulUnicodeRange3     (ULONG    ),
        ulUnicodeRange4     (ULONG    ),
        achVendID           (Vec<CHAR>) |this| { Ok(4) },
        fsSelection         (USHORT   ),
        usFirstCharIndex    (USHORT   ),
        usLastCharIndex     (USHORT   ),
        sTypoAscender       (SHORT    ),
        sTypoDescender      (SHORT    ),
        sTypoLineGap        (SHORT    ),
        usWinAscent         (USHORT   ),
        usWinDescent        (USHORT   ),
        ulCodePageRange1    (ULONG    ),
        ulCodePageRange2    (ULONG    ),
        sxHeight            (SHORT    ),
        sCapHeight          (SHORT    ),
        usDefaultChar       (USHORT   ),
        usBreakChar         (USHORT   ),
        usMaxContext        (USHORT   ),
    }
}

spec! {
    pub WindowsMetrics5 {
        version                 (USHORT   ),
        xAvgCharWidth           (SHORT    ),
        usWeightClass           (USHORT   ),
        usWidthClass            (USHORT   ),
        fsType                  (USHORT   ),
        ySubscriptXSize         (SHORT    ),
        ySubscriptYSize         (SHORT    ),
        ySubscriptXOffset       (SHORT    ),
        ySubscriptYOffset       (SHORT    ),
        ySuperscriptXSize       (SHORT    ),
        ySuperscriptYSize       (SHORT    ),
        ySuperscriptXOffset     (SHORT    ),
        ySuperscriptYOffset     (SHORT    ),
        yStrikeoutSize          (SHORT    ),
        yStrikeoutPosition      (SHORT    ),
        sFamilyClass            (SHORT    ),
        panose                  (Vec<BYTE>) |this| { Ok(10) },
        ulUnicodeRange1         (ULONG    ),
        ulUnicodeRange2         (ULONG    ),
        ulUnicodeRange3         (ULONG    ),
        ulUnicodeRange4         (ULONG    ),
        achVendID               (Vec<CHAR>) |this| { Ok(4) },
        fsSelection             (USHORT   ),
        usFirstCharIndex        (USHORT   ),
        usLastCharIndex         (USHORT   ),
        sTypoAscender           (SHORT    ),
        sTypoDescender          (SHORT    ),
        sTypoLineGap            (SHORT    ),
        usWinAscent             (USHORT   ),
        usWinDescent            (USHORT   ),
        ulCodePageRange1        (ULONG    ),
        ulCodePageRange2        (ULONG    ),
        sxHeight                (SHORT    ),
        sCapHeight              (SHORT    ),
        usDefaultChar           (USHORT   ),
        usBreakChar             (USHORT   ),
        usMaxContext            (USHORT   ),
        usLowerOpticalPointSize (USHORT   ),
        usUpperOpticalPointSize (USHORT   ),
    }
}
