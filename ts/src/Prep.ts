import { doOnDOMReady } from './Internal'

const FETCH_TIMEOUT = 500;

let timeoutHandle: number;


export function checkPartner(receiver: HTMLInputElement) {
    function setIcon(name: string, second?: string) {
        let iconContainer = receiver.parentElement.firstElementChild.firstElementChild;
        iconContainer.classList.remove('fa-user');
        iconContainer.classList.remove('fa-check-square-o');
        iconContainer.classList.remove('fa-exclamation-triangle');
        iconContainer.classList.remove('fa-refresh');
        iconContainer.classList.remove('fa-spin');
        iconContainer.classList.add(name);
        if (second) {
            iconContainer.classList.add(second);
        }
    }

    function checkPartnerImpl() {
        // Set icon to spinning and reset field classes
        setIcon('fa-refresh', 'fa-spin');
        receiver.classList.remove('c-field--success');
        receiver.classList.remove('c-field--error');

        // Fetch the user with the given username from the API
        let url = "/api/user/by_username/" + receiver.value;
        let opts: RequestInit = {
            credentials: 'same-origin'
        };

        fetch(url, opts)
            .then(response => response.json())
            .then(json => {
                // Check if the response returned ok, if data is there and if
                // the user is actually a student.
                if (json.ok === true && json.data && json.data.role == 'Student') {
                    receiver.classList.add('c-field--success');
                    setIcon('fa-check-square-o');
                } else {
                    receiver.classList.add('c-field--error');
                    setIcon('fa-exclamation-triangle');
                }
            })
            .catch(ex => {
                receiver.classList.add('c-field--error');
                setIcon('fa-exclamation-triangle');

                console.log('invalid response');
            })
    }

    // We want to cancel any job waiting for execution
    window.clearTimeout(timeoutHandle);

    // If the input is empty, we reset the icon and nothing more.
    if (!receiver.value) {
        setIcon('fa-user');
        return;
    }

    // Start the actual job (except if it is cancelled later).
    timeoutHandle = window.setTimeout(checkPartnerImpl, FETCH_TIMEOUT);
}
