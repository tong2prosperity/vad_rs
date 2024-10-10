#import "VadWrapper.h"

// 声明外部的Rust函数
extern void *init_silero_apple(void);
extern float process_audio_apple(const int16_t *audio_data, NSUInteger audio_len);
extern long init_vad_iter_apple(const char *paramStr);
extern VadRes process_vad_iter_apple(long handle, const int16_t *audio_data, NSUInteger audio_len);
extern void cleanup_vad_iter_apple(long handle);

@implementation RustWrapper

- (instancetype)init {
    self = [super init];
    if (self) {
        // 初始化代码
    }
    return self;
}

- (void *)initSilero {
    return init_silero_apple();
}

- (float)processAudio:(const int16_t *)audioData length:(NSUInteger)length {
    return process_audio_apple(audioData, length);
}

- (long)initVadIter:(NSString *)paramStr {
    const char *cString = [paramStr UTF8String];
    return init_vad_iter_apple(cString);
}

- (VadRes)processVadIter:(long)handle audioData:(const int16_t *)audioData length:(NSUInteger)length {
    return process_vad_iter_apple(handle, audioData, length);
}

- (void)cleanupVadIter:(long)handle {
    cleanup_vad_iter_apple(handle);
}
@end