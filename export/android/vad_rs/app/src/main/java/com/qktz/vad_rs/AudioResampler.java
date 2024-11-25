package com.qktz.vad_rs;

public class AudioResampler {
    public short[] resample(short[] input, int inputLength, int inputSampleRate, int outputSampleRate) {
        if (inputSampleRate == outputSampleRate) {
            return input;
        }
        
        int outputLength = (int)((long)inputLength * outputSampleRate / inputSampleRate);
        short[] output = new short[outputLength];
        
        double stepSize = (double)inputSampleRate / outputSampleRate;
        for (int i = 0; i < outputLength; i++) {
            int inputIndex = (int)(i * stepSize);
            if (inputIndex < inputLength) {
                output[i] = input[inputIndex];
            }
        }
        
        return output;
    }
}