let current_slide = 0;

hljs.configure({
    tabReplace: '    ',
});

window.onresize = () => {
    const slide = get_slide(current_slide);
    slide.scrollIntoView();
};

window.onload = () => {
    Array
        .from(document.querySelectorAll('code'))
        .forEach(block => hljs.highlightBlock(block));
};

window.document.addEventListener("keydown", e => {
    if (e.key === "ArrowLeft") {
        next_slide_left();
    } else if (e.key === "ArrowRight") {
        next_slide_right();
    }
});

const get_slide = (idx) => {
    return document.getElementById("unveil-slide-" + idx);
};

const next_slide_right = () => {
    let slide = get_slide(current_slide + 1);
    if (slide) {
        current_slide++;
        smooth_scroll(slide);
    }
};

const next_slide_left = () => {
    if (current_slide >= 1) {
        current_slide--;
        let slide = get_slide(current_slide);
        smooth_scroll(slide);
    }
};

const smooth_scroll = (el) => {
    el.scrollIntoView({behavior: "smooth"});
};


const timeout = (promise) => {
    return new Promise((resolve, reject) => {
        setTimeout(() => reject(new Error("timeout")), 6000);
        promise.then(resolve, reject)
    })
};

const add_playpen_button = () => {
    document.getElementsByName('pre')
        .forEach(el => {
            el.ad
        })
}
const fetch_with_timeout = (url) => {
    const params = {
        code: "pub fn main() { println!(\" hello \")}",
        edition: "2018",
        channel: "stable",
        mode: "debug",
        backtrace: false,
        tests: false,
        crateType: "bin"
    };

    const fetch_playpen = fetch("https://play.integer32.com/execute",
        {
            headers: {
                'Content-Type': "application/json",
            },
            method: 'POST',
            mode: 'cors',
            body: JSON.stringify(params)
        }
    );

    timeout(fetch_playpen)
        .catch(error => console.log(error))
        .then(response => response.json().then(json => {
            console.log(json)
        }));
};