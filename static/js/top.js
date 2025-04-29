"use strict";
function changeDetail(idx) {
    const tableOff = document.querySelectorAll('.status-table-all');
    tableOff.forEach((ele) => {
        ele.classList.remove('appear');
        ele.classList.add('hide');
    });
    const tableOn = document.getElementById("status-table" + idx);
    tableOn.classList.remove('hide');
    tableOn.classList.add('appear');
}
Window.changeDetail = changeDetail;
