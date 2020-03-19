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
    let curr_slide = get_slide(current_slide);
    let right = get_slide(current_slide + 1);
    if (right) {
        current_slide++;
        transition_and_scroll(curr_slide, right)
    }
};

const next_slide_left = () => {
    let curr_slide = get_slide(current_slide);
    let left = get_slide(current_slide - 1);
    if (left) {
        current_slide--;
        transition_and_scroll(curr_slide, left);
    }
};

const transition_and_scroll = (current_slide, target_slide) => {
    let transition_kind = getComputedStyle(target_slide).getPropertyValue("--on-enter-animation").trim();
    if (transition_kind) {
        // Second handler the new slide has finished transitioning
        const target_animation_end_handler = () => {
            console.log('target end');
            // clean up
            target_slide.removeEventListener('animationend', target_animation_end_handler);
            target_slide.classList.remove(transition_kind);
        };

        target_slide.addEventListener('animationend', target_animation_end_handler);

        target_slide.scrollIntoView({behavior: "smooth"});
        target_slide.classList.add(transition_kind);

    } else { // No transition animation
        target_slide.scrollIntoView({behavior: "smooth"});
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

let clipboard = new ClipboardJS('.btn-copy');

clipboard.on('success', function (e) {
    e.trigger.classList.add("bounce-in-active");
    setTimeout(() => e.trigger.classList.remove("bounce-in-active"), 300);
    e.clearSelection();
});
