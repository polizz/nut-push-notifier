const socket = new WebSocket(`ws://${location.host}/ws`);

socket.addEventListener('message', function(event) {
    const status_display = document.getElementById('status_display');
    const new_status = document.createElement('p');

    let data = event.data.toString()
    if (data.toLowerCase().indexOf("online") > -1) {
        new_status.className = 'online';
    }

    new_status.innerText = `${data} - ${new Date().toLocaleString()}`;
    status_display.appendChild(new_status);
});

