class UseAudioWorkletInRustProcessor extends AudioWorkletProcessor {
    constructor() {
        super();
    }

    process(inputs, outputs, parameters) {
        // inputs[0] = 代表所有的输入源，可能是麦克风，或者其他输入源
        // inputs[0][0] = 每个输入源的通道数，比如，一个左声道，一个右声道
        // inputs[0][1] = 每个输入源的通道数，比如一个左声道，一个右声道
        // console.log("input", inputs);
        this.port.postMessage("daimengen");
        return true;
    }
}

registerProcessor("use-audio-worklet-in-rust-processor", UseAudioWorkletInRustProcessor);