export function closeFlash(receiver) {
    receiver.parentElement.style.display = "none";
}

export function enableField(id: string) {
    document.getElementById(id).removeAttribute("disabled");
}

export function disableField(id: string) {
    document.getElementById(id).setAttribute("disabled", "disabled");
}
