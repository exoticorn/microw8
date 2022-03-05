import MicroW8 from './microw8.js';

function setMessage(size, error) {
    let html = size ? `${size} bytes` : 'Insert cart';
    if (error) {
        html += ` - <span class="error">${error.replaceAll('<', '&lt;')}</span>`
    }
    document.getElementById('message').innerHTML = html;
}

let uw8 = MicroW8(document.getElementById('screen'), {
    setMessage,
    keyboardElement: window,
    timerElement: document.getElementById("timer"),
});

function runModuleFromHash() {
    let hash = window.location.hash.slice(1);
    if(hash == 'devkit') {
        uw8.setDevkitMode(true);
        return;
    }
    uw8.setDevkitMode(false);
    if (hash.length > 0) {
        if (hash.startsWith("url=")) {
            uw8.runModuleFromURL(hash.slice(4), true);
        } else {
            uw8.runModuleFromURL('data:;base64,' + hash);
        }
    } else {
        uw8.runModule(new ArrayBuffer(0));
    }
}

window.onhashchange = runModuleFromHash;

let setupLoad = () => {
    let loadButton = document.getElementById('cartButton');
    loadButton.style = '';
    loadButton.onclick = () => {
        let fileInput = document.createElement('input');
        fileInput.type = 'file';
        fileInput.accept = '.wasm,.uw8,application/wasm';
        fileInput.onchange = () => {
            if (fileInput.files.length > 0) {
                uw8.runModuleFromURL(URL.createObjectURL(fileInput.files[0]));
            }
        };
        fileInput.click();
    };
    
    screen.ondragover = (e) => {
        e.preventDefault();
    };
    
    screen.ondrop = (e) => {
        let files = e.dataTransfer && e.dataTransfer.files;
        if(files && files.length == 1) {
            e.preventDefault();
            uw8.runModuleFromURL(URL.createObjectURL(e.dataTransfer.files[0]));
        }
    }

    runModuleFromHash();
};

let location = window.location;
if(location.hash.length != 0) {
    setupLoad();
} else {
    (async () => {
        let url = location.href;
        if(url.endsWith('.html')) {
            url = url.slice(0, url.length - 4) + 'uw8';
        } else {
            if(!url.endsWith('/')) {
                url += '/';
            }
            url += 'cart.uw8';
        }
        try {
            if(!await uw8.runModuleFromURL(url, true)) {
                setupLoad();
            }
        } catch(e) {
            setupLoad();
        }
    })();
}

