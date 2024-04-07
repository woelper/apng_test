use std::{fs::File, io::Read};

use zune_png::{post_process_image, PngDecoder};
use zune_core::options::EncoderOptions;
fn main() {


    let apng_path = std::env::args().into_iter().nth(1).expect("You must pass an image as argument.");
    println!("Opening {apng_path}");
    let data = std::fs::read(apng_path).unwrap();

    // read the file
    // set up decoder
    let mut decoder = PngDecoder::new(&data);
    // decode headers
    decoder.decode_headers().unwrap();
    // get useful information about the image
    let colorspace = decoder.get_colorspace().unwrap();
    let depth = decoder.get_depth().unwrap();
    //  get decoder information,we clone this because we need a standalone
    // info since we mutably modify decoder struct below
    let info = decoder.get_info().unwrap().clone();
    // set up our background variable. Soon it will contain the data for the previous
    // frame, the first frame has no background hence why this is None
    let mut background: Option<Vec<u8>> = None;
    // the output, since we know that no frame will be bigger than the width and height, we can
    // set this up outside of the loop.
    let mut output =
        vec![0; info.width * info.height * decoder.get_colorspace().unwrap().num_components()];
    let mut i = 0;

    while decoder.more_frames() {
        // decode the header, in case we haven't processed a frame header
        decoder.decode_headers().unwrap();
        // then decode the current frame information,
        // NB: Frame information is for current frame hence should be accessed before decoding the frame
        // as it will change on subsequent frames
        let frame = decoder.frame_info().unwrap();
        // decode the raw pixels, even on smaller frames, we only allocate frame_info.width*frame_info.height
        let pix = decoder.decode_raw().unwrap();
        // call post process
        post_process_image(
            &info,
            colorspace,
            &frame,
            &pix,
            background.as_ref().map(|x| x.as_slice()),
            &mut output,
            None,
        )
        .unwrap();
        // create encoder parameters
        let encoder_opts = EncoderOptions::new(info.width, info.height, colorspace, depth);

        let bytes = zune_png::PngEncoder::new(&output, encoder_opts).encode();

        std::fs::write(format!("./{i}.png"), bytes).unwrap();
        // this is expensive, but we need a copy of the previous fully rendered frame
        // we can alleviate this since we are using the same output, so DisposeOP::None will always be the
        // same as DisposeOp::Previous, but only works for this example.
        // in case you reuse the same buffer per invocation,
        // always have your background as None
        background = Some(output.clone());
        i += 1;
    }
}
