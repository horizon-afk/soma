use screenshots::Screen;



fn screensnap() {
    
    let start = Instant::now();
    //selects the screen from where the pixels are defined
    //TODO: no implementation for multiple monitors yet
    let screen = Screen::from_point(100, 100).unwrap();


    // assume the initial coords to be (x1,y1)
    // assume end coords to be (x2,y2)
    // therefore the width is given by abs(x1-x2) and height by abs(y1-y2)

    let image = screen.capture_area(300, 300, 300, 300).unwrap();

    image.save("target/capture_display_with_point.png").unwrap();
}