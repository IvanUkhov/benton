#![allow(unused_mut, unused_variables)]

use truetype::GlyphID;

use {Result, Tape, Value, Walue};
use layout::Coverage;

/// A table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Table {
    SingleSubstibution(SingleSubstibution),
    MultipleSubstibution(MultipleSubstibution),
    AlternateSubstibution(AlternateSubstibution),
    LigatureSubstibution(LigatureSubstibution),
    ContextSubstibution(ContextSubstibution),
    ChainedContextSubstibution(ChainedContextSubstibution),
    ExtensionSubstibution(ExtensionSubstibution),
    ReverseChainedContextSubstibution(ReverseChainedContextSubstibution),
}

/// A table for substituting one glyph with one glyph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SingleSubstibution {
    /// Format 1.
    Format1(SingleSubstibution1),
    /// Format 2.
    Format2(SingleSubstibution2),
}

table! {
    @position
    #[doc = "A table for substituting one glyph with one glyph in format 1."]
    pub SingleSubstibution1 {
        format          (u16) = { 1 }, // SubstFormat
        coverage_offset (u16), // Coverage
        delta_glyph_id  (i16), // DeltaGlyphID

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },
    }
}

table! {
    @position
    #[doc = "A table for substituting one glyph with one glyph in format 2."]
    pub SingleSubstibution2 {
        format          (u16) = { 2 }, // SubstFormat
        coverage_offset (u16), // Coverage
        glyph_count     (u16), // GlyphCount

        glyph_ids (Vec<GlyphID>) |this, tape, _| { // Substitute
            tape.take_given(this.glyph_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },
    }
}

table! {
    @position
    #[doc = "A table for substituting one glyph with more than one glyph."]
    pub MultipleSubstibution {
        format           (u16) = { 1 }, // SubstFormat
        coverage_offset  (u16), // Coverage
        sequence_count   (u16), // SequenceCount

        sequence_offsets (Vec<u16>) |this, tape, _| { // Sequence
            tape.take_given(this.sequence_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        sequences (Vec<Sequence>) |this, tape, position| {
            jump_take!(tape, position, this.sequence_count, this.sequence_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A table for substituting one glyph with one of many glyphs."]
    pub AlternateSubstibution {
        format          (u16) = { 1 }, // SubstFormat
        coverage_offset (u16), // Coverage
        set_count       (u16), // AlternateSetCount

        set_offsets (Vec<u16>) |this, tape, _| { // AlternateSet
            tape.take_given(this.set_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        sets (Vec<AlternateSet>) |this, tape, position| {
            jump_take!(tape, position, this.set_count, this.set_offsets)
        },
    }
}

table! {
    @position
    #[doc = "A table for substituting multiple glyphs with one glyph."]
    pub LigatureSubstibution {
        format          (u16) = { 1 }, // SubstFormat
        coverage_offset (u16), // Coverage
        set_count       (u16), // LigSetCount

        set_offsets (Vec<u16>) |this, tape, _| { // LigatureSet
            tape.take_given(this.set_count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },

        sets (Vec<LigatureSet>) |this, tape, position| {
            jump_take!(tape, position, this.set_count, this.set_offsets)
        },
    }
}

table! {
    #[doc = "A table for substituting glyphs in a context."]
    pub ContextSubstibution {
    }
}

table! {
    #[doc = "A table for substituting glyphs in a chained context."]
    pub ChainedContextSubstibution {
    }
}

table! {
    #[doc = "A table for other types of substitution."]
    pub ExtensionSubstibution {
    }
}

table! {
    #[doc = "A table for substituting glyphs in reverse order in a chained context."]
    pub ReverseChainedContextSubstibution {
    }
}

table! {
    #[doc = "A set of alternate glyphs."]
    pub AlternateSet {
        count (u16), // GlyphCount

        glyph_ids (Vec<GlyphID>) |this, tape| { // Alternate
            tape.take_given(this.count as usize)
        },
    }
}

table! {
    #[doc = "A ligature."]
    pub Ligature {
        glyph_id        (GlyphID), // LigGlyph
        component_count (u16    ), // CompCount

        component_ids (Vec<GlyphID>) |this, tape| { // Component
            if this.component_count == 0 {
                raise!("found a malformed ligature record");
            }
            tape.take_given(this.component_count as usize - 1)
        },
    }
}

table! {
    @position
    #[doc = "A set of ligatures."]
    pub LigatureSet {
        count (u16), // LigatureCount

        offsets (Vec<u16>) |this, tape, _| { // Ligature
            tape.take_given(this.count as usize)
        },

        records (Vec<Ligature>) |this, tape, position| {
            jump_take!(tape, position, this.count, this.offsets)
        },
    }
}

table! {
    #[doc = "A sequence of glyphs."]
    pub Sequence {
        count (u16), // GlyphCount

        glyph_ids (Vec<GlyphID>) |this, tape| { // Substitute
            tape.take_given(this.count as usize)
        },
    }
}

impl Walue<u16> for Table {
    fn read<T: Tape>(tape: &mut T, kind: u16) -> Result<Self> {
        Ok(match kind {
            1 => Table::SingleSubstibution(try!(tape.take())),
            2 => Table::MultipleSubstibution(try!(tape.take())),
            3 => Table::AlternateSubstibution(try!(tape.take())),
            4 => Table::LigatureSubstibution(try!(tape.take())),
            5 => Table::ContextSubstibution(try!(tape.take())),
            6 => Table::ChainedContextSubstibution(try!(tape.take())),
            7 => Table::ExtensionSubstibution(try!(tape.take())),
            8 => Table::ReverseChainedContextSubstibution(try!(tape.take())),
            _ => raise!("found an unknown glyph-substitution type"),
        })
    }
}

impl Value for SingleSubstibution {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            1 => SingleSubstibution::Format1(try!(tape.take())),
            2 => SingleSubstibution::Format2(try!(tape.take())),
            _ => raise!("found an unknown format of the single-substitution table"),
        })
    }
}
