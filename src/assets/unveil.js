let current_slide = 0;

hljs.configure({
    tabReplace: '    ',
    languages: []
});

window.onresize = () => {
    const slide = get_slide(current_slide);
    slide.scrollIntoView();
};

window.onload = () => {
    // Highlight code blocks
    Array
        .from(document.querySelectorAll('code'))
        .forEach(block => hljs.highlightBlock(block));

    // Add ClipBoardJS attr to clipboard buttons
    let clip_buttons = document.querySelectorAll('.btn-copy');
    clip_buttons.forEach(clip_button => {
        const code_block = clip_button.parentElement.parentElement;
        const code = code_block.innerText;
        clip_button.setAttribute("data-clipboard-text", code);
    });
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
    let curr_slide = get_slide(current_slide);
    if (slide) {
        current_slide++;
        smooth_scroll(slide);
        zoom_transition(curr_slide, "out");
        zoom_transition(slide, "in");
    }
};

const next_slide_left = () => {
    let curr_slide = get_slide(current_slide);
    if (current_slide >= 1) {
        current_slide--;
        let slide = get_slide(current_slide);
        smooth_scroll(slide);
        zoom_transition(curr_slide, "out");
        zoom_transition(slide, "in");
    }
};

const smooth_scroll = (el) => {
    el.scrollIntoView({behavior: "smooth"});
};

const zoom_transition = (el, mode) => {
    if (mode === "in") {
        el.classList.add('zoom-in');
        el.addEventListener('transitionend', function(){
            el.classList.remove('zoom-in');
        }, false);
    } else {
        el.classList.add('zoom-out');
        el.addEventListener('transitionend', function(){
            el.classList.remove('zoom-out');
        }, false); 
    }
};

const fade_transition = (el, mode) => {
    if (mode === "in") {
        el.classList.add('fade-in');
        el.addEventListener('animationend', function(){
            el.classList.remove('fade-in');
            console.log('test');
        }, false);
    } else {
        el.classList.add('fade-out');
        el.addEventListener('animationend', function(){
            el.classList.remove('fade-out');
            console.log('test');
        }, false); 
    }
};

const timeout = (promise) => {
    return new Promise((resolve, reject) => {
        setTimeout(() => reject(new Error("timeout")), 6000);
        promise.then(resolve, reject)
    })
};

const play_playpen = (id) => {
    let play_button = window.document.getElementById(id);
    let result = play_button.querySelector('.result');
    let code_block = play_button.parentElement.parentElement.querySelector('code');

    if (!result) {
        result = document.createElement('code');
        result.className = 'result hljs language-bash';
        code_block.append(result);
    }

    let code_text = code_block.textContent;

    fetch_with_timeout(code_text, "https://play.integer32.com/execute")
        .catch(error => result.innerText = error.message)
        .then(response => response.json()).then(json => {
        result.innerText = json.stderr + '\n' + json.stdout
    })
};

const fetch_with_timeout = (code, url) => {
    const params = {
        code,
        edition: "2018",
        channel: "stable",
        mode: "debug",
        backtrace: false,
        tests: false,
        crateType: "bin"
    };

    const fetch_playpen = fetch(url,
        {
            headers: {
                'Content-Type': "application/json",
            },
            method: 'POST',
            mode: 'cors',
            body: JSON.stringify(params)
        }
    );

    return timeout(fetch_playpen)
};
<<<<<<< HEAD

let clipboard = new ClipboardJS('.btn-copy');

clipboard.on('success', function(e) {
    e.trigger.classList.add("bounce-in-active");
    setTimeout(() => e.trigger.classList.remove("bounce-in-active"), 300);
    e.clearSelection();
});
=======
>>>>>>> feat: added CSS animation for slide transition
