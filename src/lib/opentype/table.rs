use std::{io, mem};
use super::spec;

pub struct Table;

impl Table {
    pub fn measure(table_record: &spec::TableRecord) -> uint {
        let length = table_record.length as uint;
        let size = mem::size_of::<u32>();

        ((length + size - 1) & !(size - 1)) / size
    }

    pub fn map_and_check<R: io::Reader>(stream: &mut R,
        table_record: &spec::TableRecord, process: |u32, uint| -> u32)
        -> bool {

        let mut checksum: u32 = 0;
        let length = Table::measure(table_record);

        for i in range(0, length) {
            match stream.read_be_u32() {
                Ok(value) => checksum += process(value, i),
                Err(_) => return false
            }
        }

        table_record.checkSum == checksum
    }

    pub fn check<R: io::Reader>(stream: &mut R,
        table_record: &spec::TableRecord) -> bool {

        Table::map_and_check(stream, table_record, |chunk, _| chunk)
    }
}
