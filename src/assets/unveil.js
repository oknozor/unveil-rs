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

const clipboard = (id) => {
    let code_block = document.getElementById(id).parentElement.parentElement;
    let old = document.getElementById('dummy_clipboard');

    if(old) {
        code_block.removeChild(old);
    }

    let dummy_textarea = document.createElement('textarea');
    dummy_textarea.style.display = 'none';
    dummy_textarea.id = 'dummy_clipboard';
    code_block.appendChild(dummy_textarea);
    console.log(code_block.innerText);
    dummy_textarea.value = code_block.innerText;
    dummy_textarea.select();

    document.execCommand("copy");
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
        .catch(error => console.log(result.innerText = error.message))
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