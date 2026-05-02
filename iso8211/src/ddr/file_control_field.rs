use crate::{
    DirectoryEntry, FIELD_TERMINATOR, Iso8211Error, Leader, Reader, Result, UNIT_TERMINATOR,
};
use std::io::{Read, Seek};

#[derive(Debug)]
pub struct FileControlField {
    tag_pairs: Vec<(String, String)>,
}

impl FileControlField {
    pub fn read<T: Read + Seek>(
        reader: &mut Reader<T>,
        leader: &Leader,
        directory_entry: &DirectoryEntry,
    ) -> Result<FileControlField> {
        let field_controls = reader.read_str(leader.field_control_length() as usize)?;
        if field_controls != "0000;&   " {
            return Err(Iso8211Error::Parse(format!(
                "Invalid Field Controls: {}",
                field_controls
            )));
        }

        // we should have a unit terminator here
        if reader.read_u8()? != UNIT_TERMINATOR {
            return Err(Iso8211Error::Parse(String::from(
                "Did not find a unit terminator after the Field Controls",
            )));
        }

        // calculate the number of tag pairs
        let tag_length = leader.entry_map().field_tag();
        let count = (directory_entry.field_length() as usize - 11) / (2 * tag_length as usize);
        let mut tag_pairs: Vec<(String, String)> = Vec::with_capacity(count);
        for _ in 0..count {
            let parent = reader.read_str(tag_length as usize)?;
            let child = reader.read_str(tag_length as usize)?;
            tag_pairs.push((parent, child));
        }

        // it should all end with a field terminator here
        if reader.read_u8()? != FIELD_TERMINATOR {
            return Err(Iso8211Error::Parse(String::from(
                "Did not find a field terminator after tag pairs",
            )));
        }

        Ok(FileControlField { tag_pairs })
    }

    pub fn tag_pairs(&self) -> &[(String, String)] {
        &self.tag_pairs
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::io::{BufReader, Cursor};

    use crate::directory::tests::ascii_ddr_directory;
    use crate::{
        Directory, FIELD_TERMINATOR, Leader, Reader, Result, UNIT_TERMINATOR, ddr::FileControlField,
    };

    pub fn ascii_file_control_field() -> Result<(Leader, Directory, FileControlField)> {
        let data = ascii_ddr_directory()?;

        let bytes = [
            "0000;&   ".as_bytes(),
            &[UNIT_TERMINATOR],
            "0001FRIDFRIDFOIDFRIDATTFFRIDNATFFRIDFFPCFRIDFFPTFRIDFSPCFRID".as_bytes(),
            "FSPT0001VRIDVRIDATTVVRIDVRPCVRIDVRPTVRIDSGCCVRIDSG2DVRIDSG3DVRIDARCCA".as_bytes(),
            "RCCAR2DARCCEL2DARCCCT2D".as_bytes(),
            &[FIELD_TERMINATOR],
        ]
        .concat();
        let buffer = Cursor::new(bytes);
        let bufreader = BufReader::new(buffer);
        let mut reader = Reader::new(bufreader);
        let file_control_field =
            FileControlField::read(&mut reader, &data.0, &data.1.entries()[0])?;

        Ok((data.0, data.1, file_control_field))
    }

    #[test]
    fn test_file_control_field() {
        let target = ascii_file_control_field();

        // assert
        assert!(target.is_ok());

        let target = target.unwrap().2;
        assert_eq!(target.tag_pairs.len(), 19);
    }
}
