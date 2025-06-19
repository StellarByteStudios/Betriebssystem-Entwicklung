use usrlib::gprintln;
use usrlib::graphix::picturepainting::animate::Frame;
use usrlib::graphix::picturepainting::paint::draw_picture;
use usrlib::utility::delay::delay;

pub fn animate_blue_flame(x: u32, y: u32){
    // Bilder laden
    let frames: [Frame; 60] = [

        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame000::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame000::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame000::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame000::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame001::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame001::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame001::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame001::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame002::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame002::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame002::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame002::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame003::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame003::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame003::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame003::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame004::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame004::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame004::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame004::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame005::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame005::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame005::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame005::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame006::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame006::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame006::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame006::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame007::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame007::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame007::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame007::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame008::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame008::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame008::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame008::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame009::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame009::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame009::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame009::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame010::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame010::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame010::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame010::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame011::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame011::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame011::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame011::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame012::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame012::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame012::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame012::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame013::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame013::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame013::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame013::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame014::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame014::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame014::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame014::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame015::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame015::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame015::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame015::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame016::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame016::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame016::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame016::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame017::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame017::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame017::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame017::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame018::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame018::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame018::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame018::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame019::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame019::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame019::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame019::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame020::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame020::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame020::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame020::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame021::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame021::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame021::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame021::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame022::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame022::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame022::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame022::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame023::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame023::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame023::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame023::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame024::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame024::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame024::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame024::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame025::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame025::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame025::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame025::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame026::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame026::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame026::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame026::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame027::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame027::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame027::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame027::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame028::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame028::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame028::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame028::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame029::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame029::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame029::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame029::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame030::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame030::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame030::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame030::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame031::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame031::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame031::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame031::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame032::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame032::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame032::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame032::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame033::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame033::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame033::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame033::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame034::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame034::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame034::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame034::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame035::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame035::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame035::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame035::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame036::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame036::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame036::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame036::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame037::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame037::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame037::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame037::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame038::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame038::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame038::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame038::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame039::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame039::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame039::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame039::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame040::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame040::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame040::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame040::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame041::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame041::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame041::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame041::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame042::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame042::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame042::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame042::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame043::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame043::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame043::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame043::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame044::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame044::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame044::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame044::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame045::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame045::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame045::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame045::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame046::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame046::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame046::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame046::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame047::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame047::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame047::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame047::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame048::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame048::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame048::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame048::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame049::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame049::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame049::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame049::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame050::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame050::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame050::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame050::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame051::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame051::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame051::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame051::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame052::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame052::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame052::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame052::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame053::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame053::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame053::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame053::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame054::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame054::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame054::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame054::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame055::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame055::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame055::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame055::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame056::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame056::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame056::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame056::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame057::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame057::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame057::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame057::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame058::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame058::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame058::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame058::DATA.to_vec(),
        },
        Frame {
            width: crate::custom_animations::blue_flame::BlueFlame059::WIDTH,
            height:crate::custom_animations::blue_flame::BlueFlame059::HEIGHT,
            bpp: crate::custom_animations::blue_flame::BlueFlame059::BPP,
            data: crate::custom_animations::blue_flame::BlueFlame059::DATA.to_vec(),
        },
    ];

    // Frames nacheinander zeichnen
    gprintln!("animating Blue Flame");
    loop {
        for i in 0..frames.len() {
            draw_picture(x as usize, y as usize, &frames[i]);

            delay(1);
        }
    }
}
