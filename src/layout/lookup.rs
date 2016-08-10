//! The lookup list.

use {Result, Tape, Value, Walue};

/// A lookup list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lookups<T> {
    pub count:   (u16           ), // LookupCount
    pub offsets: (Vec<u16>      ), // Lookup
    pub records: (Vec<Record<T>>),
}

/// A lookup record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Record<T> {
    pub kind:               (u16        ), // LookupType
    pub flags:              (Flags      ), // LookupFlag
    pub table_count:        (u16        ), // SubTableCount
    pub table_offsets:      (Vec<u16>   ), // SubTable
    pub mark_filtering_set: (Option<u16>), // MarkFilteringSet
    pub tables:             (Vec<T>     ),
}

flags! {
    #[doc = "Lookup flags."]
    pub Flags(u16) {
        0b0000_0000_0000_0001 => is_right_to_left,
        0b0000_0000_0000_0010 => should_ignore_base_glyphs,
        0b0000_0000_0000_0100 => should_ignore_ligature,
        0b0000_0000_0000_1000 => should_ignore_marks,
        0b0000_0000_0001_0000 => has_mark_filtering,
        0b0000_0000_1110_0000 => is_invalid,
    }
}

impl<U> Value for Lookups<U> where U: Walue<u16> {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = try!(tape.position());
        let count = try!(tape.take::<u16>());
        let offsets: Vec<u16> = try!(tape.take_given(count as usize));
        let mut records = Vec::with_capacity(count as usize);
        for i in 0..(count as usize) {
            try!(tape.jump(position + offsets[i] as u64));
            records.push(try!(tape.take()));
        }
        Ok(Lookups { count: count, offsets: offsets, records: records })
    }
}

impl<U> Value for Record<U> where U: Walue<u16> {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = try!(tape.position());
        let kind = try!(tape.take());
        let flags = try!(tape.take::<Flags>());
        let table_count = try!(tape.take::<u16>());
        let table_offsets: Vec<u16> = try!(tape.take_given(table_count as usize));
        let mark_filtering_set = if flags.has_mark_filtering() {
            Some(try!(tape.take()))
        } else {
            None
        };
        let mut tables = Vec::with_capacity(table_count as usize);
        for i in 0..(table_count as usize) {
            try!(tape.jump(position + table_offsets[i] as u64));
            tables.push(try!(tape.take_given(kind)));
        }
        Ok(Record {
            kind: kind,
            flags: flags,
            table_count: table_count,
            table_offsets: table_offsets,
            mark_filtering_set: mark_filtering_set,
            tables: tables,
        })
    }
}
