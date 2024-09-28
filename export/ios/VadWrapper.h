#import <Foundation/Foundation.h>

@interface RustWrapper : NSObject

- (instancetype)init;
- (void *)initSilero;
- (float)processAudio:(const int16_t *)audioData length:(NSUInteger)length;

@end