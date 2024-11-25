package com.qktz.kimivad;

public class KimiVad {

    static {
        System.loadLibrary("vad_rs");
    }

    public static native long init_vad_iter(String params);
    public static native long process_vad_iter(long handle, byte[] audioData);
    
}
