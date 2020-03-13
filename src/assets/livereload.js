let socket = new WebSocket("ws://localhost:3000");

socket.onmessage = function (event) {
    if (event.data === "reload") {
        socket.close();
        location.reload();
    }
};

window.onbeforeunload = function () {
    socket.close();
};