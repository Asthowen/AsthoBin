document.getElementById('saveButton').onclick = async () => {
    const content = document.getElementById('codeArea').value;
    if (content.trim() !== ''){
        let headers = new Headers();
        let init = { method: 'POST', headers: headers, mode: 'cors', cache: 'default', body: content, redirect: 'follow' };
        const response = await fetch('/new', init);
        const responseContent = await response.json();
        if (responseContent !== null && responseContent.hasOwnProperty('status') && responseContent['status'] === 'error'){
            // TODO
        } else {
            history.pushState({}, null, response.headers.get('Location'));
            window.location.reload();
        }
    }
}
document.getElementById('codeArea').focus();