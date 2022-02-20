import MicroW8 from './microw8.js';

let uw8 = MicroW8(document.getElementById('screen'));
let events = new EventSource('events');
events.onmessage = event => {
    console.log(event.data);
    if(event.data == 'L') {
        uw8.runModuleFromURL('cart', true);
    }
};
uw8.runModuleFromURL('cart', true);
