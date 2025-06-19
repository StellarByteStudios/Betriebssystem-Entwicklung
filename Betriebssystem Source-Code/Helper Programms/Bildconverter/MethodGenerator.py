prepath = "crate::custom_animations::blue_flame::BlueFlame0"
method_name = "animate_blue_flame"
animation_name = "Blue Flame"
framecount = 60


def main():
    # Methodenkopf
    print(f"pub fn {method_name}(x: u32, y: u32){{")

    # Frame Anfang
    print(f"\t// Bilder laden\n\tlet frames: [Frame; {framecount}] = [")

    # Konstrukt für jeden Frame
    for i in range(0, framecount):
        frameblock = f"""
                Frame {{
                    width: {prepath}{i:02d}::WIDTH,
                    height:{prepath}{i:02d}::HEIGHT,
                    bpp: {prepath}{i:02d}::BPP,
                    data: {prepath}{i:02d}::DATA.to_vec(),
                }},"""
        print(frameblock, end='')


    # Abschließen
    endblock = f"""
        ];

        // Frames nacheinander zeichnen
        gprintln!("animating {animation_name}");
        loop {{
            for i in 0..frames.len() {{
                draw_picture(x as usize, y as usize, &frames[i]);

                delay(25);
            }}
        }}
    }}
    """
    print(endblock, end='')


    print("finished")



if __name__ == "__main__":
    main()
