use crate::qoidecoder::parsechunks::*;
use rgb::RGBA;
use nom::{
    multi:: many_till,
    bytes::complete::tag,
    IResult,
    sequence::tuple,
};


const PREV_PIXEL_ARRAY_LEN: usize = 64;

pub fn get_end(input: &[u8]) -> IResult<&[u8], ()> {
    let (i, _remainder) = tag([0, 0, 0, 0, 0, 0, 0, 1])(input)?;
    Ok((i, ()))
}

pub fn parse_all_chunks(bytes: &[u8]) -> IResult<&[u8], (QOIHeader, Vec<CHUNK>)> {
    let (i, (header, (chunks, _))) = tuple((parse_header, many_till(parse_chunks, get_end)))(bytes)?;

    Ok((i, (header, chunks)))
}

#[allow(unused_variables, dead_code, unused_mut)]
pub fn get_pixels(input: &[u8]) -> Vec<RGBA<u8>> {
    // let (i, header) = parse_header(input).expect("Invalid header");
    // let img_width = header.width;
    // let img_height = header.height;
    // let num_pixels = img_height * img_height;

    // let mut pixels: Vec<RGBA<u8>> = Vec::with_capacity(num_pixels as usize);
    // let mut run_prev_pixels: Vec<RGBA<u8>> = vec![RGBA { r: 0x00, g: 0x00, b: 0x00, a: 0x00 }; PREV_PIXEL_ARRAY_LEN];
    // let mut prev_pixel = RGBA {
    //     r: 0x00,
    //     g: 0x00,
    //     b: 0x00,
    //     a: 0xFF,
    // };

    let res = crate::qoidecoder::readimg::parse_all_chunks(input);
    let (_, (header, chunks)) = res.expect("Failed to unwrap res in get_pixels()");

    let num_pixels = header.width * header.height;
    let mut pixels: Vec<RGBA<u8>> = Vec::new();
    let mut run_prev_pixels: Vec<RGBA<u8>> = vec![RGBA { r: 0x00, g: 0x00, b: 0x00, a: 0x00 }; PREV_PIXEL_ARRAY_LEN];
    let mut prev_pixel = RGBA {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    };

    let chunks_iter = chunks.iter();
    
    for i in chunks_iter {
        match i {
            CHUNK::RGBChunk(ch) => {
                pixels.push(RGBA::new(ch.r, ch.g, ch.b, prev_pixel.a));
            }
            CHUNK::RGBAChunk(ch) => {
                pixels.push(RGBA::new(ch.r, ch.g, ch.b, ch.a));
            }
            CHUNK::IndexChunk(ch) => {
                let pix = run_prev_pixels[ch.index as usize];
                pixels.push(pix);
            }
            CHUNK::DiffChunk(ch) => {
                const BIAS: i16 = 2;
                let prev_r_signed: i16 = prev_pixel.r as i16;
                let prev_g_signed: i16 = prev_pixel.g as i16;
                let prev_b_signed: i16 = prev_pixel.b as i16;
                let dr_signed: i16 = ch.dr as i16;
                let dg_signed: i16 = ch.dg as i16;
                let db_signed: i16 = ch.db as i16;

                let r = ((prev_r_signed + dr_signed - BIAS) % 255) as u8;
                let g = ((prev_g_signed + dg_signed - BIAS) % 255) as u8;
                let b = ((prev_b_signed + db_signed - BIAS) % 255) as u8;

                pixels.push(RGBA::new(
                    r,
                    g,
                    b,
                    prev_pixel.a,
                ));
            }
            CHUNK::LumaChunk(ch) => {
                const G_BIAS: i16 = 32;
                const R_B_BIAS: i16 = 8;
                let prev_r_signed: i16 = prev_pixel.r as i16;
                let prev_g_signed: i16 = prev_pixel.g as i16;
                let prev_b_signed: i16 = prev_pixel.b as i16;
                let dg_signed: i16 = ch.diff_g as i16 - G_BIAS;
                let dr_dg_signed: i16 = ch.dr_minus_dg as i16 - R_B_BIAS;
                let db_dg_signed: i16 = ch.db_minus_dg as i16 - R_B_BIAS;

                let g = ((dg_signed + prev_g_signed) % 255) as u8;
                let r = ((dr_dg_signed + prev_r_signed + g as i16 - prev_g_signed) % 255) as u8;
                let b = ((db_dg_signed + prev_b_signed + g as i16 - prev_g_signed) % 255) as u8;

                pixels.push(RGBA::new(
                    r,
                    g,
                    b,
                    prev_pixel.a,
                ));
            }
            CHUNK::RunChunk(ch) => {
                const RUN_BIAS: u8 = 1; // add +ve bias instead of subtracting -ve bias
                let run_length = ch.run + 1;
                for i in 0..run_length {
                    pixels.push(RGBA::new(
                        prev_pixel.r,
                        prev_pixel.g,
                        prev_pixel.b,
                        prev_pixel.a,
                    ));
                }
            }
        } // end of match

       prev_pixel = pixels[pixels.len() - 1];
       let (r, g, b, a) = (prev_pixel.r as usize, prev_pixel.g as usize, prev_pixel.b as usize, prev_pixel.a as usize);
       let index_position = (r * 3 + g * 5 + b * 7 + a * 11) % 64;
       run_prev_pixels[index_position as usize] = prev_pixel;
    }

    pixels
}