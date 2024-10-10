#import <Foundation/Foundation.h>
#include <stdint.h>

typedef struct {
    int talk_state;
    int err_code;
} VadRes;

@interface RustWrapper : NSObject

- (instancetype)init;
- (void *)initSilero;
- (float)processAudio:(const int16_t *)audioData length:(NSUInteger)length;
- (long)initVadIter:(NSString *)paramStr;
- (VadRes)processVadIter:(long)handle audioData:(const int16_t *)audioData length:(NSUInteger)length;
- (void)cleanupVadIter:(long)handle;

@end