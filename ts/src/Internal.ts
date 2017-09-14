export function doOnDOMReady(func: () => void): void {
    if (document.readyState == "loading") {
        document.addEventListener("DOMContentLoaded", func);
    } else {
        func();
    }
}
