document.addEventListener("DOMContentLoaded", () => {
    const input = document.getElementById("input");
    const sendButton = document.getElementById("send");
    const responseDisplay = document.getElementById("response");

    const socket = new WebSocket("ws://127.0.0.1:3000/ws");

    // Event listener for the 'open' event
    socket.addEventListener("open", () => {
        console.log("Connected to WebSocket server");
    });

    // Event listener for the 'message' event
    socket.addEventListener("message", (event) => {
        const response = event.data;
        responseDisplay.textContent = response;
    });

    // Event listener for the 'close' event
    socket.addEventListener("close", () => {
        console.log("Disconnected from WebSocket server");
    });

    // Event listener for the 'error' event
    socket.addEventListener("error", (error) => {
        console.error("WebSocket error", error);
    });

    // Event listener for the 'click' event on the send button
    sendButton.addEventListener("click", () => {
        const message = input.value.trim();
        if (message) {
            socket.send(message);
            input.value = ""; // Clear the input field after sending
        } else {
            console.log("Input is empty. Please enter a message.");
        }
    });
});
