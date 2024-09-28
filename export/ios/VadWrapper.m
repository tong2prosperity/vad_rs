#import "VadWrapper.h"

// 声明外部的Rust函数
extern void *init_silero_apple(void);
extern float process_audio_apple(const int16_t *audio_data, NSUInteger audio_len);

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

@end