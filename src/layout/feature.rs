//! The feature list.

use truetype::Tag;

table! {
    @position
    #[doc = "A feature list."]
    pub Features {
        count (u16), // FeatureCount

        headers (Vec<Header>) |this, tape, _| { // FeatureRecord
            tape.take_given(this.count as usize)
        },

        records (Vec<Record>) |this, tape, position| {
            jump_take!(tape, position, this.count, i => this.headers[i].offset)
        },
    }
}

table! {
    #[doc = "A feature header."]
    #[derive(Copy)]
    pub Header {
        tag    (Tag), // FeatureTag
        offset (u16), // Feature
    }
}

table! {
    @position
    #[doc = "A feature record."]
    pub Record {
        parameter_offset (u16), // FeatureParams
        lookup_count     (u16), // LookupCount

        lookup_indices (Vec<u16>) |this, tape, _| { // LookupListIndex
            tape.take_given(this.lookup_count as usize)
        },

        parameters (Option<Vec<u8>>) |this, tape, position| {
            if this.parameter_offset != 0 {
                try!(tape.jump(position + this.parameter_offset as u64));
                Ok(Some(try!(tape.take_bytes(0))))
            } else {
                Ok(None)
            }
        },
    }
}

table! {
    #[doc = "A feature-variations table."]
    #[derive(Copy)]
    pub Variations {
        major_version (u16) = { 1 }, // MajorVersion
        minor_version (u16) = { 0 }, // MinorVersion
        count         (u32), // FeatureVariationRecordsCount
    }
}
