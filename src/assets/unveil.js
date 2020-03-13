let current_slide = 0;
let current_bg = "blue";


window.document.addEventListener("keydown", e => {
    if(e.key === "ArrowLeft") {
        next_slide_left();
    } else if (e.key === "ArrowRight") {
        next_slide_right();
    }
});

function get_slide(idx) {
    return document.getElementById("unveil-slide-" + idx);
}

function next_slide_right() {
    let slide = get_slide(current_slide + 1);
    if (slide) {
        current_slide++;
        slide.scrollIntoView(true);
    }
}

function next_slide_left() {
    if (current_slide >= 1) {
        current_slide--;
        let slide = get_slide(current_slide);
        slide.scrollIntoView(true);
    }
}

function scroll_changed() {
    // TODO
}