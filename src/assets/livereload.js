// The ws instantiation is made in `src/unveil.rs`
socket.onmessage = function (event) {
    if (event.data === "reload") {
        socket.close();
        location.reload();
    }
};

window.onbeforeunload = function () {
    socket.close();
};