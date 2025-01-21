self.onmessage = async event => {
    self.postMessage(event.data);
}