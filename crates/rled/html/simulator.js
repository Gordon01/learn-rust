var screens_ctl;    // Селектор экрана
var screens_info;   // Данные обо всех экранах
var canvas_scaled = false;  // Если true то canvas уже отмасштабирован
const scaling_factor = 6; // Коэффициент масштабирования для отрисовки экрана

window.onload = init;

function init() {
    screens_ctl = document.getElementById("screen");
    screens_ctl.addEventListener("change", draw_screen);
    populate_selector(screens_ctl);

    var eventSource = new EventSource("/api/events");
    eventSource.onmessage = function (event) {
        var pixel_event = JSON.parse(event.data);
        if (pixel_event.id != screens_info[screens_ctl.value].id.text) {
            // Обновляем только пиксели выбранного в данный момент экрана
            return;
        }

        const canvas = document.getElementById("pixels");
        const ctx = canvas.getContext("2d");
        let r = pixel_event.pixel.color[0];
        let g = pixel_event.pixel.color[1];
        let b = pixel_event.pixel.color[2];
        ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;
        ctx.fillRect(pixel_event.pixel.x, pixel_event.pixel.y, 1, 1);
    }
}

function populate_selector(screens) {
    var request = new XMLHttpRequest();
    request.open('GET', '/api/screens', true);
    request.onload = function () {
        if (request.status >= 200 && request.status < 400) {
            screens_info = JSON.parse(request.responseText);
            // Заполняем селектор списком экранов, которые получили с сервера
            screens_info.map((screen, i) => {
                let opt = document.createElement("option");
                opt.value = i;
                opt.innerHTML = screen.id.text;
                screens.append(opt);
            });
        } else {
            console.log("Не удалось получить список экранов")
        }
    };
    request.send();
}

function draw_screen() {
    const canvas = document.getElementById("pixels");
    const ctx = canvas.getContext("2d");
    // Очищаем canvas на случай если предыдущий экран был больше по размерам
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    if (!canvas_scaled) {
        // Пусть она занимает все оставшееся место
        ctx.canvas.width = window.innerWidth;
        ctx.canvas.height = window.innerHeight;
        ctx.scale(scaling_factor, scaling_factor);
        canvas_scaled = true;
    }
    
    var screen = screens_info[screens_ctl.value];
    var request = new XMLHttpRequest();
    request.open('GET', `/api/image/${screen.id.text}`, true);
    request.onload = function () {
        if (request.status >= 200 && request.status < 400) {
            var image_data = JSON.parse(request.responseText);
            // Отрисовываем всю картинку построчно
            for (let y = 0; y < screen.height; y += 1) {
                for (let x = 0; x < screen.width; x += 1) {
                    let offset = (x + y * screen.width) * 3;
                    let r = image_data[offset];
                    let g = image_data[offset + 1];
                    let b = image_data[offset + 2];
                    ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;
                    ctx.fillRect(x, y, 1, 1);
                }
            }
        } else {
            console.log("Не удалось получить значения пикселей экрана")
        }
    };
    request.send();


}