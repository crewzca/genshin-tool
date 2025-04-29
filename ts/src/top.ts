function changeDetail(idx: string) {
  const tableOff = document.querySelectorAll<HTMLTableElement>('.status-table-all');
  tableOff.forEach((ele) => {
    ele.classList.remove('appear');
    ele.classList.add('hide');
  });

  const tableOn = document.getElementById("status-table" + idx) as HTMLTableElement;
  tableOn.classList.remove('hide');
  tableOn.classList.add('appear');
}

(Window as any).changeDetail = changeDetail;