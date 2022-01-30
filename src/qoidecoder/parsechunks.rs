use nom::{
    branch::alt,
    number::complete::be_u8,
    number::complete::be_u32,
    error::{make_error, ErrorKind},
    Err as NomErr, IResult,
    sequence::tuple,
    bits::complete::take as bits_take,
    bits::complete::tag as bits_tag,
    bytes::complete::tag,
};

pub const QOI_OP_RGB_HEADER: u8 = 0b11111110;
pub const QOI_OP_RGBA_HEADER: u8 = 0b11111111;
pub const QOI_OP_INDEX_HEADER: usize = 0b00;
pub const QOI_OP_DIFF_HEADER: usize = 0b01;
pub const QOI_OP_LUMA_HEADER: usize = 0b10;
pub const QOI_OP_RUN_HEADER: usize = 0b11;

#[derive(Debug, Eq, PartialEq)]
pub struct QOIHeader {
    pub width: u32,
    pub height: u32,
    pub channels: u8,
    pub colorspace: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub struct RGBChunk {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub struct RGBAChunk {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub struct IndexChunk {
    pub index: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub struct DiffChunk {
    pub dr: u8,
    pub dg: u8,
    pub db: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub struct LumaChunk {
    pub diff_g: u8,
    pub dr_minus_dg: u8,
    pub db_minus_dg: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub struct RunChunk {
    pub run: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub enum CHUNK {
    RGBChunk(RGBChunk),
    RGBAChunk(RGBAChunk),
    IndexChunk(IndexChunk),
    DiffChunk(DiffChunk),
    LumaChunk(LumaChunk),
    RunChunk(RunChunk),
}

pub fn parse_header(input: &[u8]) -> IResult<&[u8], QOIHeader> {
    let (i, _) = tag("qoif")(input)?;
    let (i, (width, height, channels, colorspace)) = tuple((be_u32, be_u32, be_u8, be_u8))(i)?;
    let myheader = QOIHeader {
        width,
        height,
        channels,
        colorspace,
    };

    Ok((i, myheader))
}

pub fn parse_rgb(input: &[u8]) -> IResult<&[u8], CHUNK> {
    let (i, tag) = be_u8(input)?;
    
    if tag != QOI_OP_RGB_HEADER {
        return Err(NomErr::Error(make_error(i, ErrorKind::Tag)));
    }

    let (i, (r, g, b)) = tuple((be_u8, be_u8, be_u8))(i)?;
    let mychunk = RGBChunk {r, g, b};

    Ok((i, CHUNK::RGBChunk(mychunk)))
}

pub fn parse_rgba(input: &[u8]) -> IResult<&[u8], CHUNK> {
    let (i, tag) = be_u8(input)?;

    if tag != QOI_OP_RGBA_HEADER {
        return Err(NomErr::Error(make_error(i, ErrorKind::Tag)));
    }

    let (i, (r, g, b, a)) = tuple((be_u8, be_u8, be_u8, be_u8))(i)?;
    let mychunk = RGBAChunk {r, g, b, a};

    Ok((i, CHUNK::RGBAChunk(mychunk)))
}

pub fn parse_index(input: &[u8]) -> IResult<&[u8], CHUNK> {
    let temp: IResult<(&[u8], usize), usize> = bits_tag(QOI_OP_INDEX_HEADER, 2usize)((input, 0));
    if temp.is_err() {
        return Err(NomErr::Error(make_error(input, ErrorKind::Tag)));
    }
    
    let ((i, offset), _) = temp.unwrap();
    let temp: IResult<(&[u8], usize), u8> = bits_take(6usize)((i, offset));
    if temp.is_err() {
        return Err(NomErr::Error(make_error(input, ErrorKind::Tag)));
    }

    let ((i, _), index) = temp.unwrap();
    let mychunk = IndexChunk {index};

    Ok((i, CHUNK::IndexChunk(mychunk)))
}

pub fn parse_diff(input: &[u8]) -> IResult<&[u8], CHUNK> {
    let temp: IResult<(&[u8], usize), usize> = bits_tag(QOI_OP_DIFF_HEADER, 2usize)((input, 0));
    if temp.is_err() {
        return Err(NomErr::Error(make_error(input, ErrorKind::Tag)));
    }
    let ((i, offset), _) = temp.unwrap();

    let temp: IResult<(&[u8], usize), u8> = bits_take(2usize)((i, offset));
    if temp.is_err() {
        return Err(NomErr::Error(make_error(input, ErrorKind::Tag)));
    }
    let ((i, offset), r_diff) = temp.unwrap();

    let temp: IResult<(&[u8], usize), u8> = bits_take(2usize)((i, offset));
    if temp.is_err() {
        return Err(NomErr::Error(make_error(input, ErrorKind::Tag)));
    }
    let((i, offset), g_diff) = temp.unwrap();

    let temp: IResult<(&[u8], usize), u8> = bits_take(2usize)((i, offset));
    if temp.is_err() {
        return Err(NomErr::Error(make_error(input, ErrorKind::Tag)));
    }
    let ((i, _), b_diff) = temp.unwrap();

    let mychunk = DiffChunk {
        dr: r_diff,
        dg: g_diff,
        db: b_diff,
    };

    Ok((i, CHUNK::DiffChunk(mychunk)))
}

pub fn parse_luma(input: &[u8]) -> IResult<&[u8], CHUNK> {
    let temp: IResult<(&[u8], usize), usize> = bits_tag(QOI_OP_LUMA_HEADER, 2usize)((input, 0));
    if temp.is_err() {
        return Err(NomErr::Error(make_error(input, ErrorKind::Tag)));
    }
    let ((i, offset), _) = temp.unwrap();

    let temp: IResult<(&[u8], usize), u8> = bits_take(6usize)((i, offset));
    if temp.is_err() {
        return Err(NomErr::Error(make_error(input, ErrorKind::Tag)));
    }
    let ((i, offset), g_diff) = temp.unwrap();

    let temp: IResult<(&[u8], usize), u8> = bits_take(4usize)((i, offset));
    if temp.is_err() {
        return Err(NomErr::Error(make_error(input, ErrorKind::Tag)));
    }
    let((i, offset), dr_dg) = temp.unwrap();

    let temp: IResult<(&[u8], usize), u8> = bits_take(4usize)((i, offset));
    if temp.is_err() {
        return Err(NomErr::Error(make_error(input, ErrorKind::Tag)));
    }
    let ((i, _), db_dg) = temp.unwrap();

    let mychunk = LumaChunk {
        diff_g: g_diff,
        dr_minus_dg: dr_dg,
        db_minus_dg: db_dg,
    };

    Ok((i, CHUNK::LumaChunk(mychunk)))
}


pub fn parse_run(input: &[u8]) -> IResult<&[u8], CHUNK> {
    let temp: IResult<(&[u8], usize), usize> = bits_tag(QOI_OP_RUN_HEADER, 2usize)((input, 0));
    if temp.is_err() {
        return Err(NomErr::Error(make_error(input, ErrorKind::Tag)));
    }
    
    let ((i, offset), _) = temp.unwrap();
    let temp: IResult<(&[u8], usize), u8> = bits_take(6usize)((i, offset));
    if temp.is_err() {
        return Err(NomErr::Error(make_error(input, ErrorKind::Tag)));
    }

    let ((i, _), run) = temp.unwrap();
    let mychunk = RunChunk {run};

    Ok((i, CHUNK::RunChunk(mychunk)))
}

pub fn parse_chunks(input: &[u8]) -> IResult<&[u8], CHUNK> {
    alt((
        parse_diff,
        parse_index,
        parse_luma,
        parse_rgb,
        parse_rgba,
        parse_run,
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_parser() {
        let bytes = [0x71, 0x6F, 0x69, 0x66, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x07, 0x03, 0x01];
        let chunk = QOIHeader {
            width: 0x05,
            height: 0x07,
            channels: 0x03,
            colorspace: 0x01,
        };

        let result = parse_header(&bytes).unwrap();
        assert_eq!(
            result.1,
            chunk
        );
    }

    #[test]
    fn test_rgb_parser() {
        let bytes = [0xFE, 0xA3, 0x89, 0x43];
        let chunk = CHUNK::RGBChunk(RGBChunk {
            r: 0xA3,
            g: 0x89,
            b: 0x43,
        });

        let result = parse_rgb(&bytes).unwrap();

        assert_eq!(
            result.1,
            chunk
        );
    }

    #[test]
    fn test_rgba_parser() {
        let bytes = [0xFF, 0xA3, 0x89, 0x43, 0x44];
        let chunk = CHUNK::RGBAChunk(RGBAChunk {
            r: 0xA3,
            g: 0x89,
            b: 0x43,
            a: 0x44,
        });

        let result = parse_rgba(&bytes).unwrap();

        assert_eq!(
            result.1,
            chunk
        );
    }

    #[test]
    fn test_index_parser() {
        let bytes = [0x04, 0x11];
        let chunk = CHUNK::IndexChunk(IndexChunk {
            index: 0x04,
        });

        let result = parse_index(&bytes).unwrap();

        assert_eq!(
            result.1,
            chunk
        );
    }

    #[test]
    fn test_diff_parser() {
        let bytes = [0x71];
        let chunk = CHUNK::DiffChunk(DiffChunk {
            dr: 0x03,
            dg: 0x00,
            db: 0x01,
        });

        let result = parse_diff(&bytes).unwrap();

        assert_eq!(
            result.1,
            chunk
        );
    }

    #[test]
    fn test_luma_parser() {
        let bytes = [0b10001000, 0b11110001];
        let chunk = CHUNK::LumaChunk(LumaChunk {
            diff_g: 0x08,
            dr_minus_dg: 0x0F,
            db_minus_dg: 0x01,
        });

        let result = parse_luma(&bytes).unwrap();

        assert_eq!(
            result.1,
            chunk
        );
    }

    #[test]
    fn test_run_parser() {
        let bytes = [0b11000011];
        let chunk = CHUNK::RunChunk(RunChunk {
            run: 0x03,
        });

        let result = parse_run(&bytes).unwrap();

        assert_eq!(
            result.1,
            chunk
        );
    }
}
